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
    singbox_version: String,
    enable_clash_ui: bool,
    external_controller: Option<String>,
    external_ui_download_url: Option<String>,
    provider_urls: Vec<String>,
}

impl SingboxConfigBuilder {
    pub async fn new(
        input: &str,
        selected_rules: Vec<String>,
        lang: String,
        ua: Option<&str>,
        group_by_country: bool,
        include_auto_select: bool,
        singbox_version: Option<&str>,
        enable_clash_ui: bool,
        external_controller: Option<&str>,
        external_ui_download_url: Option<&str>,
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
        let mut provider_urls = Vec::new();

        // Check if content is SingBox JSON format with outbound_providers
        let is_singbox_format = Self::detect_singbox_format(&normalized_content);
        if is_singbox_format {
            // Parse as SingBox JSON to extract provider URLs
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&normalized_content) {
                if let Some(providers) = json.get("outbound_providers").and_then(|p| p.as_array()) {
                    for provider in providers {
                        if let Some(tag) = provider.get("tag").and_then(|t| t.as_str()) {
                            provider_urls.push(tag.to_string());
                        }
                    }
                }
            }
        }

        for line in normalized_content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                if let Ok(proxy) = ProxyParser::parse(trimmed) {
                    proxies.push(proxy);
                }
            }
        }

        // Resolve singbox version
        let resolved_version = Self::resolve_version(singbox_version.as_deref(), ua);

        // Select base config based on version
        let config = if resolved_version == "1.11" {
            singbox_config::default_v1_11()
        } else {
            singbox_config::default_v1_12()
        };

        Ok(SingboxConfigBuilder {
            config,
            proxies,
            selected_rules,
            lang,
            group_by_country,
            include_auto_select,
            singbox_version: resolved_version,
            enable_clash_ui,
            external_controller: external_controller.map(String::from),
            external_ui_download_url: external_ui_download_url.map(String::from),
            provider_urls,
        })
    }

    fn detect_singbox_format(content: &str) -> bool {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
            // Check for SingBox-specific fields
            return json.get("outbound_providers").is_some()
                || (json.get("outbounds").is_some() && json.get("route").is_some());
        }
        false
    }

    fn resolve_version(version: Option<&str>, ua: Option<&str>) -> String {
        // If version is explicitly specified, use it
        if let Some(v) = version {
            let v_lower = v.to_lowercase();
            match v_lower.as_str() {
                "1.11" | "1.12" | "legacy" | "latest" | "auto" => return v_lower,
                _ => {}
            }
        }

        // Detect from User-Agent
        if let Some(ua_str) = ua {
            let ua_lower = ua_str.to_lowercase();
            if ua_lower.contains("sing-box/1.11") || ua_lower.contains("singbox/1.11") {
                return "1.11".to_string();
            }
            if ua_lower.contains("sing-box/1.12") || ua_lower.contains("singbox/1.12") {
                return "1.12".to_string();
            }
        }

        // Default to 1.12
        "1.12".to_string()
    }

    fn detect_subscription_format(content: &str) -> &'static str {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(content) {
            if json.get("outbound_providers").is_some() {
                return "singbox";
            }
            if json.get("proxies").is_some() || json.get("proxy-groups").is_some() {
                return "clash";
            }
        }
        "unknown"
    }

    fn is_compatible_provider_format(&self, format: &str) -> bool {
        if self.singbox_version == "1.11" {
            return false;
        }
        format == "singbox"
    }

    fn generate_outbound_providers(&self) -> Vec<Value> {
        self.provider_urls.iter().enumerate().map(|(index, _)| {
            json!({
                "tag": format!("_auto_provider_{}", index + 1),
                "type": "http",
                "download_url": format!("https://example.com/provider_{}", index + 1), // Placeholder
                "path": format!("./providers/_auto_provider_{}.json", index + 1),
                "download_interval": "24h",
                "health_check": {
                    "enabled": true,
                    "url": "https://www.gstatic.com/generate_204",
                    "interval": "5m"
                }
            })
        }).collect()
    }

    fn get_provider_tags(&self) -> Vec<String> {
        self.provider_urls.iter().enumerate().map(|(index, _)| {
            format!("_auto_provider_{}", index + 1)
        }).collect()
    }

    fn get_all_provider_tags(&self) -> Vec<String> {
        if self.singbox_version == "1.11" {
            return vec![];
        }
        self.get_provider_tags()
    }

    fn get_proxy_list(&self) -> Vec<String> {
        self.proxies.iter().map(|p| p.name().to_string()).collect()
    }

    pub fn build(&mut self) -> Result<()> {
        let translator = i18n::create_translator(&self.lang);
        let node_select_tag = translator.get("outboundName-NodeSelect", "🚀 Node Select");
        let auto_select_tag = translator.get("outboundName-AutoSelect", "⚡ Auto Select");
        let fallback_tag = translator.get("outboundName-FallBack", "🐟 Fall Back");
        let manual_switch_tag = translator.get("outboundName-ManualSwitch", "🖐️ Manual Switch");

        // Add proxies to config with deduplication
        let mut proxy_names: Vec<String> = Vec::new();
        let mut existing_outbounds: Vec<Value> = Vec::new();

        if let Some(outbounds) = self.config["outbounds"].as_array_mut() {
            // Clear default outbounds
            outbounds.clear();

            // Add basic outbounds (JS version uses DIRECT and REJECT)
            outbounds.push(json!({"type": "block", "tag": "REJECT"}));
            outbounds.push(json!({"type": "direct", "tag": "DIRECT"}));

            // Add proxies with deduplication
            for proxy in &self.proxies {
                let singbox_proxy = proxy.to_singbox();
                let proxy_name = proxy.name().to_string();

                // Check if a proxy with same server+port+protocol exists
                let _proxy_key = Self::get_proxy_key(&singbox_proxy);
                let existing_names: Vec<String> = existing_outbounds.iter()
                    .filter_map(|o| o.get("tag").and_then(|t| t.as_str()).map(String::from))
                    .collect();

                let final_name = if existing_names.contains(&proxy_name) {
                    // Find a unique name
                    Self::find_unique_name(&proxy_name, &existing_names)
                } else {
                    proxy_name.clone()
                };

                let mut final_proxy = singbox_proxy;
                if final_name != proxy_name {
                    if let Some(obj) = final_proxy.as_object_mut() {
                        obj.insert("tag".to_string(), json!(final_name));
                    }
                }

                existing_outbounds.push(final_proxy.clone());
                outbounds.push(final_proxy);
                proxy_names.push(final_name);
            }

            // Build country groups
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
                    // Add url/interval/tolerance for urltest groups (JS version)
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

            // Manual Switch group (created first in JS version)
            if !proxy_names.is_empty() {
                outbounds.push(json!({
                    "type": "selector",
                    "tag": manual_switch_tag,
                    "outbounds": proxy_names.clone()
                }));
            }

            // Auto Select group (urltest) - created as standalone outbound
            if self.include_auto_select && !proxy_names.is_empty() {
                outbounds.push(json!({
                    "type": "urltest",
                    "tag": auto_select_tag,
                    "outbounds": proxy_names.clone(),
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
            selector_members.push("DIRECT".to_string());
            selector_members.push("REJECT".to_string());

            // Node Select group
            let mut node_select_members = Vec::new();
            if self.include_auto_select && !proxy_names.is_empty() {
                node_select_members.push(auto_select_tag.clone());
            }
            if !proxy_names.is_empty() {
                node_select_members.push(manual_switch_tag.clone());
            }
            if self.group_by_country {
                node_select_members.extend(country_group_tags.clone());
            } else {
                node_select_members.extend(proxy_names.clone());
            }
            node_select_members.push("DIRECT".to_string());
            node_select_members.push("REJECT".to_string());

            outbounds.insert(0, json!({
                "type": "selector",
                "tag": node_select_tag.clone(),
                "outbounds": node_select_members.clone()
            }));

            // Add other rule groups
            for rule_name in &self.selected_rules {
                // Replace spaces and colons but don't add hyphens
                let normalized_rule = rule_name.replace(" ", "").replace(":", "");
                let tag = translator.get(&format!("outboundName-{}", normalized_rule), rule_name);
                let mut members = selector_members.clone();
                if rules::DIRECT_DEFAULT_RULES.contains(&rule_name.as_str()) {
                    // Move DIRECT to front
                    if let Some(pos) = members.iter().position(|x| x == "DIRECT") {
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

            // Rebuild Node Select with country groups if country grouping enabled
            if self.group_by_country && !country_group_tags.is_empty() {
                let mut new_node_select = Vec::new();
                new_node_select.push(node_select_tag.clone());
                if self.include_auto_select && !proxy_names.is_empty() {
                    new_node_select.push(auto_select_tag.clone());
                }
                new_node_select.push(manual_switch_tag.clone());
                new_node_select.extend(country_group_tags);
                new_node_select.push("DIRECT".to_string());
                new_node_select.push("REJECT".to_string());

                // Update Node Select group
                if let Some(node_select) = outbounds.iter_mut().find(|o| o.get("tag").and_then(|t| t.as_str()) == Some(&node_select_tag)) {
                    if let Some(obj) = node_select.as_object_mut() {
                        obj.insert("outbounds".to_string(), json!(new_node_select));
                    }
                }
            }

            // Fall Back group
            outbounds.push(json!({
                "type": "selector",
                "tag": fallback_tag,
                "outbounds": selector_members
            }));

            // Add outbound_providers if supported (version 1.12+)
            if self.singbox_version != "1.11" && !self.provider_urls.is_empty() {
                let providers = self.generate_outbound_providers();
                if let Some(existing) = self.config.get("outbound_providers").and_then(|p| p.as_array()) {
                    // Merge with existing providers
                    let mut merged = existing.clone();
                    for provider in &providers {
                        if !merged.iter().any(|p| p.get("tag") == provider.get("tag")) {
                            merged.push(provider.clone());
                        }
                    }
                    self.config["outbound_providers"] = json!(merged);
                } else {
                    self.config["outbound_providers"] = json!(providers);
                }
            }
        }

        // Generate rules based on selected rules
        self.generate_rules(&translator);

        // Add Clash UI configuration if enabled
        if self.enable_clash_ui || self.external_controller.is_some() || self.external_ui_download_url.is_some() {
            self.add_clash_api_config();
        }

        // Validate outbounds
        self.validate_outbounds();

        Ok(())
    }

    fn get_proxy_key(proxy: &Value) -> String {
        let server = proxy.get("server").or(proxy.get("address")).map(|v| v.to_string()).unwrap_or_default();
        let port = proxy.get("port").map(|v| v.to_string()).unwrap_or_default();
        let ptype = proxy.get("type").map(|v| v.to_string()).unwrap_or_default();
        format!("{}:{}:{}", server, port, ptype)
    }

    fn find_unique_name(base_name: &str, existing_names: &[String]) -> String {
        let mut counter = 1;
        let mut new_name = format!("{} {}", base_name, counter);
        while existing_names.contains(&new_name) {
            counter += 1;
            new_name = format!("{} {}", base_name, counter);
        }
        new_name
    }

    fn add_clash_api_config(&mut self) {
        let default_external_controller = "0.0.0.0:9090";
        let default_external_ui_download_url = "https://gh-proxy.com/https://github.com/Zephyruso/zashboard/archive/refs/heads/gh-pages.zip";
        let default_external_ui = "./ui";
        let default_secret = "";
        let default_download_detour = "DIRECT";
        let default_clash_mode = "rule";

        // Ensure experimental object exists
        if !self.config["experimental"].is_object() {
            self.config["experimental"] = json!({});
        }

        let experimental = self.config["experimental"].as_object_mut().unwrap();

        let existing_clash_api = experimental.get("clash_api")
            .and_then(|v: &Value| v.as_object())
            .cloned()
            .unwrap_or_else(|| serde_json::Map::new());

        let external_controller = self.external_controller.clone()
            .or_else(|| existing_clash_api.get("external_controller").and_then(|v: &Value| v.as_str()).map(String::from))
            .unwrap_or_else(|| default_external_controller.to_string());

        let external_ui_download_url = self.external_ui_download_url.clone()
            .or_else(|| existing_clash_api.get("external_ui_download_url").and_then(|v: &Value| v.as_str()).map(String::from))
            .unwrap_or_else(|| default_external_ui_download_url.to_string());

        let external_ui = existing_clash_api.get("external_ui")
            .and_then(|v: &Value| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| default_external_ui.to_string());

        let secret = existing_clash_api.get("secret")
            .and_then(|v: &Value| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| default_secret.to_string());

        let external_ui_download_detour = existing_clash_api.get("external_ui_download_detour")
            .and_then(|v: &Value| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| default_download_detour.to_string());

        let clash_mode = existing_clash_api.get("default_mode")
            .and_then(|v: &Value| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| default_clash_mode.to_string());

        experimental.insert("clash_api".to_string(), json!({
            "external_controller": external_controller,
            "external_ui": external_ui,
            "external_ui_download_url": external_ui_download_url,
            "external_ui_download_detour": external_ui_download_detour,
            "secret": if secret.is_empty() { Value::Null } else { json!(secret) },
            "default_mode": clash_mode
        }));
    }

    fn validate_outbounds(&mut self) {
        let proxy_list = self.get_proxy_list();
        let provider_tags = self.get_all_provider_tags();

        if let Some(outbounds) = self.config["outbounds"].as_array_mut() {
            for outbound in outbounds.iter_mut() {
                let outbound_type = outbound.get("type").and_then(|v| v.as_str()).unwrap_or("");
                if outbound_type == "urltest" {
                    let has_outbounds = outbound.get("outbounds")
                        .and_then(|v| v.as_array())
                        .map(|arr| !arr.is_empty())
                        .unwrap_or(false);
                    let has_providers = outbound.get("providers")
                        .and_then(|v| v.as_array())
                        .map(|arr| !arr.is_empty())
                        .unwrap_or(false);

                    if !has_outbounds && !has_providers {
                        // Fill with all available proxy tags
                        outbound["outbounds"] = json!(proxy_list);
                        if !provider_tags.is_empty() {
                            outbound["providers"] = json!(provider_tags);
                        }
                    }
                }
            }
        }
    }

    fn generate_rules(&mut self, translator: &i18n::Translator) {
        let node_select_tag = translator.get("outboundName-NodeSelect", "🚀 Node Select");
        let fallback_tag = translator.get("outboundName-FallBack", "🐟 Fall Back");
        let rule_sets = self.generate_rule_sets(&node_select_tag);
        let rules_list = self.generate_rules_list(translator);

        if let Some(route) = self.config["route"].as_object_mut() {
            // Add default_domain_resolver (JS version has this)
            route.insert("default_domain_resolver".to_string(), json!("dns_resolver"));
            route.insert("rule_set".to_string(), json!(rule_sets));
            route.insert("rules".to_string(), json!(rules_list));
            route.insert("final".to_string(), json!(fallback_tag));
            route.insert("auto_detect_interface".to_string(), json!(true));
        }
    }

    fn generate_rule_sets(&self, proxy_tag: &str) -> Vec<Value> {
        let mut rule_sets = Vec::new();
        let geosite_base_url = "https://raw.githubusercontent.com/lyc8503/sing-box-rules/rule-set-geosite/";
        let geoip_base_url = "https://raw.githubusercontent.com/lyc8503/sing-box-rules/rule-set-geoip/";

        // v1.12 uses geosite-geolocation-!cn format, v1.11 uses geolocation-!cn
        for rule_name in &self.selected_rules {
            if let Some(rule) = rules::get_rule_by_name(rule_name) {
                // Add site rule sets
                for site_rule in rule.site_rules {
                    rule_sets.push(json!({
                        "tag": site_rule,
                        "type": "remote",
                        "format": "binary",
                        "url": format!("{}geosite-{}.srs", geosite_base_url, site_rule),
                        "download_detour": proxy_tag
                    }));
                }
                // Add IP rule sets
                for ip_rule in rule.ip_rules {
                    rule_sets.push(json!({
                        "tag": format!("{}-ip", ip_rule),
                        "type": "remote",
                        "format": "binary",
                        "url": format!("{}geoip-{}.srs", geoip_base_url, ip_rule),
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

        // Basic rules (JS version uses DIRECT, not direct)
        rules_list.push(json!({ "clash_mode": "direct", "outbound": "DIRECT" }));
        rules_list.push(json!({ "clash_mode": "global", "outbound": node_select_tag }));
        rules_list.push(json!({ "action": "sniff" }));
        rules_list.push(json!({ "protocol": "dns", "action": "hijack-dns" }));

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
                    let normalized_rule = rule_name.replace(" ", "").replace(":", "");
                    let outbound_tag = translator.get(&format!("outboundName-{}", normalized_rule), rule_name);

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

        rules_list
    }

    pub fn format_config(&self) -> String {
        serde_json::to_string_pretty(&self.config).unwrap_or_default()
    }

    pub fn get_config(&self) -> &Value {
        &self.config
    }
}
