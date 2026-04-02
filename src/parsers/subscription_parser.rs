//! Subscription parser - parses subscription links and formats

use anyhow::Result;
use crate::utils::base64_utils;

/// Subscription parser
pub struct SubscriptionParser;

impl SubscriptionParser {
    /// Parse subscription content into a list of proxy URLs
    pub fn parse(content: &str) -> Result<Vec<String>> {
        // Try Base64 decode first
        if let Ok(decoded) = base64_utils::decode(content) {
            if let Ok(text) = String::from_utf8(decoded) {
                return Ok(Self::extract_proxy_urls(&text));
            }
        }

        // Otherwise treat as plain text
        Ok(Self::extract_proxy_urls(content))
    }

    fn extract_proxy_urls(content: &str) -> Vec<String> {
        content
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty() && Self::is_proxy_url(trimmed)
            })
            .map(|s| s.to_string())
            .collect()
    }

    fn is_proxy_url(line: &str) -> bool {
        line.starts_with("ss://")
            || line.starts_with("vmess://")
            || line.starts_with("vless://")
            || line.starts_with("hysteria2://")
            || line.starts_with("hy2://")
            || line.starts_with("hysteria://")
            || line.starts_with("trojan://")
            || line.starts_with("tuic://")
    }
}
