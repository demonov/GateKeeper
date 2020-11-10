use async_trait::async_trait;

#[async_trait]
trait KVStore<Key, Err> {
    async fn load<T>(key: Key) -> Result<T, Err>;
    async fn save<T>(key: Key, data: T) -> Result<(), Err>;
}