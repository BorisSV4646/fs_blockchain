use rlp::RlpStream;
use secp256k1::ecdsa::{RecoverableSignature, RecoveryId};
use secp256k1::{Message, Secp256k1};

use crate::error::TransactionError;
use crate::types::{Address, Hash, Signature};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: Hash,
    pub chain_id: u64,
    pub nonce: u64,
    pub to: Address,
    pub value: u64,
    pub gas_limit: u64,
    pub max_fee_per_gas: u64,
    pub max_priority_fee_per_gas: u64,
    pub data: Vec<u8>,
    pub signature: Signature,
}

impl Transaction {
    const TYPE_BYTE: u8 = 0x02;

    fn unsigned_payload(
        chain_id: u64,
        nonce: u64,
        to: &Address,
        value: u64,
        gas_limit: u64,
        max_fee_per_gas: u64,
        max_priority_fee_per_gas: u64,
        data: &[u8],
    ) -> Vec<u8> {
        let mut stream = RlpStream::new_list(9);
        stream.append(&chain_id);
        stream.append(&nonce);
        stream.append(&max_priority_fee_per_gas);
        stream.append(&max_fee_per_gas);
        stream.append(&gas_limit);
        stream.append(&to.as_bytes().to_vec());
        stream.append(&value);
        stream.append(&data.to_vec());
        stream.begin_list(0);
        stream.out().to_vec()
    }

    fn signed_payload(
        chain_id: u64,
        nonce: u64,
        to: &Address,
        value: u64,
        gas_limit: u64,
        max_fee_per_gas: u64,
        max_priority_fee_per_gas: u64,
        data: &[u8],
        signature: &Signature,
    ) -> Vec<u8> {
        let mut stream = RlpStream::new_list(12);
        stream.append(&chain_id);
        stream.append(&nonce);
        stream.append(&max_priority_fee_per_gas);
        stream.append(&max_fee_per_gas);
        stream.append(&gas_limit);
        stream.append(&to.as_bytes().to_vec());
        stream.append(&value);
        stream.append(&data.to_vec());
        stream.begin_list(0);
        stream.append(&u8::from(signature.y_parity()));
        stream.append(&trim_leading_zeroes(signature.r()));
        stream.append(&trim_leading_zeroes(signature.s()));
        stream.out().to_vec()
    }

    fn prefixed_hash(payload: Vec<u8>) -> Hash {
        let mut typed_payload = Vec::with_capacity(payload.len() + 1);
        typed_payload.push(Self::TYPE_BYTE);
        typed_payload.extend(payload);
        Hash::keccak(&typed_payload)
    }

    fn calculate_signing_hash(
        chain_id: u64,
        nonce: u64,
        to: &Address,
        value: u64,
        gas_limit: u64,
        max_fee_per_gas: u64,
        max_priority_fee_per_gas: u64,
        data: &[u8],
    ) -> Hash {
        let payload = Self::unsigned_payload(
            chain_id,
            nonce,
            to,
            value,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            data,
        );
        Self::prefixed_hash(payload)
    }

    fn calculate_hash(
        chain_id: u64,
        nonce: u64,
        to: &Address,
        value: u64,
        gas_limit: u64,
        max_fee_per_gas: u64,
        max_priority_fee_per_gas: u64,
        data: &[u8],
        signature: &Signature,
    ) -> Hash {
        let payload = Self::signed_payload(
            chain_id,
            nonce,
            to,
            value,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            data,
            signature,
        );
        Self::prefixed_hash(payload)
    }

    pub fn new(
        chain_id: u64,
        nonce: u64,
        to: Address,
        value: u64,
        gas_limit: u64,
        max_fee_per_gas: u64,
        max_priority_fee_per_gas: u64,
        data: Vec<u8>,
        signature: Signature,
    ) -> Self {
        let hash = Self::calculate_hash(
            chain_id,
            nonce,
            &to,
            value,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            &data,
            &signature,
        );

        Self {
            hash,
            chain_id,
            nonce,
            to,
            value,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            data,
            signature,
        }
    }

    pub fn signing_hash(&self) -> Hash {
        Self::calculate_signing_hash(
            self.chain_id,
            self.nonce,
            &self.to,
            self.value,
            self.gas_limit,
            self.max_fee_per_gas,
            self.max_priority_fee_per_gas,
            &self.data,
        )
    }

    pub fn recover_signer(&self) -> Result<Address, TransactionError> {
        let signing_hash = self.signing_hash();
        let message = Message::from_digest(*signing_hash.as_bytes());
        let recovery_id = RecoveryId::try_from(i32::from(self.signature.recovery_id()))
            .map_err(|_| TransactionError::InvalidSignature)?;
        let recoverable_signature =
            RecoverableSignature::from_compact(&self.signature.compact_bytes(), recovery_id)
                .map_err(|_| TransactionError::InvalidSignature)?;
        let public_key = Secp256k1::new()
            .recover_ecdsa(message, &recoverable_signature)
            .map_err(|_| TransactionError::InvalidSignature)?;
        let serialized = public_key.serialize_uncompressed();
        Ok(Address::from_public_key(&serialized[1..]))
    }

    pub fn validate(&self) -> Result<(), TransactionError> {
        if self.chain_id == 0 {
            return Err(TransactionError::InvalidChainId);
        }

        if self.gas_limit == 0 {
            return Err(TransactionError::InvalidGasLimit);
        }

        if self.max_priority_fee_per_gas > self.max_fee_per_gas {
            return Err(TransactionError::PriorityFeeExceedsMaxFee);
        }

        let expected_hash = Self::calculate_hash(
            self.chain_id,
            self.nonce,
            &self.to,
            self.value,
            self.gas_limit,
            self.max_fee_per_gas,
            self.max_priority_fee_per_gas,
            &self.data,
            &self.signature,
        );

        if expected_hash != self.hash {
            return Err(TransactionError::InvalidHash);
        }

        self.recover_signer()?;

        Ok(())
    }

    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

fn trim_leading_zeroes(bytes: &[u8; 32]) -> Vec<u8> {
    let first_non_zero = bytes
        .iter()
        .position(|byte| *byte != 0)
        .unwrap_or(bytes.len());
    bytes[first_non_zero..].to_vec()
}
