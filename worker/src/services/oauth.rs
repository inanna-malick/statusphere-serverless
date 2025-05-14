use crate::storage::kv::KvStoreWrapper;
use crate::types::errors::AppError;

use atrium_api::types::string::{Did, Handle};
use atrium_oauth::{
    AtprotoClientMetadata, AtprotoLocalhostClientMetadata, AuthorizeOptions, CallbackParams,
    DefaultHttpClient, GrantType, KnownScope, OAuthClient as AtriumOAuthClient, OAuthClientConfig,
    OAuthClientMetadata, OAuthResolverConfig, Scope,
};
use std::{sync::Arc, time::Duration};

use super::agent::Agent;
use super::resolvers;
use crate::storage::kv::{KvSessionStore, KvStateStore};
use anyhow::anyhow;
use atrium_api::agent::Agent as AtriumAgent;

pub type ClientType = AtriumOAuthClient<
    KvStateStore,
    KvSessionStore,
    resolvers::DidResolver,
    resolvers::HandleResolver,
>;

pub type SessionType = atrium_oauth::OAuthSession<
    DefaultHttpClient,
    resolvers::DidResolver,
    resolvers::HandleResolver,
    KvStoreWrapper<Did, atrium_oauth::store::session::Session>,
>;

const OAUTH_STORE_TTL: Duration = Duration::new(60 * 60 * 24 * 30, 0);

#[derive(Clone)]
pub struct OAuthClient {
    client: Arc<ClientType>,
}

impl OAuthClient {
    pub async fn restore_session(&self, did: &Did) -> Result<Agent, AppError> {
        let session = self.client.restore(did).await?;

        Ok(Agent::from_session(session, did.clone()))
    }

    pub async fn callback(&self, params: CallbackParams) -> Result<Did, AppError> {
        let (bsky_session, _) = self.client.callback(params).await?;
        let agent = AtriumAgent::new(bsky_session);

        let did = agent.did().await.ok_or(anyhow!(
            "The OAuth agent didn't return a DID. May try re-logging in?"
        ))?;

        Ok(did)
    }

    pub fn client_metadata(&self) -> OAuthClientMetadata {
        self.client.client_metadata.clone()
    }

    pub async fn auth_redirect_url(&self, handle: Handle) -> Result<String, AppError> {
        let auth_url = self
            .client
            .authorize(
                handle,
                AuthorizeOptions {
                    scopes: vec![
                        Scope::Known(KnownScope::Atproto),
                        Scope::Known(KnownScope::TransitionGeneric),
                    ],
                    ..Default::default()
                },
            )
            .await?;

        Ok(auth_url)
    }

    pub fn new(url: String, kv: &Arc<worker::kv::KvStore>) -> anyhow::Result<Self> {
        let http_client = Arc::new(DefaultHttpClient::default());

        let resolver = OAuthResolverConfig {
            did_resolver: resolvers::did_resolver(&http_client, kv),
            handle_resolver: resolvers::handle_resolver(&http_client, kv),
            authorization_server_metadata: Default::default(),
            protected_resource_metadata: Default::default(),
        };

        let state_store = KvStoreWrapper::new(kv.clone(), "oauth:state", OAUTH_STORE_TTL);
        let session_store = KvStoreWrapper::new(kv.clone(), "oauth:session", OAUTH_STORE_TTL);

        // NOTE: duplicated code here is because TryIntoOAuthClientMetadata is a private trait
        if url.contains("http://127.0.0.1") {
            let client_metadata = AtprotoLocalhostClientMetadata {
                scopes: Some(vec![
                    Scope::Known(KnownScope::Atproto),
                    Scope::Known(KnownScope::TransitionGeneric),
                ]),

                redirect_uris: Some(vec![format!("{url}/oauth/callback")]),
            };

            let config = OAuthClientConfig {
                client_metadata,
                keys: None,
                resolver,
                state_store,
                session_store,
            };

            Ok(OAuthClient {
                client: Arc::new(AtriumOAuthClient::new(config)?),
            })
        } else {
            let client_metadata = AtprotoClientMetadata {
                client_id: format!("{url}/client-metadata.json"),
                client_uri: Some(url.to_string()),
                redirect_uris: vec![format!("{url}/oauth/callback")],
                token_endpoint_auth_method: atrium_oauth::AuthMethod::None,
                grant_types: vec![GrantType::AuthorizationCode, GrantType::RefreshToken],
                scopes: vec![
                    Scope::Known(KnownScope::Atproto),
                    Scope::Known(KnownScope::TransitionGeneric),
                ],
                jwks_uri: None,
                token_endpoint_auth_signing_alg: None,
            };

            let config = OAuthClientConfig {
                client_metadata,
                keys: None,
                resolver,
                state_store,
                session_store,
            };

            Ok(OAuthClient {
                client: Arc::new(AtriumOAuthClient::new(config)?),
            })
        }
    }
}
