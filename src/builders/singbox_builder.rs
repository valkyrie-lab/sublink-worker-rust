//! SingBox configuration builder

use anyhow::Result;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use crate::parsers::{ProxyParser, ProxyConfig};
use crate::config::{singbox_config, rules};
use crate::utils::{http_utils, base64_utils, country_utils};
use crate::i18n;

/// SingBox configuration builder
pub struct SingboxConfigBuilder {
    config: Value,
    proxies: Vec<ProxyConfig>,
    selected_rules: Vec<String>,
    lang: String,
    group_by_country: bool,
    include_auto_select: bool,
}

impl SingboxConfigBuilder {
    pub async fn new(
        input: &str, 
        selected_rules: Vec<String>, 
        lang: String, 
        ua: Option<&str>,
        group_by_country: bool,
        include_auto_select: bool,
    ) -> Result<Self> {
        // Fetch content from URL if input is a HTTP/HTTPS URL
        let content = if input.starts_with("http://") || input.starts_with("https://") {
            let user_agent = ua.unwrap_or("curl/7.74.0");
            http_utils::fetch_url_with_ua(input, user_agent).await?
        } else {
            input.to_string()
        };

        // Try to decode Base64 content
        let decoded_content = if let Ok(decoded) = base64_utils::decode(&content) {
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

        Ok(SingboxConfigBuilder {
            config: singbox_config::default_v1_12(),
            proxies,
            selected_rules,
            lang,
            group_by_country,
            include_auto_select,
        })
    }

    pub fn build(&mut self) -> Result<()> {
        let translator = i18n::create_translator(&self.lang);
        let node_select_tag = translator.get("outboundName-NodeSelect", "🚀 Node Select");
        let auto_select_tag = translator.get("outboundName-AutoSelect", "⚡ Auto Select");
        let fallback_tag = translator.get("outboundName-FallBack", "🐟 Fall Back");
        let manual_switch_tag = translator.get("outboundName-ManualSwitch", "🖐️ Manual Switch");

        // Add proxies to config
        let mut proxy_names: Vec<String> = Vec::new();
        if let Some(outbounds) = self.config["outbounds"].as_array_mut() {
            // Clear default outbounds except special ones
            outbounds.clear();
            
            // Add basic outbounds
            outbounds.push(json!({"type": "direct", "tag": "direct"}));
            outbounds.push(json!({"type": "block", "tag": "block"}));
            outbounds.push(json!({"type": "dns", "tag": "dns-out"}));

            for proxy in &self.proxies {
                outbounds.push(proxy.to_singbox());
                proxy_names.push(proxy.name().to_string());
            }

            let mut country_group_tags = Vec::new();
            if self.group_by_country && !self.proxies.is_empty() {
                let mut grouped: BTreeMap<String, (country_utils::CountryInfo, Vec<String>)> = BTreeMap::new();
                for proxy in &self.proxies {
                    if let Some(country) = country_utils::parse_country_from_node_name(proxy.name()) {
                        grouped.entry(country.name.clone())
                            .or_insert((country, Vec::new()))
                            .1.push(proxy.name().to_string());
                    }
                }

                for (_name, (info, members)) in grouped {
                    let tag = format!("{} {}", info.emoji, info.name);
                    outbounds.push(json!({
                        "type": "urltest",
                        "tag": tag,
                        "outbounds": members,
                        "url": "http://www.gstatic.com/generate_204",
                        "interval": "3m",
                        "tolerance": 50
                    }));
                    country_group_tags.push(tag);
                }
            }

            // Manual Switch group
            if !proxy_names.is_empty() {
                outbounds.push(json!({
                    "type": "selector",
                    "tag": manual_switch_tag,
                    "outbounds": proxy_names
                }));
            }

            // Auto Select group
            if self.include_auto_select && !proxy_names.is_empty() {
                outbounds.push(json!({
                    "type": "urltest",
                    "tag": auto_select_tag,
                    "outbounds": proxy_names,
                    "url": "http://www.gstatic.com/generate_204",
                    "interval": "3m",
                    "tolerance": 50
                }));
            }

            // Pre-calculate selector members for all rules
            let mut selector_members = Vec::new();
            selector_members.push(node_select_tag.clone());
            if self.include_auto_select && !proxy_names.is_empty() {
                selector_members.push(auto_select_tag.clone());
            }
            if !proxy_names.is_empty() {
                selector_members.push(manual_switch_tag.clone());
            }
            if self.group_by_country {
                selector_members.extend(country_group_tags.clone());
            }
            selector_members.push("direct".to_string());
            selector_members.push("block".to_string());

            // Node Select group
            let mut node_select_members = Vec::new();
            if self.include_auto_select && !proxy_names.is_empty() {
                node_select_members.push(auto_select_tag.clone());
            }
            if !proxy_names.is_empty() {
                node_select_members.push(manual_switch_tag.clone());
            }
            if self.group_by_country {
                node_select_members.extend(country_group_tags);
            } else {
                node_select_members.extend(proxy_names.clone());
            }
            node_select_members.push("direct".to_string());
            node_select_members.push("block".to_string());

            outbounds.insert(0, json!({
                "type": "selector",
                "tag": node_select_tag.clone(),
                "outbounds": node_select_members
            }));

            // Add other rule groups
            for rule_name in &self.selected_rules {
                if rule_name == "Non-China" { continue; }
                let tag = translator.get(&format!("outboundName-{}", rule_name.replace(" ", "")), rule_name);
                let mut members = selector_members.clone();
                if rules::DIRECT_DEFAULT_RULES.contains(&rule_name.as_str()) {
                    // Move direct to front
                    if let Some(pos) = members.iter().position(|x| x == "direct") {
                        let direct = members.remove(pos);
                        members.insert(0, direct);
                    }
                }
                outbounds.push(json!({
                    "type": "selector",
                    "tag": tag,
                    "outbounds": members
                }));
            }

            // Fall Back group
            let mut fallback_members = selector_members.clone();
            outbounds.push(json!({
                "type": "selector",
                "tag": fallback_tag,
                "outbounds": fallback_members
            }));
        }

        // Generate rules based on selected rules
        self.generate_rules(&translator);

        Ok(())
    }

    fn generate_rules(&mut self, translator: &i18n::Translator) {
        let node_select_tag = translator.get("outboundName-NodeSelect", "🚀 Node Select");
        let rule_sets = self.generate_rule_sets(&node_select_tag);
        let rules_list = self.generate_rules_list(translator);

        if let Some(route) = self.config["route"].as_object_mut() {
            route.insert("rule_set".to_string(), json!(rule_sets));
            route.insert("rules".to_string(), json!(rules_list));
            route.insert("final".to_string(), json!(translator.get("outboundName-FallBack", "🐟 Fall Back")));
            route.insert("auto_detect_interface".to_string(), json!(true));
        }
    }

    fn generate_rule_sets(&self, proxy_tag: &str) -> Vec<Value> {
        let mut rule_sets = Vec::new();
        let base_url = "https://raw.githubusercontent.com/lyc8503/sing-box-rules/rule-set-geosite/";

        for rule_name in &self.selected_rules {
            if let Some(rule) = rules::get_rule_by_name(rule_name) {
                // Add site rule sets
                for site_rule in rule.site_rules {
                    rule_sets.push(json!({
                        "tag": site_rule,
                        "type": "remote",
                        "format": "binary",
                        "url": format!("{}geosite-{}.srs", base_url, site_rule),
                        "download_detour": proxy_tag
                    }));
                }
                // Add IP rule sets
                for ip_rule in rule.ip_rules {
                    rule_sets.push(json!({
                        "tag": format!("{}-ip", ip_rule),
                        "type": "remote",
                        "format": "binary",
                        "url": format!("{}geoip-{}.srs", base_url, ip_rule),
                        "download_detour": proxy_tag
                    }));
                }
            }
        }

        rule_sets
    }

    fn generate_rules_list(&self, translator: &i18n::Translator) -> Vec<Value> {
        let mut rules_list = Vec::new();
        let node_select_tag = translator.get("outboundName-NodeSelect", "🚀 Node Select");

        // Basic rules
        rules_list.push(json!({ "clash_mode": "direct", "outbound": "direct" }));
        rules_list.push(json!({ "clash_mode": "global", "outbound": node_select_tag }));
        rules_list.push(json!({ "action": "sniff" }));
        rules_list.push(json!({ "protocol": "dns", "action": "hijack-dns" }));
        rules_list.push(json!({ "protocol": "dns", "outbound": "dns-out" }));

        // Rules for selected rule sets
        for rule_name in &self.selected_rules {
            if let Some(rule) = rules::get_rule_by_name(rule_name) {
                let mut rule_set_tags: Vec<String> = Vec::new();

                // Add site rules
                for site_rule in rule.site_rules {
                    rule_set_tags.push(site_rule.to_string());
                }

                // Add IP rules
                for ip_rule in rule.ip_rules {
                    rule_set_tags.push(format!("{}-ip", ip_rule));
                }
                
                if !rule_set_tags.is_empty() {
                    let outbound_tag = translator.get(&format!("outboundName-{}", rule_name.replace(" ", "")), rule_name);
                    
                    rules_list.push(json!({
                        "rule_set": rule_set_tags,
                        "outbound": outbound_tag
                    }));
                }
            }
        }

        // Final catch-all rule for Non-China if not explicitly included
        if !self.selected_rules.contains(&"Non-China".to_string()) {
            rules_list.push(json!({
                "rule_set": ["geolocation-!cn"],
                "outbound": node_select_tag
            }));
        }

        // Private IP rule
        rules_list.push(json!({
            "ip_is_private": true,
            "outbound": "direct"
        }));

        rules_list
    }

    pub fn format_config(&self) -> String {
        serde_json::to_string_pretty(&self.config).unwrap_or_default()
    }

    pub fn get_config(&self) -> &Value {
        &self.config
    }
}
