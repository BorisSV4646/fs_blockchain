use crate::transaction::Transaction;

#[derive(Debug, Default, Clone)]
pub struct Mempool {
    pub transactions: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        todo!("mempool constructor implementation is not added yet")
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> bool {
        let _ = (self, transaction);
        todo!("mempool add transaction implementation is not added yet")
    }

    pub fn drain_all(&mut self) -> Vec<Transaction> {
        let _ = self;
        todo!("mempool drain implementation is not added yet")
    }

    pub fn len(&self) -> usize {
        let _ = self;
        todo!("mempool len implementation is not added yet")
    }
}
