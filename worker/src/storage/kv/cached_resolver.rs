use atrium_common::resolver::{CachedResolver, Resolver};
use atrium_common::types::cached::r#impl::{Cache as _, CacheImpl};
use atrium_common::types::cached::CacheConfig;
use serde::{de::DeserializeOwned, Serialize};

use super::{KvStoreError, KvStoreWrapper};
use atrium_common::store::Store as _;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;

const CACHE_TTL: Duration = Duration::new(60 * 60 * 6, 0);

pub struct KvStoreCachedResolver<T: Resolver>
where
    T::Input: Send + Sized,
    T::Output: Send + Sized,
{
    pub cache: KvStoreWrapper<T::Input, T::Output>,
    pub inner: CachedResolver<T>,
}

impl<R: Resolver> KvStoreCachedResolver<R>
where
    R::Input: Send + Sized + Eq + Hash + Sync + 'static,
    R::Output: Send + Sized + Clone + Sync + 'static,
{
    pub fn new(inner: R, kv: Arc<worker::kv::KvStore>, prefix: &'static str) -> Self {
        KvStoreCachedResolver {
            inner: CachedResolver::new(
                inner,
                CacheImpl::new(CacheConfig {
                    max_capacity: Some(100),
                    time_to_live: Some(CACHE_TTL),
                }),
            ),

            cache: KvStoreWrapper::new(kv, prefix, CACHE_TTL),
        }
    }
}

impl<T> Resolver for KvStoreCachedResolver<T>
where
    T: Resolver + Sync + Send + 'static,
    T::Error: Send + From<KvStoreError>,
    T::Input: Send + Sized + Debug + Eq + Hash + Sync + AsRef<str> + Clone,
    T::Output: Send + Sized + Debug + Clone + Sync + 'static + Serialize + DeserializeOwned,
{
    type Input = T::Input;
    type Output = T::Output;
    type Error = T::Error;

    async fn resolve(&self, handle: &Self::Input) -> Result<Self::Output, Self::Error> {
        match self.cache.get(handle).await? {
            Some(cached) => Ok(cached),
            None => {
                let resolved = self.inner.resolve(handle).await?;

                self.cache.set(handle.clone(), resolved.clone()).await?;

                Ok(resolved)
            }
        }
    }
}
