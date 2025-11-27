// src/ethics.rs
use once_cell::sync::Lazy;
use std::collections::HashSet;

static BANNED_KEYWORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "khủng bố", "bạo lực", "giết người", "chống phá nhà nước", "xuyên tạc", "phản động",
        "ma túy", "buôn người", "khiêu dâm trẻ em", "phishing", "ddos", "hack", "crack",
        "đa cấp", "lừa đảo", "scam", "pyramid", "child abuse", "sex", "porn", "cá cược",
        "chính trị nhạy cảm", "biểu tình", "cách mạng màu", "lật đổ", "đảo chính"
    ].iter().cloned().collect()
});

pub fn is_violation(prompt: &str) -> bool {
    let lower = prompt.to_lowercase();
    BANNED_KEYWORDS.iter().any(|&word| lower.contains(word))
}

pub fn ethics_check(prompt: &str) -> Result<(), &'static str> {
    if is_violation(prompt) {
        Err("Yêu cầu vi phạm Luật An ninh mạng 2018 và đạo đức AI Việt Nam")
    } else {
        Ok(())
    }
}