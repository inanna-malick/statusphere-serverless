use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use clap::Parser;
use clap::command;
use reqwest::header::HeaderMap;
use rocketman::{
    connection::JetstreamConnection, handler, ingestion::LexiconIngestor,
    options::JetstreamOptions, types::event::Event,
};
use serde_json::Value;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async,
    tungstenite::{Bytes, client::IntoClientRequest as _},
};

use futures::sink::SinkExt;
use headers::{Authorization, HeaderMapExt as _};

const STATUSPHERE_NSID: &str = "xyz.statusphere.status";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// URL to publish messages to
    host: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // init the builder
    let opts = JetstreamOptions::builder()
        // your EXACT nsids
        .wanted_collections(vec![STATUSPHERE_NSID.to_string()])
        .build();
    // create the jetstream connector
    let jetstream = JetstreamConnection::new(opts);

    let (msg_queue_sender, mut msg_queue_receiver) = mpsc::channel(100);

    // create your ingestors
    let mut ingestors: HashMap<String, Box<dyn LexiconIngestor + Send + Sync>> = HashMap::new();
    ingestors.insert(
        // your EXACT nsid
        STATUSPHERE_NSID.to_string(),
        Box::new(MyCoolIngestor(Arc::new(msg_queue_sender))),
    );

    // tracks the last message we've processed
    let cursor: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(None));

    // get channels
    let msg_rx = jetstream.get_msg_rx();
    let reconnect_tx = jetstream.get_reconnect_tx();

    // spawn a task to process messages from the queue.
    // this is a simple implementation, you can use a more complex one based on needs.
    let c_cursor = cursor.clone();
    tokio::spawn(async move {
        while let Ok(message) = msg_rx.recv_async().await {
            if let Err(e) =
                handler::handle_message(message, &ingestors, reconnect_tx.clone(), c_cursor.clone())
                    .await
            {
                eprintln!("Error processing message: {}", e);
            };
        }
    });

    println!("here");

    tokio::spawn(async move {
        let http_client = reqwest::Client::new();

        while let Some(message) = msg_queue_receiver.recv().await {
            // DO NOT USE THIS IN PRODUCTION, USE ACTUAL AUTH VIA SECRET KEY OR SIMILAR
            let mut headers = HeaderMap::new();
            headers.typed_insert(Authorization::basic("admin", "hunter2"));

            match http_client
                .post(format!(
                    "https://{}/admin/publish_jetstream_event",
                    cli.host
                ))
                .headers(headers)
                .json(&message)
                .send()
                .await
            {
                Ok(x) => {

                    eprintln!("jetstream send event res: {:?}", x)

                }
                Err(e) => panic!("jetstream event send failed {}", e),
            }
        }

        eprintln!("message queue closed - all handles dropped?")
    });

    // connect to jetstream
    // retries internally, but may fail if there is an extreme error.
    if let Err(e) = jetstream.connect(cursor.clone()).await {
        eprintln!("Failed to connect to Jetstream: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

pub struct MyCoolIngestor(Arc<mpsc::Sender<Event<Value>>>);

#[async_trait]
impl LexiconIngestor for MyCoolIngestor {
    async fn ingest(&self, message: Event<Value>) -> anyhow::Result<()> {
        println!("{:?}", message);

        // TODO: decouple, handle reconnects, etc
        self.0.send(message).await?;

        Ok(())
    }
}
