#![cfg_attr(not(feature = "std"), no_std)]

// FRAME-style pallet for zkGhost.
//
// Functionality (MVP):
// - Store verifying keys (VK) by circuit id.
// - Track a small metadata tuple (vk_hash, last_updated_block).
// - Enforce per-block cap on proof submissions.
// - `set_vk` (Root) and `submit_proof` (Signed).
// - Input length checks using BoundedVec and explicit ensures.
// - Placeholder verifier that always returns false.
//
// Integrators: replace `verify_proof` with arkworks-based verification (e.g., ark-groth16).

use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use core::marker::PhantomData;
    use frame_support::{
        pallet_prelude::*,
        BoundedVec,
        Blake2_128Concat,
    };
    use frame_system::pallet_prelude::*;
    use sp_io::hashing::blake2_256;

    pub type CircuitId = u32;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Max serialized length in bytes of a verifying key.
        #[pallet::constant]
        type MaxVkLength: Get<u32>;

        /// Max serialized length of a proof (bytes).
        #[pallet::constant]
        type MaxProofLength: Get<u32>;

        /// Max serialized length of public inputs (bytes).
        #[pallet::constant]
        type MaxPublicInputsLength: Get<u32>;

        /// Max proofs allowed per block to throttle verification.
        #[pallet::constant]
        type MaxProofsPerBlock: Get<u32>;

        /// Weight info placeholder. Replace with real weights when available.
        type WeightInfo: Default;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::storage]
    #[pallet::getter(fn verifying_keys)]
    pub type VerifyingKeys<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        CircuitId,
        BoundedVec<u8, T::MaxVkLength>,
        OptionQuery
    >;

    /// (vk_hash, last_updated_block) where vk_hash = blake2_256(serialized_vk)
    #[pallet::storage]
    #[pallet::getter(fn circuit_meta)]
    pub type CircuitMeta<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        CircuitId,
        ([u8; 32], BlockNumberFor<T>),
        OptionQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn proofs_this_block)]
    pub type ProofsThisBlock<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A verifying key was set for a circuit id.
        VkSet { circuit_id: CircuitId, vk_hash: [u8; 32] },
        /// A proof was verified on-chain.
        ProofVerified { who: T::AccountId, circuit_id: CircuitId, proof_hash: [u8; 32] },
    }

    #[pallet::error]
    pub enum Error<T> {
        VerifyingKeyTooLong,
        ProofTooLong,
        PublicInputsTooLong,
        UnknownCircuit,
        TooManyProofsThisBlock,
        InvalidProof,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            // Reset per-block proof counter.
            ProofsThisBlock::<T>::put(0u32);
            // Weight accounting intentionally omitted in this scaffold.
            0
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set the verifying key bytes for a circuit id. Only Root may call.
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn set_vk(
            origin: OriginFor<T>,
            circuit_id: CircuitId,
            vk: Vec<u8>,
        ) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                (vk.len() as u32) <= T::MaxVkLength::get(),
                Error::<T>::VerifyingKeyTooLong
            );
            let bounded: BoundedVec<u8, T::MaxVkLength> =
                BoundedVec::try_from(vk).expect("length checked; qed");

            let vk_hash = blake2_256(&bounded);
            VerifyingKeys::<T>::insert(circuit_id, &bounded);
            CircuitMeta::<T>::insert(
                circuit_id,
                (vk_hash, <frame_system::Pallet<T>>::block_number()),
            );

            Self::deposit_event(Event::VkSet { circuit_id, vk_hash });
            Ok(())
        }

        /// Submit a proof for verification. Will fail in MVP since verifier returns false.
        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn submit_proof(
            origin: OriginFor<T>,
            circuit_id: CircuitId,
            proof: Vec<u8>,
            public_inputs: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Input size checks prior to bounding.
            ensure!(
                (proof.len() as u32) <= T::MaxProofLength::get(),
                Error::<T>::ProofTooLong
            );
            ensure!(
                (public_inputs.len() as u32) <= T::MaxPublicInputsLength::get(),
                Error::<T>::PublicInputsTooLong
            );

            let proof_bounded: BoundedVec<u8, T::MaxProofLength> =
                BoundedVec::try_from(proof).expect("length checked; qed");
            let inputs_bounded: BoundedVec<u8, T::MaxPublicInputsLength> =
                BoundedVec::try_from(public_inputs).expect("length checked; qed");

            // Ensure circuit exists.
            ensure!(VerifyingKeys::<T>::contains_key(circuit_id), Error::<T>::UnknownCircuit);

            // Throttle per-block.
            let count = ProofsThisBlock::<T>::get();
            ensure!(count < T::MaxProofsPerBlock::get(), Error::<T>::TooManyProofsThisBlock);
            ProofsThisBlock::<T>::put(count + 1);

            let proof_hash = blake2_256(&proof_bounded);

            // Placeholder: always false. Replace with arkworks verifier integration.
            let ok = Self::verify_proof(circuit_id, &proof_bounded, &inputs_bounded);
            ensure!(ok, Error::<T>::InvalidProof);

            Self::deposit_event(Event::ProofVerified { who, circuit_id, proof_hash });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Placeholder verifier.
        /// Replace with ark-groth16 verification during integration.
        fn verify_proof(
            _circuit_id: CircuitId,
            _proof: &BoundedVec<u8, T::MaxProofLength>,
            _public_inputs: &BoundedVec<u8, T::MaxPublicInputsLength>,
        ) -> bool {
            false
        }
    }
}
