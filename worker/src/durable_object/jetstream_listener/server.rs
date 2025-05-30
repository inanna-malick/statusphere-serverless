use crate::durable_object::websocket_broker::client::WebsocketBroker;
use crate::storage::db::StatusDb;
use crate::types::errors::AppError;
use crate::types::status::Status;
use atrium_api::types::Collection as _;
use worker::console_error;
use worker::Method;
use worker::{
    console_log, durable_object, wasm_bindgen, wasm_bindgen_futures, Env, State, WebSocket,
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
// NOTE: this is deliberately small to bias towards not wasting resources if a bunch of people clone this - note in post?
const MAX_ALARMS_SINCE_LAST_KEEPALIVE: usize = 15;

#[durable_object]
pub struct JetstreamListener {
    state: State,
    env: Env,
    status_db: StatusDb,
    websocket_brokers: Vec<WebsocketBroker>,
    alarms_since_keepalive: usize,
}

#[durable_object]
impl DurableObject for JetstreamListener {
    fn new(state: State, env: Env) -> Self {
        // TODO: expect panics, this is bad, but idk what else to do - invalid state
        //       if these don't exist
        // TODO: ah yes, oncecell - just create empty oncecell here and save env, then construct
        //       the fallible parts later
        let status_db = StatusDb::from_env(&env).expect("invalid D1 DB binding");

        let websocket_brokers = WebsocketBroker::for_all_brokers(&env)
            .expect("invalid websocket durable object config");

        Self {
            env,
            state,
            status_db,
            websocket_brokers,
            alarms_since_keepalive: 0,
        }
    }

    async fn fetch(&mut self, mut req: worker::Request) -> worker::Result<worker::Response> {
        console_log!("fetch {}", req.url()?.path());
        // the communication here is all between two closely coupled workers so
        // we can abandon the axum routing used in the frontend-facing worker
        // which must support content encodings, work with headers, etc
        match req.url()?.path() {
            "/keepalive" => {
                if req.method() == Method::Post {
                    todo!()
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
            self.alarms_since_keepalive += 1;
            console_log!(
                "triggered alarm with no active websockets, incrementing counter: {}",
                self.alarms_since_keepalive
            );
        } else {
            console_log!(
                "triggered alarm with {} active websockets, resetting counter to 0 from {}",
                self.state.get_websockets().len(),
                self.alarms_since_keepalive
            );
            self.alarms_since_keepalive = 0;
        }

        // TTL: stop alarm if no active websockets for greater than N alarm cycles,
        //      this worker is no longer active and can be killed to save resources
        if self.alarms_since_keepalive > MAX_ALARMS_SINCE_LAST_KEEPALIVE {
            console_log!(
                "reached max alarms without active websockets ({}), terminating alarm",
                MAX_ALARMS_SINCE_LAST_KEEPALIVE
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

impl JetstreamListener {
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

                            // TODO: broadcast to all brokers
                            // self.broadcast(updated).await?;
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
