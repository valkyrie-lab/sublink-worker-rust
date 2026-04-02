//! Builders module - generates configurations for different clients

pub mod singbox_builder;
pub mod clash_builder;
pub mod surge_builder;
pub mod base_builder;

pub use singbox_builder::SingboxConfigBuilder;
pub use clash_builder::ClashConfigBuilder;
pub use surge_builder::SurgeConfigBuilder;
