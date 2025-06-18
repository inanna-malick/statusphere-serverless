use crate::types::jetstream;
use crate::types::lexicons::xyz;
use crate::types::status::STATUS_OPTIONS;
use crate::{types::errors::AppError, types::templates::HomeTemplate};
use crate::{types::status::Status, types::templates::Profile};
use anyhow::Context as _;
use atrium_api::types::string::Handle;
use atrium_oauth::{CallbackParams, OAuthClientMetadata};
use axum::{
    extract::{Query, State},
    response::Redirect,
};
use axum::{Form, Json};
use axum_extra::TypedHeader;
use headers::{Authorization, Upgrade};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use worker::{console_log, HttpResponse};

use super::state::AppState;

#[worker::send]
pub async fn client_metadata(
    State(AppState { oauth, .. }): State<AppState>,
) -> Json<OAuthClientMetadata> {
    Json(oauth.client_metadata())
}

/// OAuth callback endpoint to complete session creation
#[worker::send]
pub async fn oauth_callback(
    Query(params): Query<CallbackParams>,
    State(AppState { oauth, .. }): State<AppState>,
    session: tower_sessions::Session,
) -> Result<Redirect, AppError> {
    let did = oauth.callback(params).await?;
    session.insert("did", did).await?;
    Ok(Redirect::to("/"))
}

/// Log out of current session
pub async fn logout(session: Session) -> Result<Redirect, AppError> {
    session.flush().await.context("session delete")?;

    Ok(Redirect::to("/"))
}

#[derive(Deserialize)]
pub struct LoginForm {
    handle: Handle,
}

/// Establish a session via oauth
#[worker::send]
pub async fn login(
    State(AppState { oauth, .. }): State<AppState>,
    Form(LoginForm { handle }): Form<LoginForm>,
) -> Result<Redirect, AppError> {
    Ok(Redirect::to(&oauth.auth_redirect_url(handle).await?))
}

/// Render the home page
#[worker::send]
pub async fn home(
    State(AppState { oauth, status_db, .. }): State<AppState>,
    session: tower_sessions::Session,
) -> Result<HomeTemplate, AppError> {
    // Fetch recent statuses for template seeding (no handle resolution for now)
    let recent_statuses = match status_db.load_recent_statuses_for_seeding(20).await {
        Ok(statuses) => statuses.into_iter().map(|s| {
            let mut status = crate::types::status::StatusWithHandle::from(s);
            // Leave handle as None for now - we'll resolve handles in a future enhancement
            status.handle = None;
            status
        }).collect(),
        Err(e) => {
            console_log!("Error loading recent statuses for seeding: {}", e);
            Vec::new()
        }
    };

    let did = if let Some(did) = session.get("did").await? {
        did
    } else {
        return Ok(HomeTemplate {
            status_options: &STATUS_OPTIONS,
            profile: None,
            my_status: None,
            recent_statuses,
        });
    };

    let agent = match oauth.restore_session(&did).await {
        Ok(agent) => agent,
        Err(err) => {
            // Destroys the system or you're in a loop
            session.flush().await?;
            return Err(err);
        }
    };

    let current_status = agent.current_status().await?;

    let profile = match agent.bsky_profile().await {
        Ok(profile) => profile,
        Err(AppError::AuthenticationInvalid) => {
            session.flush().await?;
            return Ok(HomeTemplate {
                status_options: &STATUS_OPTIONS,
                profile: None,
                my_status: None,
                recent_statuses,
            });
        }
        Err(e) => return Err(e),
    };

    let username = match profile.display_name {
        Some(username) => username,
        // we could also resolve this via com.api.atproto.identity
        None => profile.handle.to_string(),
    };

    Ok(HomeTemplate {
        status_options: &STATUS_OPTIONS,
        profile: Some(Profile {
            did: did.to_string(),
            display_name: Some(username),
        }),
        my_status: current_status,
        recent_statuses,
    })
}

/// Post body for changing your status
#[derive(Serialize, Deserialize, Clone)]
pub struct StatusForm {
    status: String,
}

/// Publish a status record
#[worker::send]
pub async fn status(
    State(AppState {
        oauth,
        status_db,
        durable_object,
    }): State<AppState>,
    session: Session,
    form: Json<StatusForm>,
) -> Result<(), AppError> {
    console_log!("status handler");
    let did = session.get("did").await?.ok_or(AppError::NoSessionAuth)?;

    let agent = match oauth.restore_session(&did).await {
        Ok(agent) => agent,
        Err(err) => {
            // Destroys the system or you're in a loop
            session.flush().await?;
            return Err(err);
        }
    };

    let uri = agent.create_status(form.status.clone()).await?.uri;

    let status = Status::new(uri, did, form.status.clone());
    let status = status_db
        .save_optimistic(&status)
        .await
        .context("saving status")?;
    durable_object.broadcast(status).await?;

    Ok(())
}

#[worker::send]
pub async fn websocket(
    State(AppState { durable_object, .. }): State<AppState>,
    TypedHeader(_upgrade_to_websocket): TypedHeader<Upgrade>,
) -> Result<HttpResponse, AppError> {
    durable_object.subscriber_websocket().await
}

#[worker::send]
pub async fn admin_publish_jetstream_event(
    State(AppState { durable_object, .. }): State<AppState>,
    // deliberately only implementing basic authorization because it's not the
    // focus of this post - do not use this in production apps
    TypedHeader(auth): TypedHeader<Authorization<headers::authorization::Basic>>,
    Json(status): Json<jetstream::Event<xyz::statusphere::status::RecordData>>,
) -> Result<(), AppError> {
    // DO NOT USE THIS IN PRODUCTION
    if auth.username() != "admin" && auth.password() != "hunter2" {
        return Err(AppError::NoAdminAuth);
    }

    durable_object.broadcast_jetstream_event(status).await?;

    Ok(())
}
