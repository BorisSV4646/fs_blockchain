use crate::types::{Address, Hash, Signature, Timestamp};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: Hash,
    pub nonce: u64,
    pub signature: Signature,
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub timestamp: Timestamp,
    pub fee: u64,
    pub status: TransactionStatus,
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

impl Transaction {
    pub fn new(
        from: Address,
        to: Address,
        amount: u64,
        timestamp: Timestamp,
        fee: u64,
        status: TransactionStatus,
        nonce: u64,
        signature: Signature,
    ) -> Self {
        let raw = format!(
            "{}:{}:{}:{}:{}:{}",
            from.as_str(),
            to.as_str(),
            amount,
            timestamp.as_u64(),
            fee,
            nonce
        );
        let hash = Hash::new(&raw);

        Self {
            hash: hash,
            nonce: nonce,
            signature: signature,
            from: from,
            to: to,
            amount: amount,
            timestamp: timestamp,
            fee: fee,
            status: status,
        }
    }

    pub fn is_valid(&self) -> bool {
        let _ = self;
        todo!("transaction validation implementation is not added yet")
    }
}
