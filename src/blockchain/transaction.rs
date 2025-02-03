use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub data: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, data: String, prev_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let nonce = 0;
        let mut block = Block {
            index,
            timestamp,
            data,
            prev_hash,
            nonce,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let record = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.prev_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(record.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Блок замайнен! Хэш: {}", self.hash);
    }
}
