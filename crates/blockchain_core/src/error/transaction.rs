use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TransactionError {
    #[error("transaction amount must be greater than zero")]
    InvalidAmount,

    #[error("transaction fee cannot exceed amount")]
    FeeExceedsAmount,

    #[error("transaction hash does not match payload")]
    InvalidHash,

    #[error("transaction signature is invalid")]
    InvalidSignature,
}
