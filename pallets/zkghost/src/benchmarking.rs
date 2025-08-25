#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;

benchmarks! {
    set_vk {
        let vk = vec![0u8; 128];
    }: _(RawOrigin::Root, 1u32, vk)

    submit_proof {
        let proof = vec![0u8; 256];
        let inputs = vec![0u8; 32];
    }: _(RawOrigin::Signed(1u64), 1u32, proof, inputs)
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
