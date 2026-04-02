//! Clash configuration builder

use anyhow::Result;
use serde_yaml::Value;
use crate::parsers::{ProxyParser, ProxyConfig};
use crate::config::rules;
use crate::utils::http_utils;

/// Clash configuration builder
pub struct ClashConfigBuilder {
    config: Value,
    proxies: Vec<ProxyConfig>,
    selected_rules: Vec<String>,
}

impl ClashConfigBuilder {
    pub async fn new(input: &str, selected_rules: Vec<String>, ua: Option<&str>) -> Result<Self> {
        // Fetch content from URL if input is a HTTP/HTTPS URL
        let content = if input.starts_with("http://") || input.starts_with("https://") {
            let user_agent = ua.unwrap_or("curl/7.74.0");
            http_utils::fetch_url_with_ua(input, user_agent).await?
        } else {
            input.to_string()
        };

        // Try to decode Base64 content
        let decoded_content = if let Ok(decoded) = crate::utils::base64_utils::decode(&content) {
            String::from_utf8_lossy(&decoded).to_string()
        } else {
            content
        };

        // Normalize line endings (CRLF -> LF) and parse all proxy URLs
        let normalized_content = decoded_content.replace("\r\n", "\n").replace("\r", "\n");
        let mut proxies = Vec::new();
        for line in normalized_content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                if let Ok(proxy) = ProxyParser::parse(trimmed) {
                    proxies.push(proxy);
                }
            }
        }

        Ok(ClashConfigBuilder {
            config: Self::default_config(),
            proxies,
            selected_rules,
        })
    }

    fn default_config() -> Value {
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
  fallback-filter:
    geoip: true
    ipcidr:
      - 240.0.0.0/4
proxies: []
proxy-groups: []
rules: []
"#).unwrap()
    }

    pub fn build(&mut self) -> Result<()> {
        // Add proxies to config
        let mut proxy_list: Vec<Value> = Vec::new();
        for proxy in &self.proxies {
            proxy_list.push(proxy.to_clash());
        }

        // Generate rules first to avoid borrow conflict
        let rules_list = self.generate_rules();

        if let Some(config_map) = self.config.as_mapping_mut() {
            config_map.insert(
                serde_yaml::Value::String("proxies".to_string()),
                serde_yaml::Value::Sequence(proxy_list.clone())
            );

            // Add proxy groups
            let mut proxy_groups: Vec<Value> = Vec::new();

            // Proxy group
            let mut proxy_group = serde_yaml::Mapping::new();
            proxy_group.insert(
                serde_yaml::Value::String("name".to_string()),
                serde_yaml::Value::String("Proxy".to_string())
            );
            proxy_group.insert(
                serde_yaml::Value::String("type".to_string()),
                serde_yaml::Value::String("selector".to_string())
            );

            let mut group_proxies: Vec<Value> = vec![
                serde_yaml::Value::String("DIRECT".to_string()),
                serde_yaml::Value::String("REJECT".to_string()),
            ];
            for proxy in &self.proxies {
                group_proxies.push(serde_yaml::Value::String(proxy.name().to_string()));
            }
            proxy_group.insert(
                serde_yaml::Value::String("proxies".to_string()),
                serde_yaml::Value::Sequence(group_proxies)
            );
            proxy_groups.push(serde_yaml::Value::Mapping(proxy_group));

            // Auto group
            if !self.proxies.is_empty() {
                let mut auto_group = serde_yaml::Mapping::new();
                auto_group.insert(
                    serde_yaml::Value::String("name".to_string()),
                    serde_yaml::Value::String("Auto".to_string())
                );
                auto_group.insert(
                    serde_yaml::Value::String("type".to_string()),
                    serde_yaml::Value::String("url-test".to_string())
                );

                let mut auto_proxies: Vec<Value> = Vec::new();
                for proxy in &self.proxies {
                    auto_proxies.push(serde_yaml::Value::String(proxy.name().to_string()));
                }
                auto_group.insert(
                    serde_yaml::Value::String("proxies".to_string()),
                    serde_yaml::Value::Sequence(auto_proxies)
                );
                auto_group.insert(
                    serde_yaml::Value::String("url".to_string()),
                    serde_yaml::Value::String("http://www.gstatic.com/generate_204".to_string())
                );
                auto_group.insert(
                    serde_yaml::Value::String("interval".to_string()),
                    serde_yaml::Value::Number(180.into())
                );
                proxy_groups.push(serde_yaml::Value::Mapping(auto_group));
            }

            config_map.insert(
                serde_yaml::Value::String("proxy-groups".to_string()),
                serde_yaml::Value::Sequence(proxy_groups)
            );

            // Add rules
            config_map.insert(
                serde_yaml::Value::String("rules".to_string()),
                serde_yaml::Value::Sequence(rules_list)
            );
        }

        Ok(())
    }

    fn generate_rules(&self) -> Vec<Value> {
        let mut rules: Vec<Value> = Vec::new();

        // DNS rule
        rules.push(serde_yaml::Value::String("PROTOCOL,DNS,DIRECT".to_string()));

        // Rules for selected rule sets
        for rule_name in &self.selected_rules {
            if let Some(rule) = rules::get_rule_by_name(rule_name) {
                let outbound = if rules::DIRECT_DEFAULT_RULES.contains(&rule_name.as_str()) {
                    "DIRECT"
                } else {
                    "Proxy"
                };
                
                // Add site rules (GEOSITE)
                for site_rule in rule.site_rules {
                    rules.push(serde_yaml::Value::String(format!(
                        "GEOSITE,{},{}",
                        site_rule, outbound
                    )));
                }
                
                // Add IP rules (GEOIP)
                for ip_rule in rule.ip_rules {
                    rules.push(serde_yaml::Value::String(format!(
                        "GEOIP,{},{},no-resolve",
                        ip_rule, outbound
                    )));
                }
            }
        }

        // Non-China rule if not explicitly included
        if !self.selected_rules.contains(&"Non-China".to_string()) {
            rules.push(serde_yaml::Value::String("GEOSITE,geolocation-!cn,Proxy".to_string()));
        }

        // Private IP rules
        rules.push(serde_yaml::Value::String("IP-CIDR,192.168.0.0/16,DIRECT".to_string()));
        rules.push(serde_yaml::Value::String("IP-CIDR,10.0.0.0/8,DIRECT".to_string()));
        rules.push(serde_yaml::Value::String("IP-CIDR,172.16.0.0/12,DIRECT".to_string()));
        rules.push(serde_yaml::Value::String("IP-CIDR,127.0.0.0/8,DIRECT".to_string()));

        // Final match
        rules.push(serde_yaml::Value::String("MATCH,Proxy".to_string()));

        rules
    }

    pub fn format_config(&self) -> String {
        serde_yaml::to_string(&self.config).unwrap_or_default()
    }
}
