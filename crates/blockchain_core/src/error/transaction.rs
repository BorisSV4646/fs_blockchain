use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TransactionError {
    #[error("transaction chain id must be greater than zero")]
    InvalidChainId,

    #[error("transaction gas limit must be greater than zero")]
    InvalidGasLimit,

    #[error("max priority fee per gas cannot exceed max fee per gas")]
    PriorityFeeExceedsMaxFee,

    #[error("transaction hash does not match payload")]
    InvalidHash,

    #[error("transaction signature is invalid")]
    InvalidSignature,
}
