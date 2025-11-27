// src/knowledge_chain.rs
use ipfs_api::{IpfsClient, TryFromUri};

#[derive(serde::Serialize, Deserialize)]
pub struct KnowledgeBlock {
    pub question: String,
    pub answer: String,
    pub sources: Vec<String>,
    pub timestamp: i64,
    pub author: String, // PeerId
    pub signature: String,
}

impl KnowledgeBlock {
    pub async fn publish(&self) -> String {
        let data = serde_json::to_vec(self).unwrap();
        let cid = IpfsClient::default().add(data).await.unwrap();
        format!("bafy{}", cid.hash)
    }
}