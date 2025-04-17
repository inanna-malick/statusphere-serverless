use crate::types::status::{Status, StatusFromDb};
use std::sync::Arc;
use worker::{console_log, query, D1Database, Result};

#[derive(Clone)]
pub struct StatusDb(Arc<D1Database>);

impl StatusDb {
    pub fn from_env(env: &worker::Env) -> worker::Result<Self> {
        let d1 = env.d1("DB")?;
        Ok(Self(Arc::new(d1)))
    }

    // optimistic update from local write. Due to race conditions sometimes this hits the db after
    // an update from jetstream from the same uri
    pub async fn save_optimistic(&self, status: &Status) -> Result<StatusFromDb> {
        let res = query!(&self.0, r#"INSERT INTO status (uri, authorDid, status, createdAt, indexedAt, seenOnJetstream, createdViaThisApp) VALUES (?1, ?2, ?3, ?4, ?5, FALSE, TRUE)
                      ON CONFLICT (uri)
                      DO UPDATE
                      SET
                        createdViaThisApp = TRUE
                      RETURNING *
                      "#,
                    &status.uri,
                    &status.author_did,
                    &status.status,
                    &status.created_at,
                    &status.indexed_at,
        )?.first(None).await?;

        // insert or update should _always_ return one row
        let res = res.ok_or(worker::Error::Infallible)?;

        Ok(res)
    }

    /// Saves or updates a status by its did(uri), returning the created/updated row
    pub async fn save_or_update_from_jetstream(&self, status: &Status) -> Result<StatusFromDb> {
        console_log!("save or update from jetstream: {:?}", &status);
        let res = query!(&self.0, r#"INSERT INTO status (uri, authorDid, status, createdAt, indexedAt, seenOnJetstream, createdViaThisApp) VALUES (?1, ?2, ?3, ?4, ?5, TRUE, FALSE)
                      ON CONFLICT (uri)
                      DO UPDATE
                      SET
                        status = ?6,
                        indexedAt = ?7,
                        seenOnJetstream = TRUE 
                      RETURNING *
                      "#,  
                    // insert
                    &status.uri,
                    &status.author_did,
                    &status.status,
                    &status.created_at,
                    &status.indexed_at,
                    // update
                    &status.status,
                    &status.indexed_at,
        )?.first(None).await?;
        // insert or update should _always_ return one row
        let res = res.ok_or(worker::Error::Infallible)?;

        console_log!("save or update from jetstream done: {:?}", &res);

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
        query!(
            &self.0,
            "SELECT * FROM status ORDER BY indexedAt DESC LIMIT ?1",
            n
        )?
        .all()
        .await?
        .results()
    }
}
