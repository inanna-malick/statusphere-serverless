use crate::types::status::StatusFromDb;
use std::sync::Arc;
use worker::{query, D1Database, Result};

#[derive(Clone)]
pub struct StatusDb(Arc<D1Database>);

impl StatusDb {
    pub fn new(d: D1Database) -> Self {
        Self(Arc::new(d))
    }

    /// Saves the [StatusDb]
    pub async fn save(&self, status: &StatusFromDb) -> Result<()> {
        query!(&self.0, "INSERT INTO status (uri, authorDid, status, createdAt, indexedAt, source) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    &status.uri,
                    &status.author_did,
                    &status.status,
                    &status.created_at,
                    &status.indexed_at,
                    &status.source,
        )?.run().await?;

        Ok(())
    }

    /// Saves or updates a status by its did(uri), returning the created/updated row
    pub async fn save_or_update_from_jetstream(
        &self,
        status: &StatusFromDb,
    ) -> Result<StatusFromDb> {
        let res = query!(&self.0, r#"INSERT INTO status (uri, authorDid, status, createdAt, indexedAt, source) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                      ON CONFLICT (uri)
                      DO UPDATE
                      SET
                        status = ?7,
                        indexedAt = ?8,
                        source = CASE WHEN source='ThisInstance' THEN 'ThisInstanceAndJetstream'
                                      ELSE 'Jetstream'
                                 END
                      RETURNING *
                      "#,  
                    // insert
                    &status.uri,
                    &status.author_did,
                    &status.status,
                    &status.created_at,
                    &status.indexed_at,
                    &status.source,
                    // update
                    &status.status,
                    &status.indexed_at,
        )?.first(None).await?;
        // insert or update should _always_ return one row
        let res = res.ok_or(worker::Error::Infallible)?;
        Ok(res)
    }

    /// delete a status
    pub async fn delete_by_uri(&self, uri: &str) -> Result<()> {
        query!(&self.0, "DELETE FROM status WHERE uri = ?1", &uri)?
            .run()
            .await?;

        Ok(())
    }

    /// Loads the last n statuses we have saved
    pub async fn load_latest_statuses(&self, n: usize) -> Result<Vec<StatusFromDb>> {
        let results: Vec<serde_json::Value> = query!(
            &self.0,
            "SELECT * FROM status ORDER BY indexedAt DESC LIMIT ?1",
            n
        )?
        .all()
        .await?
        .results()?;

        let mut res = Vec::new();

        for v in results.into_iter() {
            res.push(serde_json::from_value(v)?);
        }

        Ok(res)
    }

    /// Loads the logged-in user's current status
    pub async fn my_status(&self, did: &str) -> Result<Option<StatusFromDb>> {
        let res = query!(
            &self.0,
            "SELECT * FROM status WHERE authorDid = ?1 ORDER BY createdAt DESC LIMIT 1",
            &did
        )?
        .first(None)
        .await?;

        let res = res.map(serde_json::from_value).transpose()?;

        Ok(res)
    }
}
