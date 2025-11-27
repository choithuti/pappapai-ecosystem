// src/bus.rs – FINAL VERSION
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct GlobalBus {
    tx: broadcast::Sender<(String, Vec<u8>)>,
}

impl GlobalBus {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(4096);
        Self { tx }
    }

    pub fn sender(&self) -> broadcast::Sender<(String, Vec<u8>)> {
        self.tx.clone()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<(String, Vec<u8>)> {
        self.tx.subscribe()
    }
}