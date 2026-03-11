use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum BlockError {
    #[error("block index is invalid")]
    InvalidIndex,

    #[error("block previous hash does not match chain")]
    InvalidPreviousHash,

    #[error("block must contain at least one transaction")]
    EmptyTransactions,

    #[error("block hash does not match payload")]
    InvalidHash,
}
