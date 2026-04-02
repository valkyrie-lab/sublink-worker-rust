//! Configuration module

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Server host
    #[serde(default = "default_host")]
    pub host: String,
    
    /// Server port
    #[serde(default = "default_port")]
    pub port: u16,
    
    /// Redis URL (optional)
    pub redis_url: Option<String>,
    
    /// SQLite database path (optional)
    pub database_path: Option<String>,
    
    /// Short link TTL in seconds
    #[serde(default = "default_short_link_ttl")]
    pub short_link_ttl_seconds: u64,
    
    /// Config TTL in seconds
    #[serde(default = "default_config_ttl")]
    pub config_ttl_seconds: u64,
    
    /// Log level
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8787
}

fn default_short_link_ttl() -> u64 {
    3600 // 1 hour
}

fn default_config_ttl() -> u64 {
    86400 // 24 hours
}

fn default_log_level() -> String {
    "info".to_string()
}

impl AppConfig {
    /// Load configuration from file and environment
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
        
        if Path::new(&config_path).exists() {
            let config_str = std::fs::read_to_string(&config_path)?;
            let config: AppConfig = toml::from_str(&config_str)?;
            Ok(config)
        } else {
            // Load from environment
            Ok(AppConfig {
                host: std::env::var("HOST").unwrap_or_else(|_| default_host()),
                port: std::env::var("PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or_else(default_port),
                redis_url: std::env::var("REDIS_URL").ok(),
                database_path: std::env::var("DATABASE_PATH").ok(),
                short_link_ttl_seconds: std::env::var("SHORT_LINK_TTL")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(default_short_link_ttl),
                config_ttl_seconds: std::env::var("CONFIG_TTL")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(default_config_ttl),
                log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| default_log_level()),
            })
        }
    }
}

/// Predefined rule sets
pub mod rules {
    
    
    /// Unified rule structure
    pub struct Rule {
        pub name: &'static str,
        pub site_rules: &'static [&'static str],
        pub ip_rules: &'static [&'static str],
    }
    
    pub const UNIFIED_RULES: &[Rule] = &[
        Rule {
            name: "Ad Block",
            site_rules: &["category-ads-all"],
            ip_rules: &[],
        },
        Rule {
            name: "AI Services",
            site_rules: &["category-ai-!cn"],
            ip_rules: &[],
        },
        Rule {
            name: "Bilibili",
            site_rules: &["bilibili"],
            ip_rules: &[],
        },
        Rule {
            name: "Youtube",
            site_rules: &["youtube"],
            ip_rules: &[],
        },
        Rule {
            name: "Google",
            site_rules: &["google"],
            ip_rules: &["google"],
        },
        Rule {
            name: "Private",
            site_rules: &[],
            ip_rules: &["private"],
        },
        Rule {
            name: "Location:CN",
            site_rules: &["geolocation-cn", "cn"],
            ip_rules: &["cn"],
        },
        Rule {
            name: "Telegram",
            site_rules: &[],
            ip_rules: &["telegram"],
        },
        Rule {
            name: "Github",
            site_rules: &["github", "gitlab"],
            ip_rules: &[],
        },
        Rule {
            name: "Microsoft",
            site_rules: &["microsoft"],
            ip_rules: &[],
        },
        Rule {
            name: "Apple",
            site_rules: &["apple"],
            ip_rules: &[],
        },
        Rule {
            name: "Social Media",
            site_rules: &["facebook", "instagram", "twitter", "tiktok", "linkedin"],
            ip_rules: &[],
        },
        Rule {
            name: "Streaming",
            site_rules: &["netflix", "hulu", "disney", "hbo", "amazon", "bahamut"],
            ip_rules: &[],
        },
        Rule {
            name: "Gaming",
            site_rules: &["steam", "epicgames", "ea", "ubisoft", "blizzard"],
            ip_rules: &[],
        },
        Rule {
            name: "Education",
            site_rules: &["coursera", "edx", "udemy", "khanacademy", "category-scholar-!cn"],
            ip_rules: &[],
        },
        Rule {
            name: "Financial",
            site_rules: &["paypal", "visa", "mastercard", "stripe", "wise"],
            ip_rules: &[],
        },
        Rule {
            name: "Cloud Services",
            site_rules: &["aws", "azure", "digitalocean", "heroku", "dropbox"],
            ip_rules: &[],
        },
        Rule {
            name: "Non-China",
            site_rules: &["geolocation-!cn"],
            ip_rules: &[],
        },
    ];
    
    /// Rule names that should default to DIRECT instead of Node Select
    pub const DIRECT_DEFAULT_RULES: &[&str] = &["Private", "Location:CN"];
    
    pub const MINIMAL: &[&str] = &["Location:CN", "Private", "Non-China"];
    pub const BALANCED: &[&str] = &[
        "Location:CN", "Private", "Non-China", "Github", "Google", "Youtube", "AI Services", "Telegram",
    ];
    pub const COMPREHENSIVE: &[&str] = &[
        "Ad Block", "AI Services", "Bilibili", "Youtube", "Google", "Private", 
        "Location:CN", "Telegram", "Github", "Microsoft", "Apple", "Social Media",
        "Streaming", "Gaming", "Education", "Financial", "Cloud Services", "Non-China",
    ];
    
    pub fn get_preset(name: &str) -> Option<&[&str]> {
        match name {
            "minimal" => Some(MINIMAL),
            "balanced" => Some(BALANCED),
            "comprehensive" => Some(COMPREHENSIVE),
            _ => None,
        }
    }
    
    pub fn get_rule_by_name(name: &str) -> Option<&'static Rule> {
        UNIFIED_RULES.iter().find(|rule| rule.name == name)
    }
    
    pub fn get_all_rule_names() -> &'static [&'static str] {
        &[
            "Ad Block", "AI Services", "Bilibili", "Youtube", "Google", "Private",
            "Location:CN", "Telegram", "Github", "Microsoft", "Apple", "Social Media",
            "Streaming", "Gaming", "Education", "Financial", "Cloud Services", "Non-China",
        ]
    }
}

