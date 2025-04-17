use crate::msg_broker::client::MessageBroker;
use crate::services::oauth::OAuthClient;
use crate::services::storage::db::StatusDb;
use crate::services::storage::kv::session_state::KvTowerSessionStore;
use crate::types::lexicons::{record::KnownRecord, xyz::statusphere::Status};
use crate::types::status::STATUS_OPTIONS;
use crate::{types::errors::AppError, types::templates::HomeTemplate};
use crate::{
    types::status::{Source, StatusFromDb},
    types::templates::Profile,
};
use anyhow::anyhow;
use anyhow::Context as _;
use askama::Template;
use atrium_api::{
    agent::Agent,
    types::{
        string::{Datetime, Did},
        Collection,
    },
};
use atrium_oauth::{AuthorizeOptions, CallbackParams, KnownScope, OAuthClientMetadata, Scope};
use axum::routing::{get, post};
use axum::{
    extract::{Query, State},
    response::{Html, Redirect},
};
use axum::{Json, Router};
use chrono::Utc;
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use tower_sessions::cookie::SameSite;
use tower_sessions::{Session, SessionManagerLayer};
use worker::{console_log, HttpResponse};

#[derive(Clone)]
pub struct AppState {
    pub oauth: OAuthClient,
    pub db: StatusDb,
    pub message_broker: MessageBroker,
}

const BSKY_PDS: &str = "https://bsky.social";

pub fn router(state: AppState, session_store: KvTowerSessionStore) -> Router {
    let session_layer = SessionManagerLayer::new(session_store)
        // NOTE: using lax here does fix the immediate problem (cookie not sent) but probably degrades security a wee bit
        .with_same_site(SameSite::Lax);

    axum::Router::new()
        .route("/client-metadata.json", get(client_metadata))
        .route("/oauth/callback", get(oauth_callback))
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/status", post(status))
        .route("/websocket", get(websocket))
        .route("/", get(home))
        .layer(session_layer)
        .with_state(state)
}

#[worker::send]
pub async fn client_metadata(State(state): State<AppState>) -> Json<OAuthClientMetadata> {
    Json(state.oauth.client_metadata.clone())
}

/// TS version https://github.com/bluesky-social/statusphere-example-app/blob/e4721616df50cd317c198f4c00a4818d5626d4ce/src/routes.ts#L71
/// OAuth callback endpoint to complete session creation
// #[get("/oauth/callback")]
#[worker::send]
pub async fn oauth_callback(
    Query(params): Query<CallbackParams>,
    State(state): State<AppState>,
    session: Session,
) -> Result<Redirect, AppError> {
    //Processes the call back and parses out a session if found and valid
    let (bsky_session, _) = state.oauth.callback(params).await?;

    let agent = Agent::new(bsky_session);

    match agent.did().await {
        Some(did) => {
            let did = did.to_string();
            session
                .insert("did", did)
                .await
                .context("inserting did into session")?;

            Ok(Redirect::to("/"))
        }
        None => Err(anyhow!("The OAuth agent did not return a DID. May try re-logging in?").into()),
    }
}

/// Log out of current session
pub async fn logout(session: Session) -> Result<Redirect, AppError> {
    session.flush().await.context("session delete")?;

    Ok(Redirect::to("/"))
}

/// Establish a session via oauth
#[worker::send]
pub async fn login(State(state): State<AppState>) -> Result<Redirect, AppError> {
    //Creates the oauth url to redirect to for the user to log in with their credentials
    let oauth_url = state
        .oauth
        .authorize(
            BSKY_PDS,
            AuthorizeOptions {
                scopes: vec![
                    Scope::Known(KnownScope::Atproto),
                    Scope::Known(KnownScope::TransitionGeneric),
                ],
                ..Default::default()
            },
        )
        .await?;

    Ok(Redirect::to(&oauth_url))
}

