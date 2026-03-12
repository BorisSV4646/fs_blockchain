use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TypeError {
    #[error("address cannot be empty")]
    EmptyAddress,

    #[error("address must be exactly 20 bytes")]
    InvalidAddressLength,

    #[error("address must contain only hexadecimal characters")]
    InvalidAddressHex,

    #[error("signature cannot be empty")]
    EmptySignature,

    #[error("signature must be exactly 65 bytes")]
    InvalidSignatureLength,

    #[error("signature must contain only hexadecimal characters")]
    InvalidSignatureHex,

    #[error("signature recovery id must be 0, 1, 27 or 28")]
    InvalidRecoveryId,

    #[error("hash cannot be empty")]
    EmptyHash,

    #[error("hash must be exactly 32 bytes")]
    InvalidHashLength,

    #[error("hash must contain only hexadecimal characters")]
    InvalidHashHex,
}
