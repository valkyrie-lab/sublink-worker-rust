//! Base configuration builder trait

use anyhow::Result;

/// Base trait for all config builders
pub trait BaseConfigBuilder: Send + Sync {
    fn build(&mut self) -> Result<()>;
    fn format_config(&self) -> String;
}
