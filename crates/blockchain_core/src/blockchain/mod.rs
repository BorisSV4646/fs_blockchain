use crate::block::Block;
use crate::error::BlockchainError;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        todo!("blockchain constructor implementation is not added yet")
    }

    pub fn add_block(&mut self, block: Block) -> Result<(), BlockchainError> {
        let _ = (self, block);
        todo!("add block implementation is not added yet")
    }

    pub fn validate_chain(&self) -> bool {
        let _ = self;
        todo!("chain validation implementation is not added yet")
    }
}
