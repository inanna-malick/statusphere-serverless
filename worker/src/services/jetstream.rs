use crate::durable_object::client::MessageBroker;
use crate::frontend_worker::state::AppState;
use crate::frontend_worker::state::ScheduledEventState;
use crate::services::resolvers;
use crate::services::resolvers::did_resolver;
use crate::storage::db::StatusDb;
use crate::types::errors::AppError;
use crate::types::status::Status;
use crate::types::status::StatusFromDb;
use crate::types::status::StatusWithHandle;
use anyhow::Context;
use atrium_api::types::Collection as _;
use atrium_oauth::DefaultHttpClient;
use chrono::Date;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use worker::console_debug;
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

const ALARM_INTERVAL_MS: i64 = 60 * 1000;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Cursor {
    last_seen: u64,
}

pub async fn ingest_(env: Env) -> anyhow::Result<()> {
    // TODO: use kv, TTL it w/ short TTL, if none present just use now - offset
    // let cursor = match self.state.storage().get::<u64>("last_seen").await {
    //     Ok(cursor) => cursor,
    //     Err(_) => {
    //         console_log!("establish initial cursor");
    //         // assume all errors signify no value exists for this key,
    //         // and start with an initial cursor of one alarm interval ago

    //         let now = Utc::now().timestamp_micros();
    //         let cursor = now - ALARM_INTERVAL_MS;
    //         cursor
    //             .try_into()
    //             .expect("cursor timestamp should not be negative")
    //     }
    // };

    let kv = Arc::new(env.kv("KV")?);
    let status_db = StatusDb::from_env(&env)?;

    let ns = env.durable_object("MSGBROKER")?;
    let durable_object = MessageBroker::from_namespace(&ns)?;

    let state = ScheduledEventState {
        status_db,
        durable_object,
        // TODO: do did resolution here? instead of durable object? idk
    };

    let kv = Arc::new(env.kv("KV")?);

    const LAST_SEEN_KEY: &str = "singleton::last_seen::timestamp";

    let cursor = match kv.get(LAST_SEEN_KEY).json().await {
        Ok(Some(Cursor { last_seen })) => last_seen,
        Ok(None) => {
            let now = Utc::now().timestamp_micros();
            let cursor = now - ALARM_INTERVAL_MS;
            cursor
                .try_into()
                .expect("cursor timestamp should not be negative")
        }
        Err(e) => {
            console_error!("error loading last seen key from KV. {}", e);

            let now = Utc::now().timestamp_micros();
            let cursor = now - ALARM_INTERVAL_MS;
            cursor
                .try_into()
                .expect("cursor timestamp should not be negative")
        }
    };

    let last_seen = ingest(&state, cursor)
        .await
        .map_err(|e| worker::Error::RustError(format!("some error on ingest: {}", e)))?;

    console_log!("done ingesting, last seen: {last_seen:?}");

    match last_seen {
        Some(last_seen) => {
            kv.put(LAST_SEEN_KEY, Cursor { last_seen }).map_err(|e| anyhow!("kv: {}", e))?
                .execute()
                .await.map_err(|e| anyhow!("kv: {}", e))?;
        }
        None => {
            console_log!("no events observed (including account/identity events). weird, but not necessarily an error")
        }
    }

    Ok(())
}

pub async fn ingest(
    state: &ScheduledEventState,
    cursor: TimestampMicros,
) -> anyhow::Result<Option<TimestampMicros>> {
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
                let message: Event<xyz::statusphere::status::RecordData> = message_event.json()?;

                handle_jetstream_event(&state, &message).await?;

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
    state: &ScheduledEventState,
    event: &Event<xyz::statusphere::status::RecordData>,
) -> anyhow::Result<()> {
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

                        let updated = state
                            .status_db
                            .save_or_update_from_jetstream(&status)
                            .await?;

                        state.durable_object.broadcast(updated).await?;
                    }
                }
            }
            Operation::Delete => {
                // TODO: could broadcast this to the frontend as an update
                state.status_db.delete_by_uri(&record_uri).await?;
            }
        }
    }

    Ok(())
}



type TimestampMicros = u64;

