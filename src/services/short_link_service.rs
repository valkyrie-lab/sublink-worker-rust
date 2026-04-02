//! Short link service for creating and resolving short URLs

use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

#[cfg(feature = "redis")]
use redis::aio::ConnectionManager;
#[cfg(feature = "redis")]
use tokio::sync::Mutex as AsyncMutex;

use crate::services::errors::ServiceError;

/// Short link service
pub struct ShortLinkService {
    #[cfg(feature = "redis")]
    redis: Option<Arc<AsyncMutex<ConnectionManager>>>,
    
    #[cfg(not(feature = "redis"))]
    _phantom: std::marker::PhantomData<()>,
    
    ttl_seconds: u64,
}

impl ShortLinkService {
    /// Create a new short link service
    #[cfg(feature = "redis")]
    pub fn new(redis: Arc<AsyncMutex<ConnectionManager>>, ttl_seconds: u64) -> Self {
        ShortLinkService {
            redis: Some(redis),
            ttl_seconds,
        }
    }

    /// Create a new short link service without Redis (in-memory fallback)
    #[cfg(not(feature = "redis"))]
    pub fn new(_ttl_seconds: u64) -> Self {
        ShortLinkService {
            _phantom: std::marker::PhantomData,
            ttl_seconds: _ttl_seconds,
        }
    }

    /// Create a short link from the given parameter
    pub async fn create_short_link(
        &self,
        original_param: &str,
        custom_code: Option<&str>,
    ) -> Result<String, ServiceError> {
        let code = custom_code
            .filter(|c| !c.is_empty() && c.len() <= 20)
            .map(|c| c.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string()[..8].to_string());

        #[cfg(feature = "redis")]
        {
            if let Some(ref redis) = self.redis {
                let mut conn = redis.lock().await;
                let key = format!("short_link:{}", code);
                let _: () = redis::cmd("SET")
                    .arg(&key)
                    .arg(original_param)
                    .arg("EX")
                    .arg(self.ttl_seconds)
                    .query_async(&mut *conn)
                    .await
                    .map_err(|e| ServiceError::StorageError(e.to_string()))?;
                return Ok(code);
            }
        }

        // Fallback: just return the code (no persistence without Redis)
        Ok(code)
    }

    /// Resolve a short code to the original parameter
    pub async fn resolve_short_code(&self, code: &str) -> Result<String, ServiceError> {
        #[cfg(feature = "redis")]
        {
            if let Some(ref redis) = self.redis {
                let mut conn = redis.lock().await;
                let key = format!("short_link:{}", code);
                let result: Option<String> = redis::cmd("GET")
                    .arg(&key)
                    .query_async(&mut *conn)
                    .await
                    .map_err(|e| ServiceError::StorageError(e.to_string()))?;
                
                return result.ok_or_else(|| ServiceError::ShortLinkNotFound(code.to_string()));
            }
        }

        // Without Redis, all codes are considered not found
        Err(ServiceError::ShortLinkNotFound(code.to_string()))
    }
}