/// Rule set base URLs
pub mod rule_urls {
    pub const SITE_RULE_SET_BASE_URL: &str = "https://raw.githubusercontent.com/lyc8503/sing-box-rules/rule-set-geosite/";
    pub const IP_RULE_SET_BASE_URL: &str = "https://raw.githubusercontent.com/lyc8503/sing-box-rules/rule-set-geoip/";
    pub const CLASH_SITE_RULE_SET_BASE_URL: &str = "https://raw.githubusercontent.com/MetaCubeX/meta-rules-dat/meta/geo/geosite/";
    pub const CLASH_IP_RULE_SET_BASE_URL: &str = "https://raw.githubusercontent.com/MetaCubeX/meta-rules-dat/meta/geo/geoip/";
    pub const SURGE_SITE_RULE_SET_BASEURL: &str = "https://raw.githubusercontent.com/DivineEngine/Profiles/master/Surge/";
    pub const SURGE_IP_RULE_SET_BASEURL: &str = "https://raw.githubusercontent.com/DivineEngine/Profiles/master/Surge/";
}

/// SingBox default configurations
pub mod singbox_config {
    use serde_json::json;
    
    pub fn default_v1_12() -> serde_json::Value {
        json!({
            "log": {
                "level": "info",
                "timestamp": true
            },
            "dns": {
                "servers": [
                    {
                        "type": "tcp",
                        "tag": "dns_proxy",
                        "server": "1.1.1.1",
                        "detour": "🚀 节点选择",
                        "domain_resolver": "dns_resolver"
                    },
                    {
                        "type": "https",
                        "tag": "dns_direct",
                        "server": "dns.alidns.com",
                        "domain_resolver": "dns_resolver"
                    },
                    {
                        "type": "udp",
                        "tag": "dns_resolver",
                        "server": "223.5.5.5"
                    },
                    {
                        "type": "fakeip",
                        "tag": "dns_fakeip",
                        "inet4_range": "198.18.0.0/15",
                        "inet6_range": "fc00::/18"
                    }
                ],
                "rules": [
                    {
                        "rule_set": "geolocation-!cn",
                        "query_type": ["A", "AAAA"],
                        "server": "dns_fakeip"
                    },
                    {
                        "rule_set": "geolocation-!cn",
                        "query_type": "CNAME",
                        "server": "dns_proxy"
                    },
                    {
                        "query_type": ["A", "AAAA", "CNAME"],
                        "invert": true,
                        "action": "predefined",
                        "rcode": "REFUSED"
                    }
                ],
                "final": "dns_direct",
                "independent_cache": true
            },
            "ntp": {
                "enabled": true,
                "server": "time.apple.com",
                "server_port": 123,
                "interval": "30m"
            },
            "inbounds": [
                { "type": "mixed", "tag": "mixed-in", "listen": "0.0.0.0", "listen_port": 2080 },
                { "type": "tun", "tag": "tun-in", "address": "172.19.0.1/30", "auto_route": true, "strict_route": true, "stack": "mixed", "sniff": true }
            ],
            "outbounds": [
                { "type": "block", "tag": "REJECT" },
                { "type": "direct", "tag": "DIRECT" }
            ],
            "route": {
                "default_domain_resolver": "dns_resolver",
                "rule_set": [
                    {
                        "tag": "geosite-geolocation-!cn",
                        "type": "local",
                        "format": "binary",
                        "path": "geosite-geolocation-!cn.srs"
                    }
                ],
                "rules": []
            },
            "experimental": {
                "cache_file": {
                    "enabled": true,
                    "store_fakeip": true
                }
            }
        })
    }
    
