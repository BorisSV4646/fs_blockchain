#[derive(Debug)]
pub enum BlockchainError {
    EmptyChain,
    InvalidBlock,
    StorageError(String),
}
