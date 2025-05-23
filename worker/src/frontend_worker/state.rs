use crate::durable_object::client::MessageBroker;
use crate::services::oauth::OAuthClient;
use crate::storage::db::StatusDb;

#[derive(Clone)]
pub struct AppState {
    pub oauth: OAuthClient,
    pub status_db: StatusDb,
    pub durable_object: MessageBroker,
}
