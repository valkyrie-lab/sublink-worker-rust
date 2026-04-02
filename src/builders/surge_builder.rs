//! Surge configuration builder

use anyhow::Result;
use crate::parsers::{ProxyParser, ProxyConfig};
use crate::utils::http_utils;

/// Surge configuration builder
pub struct SurgeConfigBuilder {
    config: String,
    proxies: Vec<ProxyConfig>,
    selected_rules: Vec<String>,
    // 暂时未使用的字段
    // subscription_url: Option<String>,
}

impl SurgeConfigBuilder {
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

        Ok(SurgeConfigBuilder {
            config: Self::default_config(),
            proxies,
            selected_rules,
        })
    }

    fn default_config() -> String {
        r#"[General]
loglevel = notify
dns-server = 223.5.5.5, 114.114.114.114
allow-wifi-access = false
wifi-access-http-port = 6152
wifi-access-socks5-port = 6153
http-listen = 127.0.0.1:6152
socks5-listen = 127.0.0.1:6153
skip-proxy = 127.0.0.1, 192.168.0.0/16, 10.0.0.0/8, 172.16.0.0/12, localhost, *.local
ipv6 = false
test-timeout = 5
proxy-test-url = http://www.gstatic.com/generate_204

[Proxy]

[Proxy Group]

[Rule]
"#.to_string()
    }

    // 暂时未使用的方法
    // pub fn set_subscription_url(&mut self, url: &str) {
    //     self.subscription_url = Some(url.to_string());
    // }

    pub fn build(&mut self) -> Result<()> {
        let mut proxy_lines: Vec<String> = Vec::new();
        let mut proxy_names: Vec<String> = Vec::new();

        for proxy in &self.proxies {
            let (proxy_line, name) = self.proxy_to_surge(proxy);
            proxy_lines.push(proxy_line);
            proxy_names.push(name);
        }

        // Build final config
        let mut result = String::new();
        
        // Add general section
        result.push_str(&self.config);
        
        // Replace [Proxy] section
        if let Some(pos) = result.find("[Proxy Group]") {
            let mut new_config = result[..pos].to_string();
            
            // Add proxies
            new_config.push_str("[Proxy]\n");
            for line in &proxy_lines {
                new_config.push_str(line);
                new_config.push('\n');
            }
            new_config.push('\n');
            
            // Add proxy groups
            new_config.push_str("[Proxy Group]\n");
            new_config.push_str(&self.format_proxy_group("Proxy", &proxy_names, "selector"));
            new_config.push_str(&self.format_proxy_group("Auto", &proxy_names, "url-test"));
            new_config.push('\n');
            
            // Add rules
            new_config.push_str("[Rule]\n");
            for rule in self.generate_rules() {
                new_config.push_str(&rule);
                new_config.push('\n');
            }
            
            // Add remaining content
            if let Some(rule_pos) = result[pos..].find("[Rule]") {
                let rule_start = pos + rule_pos;
                if let Some(next_section) = result[rule_start..].find('[') {
                    if next_section > 0 {
                        new_config.push_str(&result[rule_start + 6..rule_start + next_section]);
                    }
                }
            }
            
            self.config = new_config;
        }

        Ok(())
    }

    fn proxy_to_surge(&self, proxy: &ProxyConfig) -> (String, String) {
        match proxy {
            ProxyConfig::ShadowSocks(ss) => {
                let line = format!(
                    "{} = ss, {}, {}, encrypt-method={}, password={}",
                    ss.name, ss.server, ss.port, ss.method, ss.password
                );
                (line, ss.name.clone())
            }
            ProxyConfig::VMess(vmess) => {
                let mut params = Vec::new();
                params.push(format!("username={}", vmess.uuid));
                
                if let Some(ref tls) = vmess.tls {
                    params.push(format!("tls={}", if tls == "tls" { "true" } else { "false" }));
                }
                
                if let Some(ref sni) = vmess.sni {
                    params.push(format!("sni={}", sni));
                }
                
                params.push(format!("network={}", vmess.network));
                
                if let Some(ref path) = vmess.path {
                    params.push(format!("ws-path={}", path));
                }
                
                let line = format!(
                    "{} = vmess, {}, {}, {}",
                    vmess.name, vmess.server, vmess.port, params.join(", ")
                );
                (line, vmess.name.clone())
            }
            ProxyConfig::Trojan(trojan) => {
                let mut params = Vec::new();
                params.push(format!("password={}", trojan.password));
                
                if let Some(ref sni) = trojan.sni {
                    params.push(format!("sni={}", sni));
                }
                
                let line = format!(
                    "{} = trojan, {}, {}, {}",
                    trojan.name, trojan.server, trojan.port, params.join(", ")
                );
                (line, trojan.name.clone())
            }
            ProxyConfig::Hysteria2(hy2) => {
                let mut params = Vec::new();
                params.push(format!("password={}", hy2.password));
                
                if let Some(ref sni) = hy2.sni {
                    params.push(format!("sni={}", sni));
                }
                
                let line = format!(
                    "{} = hysteria2, {}, {}, {}",
                    hy2.name, hy2.server, hy2.port, params.join(", ")
                );
                (line, hy2.name.clone())
            }
            _ => {
                // For unsupported types, create a comment
                (format!("# {} (unsupported)", proxy.name()), proxy.name().to_string())
            }
        }
    }

    fn format_proxy_group(&self, name: &str, proxies: &[String], group_type: &str) -> String {
        let mut group = String::new();
        group.push_str(&format!("{} = {}, ", name, group_type));
        
        let proxy_list = if name == "Auto" {
            proxies.join(", ")
        } else {
            let mut all_proxies = vec!["DIRECT".to_string(), "REJECT".to_string()];
            all_proxies.extend_from_slice(proxies);
            all_proxies.join(", ")
        };
        
        group.push_str(&proxy_list);
        
        if group_type == "url-test" {
            group.push_str(", url=http://www.gstatic.com/generate_204, interval=180");
        }
        
        group.push('\n');
        group
    }

    fn generate_rules(&self) -> Vec<String> {
        let mut rules = Vec::new();

        // DNS rule
        rules.push("PROTOCOL,DNS,DIRECT".to_string());

        // Rules for selected rule sets
        for rule_name in &self.selected_rules {
            rules.push(format!("DOMAIN-SUFFIX,{},Proxy", rule_name));
        }

        // Private IP rules
        rules.push("IP-CIDR,192.168.0.0/16,DIRECT".to_string());
        rules.push("IP-CIDR,10.0.0.0/8,DIRECT".to_string());
        rules.push("IP-CIDR,172.16.0.0/12,DIRECT".to_string());
        rules.push("IP-CIDR,127.0.0.0/8,DIRECT".to_string());

        // Final match
        rules.push("MATCH,Proxy".to_string());

        rules
    }

    pub fn format_config(&self) -> String {
        self.config.clone()
    }
}
