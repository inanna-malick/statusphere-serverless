use crate::services::dns_over_http::DnsOverHttps;
use crate::services::storage::kv::KvStoreWrapper;

use atrium_identity::{
    did::{CommonDidResolver, CommonDidResolverConfig, DEFAULT_PLC_DIRECTORY_URL},
    handle::{AtprotoHandleResolver, AtprotoHandleResolverConfig},
};
use atrium_oauth::{
    AtprotoClientMetadata, AtprotoLocalhostClientMetadata, DefaultHttpClient, GrantType,
    KnownScope, OAuthClient as AtriumOAuthClient, OAuthClientConfig, OAuthResolverConfig, Scope,
};
use std::{ops::Deref, sync::Arc};

use super::storage::kv::{cached_resolver::KvStoreCachedResolver, KvSessionStore, KvStateStore};

#[derive(Clone)]
pub struct OAuthClient {
    client: Arc<ClientType>,
}

type ClientType = AtriumOAuthClient<
    KvStateStore,
    KvSessionStore,
    KvStoreCachedResolver<CommonDidResolver<DefaultHttpClient>>,
    HandleResolver,
>;

pub type HandleResolver =
    KvStoreCachedResolver<AtprotoHandleResolver<DnsOverHttps, DefaultHttpClient>>;

impl Deref for OAuthClient {
    type Target = Arc<ClientType>;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl OAuthClient {
    pub fn new(url: String, kv: &Arc<worker::kv::KvStore>) -> anyhow::Result<Self> {
        // Create a new OAuth client
        let http_client = Arc::new(DefaultHttpClient::default());

        let resolver = OAuthResolverConfig {
            did_resolver: KvStoreCachedResolver {
                inner: CommonDidResolver::new(CommonDidResolverConfig {
                    plc_directory_url: DEFAULT_PLC_DIRECTORY_URL.to_string(),
                    http_client: http_client.clone(),
                }),
                cache: KvStoreWrapper::new(kv.clone(), "resolved:did", 60 * 60 * 8),
            },
            handle_resolver: KvStoreCachedResolver {
                inner: AtprotoHandleResolver::new(AtprotoHandleResolverConfig {
                    dns_txt_resolver: DnsOverHttps::new(),
                    http_client: http_client.clone(),
                }),
                cache: KvStoreWrapper::new(kv.clone(), "resolved:handle", 60 * 60 * 8),
            },
            authorization_server_metadata: Default::default(),
            protected_resource_metadata: Default::default(),
        };

        let state_store = KvStoreWrapper::new(kv.clone(), "oauth:state", 60 * 60 * 24 * 30);
        let session_store = KvStoreWrapper::new(kv.clone(), "oauth:session", 60 * 60 * 24 * 30);

        // NOTE: duplicated code here is because TryIntoOAuthClientMetadata is a private trait :(
        if url.contains("http://127.0.0.1") {
            let client_metadata = AtprotoLocalhostClientMetadata {
                scopes: Some(vec![
                    Scope::Known(KnownScope::Atproto),
                    Scope::Known(KnownScope::TransitionGeneric),
                ]),

                redirect_uris: Some(vec![format!("{url}/oauth/callback")]),
            };

            // TODO: cache these resolvers in kv store
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
