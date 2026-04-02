use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CountryInfo {
    pub code: String,
    pub name: String,
    pub emoji: String,
}

struct CountryData {
    code: &'static str,
    name: &'static str,
    emoji: &'static str,
    aliases: Vec<&'static str>,
}

fn get_country_data() -> &'static Vec<CountryData> {
    static DATA: OnceLock<Vec<CountryData>> = OnceLock::new();
    DATA.get_or_init(|| {
        vec![
            CountryData { code: "HK", name: "Hong Kong", emoji: "🇭🇰", aliases: vec!["香港", "Hong Kong", "HK"] },
            CountryData { code: "TW", name: "Taiwan", emoji: "🇹🇼", aliases: vec!["台湾", "Taiwan", "TW"] },
            CountryData { code: "JP", name: "Japan", emoji: "🇯🇵", aliases: vec!["日本", "Japan", "JP"] },
            CountryData { code: "KR", name: "Korea", emoji: "🇰🇷", aliases: vec!["韩国", "Korea", "KR"] },
            CountryData { code: "SG", name: "Singapore", emoji: "🇸🇬", aliases: vec!["新加坡", "Singapore", "SG"] },
            CountryData { code: "US", name: "United States", emoji: "🇺🇸", aliases: vec!["美国", "United States", "US"] },
            CountryData { code: "GB", name: "United Kingdom", emoji: "🇬🇧", aliases: vec!["英国", "United Kingdom", "UK", "GB"] },
            CountryData { code: "DE", name: "Germany", emoji: "🇩🇪", aliases: vec!["德国", "Germany"] },
            CountryData { code: "FR", name: "France", emoji: "🇫🇷", aliases: vec!["法国", "France"] },
            CountryData { code: "RU", name: "Russia", emoji: "🇷🇺", aliases: vec!["俄罗斯", "Russia"] },
            CountryData { code: "CA", name: "Canada", emoji: "🇨🇦", aliases: vec!["加拿大", "Canada"] },
            CountryData { code: "AU", name: "Australia", emoji: "🇦🇺", aliases: vec!["澳大利亚", "Australia"] },
            CountryData { code: "IN", name: "India", emoji: "🇮🇳", aliases: vec!["印度", "India"] },
            CountryData { code: "BR", name: "Brazil", emoji: "🇧🇷", aliases: vec!["巴西", "Brazil"] },
            CountryData { code: "ZA", name: "South Africa", emoji: "🇿🇦", aliases: vec!["南非", "South Africa"] },
            CountryData { code: "AR", name: "Argentina", emoji: "🇦🇷", aliases: vec!["阿根廷", "Argentina"] },
            CountryData { code: "TR", name: "Turkey", emoji: "🇹🇷", aliases: vec!["土耳其", "Turkey"] },
            CountryData { code: "NL", name: "Netherlands", emoji: "🇳🇱", aliases: vec!["荷兰", "Netherlands"] },
            CountryData { code: "CH", name: "Switzerland", emoji: "🇨🇭", aliases: vec!["瑞士", "Switzerland"] },
            CountryData { code: "SE", name: "Sweden", emoji: "🇸🇪", aliases: vec!["瑞典", "Sweden"] },
            CountryData { code: "IT", name: "Italy", emoji: "🇮🇹", aliases: vec!["意大利", "Italy"] },
            CountryData { code: "ES", name: "Spain", emoji: "🇪🇸", aliases: vec!["西班牙", "Spain"] },
            CountryData { code: "IE", name: "Ireland", emoji: "🇮🇪", aliases: vec!["爱尔兰", "Ireland"] },
            CountryData { code: "MY", name: "Malaysia", emoji: "🇲🇾", aliases: vec!["马来西亚", "Malaysia"] },
            CountryData { code: "TH", name: "Thailand", emoji: "🇹🇭", aliases: vec!["泰国", "Thailand"] },
            CountryData { code: "VN", name: "Vietnam", emoji: "🇻🇳", aliases: vec!["越南", "Vietnam"] },
            CountryData { code: "PH", name: "Philippines", emoji: "🇵🇭", aliases: vec!["菲律宾", "Philippines"] },
            CountryData { code: "ID", name: "Indonesia", emoji: "🇮🇩", aliases: vec!["印度尼西亚", "Indonesia"] },
            CountryData { code: "NZ", name: "New Zealand", emoji: "🇳🇿", aliases: vec!["新西兰", "New Zealand"] },
            CountryData { code: "AE", name: "United Arab Emirates", emoji: "🇦🇪", aliases: vec!["阿联酋", "United Arab Emirates"] },
        ]
    })
}

pub fn parse_country_from_node_name(node_name: &str) -> Option<CountryInfo> {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let re = REGEX.get_or_init(|| {
        let mut all_entries = Vec::new();
        for country in get_country_data() {
            for alias in &country.aliases {
                all_entries.push((alias, regex::escape(alias)));
            }
        }
        // Sort by length descending
        all_entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        let mut patterns = Vec::new();
        for (alias, escaped) in all_entries {
            if alias.len() <= 3 && alias.chars().all(|c| c.is_ascii_alphabetic()) {
                patterns.push(format!(r"\b{}\b", escaped));
            } else {
                patterns.push(escaped);
            }
        }

        Regex::new(&format!(r"(?i){}", patterns.join("|"))).unwrap()
    });

    if let Some(mat) = re.find(node_name) {
        let matched_alias = mat.as_str().to_lowercase();
        for country in get_country_data() {
            if country.aliases.iter().any(|&a| a.to_lowercase() == matched_alias) {
                return Some(CountryInfo {
                    code: country.code.to_string(),
                    name: country.name.to_string(),
                    emoji: country.emoji.to_string(),
                });
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_country() {
        let hk = parse_country_from_node_name("香港 01").unwrap();
        assert_eq!(hk.code, "HK");
        assert_eq!(hk.emoji, "🇭🇰");

        let us = parse_country_from_node_name("US-SFO-01").unwrap();
        assert_eq!(us.code, "US");

        let jp = parse_country_from_node_name("东京日本节点").unwrap();
        assert_eq!(jp.code, "JP");

        let unknown = parse_country_from_node_name("Maybe somewhere else");
        assert!(unknown.is_none());
    }
}
