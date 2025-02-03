use super::{block::Block, chain::Blockchain, transaction::Transaction};

pub struct Miner {
    pub blockchain: Blockchain,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: usize,
}

impl Miner {
    pub fn new(difficulty: usize) -> Self {
        Miner {
            blockchain: Blockchain::new(),
            pending_transactions: Vec::new(),
            difficulty,
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self) {
        let data = serde_json::to_string(&self.pending_transactions)
            .unwrap_or_else(|_| "Ошибка сериализации".to_string());
        let index = self.blockchain.blocks.len() as u32;
        let prev_hash = self.blockchain.last_hash();

        let mut new_block = Block::new(index, data, prev_hash);
        new_block.mine(self.difficulty);

        self.blockchain.add_block(new_block);
        // Очистка пула транзакций после добавления блока
        self.pending_transactions.clear();
    }
}
