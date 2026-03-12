use sha3::{Digest, Keccak256};

use crate::error::TypeError;
use crate::utils::hex::decode_fixed_hex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(input: &str) -> Self {
        Self::keccak(input.as_bytes())
    }

    pub fn keccak(input: &[u8]) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(input);
        let result = hasher.finalize();
        let mut raw = [0_u8; 32];
        raw.copy_from_slice(&result);
        Self(raw)
    }

    pub fn from_hex(hex: &str) -> Result<Self, TypeError> {
        let raw = decode_fixed_hex(
            hex,
            TypeError::EmptyHash,
            TypeError::InvalidHashLength,
            TypeError::InvalidHashHex,
        )?;
        Ok(Self(raw))
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }
}

impl core::fmt::Display for Hash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_hex())
    }
}
