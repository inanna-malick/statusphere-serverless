use std::sync::Arc;

use worker::send::SendWrapper;
use worker::{console_log, ObjectNamespace, Stub};

#[derive(Clone)]
pub struct ListenerClient {
    stub: Arc<SendWrapper<Stub>>,
}

impl ListenerClient {
    pub fn from_namespace(ns: &ObjectNamespace) -> worker::Result<Self> {
        console_log!("from namespace from listener - should init");
        // start in east north america b/c that's where the jetstream server we're using lives
        let stub = Arc::new(SendWrapper(
            ns.id_from_name("single-instance")?
                .get_stub_with_location_hint("enam")?,
        ));
        Ok(Self { stub })
    }

    // NOTE: I Think we kinda just need the above - this doesn't really have a client (altho reading last read date would be nice)

    // poke to ensure alarm is set if needed - TODO: feels inefficient, figure out something better?
    // AHAHA OR: return last read cursor timestamp and show it in the UI to justify the API call
    pub async fn poke(&self) -> worker::Result<()> {
        let request = worker::Request::new("https://stub.com/websocket", worker::Method::Get)?;

        self.stub.fetch_with_request(request).await?;

        Ok(())
    }
}
