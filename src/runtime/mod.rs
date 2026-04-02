//! Runtime module - provides runtime environment abstraction

use anyhow::Result;
use std::sync::Arc;

#[cfg(feature = "redis")]
use redis::aio::ConnectionManager;
#[cfg(feature = "redis")]
use tokio::sync::Mutex as AsyncMutex;

#[cfg(feature = "sqlite")]
use rusqlite::Connection;
#[cfg(feature = "sqlite")]
use std::sync::Mutex;

use crate::config::AppConfig;

/// Runtime environment
pub struct Runtime {
    pub config: AppConfig,
    
    #[cfg(feature = "redis")]
    pub redis: Option<Arc<AsyncMutex<ConnectionManager>>>,
    
    #[cfg(not(feature = "redis"))]
    pub redis: Option<()>,
    
    #[cfg(feature = "sqlite")]
    pub sqlite: Option<Arc<Mutex<Connection>>>,
    
    #[cfg(not(feature = "sqlite"))]
    pub sqlite: Option<()>,
}

impl Runtime {
    /// Create a new runtime environment
    pub async fn new(config: AppConfig) -> Result<Self> {
        #[cfg(feature = "redis")]
        let redis = if let Some(ref url) = config.redis_url {
            let client = redis::Client::open(url.as_str())?;
            let manager = ConnectionManager::new(client).await?;
            Some(Arc::new(AsyncMutex::new(manager)))
        } else {
            None
        };

        #[cfg(not(feature = "redis"))]
        let redis = None;

        #[cfg(feature = "sqlite")]
        let sqlite = if let Some(ref path) = config.database_path {
            let conn = Connection::open(path)?;
            // Initialize tables
            init_sqlite(&conn)?;
            Some(Arc::new(Mutex::new(conn)))
        } else {
            None
        };

        #[cfg(not(feature = "sqlite"))]
        let sqlite = None;

        Ok(Runtime {
            config,
            redis,
            sqlite,
        })
    }
}

#[cfg(feature = "sqlite")]
fn init_sqlite(conn: &Connection) -> Result<()> {
    // Create short_links table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS short_links (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            code TEXT NOT NULL UNIQUE,
            original_param TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            expires_at INTEGER
        )",
        [],
    )?;

    // Create configs table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS configs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            config_type TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            expires_at INTEGER
        )",
        [],
    )?;

    // Create index on code
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_short_links_code ON short_links(code)",
        [],
    )?;

    Ok(())
}
