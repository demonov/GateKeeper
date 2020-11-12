use anyhow::Result;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait KVStore<Key> {
    async fn load<T>(key: Key) -> Result<T>
    where
        T: Send + DeserializeOwned;

    async fn save<T>(key: Key, data: T) -> Result<()>
    where
        T: Send + Serialize;
}
