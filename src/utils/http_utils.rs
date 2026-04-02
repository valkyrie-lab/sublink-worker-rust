//! HTTP utility functions

use reqwest::Client;
use anyhow::Result;

// 暂时未使用的常量和函数，保留以备将来需要自定义 User-Agent 时使用
/// Default User-Agent
#[allow(dead_code)]
pub const DEFAULT_USER_AGENT: &str = "curl/7.74.0";

// 暂时未使用的函数
/// Fetch content from URL
#[allow(dead_code)]
pub async fn fetch_url(url: &str) -> Result<String> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", DEFAULT_USER_AGENT)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

/// Fetch content from URL with custom User-Agent
pub async fn fetch_url_with_ua(url: &str, user_agent: &str) -> Result<String> {
    let client = Client::new();
    let response = client
        .get(url)
        .header("User-Agent", user_agent)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

// 暂时未使用的函数
/// Check if URL is valid
#[allow(dead_code)]
pub fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}
