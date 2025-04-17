// use crate::services::jetstream_listener;
use axum::response::IntoResponse;
use jetstream_listener::client::ListenerClient;
use msg_broker::client::MessageBroker;
use routes::{router, AppState};
use services::oauth::OAuthClient;
use services::storage::{db::StatusDb, kv::KvStoreWrapper};
use std::sync::Arc;
use worker::{event, Context, Env, HttpRequest};

use tower::Service as _;

mod jetstream_listener;
mod msg_broker;
mod routes;
mod services;
mod types;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> worker::Result<http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    // initialize listener by creating stub - feels weird but idk how else to fit this in
    let l = ListenerClient::from_namespace(&env.durable_object("LISTENER")?)?;
    l.poke().await?;

    let kv = Arc::new(env.kv("KV")?);
    let db = StatusDb::new(env.d1("DB")?);

    let url = env.var("HOST")?;

    let client = match OAuthClient::new(url.to_string(), &kv) {
        Ok(c) => c,
        // TODO: move to domain error probably, fixme and etc
        Err(e) => return Ok(format!("oauth client init err: {}", e).into_response()),
    };

    let ns = env.durable_object("MSGBROKER")?;
    let message_broker = MessageBroker::from_namespace(&ns)?;

    let session_store = KvStoreWrapper::new(kv, "tower:session", 60 * 60 * 24 * 30);

    let state = AppState {
        oauth: client,
        db,
        message_broker,
    };

    Ok(router(state, session_store).call(req).await?)
}
