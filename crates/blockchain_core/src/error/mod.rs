pub mod block;
pub mod transaction;
pub mod types;

pub use block::*;
pub use transaction::*;
pub use types::*;

use thiserror::Error;

#[derive(Debug, Error)]1
pub enum BlockchainError {
    #[error(transparent)]
    Type(#[from] TypeError),

    #[error(transparent)]
    Transaction(#[from] TransactionError),

    #[error(transparent)]
    Block(#[from] BlockError),
}
