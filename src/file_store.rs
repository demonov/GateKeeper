use crate::kv_store::KVStore;
use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;

struct FileStore {
    pub folder:String,
}

impl FileStore {
    pub fn new(folder: String) -> Self {
        Self {
            folder
        }
    }
}

#[async_trait]
impl KVStore<String> for FileStore {
    async fn load<T: Send + DeserializeOwned>(key: String) -> Result<T> {
        unimplemented!()
    }

    async fn save<T: Send + Serialize>(key: String, data: T) -> Result<()> {
        unimplemented!()
    }
}
