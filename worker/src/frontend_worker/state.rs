use std::sync::Arc;

use crate::durable_object::client::MessageBroker;
use crate::services::oauth::OAuthClient;
use crate::services::resolvers::DidResolver;
use crate::storage::db::StatusDb;

#[derive(Clone)]
pub struct AppState {
    pub oauth: OAuthClient,
    pub status_db: StatusDb,
    pub durable_object: MessageBroker,
    pub did_resolver: Arc<DidResolver>,
}

#[derive(Clone)]
pub struct ScheduledEventState {
    pub status_db: StatusDb,
    pub durable_object: MessageBroker,
}
