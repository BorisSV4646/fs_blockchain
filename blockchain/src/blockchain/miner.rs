use super::{block::Block, chain::Blockchain, transaction::Transaction};
use crate::{consensus::dpos::DPoS, core::errors::Result};

pub struct Miner {
    pub blockchain: Blockchain,
    pub pending_transactions: Vec<Transaction>,
}

impl Miner {
    pub fn new() -> Self {
        Miner {
            blockchain: Blockchain::new(),
            pending_transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn produce_block(&mut self, dpos: &mut DPoS) -> Result<()> {
        let selected = dpos.select_delegate()?;
        let index = self.blockchain.blocks.len() as u32;
        let prev_hash = self.blockchain.last_hash();
        let transactions = self.pending_transactions.clone();
        let new_block = Block::new(index, transactions, prev_hash);
        self.blockchain.add_block(new_block);
        self.pending_transactions.clear();

        log::info!(
            "Блок, созданный делегатом {} (id: {}), добавлен в блокчейн.",
            selected.name,
            selected.id
        );

        Ok(())
    }
}
