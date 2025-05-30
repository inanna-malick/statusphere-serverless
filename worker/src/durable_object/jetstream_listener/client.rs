use std::sync::Arc;

use crate::types::jetstream;
use crate::types::lexicons::xyz;
use anyhow::{anyhow, Context as _};
use http::Request;
use worker::send::SendWrapper;
use worker::{request_to_wasm, Env, Stub};

#[derive(Clone)]
pub struct JetstreamListener {
    stub: Arc<SendWrapper<Stub>>,
}

impl JetstreamListener {
    pub fn from_env(env: &Env) -> worker::Result<Self> {
        let ns = env.durable_object("JETSTREAMLISTENER")?;

        let stub = Arc::new(SendWrapper(
            ns.id_from_name("single-instance")?
                // spawn in enam b/c that's where the jetstream instance we're reading from lives
                .get_stub_with_location_hint("enam")?,
        ));
        Ok(Self { stub })
    }

    // poke when new websocket connection established, if no keepalive in last N it self destructs
    pub async fn keepalive(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn broadcast_jetstream_event(
        &self,
        status: jetstream::Event<xyz::statusphere::status::RecordData>,
    ) -> anyhow::Result<()> {
        let req = Request::builder()
            .method("POST")
            .uri("https://stub.com/broadcast_jetstream_event")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&status).context("convert to json")?)
            .context("building request")?;

        let req = request_to_wasm(req).context("building req")?;

        // send update to message broker
        self.stub
            .fetch_with_request(req.into())
            .await
            .map_err(|e| anyhow!("fetch with request {e:?}"))?;

        Ok(())
    }
}
