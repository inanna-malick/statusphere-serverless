use crate::durable_object::jetstream_listener::client::JetstreamListener;
use crate::durable_object::websocket_broker::client::WebsocketBroker;
use crate::services::oauth::OAuthClient;
use crate::storage::db::StatusDb;

#[derive(Clone)]
pub struct AppState {
    pub oauth: OAuthClient,
    pub status_db: StatusDb,
    pub websocket_broker: WebsocketBroker,
    pub jetstream_listener: JetstreamListener,
}
