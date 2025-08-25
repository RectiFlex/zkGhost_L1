#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum VerifyError {
    Disabled,
    Malformed,
    Backend,
}

pub trait VerifierBackend {
    fn verify_groth16_bn254(vk_bytes: &[u8], proof_bytes: &[u8], pub_inputs_bytes: &[u8]) -> Result<bool, VerifyError>;
}

// Stub backend when zk-verify feature is not enabled
#[cfg(not(feature = "zk-verify"))]
pub struct StubBackend;
#[cfg(not(feature = "zk-verify"))]
impl VerifierBackend for StubBackend {
    fn verify_groth16_bn254(_vk_bytes: &[u8], _proof_bytes: &[u8], _pub_inputs_bytes: &[u8]) -> Result<bool, VerifyError> {
        Err(VerifyError::Disabled)
    }
}

// Arkworks-backed verifier for Groth16 over BN254
#[cfg(feature = "zk-verify")]
mod ark_backend {
    extern crate alloc;
    use super::{VerifyError, VerifierBackend};
    use alloc::vec::Vec;
    use ark_bn254::Bn254;
    use ark_ff::PrimeField;
    use ark_groth16::{prepare_verifying_key, verify_proof, Proof, VerifyingKey};
    use ark_serialize::CanonicalDeserialize;

    pub struct ArkBn254Backend;

    impl VerifierBackend for ArkBn254Backend {
        fn verify_groth16_bn254(vk_bytes: &[u8], proof_bytes: &[u8], pub_inputs_bytes: &[u8]) -> Result<bool, VerifyError> {
            // Deserialize VK and Proof from canonical ark-serialize bytes
            let vk = VerifyingKey::<Bn254>::deserialize_compressed(vk_bytes).or_else(|_| VerifyingKey::<Bn254>::deserialize_uncompressed(vk_bytes)).map_err(|_| VerifyError::Malformed)?;
            let proof = Proof::<Bn254>::deserialize_compressed(proof_bytes).or_else(|_| Proof::<Bn254>::deserialize_uncompressed(proof_bytes)).map_err(|_| VerifyError::Malformed)?;
            // Parse public inputs as 32-byte little-endian Fr field elements concatenated
            if pub_inputs_bytes.len() % 32 != 0 { return Err(VerifyError::Malformed); }
            let mut inputs = Vec::with_capacity(pub_inputs_bytes.len() / 32);
            for chunk in pub_inputs_bytes.chunks(32) {
                let mut le = [0u8; 32];
                le.copy_from_slice(chunk);
                let fr = <ark_bn254::Fr as PrimeField>::from_le_bytes_mod_order(&le);
                inputs.push(fr);
            }
            let pvk = prepare_verifying_key(&vk);
            verify_proof(&pvk, &proof, &inputs).map_err(|_| VerifyError::Backend)
        }
    }

    pub use ArkBn254Backend as Backend;
}

#[cfg(feature = "zk-verify")]
pub use ark_backend::Backend;
#[cfg(not(feature = "zk-verify"))]
pub use StubBackend as Backend;
