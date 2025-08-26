#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub use frame_support::{construct_runtime, parameter_types};
use frame_support::traits::Everything;
use frame_system as system;
use sp_api::impl_runtime_apis;
use sp_core::{OpaqueMetadata, H256};
use sp_runtime::{
    generic, MultiAddress, MultiSignature,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, Verify},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult,
};
use sp_version::RuntimeVersion;

pub type BlockNumber = u32;
pub type Signature = MultiSignature;
pub type AccountId = <Signature as Verify>::Signer::AccountId;
pub type Balance = u128;
pub type Index = u32;
pub type Hash = H256;

pub type Address = MultiAddress<AccountId, ()>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type UncheckedExtrinsic = sp_runtime::OpaqueExtrinsic;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

// Aura authority ID type
pub type AuraId = sp_consensus_aura::sr25519::AuthorityId;

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: RuntimeVersion = RuntimeVersion {
        spec_name: sp_version::create_runtime_str!("zkghost"),
        impl_name: sp_version::create_runtime_str!("zkghost"),
        authoring_version: 1,
        spec_version: 1,
        impl_version: 1,
        apis: RUNTIME_API_VERSIONS,
        transaction_version: 1,
        state_version: 1,
    };
    pub const ExistentialDeposit: Balance = 1_000_000_000_000; // 0.001 GHOST if decimals=12
    pub const MinimumPeriod: u64 = 3000; // 6s block time => 3s minimum period
}

impl system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = frame_support::traits::ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_timestamp::Config for Runtime {
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
}

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type KeyOwnerProof = sp_core::Void;
    type EquivocationReportSystem = ();
    type MaxAuthorities = frame_support::traits::ConstU32<32>;
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = frame_support::traits::ConstU32<50>;
    type MaxReserves = frame_support::traits::ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<pallet_balances::Pallet<Runtime>, ()>;
    type OperationalFeeMultiplier = frame_support::traits::ConstU8<5>;
    type WeightToFee = ();
    type LengthToFee = ();
    type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
}

impl pallet_zkghost::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxVkLength = frame_support::traits::ConstU32<65536>; // 64 KiB
    type MaxProofLength = frame_support::traits::ConstU32<65536>;
    type MaxPublicInputsLength = frame_support::traits::ConstU32<8192>;
    type MaxProofsPerBlock = frame_support::traits::ConstU32<50>;
    type WeightInfo = pallet_zkghost::weights::DefaultWeight;
}

type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        TransactionPayment: pallet_transaction_payment,
        Sudo: pallet_sudo,
        ZkGhost: pallet_zkghost,
    }
);

impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion { Version::get() }
        fn execute_block(block: Block) { Executive::execute_block(block) }
        fn initialize_block(header: &<Block as BlockT>::Header) { Executive::initialize_block(header) }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata { OpaqueMetadata::new(Runtime::metadata().into()) }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: UncheckedExtrinsic) -> ApplyExtrinsicResult { Executive::apply_extrinsic(extrinsic) }
        fn finalize_block() -> <Block as BlockT>::Header { Executive::finalize_block() }
        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<UncheckedExtrinsic> { Executive::inherent_extrinsics(data) }
        fn check_inherents(block: Block, data: sp_inherents::InherentData) -> sp_inherents::CheckInherentsResult { Executive::check_inherents(block, data) }
        fn random_seed() -> <Block as BlockT>::Hash { System::random_seed() }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(source: TransactionSource, tx: UncheckedExtrinsic, block_hash: <Block as BlockT>::Hash) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration { sp_consensus_aura::SlotDuration::from_millis(MinimumPeriod::get() * 2) }
        fn authorities() -> Vec<AuraId> { Aura::authorities().into_inner() }
    }

    impl sp_finality_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> sp_finality_grandpa::AuthorityList { Grandpa::grandpa_authorities() }
    }

    impl substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index { System::account_nonce(account) }
    }

    impl pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance> for Runtime {
        fn query_info(uxt: UncheckedExtrinsic, len: u32) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(uxt: UncheckedExtrinsic, len: u32) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
    }
}


#[cfg(feature = "runtime-benchmarks")]
mod benches_runtime_api {
    use super::*;
    use frame_benchmarking::BenchmarkList;
    use sp_api::impl_runtime_apis;
    use sp_core::OpaqueMetadata;
    use sp_runtime::traits::Block as BlockT;

    impl_runtime_apis! {
        impl frame_benchmarking::Benchmark<Block> for Runtime {
            fn benchmark_metadata(extrinsic: Option<sp_runtime::RuntimeString>) -> Result<(BenchmarkList, Vec<u8>), sp_runtime::RuntimeString> {
                let list = vec![];
                Ok((list, Vec::new()))
            }
            fn dispatch_benchmark(_config: frame_benchmarking::BenchmarkConfig) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
                Ok(Vec::new())
            }
        }
    }
}
