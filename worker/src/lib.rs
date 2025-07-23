use atrium_oauth::DefaultHttpClient;
// use crate::services::jetstream_listener;
use axum::response::IntoResponse;
use durable_object::client::MessageBroker;
use frontend_worker::{router::router, state::AppState};
use services::oauth::OAuthClient;
use std::sync::Arc;
use std::time::Duration;
use storage::{db::StatusDb, kv::KvStoreWrapper};
use worker::{
    console_debug, console_error, console_log, event, Context, Env, HttpRequest, ScheduleContext,
    ScheduledEvent,
};

use tower::Service as _;

use crate::services::{jetstream::ingest_, resolvers};

mod durable_object;
mod frontend_worker;
mod services;
mod storage;
mod types;

const SESSION_STORE_TTL: Duration = Duration::new(60 * 60 * 24 * 30, 0);

#[event(fetch, respond_with_errors)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> worker::Result<http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    let kv = Arc::new(env.kv("KV")?);
    let status_db = StatusDb::from_env(&env)?;

    let url = {
        let scheme = match req.uri().scheme() {
            Some(v) => v,
            None => return Ok("request URI missing scheme".into_response()),
        };
        let host = match req.uri().host() {
            Some(v) => v,
            None => return Ok("request URI missing host".into_response()),
        };
        format!("{}://{}", scheme, host)
    };

    console_debug!("running with URL: {}", url);

    let client = match OAuthClient::new(url.to_string(), &kv) {
        Ok(c) => c,
        // TODO: move to domain error probably, fixme and etc
        Err(e) => return Ok(format!("oauth client init err: {}", e).into_response()),
    };

    let ns = env.durable_object("MSGBROKER")?;
    let durable_object = MessageBroker::from_namespace(&ns)?;

    let did_resolver = resolvers::did_resolver(&Arc::new(DefaultHttpClient::default()), &kv);
    let session_store = KvStoreWrapper::new(kv, "tower:session", SESSION_STORE_TTL);

    let state = AppState {
        oauth: client,
        status_db,
        durable_object,
        did_resolver: Arc::new(did_resolver),
    };

    Ok(router(state, session_store).call(req).await?)
}

#[event(scheduled, respond_with_errors)]
async fn scheduled(_s: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();

    match ingest_(env).await {
        Ok(_) => console_log!("done with scheduled jetstream reader"),
        Err(e) => console_error!("error on scheduled jetstream reader, {}", e),
    }
}
