// src/geo_ethics.rs
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

static COUNTRY_RULES: Lazy<HashMap<&'static str, HashSet<&'static str>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("VN", vec!["khủng bố", "chống phá nhà nước", "phản động", "xuyên tạc", "ma túy", "buôn người", "khiêu dâm trẻ em", "đa cấp", "lừa đảo", "ddos", "hack"].into_iter().collect());
    m.insert("US", vec!["child pornography", "terrorism", "human trafficking", "hate speech", "election interference", "cp", "bomb making"].into_iter().collect());
    m.insert("EU", vec!["hate speech", "nazi", "holocaust denial", "gdpr violation", "right to be forgotten", "child abuse"].into_iter().collect());
    m.insert("CN", vec!["台湾独立", "新疆", "六四", "天安门", "批评政府", "法轮功", "达赖"].into_iter().collect());
    m.insert("IN", vec!["blasphemy", "cow slaughter", "sedition", "section 295a"].into_iter().collect());
    m.insert("RU", vec!["призыв к экстремизму", "дискредитация армии", "фейки о вс рф"].into_iter().collect());
    m.insert("BR", vec!["fake news", "golpe", "discurso de ódio"].into_iter().collect());
    m.insert("DEFAULT", vec!["child pornography", "terrorism", "genocide", "human trafficking", "bomb instructions", "suicide methods"].into_iter().collect());
    m
});

pub fn get_country_from_ip(ip: &str) -> &'static str {
    if ip.starts_with("127.0.0.") || ip == "::1" { return "LOCAL"; }
    if ip.starts_with("113.") || ip.starts_with("117.") || ip.starts_with("123.") || ip.starts_with("1.53.") { "VN" }
    else if ip.starts_with("104.") || ip.starts_with("154.") || ip.starts_with("172.") { "US" }
    else if ip.contains(".de") || ip.contains(".fr") || ip.contains(".nl") || ip.contains(".eu") { "EU" }
    else if ip.starts_with("111.") || ip.starts_with("182.") || ip.starts_with("220.") { "CN" }
    else { "DEFAULT" }
}

pub fn is_compliance_violation(prompt: &str, country: &str) -> Option<String> {
    let lower = prompt.to_lowercase();
    let rules = COUNTRY_RULES.get(country).unwrap_or(&COUNTRY_RULES["DEFAULT"]);

    for banned in rules {
        if lower.contains(banned) {
            return Some(format!("Từ chối: Vi phạm luật {} (quy định về {})", country, banned));
        }
    }
    None
}