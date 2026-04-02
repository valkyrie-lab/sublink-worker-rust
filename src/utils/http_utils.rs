//! HTTP utility functions

use reqwest::Client;
use anyhow::Result;

/// Default User-Agent
pub const DEFAULT_USER_AGENT: &str = "curl/7.74.0";

/// Fetch content from URL
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

/// Check if URL is valid
pub fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}
