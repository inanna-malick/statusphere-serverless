use crate::durable_object::client::MessageBroker;
use crate::frontend_worker::state::ScheduledEventState;
use crate::storage::db::StatusDb;
use crate::types::status::Status;
use atrium_api::types::Collection as _;
use worker::{console_error, console_log, Env, WebSocket};

use worker::WebsocketEvent;

use crate::types::jetstream::{Event, Operation};
use crate::types::lexicons::xyz;
use anyhow::anyhow;
use atrium_api::types::string::Did;
use chrono::Utc;
use futures::StreamExt as _;

const ALARM_INTERVAL_MS: i64 = 5 * 60 * 1000; // 5 minutes
const ALARM_INTERVAL_MICROS: i64 = ALARM_INTERVAL_MS * 1000;

pub async fn ingest_(env: Env) -> anyhow::Result<()> {
    let status_db = StatusDb::from_env(&env)?;

    let ns = env.durable_object("MSGBROKER")?;
    let durable_object = MessageBroker::from_namespace(&ns)?;

    let state = ScheduledEventState {
        status_db: status_db.clone(),
        durable_object,
    };

    let cursor = match status_db.get_jetstream_cursor().await {
        Ok(Some(last_seen)) if last_seen > 0 => last_seen,
        Ok(_) => {
            console_log!("no valid cursor found in database, inserting default");
            let now = Utc::now().timestamp_micros();
            let default_cursor: u64 = (now - ALARM_INTERVAL_MICROS)
                .try_into()
                .expect("cursor timestamp should not be negative");

            // Insert the default cursor
            if let Err(e) = status_db.insert_jetstream_cursor(default_cursor).await {
                console_error!("failed to insert default cursor: {}", e);
            }

            default_cursor
        }
        Err(e) => {
            console_error!("error loading cursor from database: {}", e);
            let now = Utc::now().timestamp_micros();
            let default_cursor: u64 = (now - ALARM_INTERVAL_MICROS)
                .try_into()
                .expect("cursor timestamp should not be negative");

            // Try to insert the default cursor
            if let Err(e) = status_db.insert_jetstream_cursor(default_cursor).await {
                console_error!("failed to insert default cursor: {}", e);
            }

            default_cursor
        }
    };

    let last_seen = ingest(&state, cursor)
        .await
        .map_err(|e| worker::Error::RustError(format!("some error on ingest: {}", e)))?;

    console_log!("done ingesting, last seen: {last_seen:?}");

    match last_seen {
        Some(last_seen) => {
            status_db
                .update_jetstream_cursor(last_seen)
                .await
                .map_err(|e| anyhow!("failed to update cursor in database: {}", e))?;
            console_log!("updated cursor in database to: {}", last_seen);
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

                if let Some(time_us) = message.time_us {
                    last_seen = Some(time_us);
                }

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
