//! Configuration storage service

use anyhow::Result;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[cfg(feature = "sqlite")]
use rusqlite::Connection;

use crate::services::errors::ServiceError;

/// Configuration storage service
pub struct ConfigStorageService {
    #[cfg(feature = "sqlite")]
    sqlite: Option<Arc<Mutex<Connection>>>,
    
    #[cfg(not(feature = "sqlite"))]
    _phantom: std::marker::PhantomData<()>,
    
    ttl_seconds: u64,
}

impl ConfigStorageService {
    /// Create a new config storage service
    #[cfg(feature = "sqlite")]
    pub fn new(sqlite: Arc<Mutex<Connection>>, ttl_seconds: u64) -> Self {
        ConfigStorageService {
            sqlite: Some(sqlite),
            ttl_seconds,
        }
    }

    /// Create a new config storage service without SQLite
    #[cfg(not(feature = "sqlite"))]
    pub fn new(_ttl_seconds: u64) -> Self {
        ConfigStorageService {
            _phantom: std::marker::PhantomData,
            ttl_seconds: _ttl_seconds,
        }
    }

    /// Save a configuration and return its ID
    pub async fn save_config(&self, config_type: &str, content: &str) -> Result<String, ServiceError> {
        let config_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let expires_at = if self.ttl_seconds > 0 {
            Some(now + self.ttl_seconds as i64)
        } else {
            None
        };

        #[cfg(feature = "sqlite")]
        {
            if let Some(ref sqlite) = self.sqlite {
                let conn = sqlite.lock().map_err(|e| ServiceError::StorageError(e.to_string()))?;
                let expires_str = expires_at.map(|e| e.to_string()).unwrap_or_default();
                conn.execute(
                    "INSERT INTO configs (config_type, content, created_at, expires_at) VALUES (?, ?, ?, ?)",
                    rusqlite::params![config_type, content, now.to_string(), expires_str],
                )
                .map_err(|e| ServiceError::StorageError(e.to_string()))?;
                return Ok(config_id);
            }
        }

        // Without SQLite, just return the ID (no persistence)
        Ok(config_id)
    }

    /// Get a configuration by ID
    #[allow(dead_code)]
    pub async fn get_config_by_id(&self, config_id: &str) -> Result<Option<String>, ServiceError> {
        #[cfg(feature = "sqlite")]
        {
            if let Some(ref sqlite) = self.sqlite {
                let conn = sqlite.lock().map_err(|e| ServiceError::StorageError(e.to_string()))?;
                let now = chrono::Utc::now().timestamp();
                let mut stmt = conn
                    .prepare("SELECT content FROM configs WHERE id = ? AND (expires_at IS NULL OR expires_at > ?)")
                    .map_err(|e| ServiceError::StorageError(e.to_string()))?;
                
                let mut rows = stmt
                    .query(rusqlite::params![config_id, now.to_string()])
                    .map_err(|e| ServiceError::StorageError(e.to_string()))?;
                
                if let Some(row) = rows.next().map_err(|e| ServiceError::StorageError(e.to_string()))? {
                    let content: String = row.get(0).map_err(|e| ServiceError::StorageError(e.to_string()))?;
                    return Ok(Some(content));
                }
                
                return Ok(None);
            }
        }

        // Without SQLite, config is never found
        Ok(None)
    }
}
