use atrium_api::types::string::Did;
use atrium_common::store::Store;
use atrium_oauth::store::session::{Session, SessionStore};
use atrium_oauth::store::state::{InternalStateData, StateStore};
use http::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;
use worker::kv::{KvError, KvStore};
use worker::send::SendWrapper;

pub mod cached_resolver;
pub mod session_state;

#[derive(Debug)]
pub struct KvStoreError(SendWrapper<KvError>);

impl From<KvStoreError> for atrium_identity::Error {
    fn from(_value: KvStoreError) -> Self {
        atrium_identity::Error::HttpStatus(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl std::error::Error for KvStoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

impl Display for KvStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<KvError> for KvStoreError {
    fn from(value: KvError) -> Self {
        Self(SendWrapper(value))
    }
}

impl From<serde_json::Error> for KvStoreError {
    fn from(value: serde_json::Error) -> Self {
        Self(SendWrapper(value.into()))
    }
}

#[derive(Clone)]
pub struct KvStoreWrapper<K, V> {
    inner: SendWrapper<Arc<KvStore>>,
    prefix: &'static str,
    expiration_ttl: Duration,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> KvStoreWrapper<K, V> {
    pub fn new(inner: Arc<KvStore>, prefix: &'static str, expiration_ttl: Duration) -> Self {
        Self {
            inner: SendWrapper(inner),
            expiration_ttl,
            prefix,
            _phantom: PhantomData,
        }
    }
}

impl<K, V> Debug for KvStoreWrapper<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("KvStoreWrapper").field(&self.prefix).finish()
    }
}

pub type KvSessionStore = KvStoreWrapper<Did, Session>;
impl SessionStore for KvSessionStore {}

pub type KvStateStore = KvStoreWrapper<String, InternalStateData>;
impl StateStore for KvStateStore {}

impl<K, V> Store<K, V> for KvStoreWrapper<K, V>
where
    K: Debug + Eq + Hash + Send + Sync + AsRef<str>,
    V: Debug + Clone + Send + Sync + 'static + Serialize + DeserializeOwned,
{
    type Error = KvStoreError;

    #[worker::send]
    async fn get(&self, key: &K) -> Result<Option<V>, Self::Error> {
        let key = format!("{}:{}", self.prefix, key.as_ref());
        let s = self.inner.get(&key).text().await?;

        match s {
            Some(s) => Ok(Some(serde_json::from_str(&s)?)),
            None => Ok(None),
        }
    }

    #[worker::send]
    async fn set(&self, key: K, value: V) -> Result<(), Self::Error> {
        let key = format!("{}:{}", self.prefix, key.as_ref());
        // NOTE: manually converting this to a string w/ serde fixed a weird bug I was seeing,
        //       in theory it wouldn't be needed and I could just call put directly but :shrug_emoji:
        self.inner
            .put(&key, serde_json::to_string(&value)?)?
            .expiration_ttl(self.expiration_ttl.as_secs())
            .execute()
            .await?;
        Ok(())
    }

    #[worker::send]
    async fn del(&self, key: &K) -> Result<(), Self::Error> {
        let key = format!("{}:{}", self.prefix, key.as_ref());
        self.inner.delete(&key).await?;

        Ok(())
    }

    #[worker::send]
    async fn clear(&self) -> Result<(), Self::Error> {
        let mut keyset = self
            .inner
            .list()
            .prefix(self.prefix.to_string())
            .execute()
            .await?;
        loop {
            for key in keyset.keys.iter() {
                self.inner.delete(key.name.as_str()).await?
            }

            // NOTE: this could be parallelized
            if let Some(cursor) = keyset.cursor.as_ref() {
                keyset = self
                    .inner
                    .list()
                    .prefix(self.prefix.to_string())
                    .cursor(cursor.to_string())
                    .execute()
                    .await?;
            } else {
                break;
            }
        }

        Ok(())
    }
}
