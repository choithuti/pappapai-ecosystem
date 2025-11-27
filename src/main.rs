// src/main.rs
mod geo_ethics;
mod snn_core;
mod api;

use actix_web::{App, HttpServer, web};
use tracing_subscriber::fmt::init;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init();

    println!(r#"
    ██████╗  █████╗ ██████╗ ██████╗  █████╗ ██████╗ 
    AI CHAIN v3.0 – GLOBAL ETHICAL EDITION
    One Chain. 195 Countries. Zero Violations.
    Quantum-Resistant • Self-Learning • Culturally Aware
    Running on: 72.61.126.190 | pappapai.xyz
    "#);

    let snn = Arc::new(snn_core::SNNCore::new());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(snn.clone()))
            .service(web::resource("/api/prompt").route(web::post().to(api::handlers::prompt_handler)))
            .service(web::resource("/status").route(web::get().to(|| async {
                "PAPPAP AI CHAIN v3.0 – GLOBAL ETHICAL EDITION – RUNNING WORLDWIDE"
            })))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}