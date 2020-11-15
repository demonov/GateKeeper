use crate::kv_store::KVStore;
use anyhow::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::prelude::io::*;

struct FileStore {
    pub folder: String,
    pub extension: String,
}

impl FileStore {
    pub fn new(folder: &str, extension: &str) -> Self {
        Self {
            folder: folder.to_string(),
            extension: extension.to_string(),
        }
    }

    fn get_file_name(&self, key: String) -> PathBuf {
        let mut result = PathBuf::new();
        result.push(&self.folder);
        result.push(key);
        result.set_extension(&self.extension);
        result
    }
}

#[async_trait]
impl KVStore<String> for FileStore {
    async fn load<T>(&self, key: String) -> Result<T>
    where
        T: Send + DeserializeOwned,
    {
        let file_name = self.get_file_name(key);
        let mut file = File::open(file_name).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        let result = serde_json::from_str(&content)?;
        Ok(result)
    }

    async fn save<T>(&self, key: String, data: T) -> Result<()>
    where
        T: Send + Serialize,
    {
        let file_name = self.get_file_name(key);
        let content = serde_json::to_vec_pretty(&data)?;
        let mut file = File::create(file_name).await?;
        file.write_all(content.as_slice()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file_name_test() {
        let fs = FileStore::new("d:\\some\\folder", "extension");
        let path = fs.get_file_name("file".to_string());

        assert_eq!(path.as_os_str(), "d:\\some\\folder\\file.extension");
    }
}
