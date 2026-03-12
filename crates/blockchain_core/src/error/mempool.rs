use thiserror::Error;

use crate::error::TransactionError;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum MempoolError {
    #[error(transparent)]
    Transaction(#[from] TransactionError),

    #[error("transaction already exists in mempool")]
    DuplicateTransaction,

    #[error("transaction failed mempool chain state validation")]
    ChainStateValidationFailed,
}
