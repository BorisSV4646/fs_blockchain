use sha2::{Digest, Sha256};

use crate::error::TypeError;

#[derive(Debug, Clone)]
pub struct Hash(String);

impl Hash {
    pub fn new(input: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        Self(format!("{:x}", result))
    }

    pub fn from_hex(hex: String) -> Result<Self, TypeError> {
        if hex.is_empty() {
            return Err(TypeError::EmptyHash);
        }

        if !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return Err(TypeError::InvalidHashHex);
        }

        Ok(Self(hex))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
