use crate::error::TypeError;
use crate::utils::hex::decode_fixed_hex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature {
    r: [u8; 32],
    s: [u8; 32],
    y_parity: bool,
}

impl Signature {
    pub fn new(r: [u8; 32], s: [u8; 32], y_parity: bool) -> Self {
        Self { r, s, y_parity }
    }

    pub fn from_hex(input: &str) -> Result<Self, TypeError> {
        let bytes: [u8; 65] = decode_fixed_hex(
            input,
            TypeError::EmptySignature,
            TypeError::InvalidSignatureLength,
            TypeError::InvalidSignatureHex,
        )?;
        let mut r = [0_u8; 32];
        let mut s = [0_u8; 32];
        r.copy_from_slice(&bytes[..32]);
        s.copy_from_slice(&bytes[32..64]);

        let y_parity = match bytes[64] {
            0 | 27 => false,
            1 | 28 => true,
            _ => return Err(TypeError::InvalidRecoveryId),
        };

        Ok(Self { r, s, y_parity })
    }

    pub fn r(&self) -> &[u8; 32] {
        &self.r
    }

    pub fn s(&self) -> &[u8; 32] {
        &self.s
    }

    pub fn y_parity(&self) -> bool {
        self.y_parity
    }

    pub fn recovery_id(&self) -> u8 {
        u8::from(self.y_parity)
    }

    pub fn compact_bytes(&self) -> [u8; 64] {
        let mut compact = [0_u8; 64];
        compact[..32].copy_from_slice(&self.r);
        compact[32..].copy_from_slice(&self.s);
        compact
    }

    pub fn to_hex(&self) -> String {
        let mut bytes = [0_u8; 65];
        bytes[..32].copy_from_slice(&self.r);
        bytes[32..64].copy_from_slice(&self.s);
        bytes[64] = self.recovery_id();
        format!("0x{}", hex::encode(bytes))
    }
}

impl core::fmt::Display for Signature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_hex())
    }
}
