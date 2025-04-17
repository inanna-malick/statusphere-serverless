use async_trait::async_trait;
use atrium_common::store::Store;
use tower_sessions::session::{Id, Record};
use tower_sessions::session_store::{Error, Result};

use super::KvStoreWrapper;

pub type KvTowerSessionStore = KvStoreWrapper<String, Record>;

#[async_trait]
impl tower_sessions::SessionStore for KvTowerSessionStore {
    async fn create(&self, record: &mut Record) -> Result<()> {
        let mut record = record.clone();

        while self
            .get(&record.id.to_string())
            .await
            .map_err(|e| Error::Backend(e.to_string()))?
            .is_some()
        {
            // Session ID collision mitigation.
            record.id = Id::default();
        }

        self.set(record.id.to_string(), record.clone())
            .await
            .map_err(|e| Error::Backend(e.to_string()))
    }

    async fn save(&self, record: &Record) -> Result<()> {
        let record = record.clone();
        self.set(record.id.to_string(), record)
            .await
            .map_err(|e| Error::Backend(e.to_string()))
    }

    async fn load(&self, session_id: &Id) -> Result<Option<Record>> {
        let k = session_id.to_string();

        let res = self
            .get(&k)
            .await
            .map_err(|e| Error::Backend(e.to_string()))?;

        fn is_active(expiry_date: time::OffsetDateTime) -> bool {
            expiry_date > time::OffsetDateTime::now_utc()
        }

        Ok(res.filter(|r| is_active(r.expiry_date)))
    }

    async fn delete(&self, session_id: &Id) -> Result<()> {
        self.del(&session_id.to_string())
            .await
            .map_err(|e| Error::Backend(e.to_string()))
    }
}
