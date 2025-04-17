use crate::services::storage::db::StatusDb;
use crate::types::status::StatusFromDb;
use std::sync::Arc;
use worker::{
    console_log, durable_object, send::SendWrapper, wasm_bindgen, wasm_bindgen_futures, Env, State,
    WebSocket, WebsocketEvent,
};

use crate::msg_broker::client::MessageBroker;
use crate::types::jetstream::{Event, Operation};
use crate::types::lexicons::xyz;
use crate::types::lexicons::xyz::statusphere::Status;
use crate::types::status::Source;
use anyhow::anyhow;
use atrium_api::types::string::Did;
use chrono::Utc;
use futures::StreamExt as _;
// use worker::{console_log, Env, WebSocket, WebsocketEvent};

use atrium_api::types::Collection;

#[durable_object]
pub struct JetstreamListener {
    state: Arc<SendWrapper<State>>,
    env: Env,
}

// read from jetstream once per minute
const ALARM_INTERVAL_MS: i64 = 60 * 1000;

#[durable_object]
impl DurableObject for JetstreamListener {
    fn new(state: State, env: Env) -> Self {
        console_log!("new listener");
        let state = Arc::new(SendWrapper(state));

        Self { env, state }
    }

    // required but no-op
    async fn fetch(&mut self, _req: worker::Request) -> worker::Result<worker::Response> {
        console_log!("listener durable object 'poke'; hax");
        match self.state.storage().get_alarm().await? {
            Some(_preexisting) => console_log!("preexisting alarm, we're all good"),
            None => {
                console_log!("no prexisting alarm");
                self.state.storage().set_alarm(ALARM_INTERVAL_MS).await?
            }
        }

        console_log!("listener poke done");

        worker::Response::empty()
    }

    async fn alarm(&mut self) -> worker::Result<worker::Response> {
        console_log!("alarm wakeup");

        // err if key doesn't exist (todo: file PR? should return Result<Option<T>>)
        let cursor = match self.state.storage().get::<u64>("last_seen").await {
            Ok(cursor) => cursor,
            Err(_) => {
                console_log!("establish initial cursor");
                // assume all errors signify no value exists for this key,
                // and start with an initial cursor of 1m ago

                let now = Utc::now().timestamp_micros();

                // fairly arbitrary, but just use the alarm interval as lookback if we don't have a cursor saved
                let cursor = now - ALARM_INTERVAL_MS;
                cursor
                    .try_into()
                    .expect("cursor timestamp should not be negative")
            }
        };

        console_log!("cursor: {cursor}");

        let last_seen = ingest(cursor, &self.env)
            .await
            .map_err(|e| worker::Error::RustError(format!("some error on ingest: {}", e)))?;

        console_log!("done ingesting, last seen: {last_seen:?}");

        match last_seen {
            Some(last_seen) => {
                self.state.storage().put("last_seen", last_seen).await?;
            }
            None => {
                console_log!("no events observed (incl account/identity events). weird, but not necessarily an error")
            }
        }

        self.state.storage().set_alarm(ALARM_INTERVAL_MS).await?;

        // not sure what this is here for, TODO, find out and add a better comment
        worker::Response::empty()
    }
}

type TimestampMicros = u64;

pub async fn ingest(cursor: TimestampMicros, env: &Env) -> anyhow::Result<Option<TimestampMicros>> {
    let mut last_seen = None;

    let db = StatusDb::new(env.d1("DB")?);
    let msg_broker = MessageBroker::from_namespace(&env.durable_object("MSGBROKER")?)?;

    let start_time = Utc::now();

    let start_time_us: u64 = start_time
        .timestamp_micros()
        .try_into()
        .expect("start time before 1970? idk");

    let jetstream_url = format!(
        "wss://jetstream1.us-east.bsky.network/subscribe?wantedCollections={}&cursor={}",
        Status::NSID,
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
                let message: Event<xyz::statusphere::status::RecordData> = message_event.json()?;

                if let Some(commit) = &message.commit {
                    console_log!("commit event: {:?}", &message);

                    //We manually construct the uri since Jetstream does not provide it
                    //at://{users did}/{collection: xyz.statusphere.status}{records key}
                    let record_uri =
                        format!("at://{}/{}/{}", message.did, commit.collection, commit.rkey);
                    match commit.operation {
                        Operation::Create | Operation::Update => {
                            if let Some(record) = &commit.record {
                                if let Some(ref _cid) = commit.cid {
                                    let created = record.created_at.as_ref();
                                    let right_now = chrono::Utc::now();

                                    let status = StatusFromDb {
                                        uri: record_uri,
                                        author_did: Did::new(message.did.clone()).map_err(|s| {
                                            anyhow!("invalid did from jetstream: {s}")
                                        })?,
                                        status: record.status.clone(),
                                        created_at: created.to_utc(),
                                        indexed_at: right_now,
                                        source: Source::Jetstream,
                                    };

                                    let updated = db.save_or_update_from_jetstream(&status).await?;

                                    msg_broker.broadcast(updated).await?;
                                }
                            }
                        }
                        Operation::Delete => {
                            // TODO: could broadcast this to the frontend as an update
                            db.delete_by_uri(&record_uri).await?;
                        }
                    }
                }

                // NOTE: this can be None - weird, idk how to handle that rn tho
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
