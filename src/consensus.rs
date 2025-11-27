// src/consensus.rs
use crate::snn_core::SNNCore;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct PoSNN {
    snn: Arc<SNNCore>,
}

impl PoSNN {
    pub fn new(snn: Arc<SNNCore>) -> Self { Self { snn } }

    pub async fn propose_block(&self, data: Vec<u8>) -> bool {
        let challenge: f32 = rand::random();
        let spike_rate = self.snn.forward(challenge * 10.0).await;
        
        // Chỉ node có spike rate cao + ổn định mới được propose
        spike_rate > 0.67 && spike_rate < 0.89
    }

    pub async fn validate_block(&self, proposer_rate: f32) -> bool {
        let local_rate = self.snn.forward(1.0).await;
        (local_rate - proposer_rate).abs() < 0.15
    }
}