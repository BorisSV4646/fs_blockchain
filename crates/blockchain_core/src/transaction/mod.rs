use crate::types::{Address, Timestamp};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub timestamp: Timestamp,
}

impl Transaction {
    pub fn new(from: Address, to: Address, amount: u64, timestamp: Timestamp) -> Self {
        let _ = (from, to, amount, timestamp);
        todo!("transaction constructor implementation is not added yet")
    }

    pub fn is_valid(&self) -> bool {
        let _ = self;
        todo!("transaction validation implementation is not added yet")
    }
}
