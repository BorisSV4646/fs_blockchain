use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TypeError {
    #[error("address cannot be empty")]
    EmptyAddress,

    #[error("signature cannot be empty")]
    EmptySignature,

    #[error("hash cannot be empty")]
    EmptyHash,

    #[error("hash must contain only hexadecimal characters")]
    InvalidHashHex,
}
