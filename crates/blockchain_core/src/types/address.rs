use crate::error::TypeError;
use crate::utils::hex::decode_fixed_hex;
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Address([u8; 20]);

impl Address {
    pub fn new(input: &str) -> Result<Self, TypeError> {
        Self::from_hex(input)
    }

    pub fn from_hex(input: &str) -> Result<Self, TypeError> {
        let raw = decode_fixed_hex(
            input,
            TypeError::EmptyAddress,
            TypeError::InvalidAddressLength,
            TypeError::InvalidAddressHex,
        )?;
        Ok(Self(raw))
    }

    pub fn from_public_key(public_key: &[u8]) -> Self {
        let digest = Keccak256::digest(public_key);
        let mut raw = [0_u8; 20];
        raw.copy_from_slice(&digest[12..]);
        Self(raw)
    }

    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }

    pub fn to_hex(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }
}

impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_hex())
    }
}
