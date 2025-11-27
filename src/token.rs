// src/token.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PappapToken {
    pub name: String,
    pub symbol: String,
    pub total_supply: u128,
    pub decimals: u8,
    pub genesis: HashMap<String, u128>,
}

impl PappapToken {
    pub fn genesis() -> Self {
        let mut genesis = HashMap::new();
        genesis.insert("MAPLE0276_GENESIS_001".to_string(), 120_000_000_000_000_000_000u128); // 120M
        genesis.insert("vietnam_ai_fund_2025".to_string(), 300_000_000_000_000_000_000u128);
        genesis.insert("community_airdrop".to_string(), 580_000_000_000_000_000_000u128);

        Self {
            name: "Pappap AI Token".to_string(),
            symbol: "PAPPAP".to_string(),
            total_supply: 1_000_000_000_000_000_000_000u128,
            decimals: 18,
            genesis,
        }
    }
}