//! Base configuration builder trait
//! 暂时未使用，保留以备将来需要统一 Builder 接口时使用

use anyhow::Result;

/// Base trait for all config builders
#[allow(dead_code)]
pub trait BaseConfigBuilder: Send + Sync {
    fn build(&mut self) -> Result<()>;
    fn format_config(&self) -> String;
}