// /// TS version https://github.com/bluesky-social/statusphere-example-app/blob/e4721616df50cd317c198f4c00a4818d5626d4ce/src/routes.ts#L146
// /// Home
// // #[get("/")]
#[worker::send]
pub async fn home(
    State(state): State<AppState>,
    session: Session,
) -> Result<Html<String>, AppError> {
    const TITLE: &str = "Home";

    match session.get::<String>("did").await? {
        Some(did) => {
            console_log!("loaded DID {:?}", Utc::now()); // fastish, ~70ms
            let did = Did::new(did).expect("failed to parse did");
            //Grabs the users last status to highlight it in the ui (fastish, ~50ms)
            let my_status = state.db.my_status(&did).await.unwrap_or(None);
            console_log!("fetched status from db {:?}", Utc::now());

            // gets the user's session from the session store to resume
            // (slowish, 400ms, idk if it hits non-cf stuff or just kv)
            match state.oauth.restore(&did).await {
                Ok(session) => {
                    console_log!("restored oauth {:?}", Utc::now());
                    //Creates an agent to make authenticated requests
                    let agent = Agent::new(session);

                    // Fetch additional information about the logged-in user (slowish, 600ms)
                    let profile = agent
                        .api
                        .app
                        .bsky
                        .actor
                        .get_profile(
                            atrium_api::app::bsky::actor::get_profile::ParametersData {
                                actor: atrium_api::types::string::AtIdentifier::Did(did),
                            }
                            .into(),
                        )
                        .await;

                    console_log!("fetched profile {:?}", Utc::now());

                    let html = HomeTemplate {
                        title: TITLE,
                        status_options: &STATUS_OPTIONS,
                        profile: match profile {
                            Ok(profile) => {
                                let profile_data = Profile {
                                    did: profile.did.to_string(),
                                    display_name: profile.display_name.clone(),
                                };
                                Some(profile_data)
                            }
                            Err(err) => {
                                console_log!("Error accessing profile: {err}");
                                None
                            }
                        },
                        my_status: my_status.as_ref().map(|s| s.status.clone()),
                    }
                    .render()
                    .expect("template should be valid");

                    Ok(Html::from(html))
                }
                Err(err) => {
                    // Destroys the system or you're in a loop
                    session.flush().await?;
                    Err(err.into())
                }
            }
        }

        None => {
            let html = HomeTemplate {
                title: TITLE,
                status_options: &STATUS_OPTIONS,
                profile: None,
                my_status: None,
            }
            .render()
            .expect("template should be valid");

            Ok(Html::from(html))
        }
    }
}

/// Post body for changing your status
#[derive(Serialize, Deserialize, Clone)]
pub struct StatusForm {
    status: String,
}

/// Publish a status record
#[worker::send]
pub async fn status(
    State(state): State<AppState>,
    session: Session,
    form: Json<StatusForm>,
) -> Result<(), AppError> {
    // Check if the user is logged in
    let did_string: String = if let Some(did_string) = session.get::<String>("did").await? {
        did_string
    } else {
        return Err(AppError::NoSessionAuth);
    };

    let did = Did::new(did_string.clone()).expect("failed to parse did");
    // gets the user's session from the session store to resume
    match state.oauth.restore(&did).await {
        Ok(session) => {
            let agent = Agent::new(session);
            //Creates a strongly typed ATProto record
            let status: KnownRecord =
                crate::types::lexicons::xyz::statusphere::status::RecordData {
                    created_at: Datetime::now(),
                    status: form.status.clone(),
                }
                .into();

            // TODO no data validation yet from esquema
            // Maybe you'd like to add it? https://github.com/fatfingers23/esquema/issues/3

            let record = agent
                .api
                .com
                .atproto
                .repo
                .create_record(
                    atrium_api::com::atproto::repo::create_record::InputData {
                        collection: Status::NSID.parse().unwrap(),
                        repo: did.clone().into(),
                        rkey: None,
                        record: status.into(),
                        swap_commit: None,
                        validate: None,
                    }
                    .into(),
                )
                .await
                .context("publish status via agent")?;

            let status = StatusFromDb::new(
                record.uri.clone(),
                did,
                form.status.clone(),
                Source::ThisInstance,
            );

            state.db.save(&status).await.context("saving status")?;

            state.message_broker.broadcast(status).await?;

            Ok(())
        }
        Err(err) => {
            // Destroys the system or you're in a loop
            session.flush().await.context("flush session")?;
            console_log!(
                "Error restoring session, we are removing the session from the cookie: {err}"
            );

            Err(anyhow!("error restoring session {}", err).into())
        }
    }
}

#[worker::send]
pub async fn websocket(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<HttpResponse, AppError> {
    console_log!("establishing websocket connection");
    match headers.get("Upgrade") {
        Some(h) if h == "websocket" => {}
        _ => return Err(AppError::BadWebSocketUpgrade),
    };

    let resp = state.message_broker.connect().await?;

    Ok(resp)
}
