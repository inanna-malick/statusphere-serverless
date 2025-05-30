use std::sync::Arc;

use crate::types::errors::AppError;
use crate::types::status::StatusFromDb;
use anyhow::{anyhow, Context as _};
use http::{HeaderMap, HeaderValue, Request};
use worker::send::SendWrapper;
use worker::{console_log, request_to_wasm, Env, HttpResponse, Stub};

use super::routing::{all_stubs, Location, DEFAULT_LOCATION};

#[derive(Clone)]
pub struct WebsocketBroker {
    stub: Arc<SendWrapper<Stub>>,
}

impl WebsocketBroker {
    pub fn from_env_and_header(env: &Env, h: &HeaderMap<HeaderValue>) -> worker::Result<Self> {
        let ns = env.durable_object("JETSTREAMLISTENER")?;

        let location = h
            .get("CF-IPCountry")
            .and_then(|h| h.to_str().ok())
            .and_then(Location::from_country_code)
            .unwrap_or(DEFAULT_LOCATION);

        let stub = location.to_durable_object_stub(&ns)?;

        let stub = Arc::new(SendWrapper(stub));
        Ok(Self { stub })
    }

    pub fn for_all_brokers(env: &Env) -> worker::Result<Vec<Self>> {
        let ns = env.durable_object("JETSTREAMLISTENER")?;

        let stubs = all_stubs(&ns)?;

        let stubs = stubs
            .into_iter()
            .map(SendWrapper)
            .map(Arc::new)
            .map(|stub| Self { stub })
            .collect();
        Ok(stubs)
    }

    pub async fn subscriber_websocket(&self) -> Result<HttpResponse, AppError> {
        console_log!("subscriber websocket client handler");
        let mut request =
            worker::Request::new("https://stub.com/subscribe_websocket", worker::Method::Get)
                .context("constructing request")?;

        request.headers_mut()?.append("Upgrade", "websocket")?;

        let resp = self
            .stub
            .fetch_with_request(request)
            .await
            .context("error calling stub")?;

        Ok(resp.try_into().unwrap())
    }

    pub async fn broadcast(&self, status: StatusFromDb) -> anyhow::Result<()> {
        console_log!("broadcast status");
        let req = Request::builder()
            .method("POST")
            .uri("https://stub.com/broadcast_status")
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
