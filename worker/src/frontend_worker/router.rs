use crate::storage::kv::session_state::KvTowerSessionStore;
use axum::routing::{get, post};
use axum::Router;
use tower_sessions::cookie::SameSite;
use tower_sessions::SessionManagerLayer;

use super::endpoints;
use super::state::AppState;

pub fn router(state: AppState, session_store: KvTowerSessionStore) -> Router {
    let session_layer = SessionManagerLayer::new(session_store)
        // NOTE: this may not need to be lax, but I think it does (b/c of oauth redirect)
        .with_same_site(SameSite::Lax);

    axum::Router::new()
        .route("/client-metadata.json", get(endpoints::client_metadata))
        .route("/oauth/callback", get(endpoints::oauth_callback))
        .route("/login", post(endpoints::login).get(endpoints::home))
        .route("/logout", get(endpoints::logout))
        .route("/status", post(endpoints::status))
        .route("/websocket", get(endpoints::websocket))
        .route(
            "/admin/publish_jetstream_event",
            post(endpoints::admin_publish_jetstream_event),
        )
        .route("/", get(endpoints::home))
        .layer(session_layer)
        .with_state(state)
}
