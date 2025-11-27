// src/storage.rs
use rocksdb::{DB, Options};
use std::path::Path;

pub struct ChainStorage {
    db: DB,
}

impl ChainStorage {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).unwrap();
        Self { db }
    }

    pub fn put_knowledge(&self, key: &str, value: &[u8]) {
        self.db.put(key, value).unwrap();
    }

    pub fn get_knowledge(&self, key: &str) -> Option<Vec<u8>> {
        self.db.get(key).unwrap()
    }
}