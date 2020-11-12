use crate::kv_store::KVStore;
use anyhow::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;

struct FileStore {
    pub folder: String,
}

impl FileStore {
    pub fn new(folder: &str) -> Self {
        Self {
            folder: folder.to_string(),
        }
    }
}

#[async_trait]
impl KVStore<String> for FileStore {
    async fn load<T>(key: String) -> Result<T>
    where
        T: Send + DeserializeOwned,
    {
        unimplemented!()
    }

    async fn save<T>(key: String, data: T) -> Result<()>
    where
        T: Send + Serialize,
    {
        unimplemented!()
    }
}
