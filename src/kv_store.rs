use anyhow::Result;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait KVStore<Key> {
    async fn load<T>(&self, key: Key) -> Result<T>
    where
        T: Send + DeserializeOwned;

    async fn save<T>(&self, key: Key, data: T) -> Result<()>
    where
        T: Send + Serialize;
}
