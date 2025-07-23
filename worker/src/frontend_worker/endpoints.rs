use crate::frontend_worker::state::ScheduledEventState;
use crate::services::jetstream::handle_jetstream_event;
use crate::types::jetstream;
use crate::types::lexicons::xyz;
use crate::types::status::STATUS_OPTIONS;
use crate::{types::errors::AppError, types::templates::HomeTemplate};
use crate::{
    types::status::{Status, StatusWithHandle},
    types::templates::Profile,
};
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
    State(AppState {
        oauth,
        status_db,
        did_resolver,
        ..
    }): State<AppState>,
    session: tower_sessions::Session,
) -> Result<HomeTemplate, AppError> {
    // Fetch recent statuses for template seeding (no handle resolution for now)
    let recent_statuses = match status_db.load_latest_statuses(20).await {
        Ok(statuses) => {
            let mut statuses_with_handles = Vec::new();
            for s in statuses.into_iter() {
                let mut status = crate::types::status::StatusWithHandle::from(s);
                status.handle = did_resolver
                    .resolve_handle_for_did(&status.author_did)
                    .await;
                statuses_with_handles.push(status);
            }
            // enforce chronological ordering
            statuses_with_handles.sort_by_key(|s| s.created_at);
            statuses_with_handles.reverse();
            statuses_with_handles
        }
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
        did_resolver,
    }): State<AppState>,
    session: Session,
    form: Json<StatusForm>,
) -> Result<Json<StatusWithHandle>, AppError> {
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
    let status_from_db = status_db
        .save_optimistic(&status)
        .await
        .context("saving status")?;

    // Broadcast to WebSocket clients
    durable_object.broadcast(status_from_db.clone()).await?;

    // Convert to StatusWithHandle and return as JSON
    let mut status_with_handle = StatusWithHandle::from(status_from_db);
    status_with_handle.handle = did_resolver
        .resolve_handle_for_did(&status_with_handle.author_did)
        .await;
    Ok(Json(status_with_handle))
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
    State(AppState {
        durable_object,
        status_db,
        ..
    }): State<AppState>,
    // deliberately only implementing basic authorization because it's not the
    // focus of this post - do not use this in production apps
    TypedHeader(auth): TypedHeader<Authorization<headers::authorization::Basic>>,
    Json(status): Json<jetstream::Event<xyz::statusphere::status::RecordData>>,
) -> Result<(), AppError> {
    // TODO: re-deploy with this disabled in some manner
    // DO NOT USE THIS IN PRODUCTION
    if auth.username() != "admin" && auth.password() != "hunter2" {
        return Err(AppError::NoAdminAuth);
    }

    handle_jetstream_event(
        &ScheduledEventState {
            status_db,
            durable_object,
        },
        &status,
    )
    .await?;

    Ok(())
}
