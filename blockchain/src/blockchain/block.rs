use super::transaction::Transaction;
use crate::wallet::balance::{Ledger, TransactionError};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, prev_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let nonce = 0;
        let mut block = Block {
            index,
            timestamp,
            prev_hash,
            nonce,
            hash: String::new(),
            transactions,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let tx_data = serde_json::to_string(&self.transactions)
            .unwrap_or_else(|_| "serialization_error".to_string());
        let record = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, tx_data, self.prev_hash, self.nonce
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

    pub fn apply_block(
        ledger: &mut Ledger,
        block: &crate::blockchain::block::Block,
    ) -> Result<(), TransactionError> {
        for tx in &block.transactions {
            ledger.apply_transaction(tx)?;
        }
        Ok(())
    }
}
