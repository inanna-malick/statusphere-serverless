use std::sync::Arc;

use crate::types::errors::AppError;
use crate::types::status::StatusFromDb;
use anyhow::{anyhow, Context as _};
use http::Request;
use worker::send::SendWrapper;
use worker::{console_log, request_to_wasm, HttpResponse, ObjectNamespace, Stub};

#[derive(Clone)]
pub struct MessageBroker {
    msg_broker: Arc<SendWrapper<Stub>>,
}

impl MessageBroker {
    pub fn from_namespace(ns: &ObjectNamespace) -> worker::Result<Self> {
        // TODO: per-region instances
        let msg_broker = Arc::new(SendWrapper(ns.id_from_name("single-instance")?.get_stub()?));
        Ok(Self { msg_broker })
    }

    pub async fn subscriber_websocket(&self) -> Result<HttpResponse, AppError> {
        console_log!("subscriber websocket client handler");
        let mut request =
            worker::Request::new("https://stub.com/subscribe_websocket", worker::Method::Get)
                .context("constructing request")?;

        request.headers_mut()?.append("Upgrade", "websocket")?;

        let resp = self
            .msg_broker
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
        self.msg_broker
            .fetch_with_request(req.into())
            .await
            .map_err(|e| anyhow!("fetch with request {e:?}"))?;

        Ok(())
    }
}
