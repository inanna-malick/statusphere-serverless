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
                console_log!("got ready message (deprecated - statuses now seeded in template)");
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
            "/broadcast_jetstream_event" => {
                if req.method() == Method::Post {
                    let status = req.json().await?;
                    self.handle_jetstream_event(&status).await.map_err(|e| {
                        worker::Error::RustError(format!("some error on ingest: {}", e))
                    })?;
                    return worker::Response::empty();
                }
            }
            _ => {}
        }

        worker::Response::error("unsupported method/endpoint", 400)
    }

    async fn alarm(&mut self) -> worker::Result<worker::Response> {
        console_log!("alarm wakeup");

        let cursor = match self.state.storage().get::<u64>("last_seen").await {
            Ok(cursor) => cursor,
            Err(_) => {
                console_log!("establish initial cursor");
                // assume all errors signify no value exists for this key,
                // and start with an initial cursor of one alarm interval ago

                let now = Utc::now().timestamp_micros();
                let cursor = now - ALARM_INTERVAL_MS;
                cursor
                    .try_into()
                    .expect("cursor timestamp should not be negative")
            }
        };

        console_log!("cursor: {cursor}");

        let last_seen = self
            .ingest(cursor)
            .await
            .map_err(|e| worker::Error::RustError(format!("some error on ingest: {}", e)))?;

        console_log!("done ingesting, last seen: {last_seen:?}");

        match last_seen {
            Some(last_seen) => {
                self.state.storage().put("last_seen", last_seen).await?;
            }
            None => {
                console_log!("no events observed (including account/identity events). weird, but not necessarily an error")
            }
        }

        if self.state.get_websockets().is_empty() {
            self.alarms_without_active_websockets += 1;
            console_log!(
                "triggered alarm with no active websockets, incrementing counter: {}",
                self.alarms_without_active_websockets
            );
        } else {
            console_log!(
                "triggered alarm with {} active websockets, resetting counter to 0 from {}",
                self.state.get_websockets().len(),
                self.alarms_without_active_websockets
            );
            self.alarms_without_active_websockets = 0;
        }

        // TTL: stop alarm if no active websockets for greater than N alarm cycles,
        //      this worker is no longer active and can be killed to save resources
        if self.alarms_without_active_websockets > MAX_ALARMS_WITHOUT_ACTIVE_WEBSOCKETS {
            console_log!(
                "reached max alarms without active websockets ({}), terminating alarm",
                MAX_ALARMS_WITHOUT_ACTIVE_WEBSOCKETS
            );
            self.state.storage().delete_alarm().await?;
            self.state.storage().delete_all().await?;
            return worker::Response::empty();
        }

        if self.listener_mode() == ListenerMode::Scheduled {
            self.state.storage().set_alarm(ALARM_INTERVAL_MS).await?;
        }

        worker::Response::empty()
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
        
        // Check current WebSocket connection count for load shedding
        let current_connections = self.state.get_websockets().len();
        const MAX_WEBSOCKET_CONNECTIONS: usize = 500;
        
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

        if self.listener_mode() == ListenerMode::Scheduled {
            console_log!(
                "listener mode is scheduled, setting up alarm if it doesn't already exist"
            );
            match self.state.storage().get_alarm().await? {
                Some(_preexisting) => {}
                None => {
                    console_log!("no prexisting alarm");
                    self.state.storage().set_alarm(ALARM_INTERVAL_MS).await?
                }
            }
        }

        worker::Response::from_websocket(ws.client)
    }

    async fn ingest(&mut self, cursor: TimestampMicros) -> anyhow::Result<Option<TimestampMicros>> {
        let mut last_seen = None;

        let start_time = Utc::now();

        let start_time_us: u64 = start_time
            .timestamp_micros()
            .try_into()
            .expect("start time before 1970? idk");

        let jetstream_url = format!(
            "wss://jetstream1.us-east.bsky.network/subscribe?wantedCollections={}&cursor={}",
            xyz::statusphere::Status::NSID,
            cursor
        );

        console_log!("connecting to jetstream with url {}", jetstream_url);

        let ws = WebSocket::connect(jetstream_url.parse()?).await?;

        let mut event_stream = ws.events()?;
        ws.accept()?;

        while let Some(event) = event_stream.next().await {
            let event = event?;

            match event {
                WebsocketEvent::Message(message_event) => {
                    let message: Event<xyz::statusphere::status::RecordData> =
                        message_event.json()?;

                    self.handle_jetstream_event(&message).await?;

                    last_seen = message.time_us;

                    if message.time_us.is_some_and(|s| s > start_time_us) {
                        console_log!("reached start time, terminate stream");
                        ws.close(None, Some("done"))?;
                        break;
                    }
                }
                WebsocketEvent::Close(_close_event) => break,
            }
        }

        Ok(last_seen)
    }

    pub async fn handle_jetstream_event(
        &mut self,
        event: &Event<xyz::statusphere::status::RecordData>,
    ) -> Result<(), AppError> {
        if let Some(commit) = &event.commit {
            console_log!("commit event: {:?}", &event);

            //We manually construct the uri since Jetstream does not provide it
            //at://{users did}/{collection: xyz.statusphere.status}{records key}
            let record_uri = format!("at://{}/{}/{}", event.did, commit.collection, commit.rkey);
            match commit.operation {
                Operation::Create | Operation::Update => {
                    if let Some(record) = &commit.record {
                        if let Some(ref _cid) = commit.cid {
                            let created = record.created_at.as_ref();
                            let right_now = chrono::Utc::now();

                            let status = Status {
                                uri: record_uri,
                                author_did: Did::new(event.did.clone())
                                    .map_err(|s| anyhow!("invalid did from jetstream: {s}"))?,
                                status: record.status.clone(),
                                created_at: created.to_utc(),
                                indexed_at: right_now,
                            };

                            let updated = self
                                .status_db
                                .save_or_update_from_jetstream(&status)
                                .await?;

                            self.broadcast(updated).await?;
                        }
                    }
                }
                Operation::Delete => {
                    // TODO: could broadcast this to the frontend as an update
                    self.status_db.delete_by_uri(&record_uri).await?;
                }
            }
        }

        Ok(())
    }

    fn listener_mode(&self) -> ListenerMode {
        match self.env.var("SCHEDULED_ALARM") {
            Ok(v) => {
                console_log!("listener mode: {}", v);
                let s = v.to_string();
                if s == "true" {
                    ListenerMode::Scheduled
                } else {
                    ListenerMode::LivePush
                }
            }
            Err(e) => {
                console_error!(
                    "SCHEDULED_ALARM env mapping not set ({}), defaulting to live push",
                    e
                );
                ListenerMode::LivePush
            }
        }
    }
}

#[derive(PartialEq, Eq)]
enum ListenerMode {
    Scheduled,
    LivePush,
}

type TimestampMicros = u64;
