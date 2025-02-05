use std::collections::HashMap;

use crate::blockchain::transaction::Transaction;

pub struct Ledger {
    pub balances: HashMap<String, f64>,
}

#[derive(Debug)]
pub enum TransactionError {
    InsufficientFunds,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            balances: HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, address: String, amount: f64) {
        self.balances.insert(address, amount);
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        *self.balances.get(address).unwrap_or(&0.0)
    }
}

impl Ledger {
    pub fn apply_transaction(&mut self, tx: &Transaction) -> Result<(), TransactionError> {
        let sender_balance = self.get_balance(&tx.sender);
        if sender_balance < tx.amount {
            return Err(TransactionError::InsufficientFunds);
        }

        self.balances
            .insert(tx.sender.clone(), sender_balance - tx.amount);

        let recipient_balance = self.get_balance(&tx.recipient);
        self.balances
            .insert(tx.recipient.clone(), recipient_balance + tx.amount);

        Ok(())
    }
}
