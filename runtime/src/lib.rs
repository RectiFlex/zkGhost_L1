//! zkGhost FRAME Runtime (placeholder)
//!
//! This file documents how to integrate the `pallet-zkghost` pallet into a real FRAME runtime.
//! All imports and macros are intentionally left out or commented to keep this crate offline-only.
//!
//! Suggested structure (to be implemented in an external Substrate environment):
//!
//! ```rust,ignore
//! // use frame_support::{construct_runtime, parameter_types, traits::Everything};
//! // use frame_system as system;
//! // use pallet_zkghost;
//!
//! pub type BlockNumber = u32;
//!
//! // Bounded sizes and per-block cap
//! parameter_types! {
//!     pub const MaxVkLength: u32 = 64 * 1024;         // 64 KiB serialized VK
//!     pub const MaxProofLength: u32 = 64 * 1024;      // 64 KiB proof bytes
//!     pub const MaxPublicInputsLength: u32 = 8 * 1024;// 8 KiB public inputs
//!     pub const MaxProofsPerBlock: u32 = 50;          // throttle
//! }
//!
//! impl pallet_zkghost::Config for Runtime {
//!     type RuntimeEvent = RuntimeEvent;
//!     type MaxVkLength = MaxVkLength;
//!     type MaxProofLength = MaxProofLength;
//!     type MaxPublicInputsLength = MaxPublicInputsLength;
//!     type MaxProofsPerBlock = MaxProofsPerBlock;
//!     type WeightInfo = ();
//! }
//!
//! construct_runtime!(
//!     pub enum Runtime where
//!         Block = Block,
//!         NodeBlock = Block,
//!         UncheckedExtrinsic = UncheckedExtrinsic,
//!     {
//!         System: system,
//!         Timestamp: pallet_timestamp,
//!         Balances: pallet_balances,
//!         TransactionPayment: pallet_transaction_payment,
//!         // zkGhost pallet
//!         ZkGhost: pallet_zkghost,
//!     }
//! );
//! ```
//!
//! Consensus (BABE/GRANDPA), fees (GHOST), and networking (libp2p) are provided by Substrate.

#![cfg_attr(not(feature = "std"), no_std)]

// Intentionally empty placeholder to keep this crate self-contained without Substrate deps.
