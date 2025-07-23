use crate::services::resolvers;
use crate::services::resolvers::did_resolver;
use crate::types::status::StatusFromDb;
use crate::types::status::StatusWithHandle;
use atrium_oauth::DefaultHttpClient;
use serde_json::json;
use std::sync::Arc;
use worker::console_debug;
use worker::Method;
use worker::{
    console_log, durable_object, wasm_bindgen, wasm_bindgen_futures, Env, State, WebSocket,
    WebSocketIncomingMessage, WebSocketPair,
};

#[durable_object]
pub struct MsgBroker {
    state: State,
    did_resolver: resolvers::DidResolver,
}

#[durable_object]
impl DurableObject for MsgBroker {
    fn new(state: State, env: Env) -> Self {
        let kv = Arc::new(env.kv("KV").expect("invalid KV binding"));

        let did_resolver = did_resolver(&Arc::new(DefaultHttpClient::default()), &kv);

        Self {
            state,
            did_resolver,
        }
    }

    async fn websocket_message(
        &mut self,
        ws: WebSocket,
        message: WebSocketIncomingMessage,
    ) -> worker::Result<()> {
        match message {
            WebSocketIncomingMessage::String(s) if s == "ready" => {
                console_debug!("got ready message (deprecated - statuses now seeded in template)");
                // No-op for backward compatibility
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
}

impl MsgBroker {
    #[worker::send]
    async fn broadcast(&mut self, status: StatusFromDb) -> worker::Result<()> {
        let mut status = StatusWithHandle::from(status);

        status.handle = self
            .did_resolver
            .resolve_handle_for_did(&status.author_did)
            .await;

        for ws in self.state.get_websockets() {
            if let Err(e) = ws.send(&status) {
                console_log!("error {e} on websocket send");
            }
        }

        Ok(())
    }

    async fn subscribe_websocket(&mut self) -> worker::Result<worker::Response> {
        console_log!("subscriber websocket server");

        // Check current WebSocket connection count for load shedding
        let current_connections = self.state.get_websockets().len();
        const MAX_WEBSOCKET_CONNECTIONS: usize = 1000;

        if current_connections >= MAX_WEBSOCKET_CONNECTIONS {
            console_log!(
                "WebSocket connection limit reached ({}/{}), rejecting new connection for load shedding",
                current_connections,
                MAX_WEBSOCKET_CONNECTIONS
            );
            return worker::Response::error("Server overloaded, please try again later", 503);
        }

        console_log!(
            "Accepting new WebSocket connection ({}/{})",
            current_connections + 1,
            MAX_WEBSOCKET_CONNECTIONS
        );

        // no need to check headers, if we're here the frontend worker already did so
        let ws = WebSocketPair::new()?;
        self.state.accept_web_socket(&ws.server);

        worker::Response::from_websocket(ws.client)
    }
}
