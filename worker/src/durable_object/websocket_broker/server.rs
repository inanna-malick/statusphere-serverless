use crate::services::resolvers;
use crate::services::resolvers::did_resolver;
use crate::storage::db::StatusDb;
use crate::types::errors::AppError;
use crate::types::status::Status;
use crate::types::status::StatusFromDb;
use crate::types::status::StatusWithHandle;
use atrium_api::types::Collection as _;
use atrium_common::resolver::Resolver;
use atrium_oauth::DefaultHttpClient;
use serde_json::json;
use std::sync::Arc;
use worker::console_error;
use worker::Method;
use worker::{
    console_log, durable_object, wasm_bindgen, wasm_bindgen_futures, Env, State, WebSocket,
    WebSocketIncomingMessage, WebSocketPair,
};

use worker::WebsocketEvent;

use crate::types::jetstream::{Event, Operation};
use crate::types::lexicons::xyz;
use anyhow::anyhow;
use atrium_api::types::string::Did;
use chrono::Utc;
use futures::StreamExt as _;

// read from jetstream once per minute
const ALARM_INTERVAL_MS: i64 = 60 * 1000;
const MAX_ALARMS_WITHOUT_ACTIVE_WEBSOCKETS: usize = 15;

#[durable_object]
pub struct MsgBroker {
    state: State,
    env: Env,
    status_db: StatusDb,
    did_resolver: resolvers::DidResolver,
    alarms_without_active_websockets: usize,
}

#[durable_object]
impl DurableObject for MsgBroker {
    fn new(state: State, env: Env) -> Self {
        let kv = Arc::new(env.kv("KV").expect("invalid KV binding"));
        let status_db = StatusDb::from_env(&env).expect("invalid D1 DB binding");

        let did_resolver = did_resolver(&Arc::new(DefaultHttpClient::default()), &kv);

        Self {
            env,
            state,
            status_db,
            did_resolver,
            alarms_without_active_websockets: 0,
        }
    }

    async fn websocket_message(
        &mut self,
        ws: WebSocket,
        message: WebSocketIncomingMessage,
    ) -> worker::Result<()> {
        // TODO: getting more of these than makes sense
        match message {
            WebSocketIncomingMessage::String(s) if s == "ready" => {
                console_log!("got ready message, queueing up last 10 statuses");

                let mut last_10 = self.status_db.load_latest_statuses(10).await?;
                last_10.reverse();

                for status in last_10.into_iter() {
                    let mut status = StatusWithHandle::from(status);
                    status.handle = self.resolve_handle_for_did(&status.author_did).await;

                    ws.send(&status)?
                }
            }
            _ => {
                console_log!("unexpected incoming message");
                ws.send(&json!({"error": "unexpected incoming message"}))?;
            }
        }

        Ok(())
    }

    async fn fetch(&mut self, mut req: worker::Request) -> worker::Result<worker::Response> {
        console_log!("fetch {}", req.url()?.path());
        // the communication here is all between two closely coupled workers so
        // we can abandon the axum routing used in the frontend-facing worker
        // which must support content encodings, work with headers, etc
        match req.url()?.path() {
            "/subscribe_websocket" => {
                if req.method() == Method::Get {
                    return self.subscribe_websocket().await;
                }
            }
            "/broadcast_status" => {
                if req.method() == Method::Post {
                    let status = req.json().await?;
                    self.broadcast(status).await?;
                    return worker::Response::empty();
                }
            }
            _ => {}
        }

        worker::Response::error("unsupported method/endpoint", 400)
    }

    async fn alarm(&mut self) -> worker::Result<worker::Response> {
        todo!("TTL goes here")
    }
}

impl MsgBroker {
    async fn resolve_handle_for_did(&self, did: &Did) -> Option<String> {
        match self.did_resolver.resolve(did).await {
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
        }
    }

    #[worker::send]
    async fn broadcast(&mut self, status: StatusFromDb) -> worker::Result<()> {
        let mut status = StatusWithHandle::from(status);

        status.handle = self.resolve_handle_for_did(&status.author_did).await;

        for ws in self.state.get_websockets() {
            if let Err(e) = ws.send(&status) {
                console_log!("error {e} on websocket send");
            }
        }

        Ok(())
    }

    async fn subscribe_websocket(&mut self) -> worker::Result<worker::Response> {
        console_log!("subscriber websocket server");
        // no need to check headers, if we're here the frontend worker already did so
        let ws = WebSocketPair::new()?;
        self.state.accept_web_socket(&ws.server);

        worker::Response::from_websocket(ws.client)
    }
}

#[derive(PartialEq, Eq)]
enum ListenerMode {
    Scheduled,
    LivePush,
}

type TimestampMicros = u64;
