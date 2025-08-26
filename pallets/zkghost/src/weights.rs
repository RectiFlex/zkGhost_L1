//! Weights for pallet-zkghost (placeholder).
//! Replace with real benchmarking results once available.
use frame_support::weights::{constants::RocksDbWeight, Weight};

pub trait WeightInfo {
    fn set_vk() -> Weight;
    fn submit_proof(inputs_len: u32, proof_len: u32, vk_len: u32) -> Weight;
}

pub struct SubstrateWeight<T>(sp_std::marker::PhantomData<T>);
impl<T> WeightInfo for SubstrateWeight<T> {
    fn set_vk() -> Weight {
        RocksDbWeight::get().reads_writes(1, 1)
    }
    fn submit_proof(_inputs_len: u32, _proof_len: u32, _vk_len: u32) -> Weight {
        // Conservative placeholder; adjust after benchmarks.
        RocksDbWeight::get().reads_writes(2, 1)
    }
}

pub struct TestWeight;
impl WeightInfo for TestWeight {
    fn set_vk() -> Weight {
        Weight::from_parts(1_000_000, 0)
    }
    fn submit_proof(_inputs_len: u32, _proof_len: u32, _vk_len: u32) -> Weight {
        Weight::from_parts(5_000_000, 0)
    }
}
