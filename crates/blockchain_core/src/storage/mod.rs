use crate::block::Block;
use crate::error::BlockchainError;

pub trait ChainStorage {
    fn save(&self, blocks: &[Block]) -> Result<(), BlockchainError>;
    fn load(&self) -> Result<Vec<Block>, BlockchainError>;
}

#[derive(Debug, Clone)]
pub struct FileStorage {
    pub path: String,
}

impl FileStorage {
    pub fn new(path: impl Into<String>) -> Self {
        let _ = path;
        todo!("file storage constructor implementation is not added yet")
    }
}

impl ChainStorage for FileStorage {
    fn save(&self, blocks: &[Block]) -> Result<(), BlockchainError> {
        let _ = (self, blocks);
        todo!("save implementation is not added yet")
    }

    fn load(&self) -> Result<Vec<Block>, BlockchainError> {
        let _ = self;
        todo!("load implementation is not added yet")
    }
}
