pub mod block;
pub mod mempool;
pub mod transaction;
pub mod types;

pub use block::*;
pub use mempool::*;
pub use transaction::*;
pub use types::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error(transparent)]
    Type(#[from] TypeError),

    #[error(transparent)]
    Transaction(#[from] TransactionError),

    #[error(transparent)]
    Mempool(#[from] MempoolError),

    #[error(transparent)]
    Block(#[from] BlockError),
}