    pub fn default_v1_11() -> serde_json::Value {
        json!({
            "log": {
                "level": "info",
                "timestamp": true
            },
            "dns": {
                "servers": [
                    {
                        "tag": "dns_proxy",
                        "address": "tls://1.1.1.1",
                        "detour": "🚀 节点选择"
                    },
                    {
                        "tag": "dns_direct",
                        "address": "https://dns.alidns.com/dns-query",
                        "detour": "DIRECT",
                        "address_resolver": "dns_resolver"
                    },
                    {
                        "tag": "dns_resolver",
                        "address": "223.5.5.5",
                        "detour": "DIRECT"
                    },
                    {
                        "tag": "dns_fakeip",
                        "address": "fakeip"
                    }
                ],
                "rules": [
                    {
                        "rule_set": "geolocation-!cn",
                        "query_type": ["A", "AAAA"],
                        "server": "dns_fakeip"
                    },
                    {
                        "rule_set": "geolocation-!cn",
                        "query_type": "CNAME",
                        "server": "dns_proxy"
                    },
                    {
                        "query_type": ["A", "AAAA", "CNAME"],
                        "invert": true,
                        "server": "dns_direct",
                        "disable_cache": true
                    }
                ],
                "final": "dns_direct",
                "strategy": "prefer_ipv4",
                "independent_cache": true,
                "fakeip": {
                    "enabled": true,
                    "inet4_range": "198.18.0.0/15",
                    "inet6_range": "fc00::/18"
                }
            },
            "ntp": {
                "enabled": true,
                "server": "time.apple.com",
                "server_port": 123,
                "interval": "30m"
            },
            "inbounds": [
                { "type": "mixed", "tag": "mixed-in", "listen": "0.0.0.0", "listen_port": 2080 },
                { "type": "tun", "tag": "tun-in", "address": "172.19.0.1/30", "auto_route": true, "strict_route": true, "stack": "mixed", "sniff": true }
            ],
            "outbounds": [
                { "type": "block", "tag": "REJECT" },
                { "type": "direct", "tag": "DIRECT" }
            ],
            "route": {
                "rule_set": [],
                "rules": []
            },
            "experimental": {
                "cache_file": {
                    "enabled": true,
                    "store_fakeip": true
                }
            }
        })
    }
}

/// Clash default configuration
pub mod clash_config {
    use serde_yaml::Value;
    
    pub fn default() -> Value {
        serde_yaml::from_str(r#"
port: 7890
socks-port: 7891
allow-lan: false
mode: rule
log-level: info
dns:
  enable: true
  listen: 0.0.0.0:53
  nameserver:
    - 223.5.5.5
    - 114.114.114.114
  fallback:
    - tls://1.1.1.1
    - tls://8.8.8.8
proxies: []
proxy-groups: []
rules: []
"#).unwrap()
    }
}

/// Surge default configuration
pub mod surge_config {
    pub fn default() -> String {
        r#"[General]
loglevel = notify
dns-server = 223.5.5.5, 114.114.114.114

[Proxy]

[Proxy Group]

[Rule]
"#.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rule_by_name() {
        let rule = rules::get_rule_by_name("Google");
        assert!(rule.is_some());
        assert_eq!(rule.unwrap().name, "Google");
        
        let rule = rules::get_rule_by_name("AI Services");
        assert!(rule.is_some());
        assert_eq!(rule.unwrap().site_rules, &["category-ai-!cn"]);
    }
}
