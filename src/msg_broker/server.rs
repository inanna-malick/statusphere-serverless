use crate::services::storage::db::StatusDb;
use crate::types::errors::AppError;
use crate::types::status::StatusFromDb;
use crate::types::status::StatusWithHandle;
use atrium_common::resolver::CachedResolver;
use atrium_common::resolver::Resolver;
use atrium_common::types::cached::r#impl::Cache;
use atrium_common::types::cached::r#impl::CacheImpl;
use atrium_common::types::cached::CacheConfig;
use atrium_identity::did::CommonDidResolver;
use atrium_identity::did::CommonDidResolverConfig;
use atrium_identity::did::DEFAULT_PLC_DIRECTORY_URL;
use atrium_oauth::DefaultHttpClient;
use axum::routing::post;
use axum::Json;
use axum::routing::get;
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tower::Service as _;
use worker::{
    console_log, durable_object, response_from_wasm, send::SendWrapper, wasm_bindgen,
    wasm_bindgen_futures, Env, HttpRequest, HttpResponse, State, WebSocket,
    WebSocketIncomingMessage, WebSocketPair,
};

#[durable_object]
pub struct MsgBroker {
    state: Arc<SendWrapper<State>>,
    env: Env,
    handle_resolver: Arc<CachedResolver<CommonDidResolver<DefaultHttpClient>>>,
}

#[durable_object]
impl DurableObject for MsgBroker {
    fn new(state: State, env: Env) -> Self {
        let state = Arc::new(SendWrapper(state));

        let handle_resolver = Arc::new(CachedResolver::new(
            CommonDidResolver::new(CommonDidResolverConfig {
                plc_directory_url: DEFAULT_PLC_DIRECTORY_URL.to_string(),
                http_client: Arc::new(DefaultHttpClient::default()),
            }),
            CacheImpl::new(CacheConfig {
                max_capacity: Some(100),
                time_to_live: Some(Duration::new(60 * 60 * 6, 0)),
            }),
        ));

        Self {
            env,
            state,
            handle_resolver,
        }
    }

    async fn websocket_message(
        &mut self,
        ws: WebSocket,
        message: WebSocketIncomingMessage,
    ) -> worker::Result<()> {
        match message {
            WebSocketIncomingMessage::String(s) if s == "ready" => {
                let status_db = StatusDb::new(self.env.d1("DB")?);

                let mut last_10 = status_db.load_latest_statuses(10).await?;
                last_10.reverse();

                for status in last_10.into_iter() {
                    let mut status = StatusWithHandle::from_db(status);
                    let handle = match self.handle_resolver.resolve(&status.author_did).await {
                        Ok(did_doc) => {
                            // first element of also known as list is highest priority handle
                            did_doc.also_known_as.and_then(|akas| {
                                akas.first().map(|s| format!("@{}", s).replace("at://", ""))
                            })
                        }
                        Err(err) => {
                            console_log!("Error resolving did: {err}");
                            None
                        }
                    };
                    status.handle = handle;

                    ws.send(&status)?
                }
            }
            _ => {
                console_log!("unexpected incoming message");
                ws.send(&json!({"error": "websocket sends you messages, not vice versa. bad client :("}))?;
            }
        }

        Ok(())
    }

    async fn fetch(&mut self, req: worker::Request) -> worker::Result<worker::Response> {
        let req = HttpRequest::try_from(req)?;

        let resp = axum::Router::new()
            .route("/websocket", get(websocket))
            .route("/broadcast", post(broadcast))
            .with_state(self.app_state())
            .call(req)
            .await?;

        resp.try_into()
    }
}

#[worker::send]
async fn broadcast(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(json): Json<StatusFromDb>,
) -> Result<(), AppError> {
    let mut status = StatusWithHandle::from_db(json);

    let handle = match state.handle_resolver.resolve(&status.author_did).await {
        Ok(did_doc) => {
            // also known as list is in priority order so take first
            did_doc
                .also_known_as
                .and_then(|akas| akas.first().map(|s| format!("@{}", s).replace("at://", "")))
        }
        Err(err) => {
            console_log!("Error resolving did: {err}");
            None
        }
    };
    status.handle = handle;

    for ws in state.state.get_websockets() {
        if let Err(e) = ws.send(&status) {
            console_log!("error {e} on websocket send");
        }
    }

    Ok(())
}

#[worker::send]
async fn websocket(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<HttpResponse, AppError> {
    // no need to check headers, if we're here the frontend worker already did so
    let ws = WebSocketPair::new()?;
    state.state.accept_web_socket(&ws.server);

    let resp = worker::Response::from_websocket(ws.client)?;
    Ok(response_from_wasm(resp.into())?)
}

#[derive(Clone)]
struct AppState {
    state: Arc<SendWrapper<worker::State>>,
    handle_resolver: Arc<CachedResolver<CommonDidResolver<DefaultHttpClient>>>,
}

impl MsgBroker {
    fn app_state(&self) -> AppState {
        AppState {
            state: self.state.clone(),
            handle_resolver: self.handle_resolver.clone(),
        }
    }
}
