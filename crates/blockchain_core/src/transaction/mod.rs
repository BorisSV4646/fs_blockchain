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
    fn calculate_hash(
        from: &Address,
        to: &Address,
        amount: u64,
        timestamp: &Timestamp,
        fee: u64,
        nonce: u64,
    ) -> Hash {
        let raw = format!(
            "{}:{}:{}:{}:{}:{}",
            from.as_str(),
            to.as_str(),
            amount,
            timestamp.as_u64(),
            fee,
            nonce
        );
        Hash::new(&raw)
    }

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
        let hash = Self::calculate_hash(&from, &to, amount, &timestamp, fee, nonce);

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
        if self.amount == 0 {
            return false;
        }

        if self.fee > self.amount {
            return false;
        }

        if self.signature.as_str().trim().is_empty() {
            return false;
        }

        let expected_hash = Self::calculate_hash(
            &self.from,
            &self.to,
            self.amount,
            &self.timestamp,
            self.fee,
            self.nonce,
        );

        expected_hash.as_str() == self.hash.as_str()
    }
}
