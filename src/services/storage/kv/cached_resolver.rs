use atrium_common::resolver::Resolver;
use serde::{de::DeserializeOwned, Serialize};

use super::{KvStoreError, KvStoreWrapper};
use atrium_common::store::Store as _;
use std::fmt::Debug;
use std::hash::Hash;

pub struct KvStoreCachedResolver<T: Resolver>
where
    T::Input: Send + Sized,
    T::Output: Send + Sized,
{
    pub cache: KvStoreWrapper<T::Input, T::Output>,
    pub inner: T,
}

impl<T> Resolver for KvStoreCachedResolver<T>
where
    T: Resolver + Sync + Send,
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
