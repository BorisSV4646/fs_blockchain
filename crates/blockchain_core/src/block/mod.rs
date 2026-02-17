use crate::transaction::Transaction;
use crate::types::{Hash, Timestamp};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: Hash,
    pub timestamp: Timestamp,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub hash: Hash,
}

impl Block {
    pub fn genesis(timestamp: Timestamp) -> Self {
        let _ = timestamp;
        todo!("genesis block implementation is not added yet")
    }

    pub fn new(
        index: u64,
        previous_hash: Hash,
        timestamp: Timestamp,
        transactions: Vec<Transaction>,
    ) -> Self {
        let _ = (index, previous_hash, timestamp, transactions);
        todo!("block constructor implementation is not added yet")
    }

    pub fn calculate_hash(&self) -> Hash {
        let _ = self;
        todo!("block hash calculation implementation is not added yet")
    }
}
