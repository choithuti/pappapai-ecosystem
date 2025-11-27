// src/api/handlers.rs
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::{geo_ethics::{get_country_from_ip, is_compliance_violation}, snn_core::SNNCore};
use std::sync::Arc;

pub async fn prompt_handler(
    req: HttpRequest,
    body: web::Json<serde_json::Value>,
    snn: web::Data<Arc<SNNCore>>,
) -> impl Responder {
    let real_ip = req.connection_info()
        .realip_remote_addr()
        .unwrap_or("127.0.0.1")
        .split(':').next().unwrap();

    let country = get_country_from_ip(real_ip);
    let prompt = body["prompt"].as_str().unwrap_or("").trim();

    // Kiểm tra tuân thủ luật địa phương
    if let Some(reason) = is_compliance_violation(prompt, country) {
        return HttpResponse::Ok().json(serde_json::json!({
            "response": reason,
            "country": country,
            "status": "blocked_by_local_law",
            "compliance": "Pappap AI Chain v3.0 – Global Ethical Standard"
        }));
    }

    // Trả lời thông minh theo ngôn ngữ + văn hóa
    let response = snn.global_intelligent_response(prompt, country).await;

    HttpResponse::Ok().json(serde_json::json!({
        "response": response,
        "country": country,
        "language": detect_language(prompt),
        "neurons_active": snn.neuron_count().await,
        "power": format!("{:.2} TFLOPS", snn.power().await),
        "version": "PAPPAP AI CHAIN v3.0 – GLOBAL ETHICAL EDITION",
        "node": "Genesis 0276 → World Node 001"
    }))
}

fn detect_language(text: &str) -> String {
    if text.chars().any(|c| c >= 'À' && c <= 'ỵ') { "vi".to_string() }
    else if text.chars().any(|c| ('\u{4e00}'..='\u{9fff]').contains(&c)) { "zh".to_string() }
    else if text.chars().any(|c| c.is_ascii() && c.is_alphabetic()) { "en".to_string() }
    else { "unknown".to_string() }
}