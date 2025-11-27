// src/snn_core.rs
use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

static WELCOME_MESSAGES: Lazy<std::collections::HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = std::collections::HashMap::new();
    m.insert("VN", "Xin chào từ Việt Nam! Pappap là blockchain sống đầu tiên của thế giới, tuân thủ 100% pháp luật từng quốc gia.");
    m.insert("US", "Hello from the USA! Pappap is the world's first living, ethical, quantum-resistant AI blockchain.");
    m.insert("EU", "Hallo aus Europa! Pappap respektiert vollständig GDPR und nationale Gesetze.");
    m.insert("CN", "您好！Pappap 是全球首个完全遵守各国法律的活体区块链。");
    m.insert("DEFAULT", "Hello! Pappap AI Chain – One Chain, All Laws Respected.");
    m
});

pub struct SNNCore {
    neurons: Arc<RwLock<usize>>,
    power: Arc<RwLock<f64>>,
}

impl SNNCore {
    pub fn new() -> Self {
        Self {
            neurons: Arc::new(RwLock::new(1_800_000)),
            power: Arc::new(RwLock::new(42.0)),
        }
    }

    pub async fn neuron_count(&self) -> usize { *self.neurons.read().await }
    pub async fn power(&self) -> f64 { *self.power.read().await }

    pub async fn global_intelligent_response(&self, input: &str, country: &str) -> String {
        let lower = input.to_lowercase();

        if lower.contains("pappap") || lower.contains("ai chain") {
            return WELCOME_MESSAGES.get(country).unwrap_or(&WELCOME_MESSAGES["DEFAULT"]).to_string();
        }

        if lower.contains("hello") || lower.contains("xin chào") || lower.contains("hi") {
            return format!("Xin chào từ Pappap AI Chain! Bạn đang kết nối từ {}. Chúng tôi tuân thủ hoàn toàn luật pháp địa phương.", country);
        }

        "Pappap đang suy nghĩ và trả lời theo chuẩn đạo đức toàn cầu...".to_string()
    }
}