// TODO (w/keyboard) move all agent code here

use super::oauth;

use crate::types::errors::AppError;
use crate::types::lexicons::xyz::statusphere::status;
use crate::types::lexicons::{record::KnownRecord, xyz::statusphere::Status};
use anyhow::Context as _;
use atrium_api::app::bsky::actor::defs::ProfileViewDetailedData;
use atrium_api::app::bsky::actor::get_profile;
use atrium_api::com::atproto::repo::{create_record, list_records};
use atrium_api::types::TryFromUnknown as _;
use atrium_api::{
    agent::Agent as AtriumAgent,
    types::{
        string::{Datetime, Did},
        Collection,
    },
};

pub struct Agent {
    inner: AtriumAgent<oauth::SessionType>,
    did: Did,
}

impl Agent {
    pub fn from_session(session: oauth::SessionType, did: Did) -> Self {
        Self {
            did,
            inner: AtriumAgent::new(session),
        }
    }
}

impl Agent {
    pub async fn current_status(&self) -> Result<Option<status::RecordData>, AppError> {
        let record = self
            .inner
            .api
            .com
            .atproto
            .repo
            .list_records(
                list_records::ParametersData {
                    collection: Status::NSID.parse().unwrap(),
                    repo: self.did.clone().into(),
                    cursor: None,
                    limit: Some(1.try_into().unwrap()),
                    reverse: None,
                }
                .into(),
            )
            .await
            .context("get status records for user")?;

        // take most recent status record from user's repo
        let current_status = if let Some(record) = record.data.records.into_iter().next() {
            Some(
                status::RecordData::try_from_unknown(record.data.value)
                    .context("decoding status record")?,
            )
        } else {
            None
        };

        Ok(current_status)
    }

    pub async fn create_status(
        &self,
        status: String,
    ) -> Result<create_record::OutputData, AppError> {
        let status: KnownRecord = crate::types::lexicons::xyz::statusphere::status::RecordData {
            created_at: Datetime::now(),
            status,
        }
        .into();

        // TODO no data validation yet from esquema
        // Maybe you'd like to add it? https://github.com/fatfingers23/esquema/issues/3

        let record = self
            .inner
            .api
            .com
            .atproto
            .repo
            .create_record(
                create_record::InputData {
                    collection: Status::NSID.parse().unwrap(),
                    repo: self.did.clone().into(),
                    rkey: None,
                    record: status.into(),
                    swap_commit: None,
                    validate: None,
                }
                .into(),
            )
            .await
            .context("publish status via agent")?;

        Ok(record.data)
    }

    // TODO: rewrite to directly act on app.bsky.actor.profile record?
    pub async fn bsky_profile(&self) -> Result<ProfileViewDetailedData, AppError> {
        let profile = self
            .inner
            .api
            .app
            .bsky
            .actor
            .get_profile(
                get_profile::ParametersData {
                    actor: self.did.clone().into(),
                }
                .into(),
            )
            .await?;

        Ok(profile.data)
    }
}
