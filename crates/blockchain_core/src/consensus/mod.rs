use crate::block::Block;

#[derive(Debug, Clone)]
pub struct PowConsensus {
    pub difficulty: usize,
}

impl PowConsensus {
    pub fn new(difficulty: usize) -> Self {
        let _ = difficulty;
        todo!("consensus constructor implementation is not added yet")
    }

    pub fn mine_block(&self, block: &mut Block) {
        let _ = (self, block);
        todo!("pow mining implementation is not added yet")
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        let _ = (self, block);
        todo!("block validation implementation is not added yet")
    }
}
