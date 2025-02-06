use super::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, Vec::new(), "0".to_string());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn last_hash(&self) -> String {
        self.blocks
            .last()
            .map(|b| b.hash.clone())
            .unwrap_or_default()
    }
}
