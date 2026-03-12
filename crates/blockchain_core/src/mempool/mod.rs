use std::collections::HashSet;

use crate::error::MempoolError;
use crate::transaction::Transaction;
use crate::types::Hash;

pub trait ChainStateView {
    fn validate_mempool_transaction(
        &self,
        transaction: &Transaction,
        mempool: &Mempool,
    ) -> Result<(), MempoolError> {
        let _ = (transaction, mempool);
        todo!("chain state view implementation is not added yet")
    }
}

impl ChainStateView for () {}

#[derive(Debug, Default, Clone)]
pub struct Mempool {
    pub transactions: Vec<Transaction>,
    hashes: HashSet<Hash>,
}

impl Mempool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_transaction(
        &mut self,
        transaction: Transaction,
        state: &impl ChainStateView,
    ) -> Result<(), MempoolError> {
        transaction.validate()?;
        self.validate_duplicate(&transaction)?;
        self.validate_against_chain_state(&transaction, state)?;

        self.hashes.insert(transaction.hash);
        self.transactions.push(transaction);
        Ok(())
    }

    pub fn drain_all(&mut self) -> Vec<Transaction> {
        self.hashes.clear();
        core::mem::take(&mut self.transactions)
    }

    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    fn validate_duplicate(&self, transaction: &Transaction) -> Result<(), MempoolError> {
        if self.hashes.contains(&transaction.hash) {
            return Err(MempoolError::DuplicateTransaction);
        }

        Ok(())
    }

    fn validate_against_chain_state(
        &self,
        transaction: &Transaction,
        state: &impl ChainStateView,
    ) -> Result<(), MempoolError> {
        state.validate_mempool_transaction(transaction, self)
    }
}
