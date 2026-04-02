//! Key-Value storage adapter
//! 暂时未使用，保留以备将来需要 Redis KV 存储时使用

use anyhow::Result;

/// Generic KV storage adapter
#[allow(dead_code)]
pub trait KVAdapter: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<String>>;
    fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> Result<()>;
    fn delete(&self, key: &str) -> Result<()>;
}

/// Redis-based KV adapter
/// 暂时未使用，保留以备将来需要 Redis KV 存储时使用
#[cfg(feature = "redis")]
#[allow(dead_code)]
pub struct RedisKVAdapter {
    client: redis::Client,
}

#[cfg(feature = "redis")]
#[allow(dead_code)]
impl RedisKVAdapter {
    pub fn new(url: &str) -> Result<Self> {
        let client = redis::Client::open(url)?;
        Ok(RedisKVAdapter { client })
    }
}

#[cfg(feature = "redis")]
#[allow(dead_code)]
impl KVAdapter for RedisKVAdapter {
    fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.client.get_connection()?;
        let result: Option<String> = redis::Commands::get(&mut conn, key)?;
        Ok(result)
    }

    fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> Result<()> {
        let mut conn = self.client.get_connection()?;
        if let Some(ttl) = ttl_seconds {
            redis::cmd("SET").arg(key).arg(value).arg("EX").arg(ttl).execute(&mut conn);
        } else {
            let _: () = redis::Commands::set(&mut conn, key, value)?;
        }
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.client.get_connection()?;
        let _: i32 = redis::Commands::del(&mut conn, key)?;
        Ok(())
    }
}
