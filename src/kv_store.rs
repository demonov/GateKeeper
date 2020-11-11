use anyhow::Result;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait KVStore<Key> {
    async fn load<T: Send + DeserializeOwned>(key: Key) -> Result<T>;
    async fn save<T: Send + Serialize>(key: Key, data: T) -> Result<()>;
}
