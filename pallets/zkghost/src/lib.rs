#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;
#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;
mod verifier;

use sp_std::prelude::*;

pub type CircuitId = u32;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        pallet_prelude::*,
        BoundedVec,
        Blake2_128Concat,
    };
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: crate::weights::WeightInfo;
        /// Max length for a proof (bytes)
        #[pallet::constant]
        type MaxProofLen: Get<u32>;
        /// Max length for inputs (bytes)
        #[pallet::constant]
        type MaxInputsLen: Get<u32>;
        /// Max length for VK metadata (bytes)
        #[pallet::constant]
        type MaxVkMetaLen: Get<u32>;
        /// Max accepted proofs per block
        #[pallet::constant]
        type MaxProofsPerBlock: Get<u32>;
    }

    type ProofOf<T> = BoundedVec<u8, <T as Config>::MaxProofLen>;
    type InputsOf<T> = BoundedVec<u8, <T as Config>::MaxInputsLen>;
    type VkMetaOf<T> = BoundedVec<u8, <T as Config>::MaxVkMetaLen>;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn verifying_keys)]
    pub type VerifyingKeys<T: Config> = StorageMap<_, Blake2_128Concat, CircuitId, VkMetaOf<T>>;

    #[pallet::storage]
    #[pallet::getter(fn proofs_this_block)]
    pub type ProofsThisBlock<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        VkSet { vk_id: CircuitId },
        ProofSubmitted { who: T::AccountId, vk_id: CircuitId },
    }

    #[pallet::error]
    pub enum Error<T> {
        VkNotFound,
        TooManyProofsThisBlock,
        InvalidProof,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            // Reset per-block counter. Minimal weight since a single write.
            ProofsThisBlock::<T>::put(0);
            Weight::from_parts(1_000, 0)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::set_vk(vk_meta.len() as u32))]
        pub fn set_vk(origin: OriginFor<T>, vk_id: CircuitId, vk_meta: VkMetaOf<T>) -> DispatchResult {
            ensure_root(origin)?;
            VerifyingKeys::<T>::insert(vk_id, vk_meta);
            Self::deposit_event(Event::VkSet { vk_id });
            Ok(())
        }

        #[pallet::weight(T::WeightInfo::submit_proof(inputs.len() as u32))]
        pub fn submit_proof(
            origin: OriginFor<T>,
            vk_id: CircuitId,
            proof: ProofOf<T>,
            inputs: InputsOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(VerifyingKeys::<T>::contains_key(vk_id), Error::<T>::VkNotFound);

            let mut count = ProofsThisBlock::<T>::get();
            ensure!(count < T::MaxProofsPerBlock::get(), Error::<T>::TooManyProofsThisBlock);

            // Verification shim (feature-gated real ZK, stub otherwise)
            let ok = Self::verify_proof_via_shim(vk_id, &proof, &inputs);
            ensure!(ok, Error::<T>::InvalidProof);

            count = count.saturating_add(1);
            ProofsThisBlock::<T>::put(count);

            Self::deposit_event(Event::ProofSubmitted { who, vk_id });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn verify_proof_via_shim(_vk_id: CircuitId, proof: &[u8], inputs: &[u8]) -> bool {
            #[cfg(feature = "zk-verify")]
            {
                // Delegate to verifier module; returns bool
                return crate::verifier::verify_groth16_bn254(proof, inputs);
            }
            // Offline/dev default: accept non-empty proof/inputs as placeholder
            !proof.is_empty() && !inputs.is_empty()
        }
    }
}
