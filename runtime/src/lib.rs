#![cfg_attr(not(feature = "std"), no_std)]


#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


pub use frame_support::{construct_runtime, parameter_types};
use frame_support::traits::Everything;
use frame_system as system;
use sp_core::OpaqueMetadata;
use sp_runtime::{traits::{BlakeTwo256, IdentifyAccount, Verify, AccountIdLookup}, generic, MultiSignature};
use sp_version::RuntimeVersion;

pub type BlockNumber = u32;
pub type Signature = MultiSignature;
pub type AccountId = <Signature as Verify>::Signer::AccountId;
pub type Balance = u128;
pub type Index = u32;
pub type Hash = sp_core::H256;

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
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
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
    type EquivocationReportSystem = (); // No equivocation handling in dev
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
    type WeightToFee = (); // customize later
    type LengthToFee = (); // customize later
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

pub type SignedExtra = (
    system::CheckNonZeroSender<Runtime>,
    system::CheckSpecVersion<Runtime>,
    system::CheckTxVersion<Runtime>,
    system::CheckGenesis<Runtime>,
    system::CheckEra<Runtime>,
    system::CheckNonce<Runtime>,
    system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<AccountId, RuntimeCall, Signature, SignedExtra>;
pub type Block = generic::Block<generic::Header<BlockNumber, BlakeTwo256>, UncheckedExtrinsic>;

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Aura: pallet_aura::{Pallet, Storage, Config<T>},
        Grandpa: pallet-grandpa::{Pallet, Call, Storage, Config, Event},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage},
        Sudo: pallet_sudo::{Pallet, Call, Storage, Event<T>},
        ZkGhost: pallet_zkghost::{Pallet, Call, Storage, Event<T>},
    }
);

// Metadata helpers
pub fn version() -> RuntimeVersion { Version::get() }
pub fn metadata() -> OpaqueMetadata { Runtime::metadata().into() }

// Benchmarking: expose pallet benchmarks when runtime-benchmarks is enabled
#[cfg(feature = "runtime-benchmarks")]
pub mod benches {
    use super::*;
    use frame_benchmarking::Benchmarking;
}


use frame_executive::Executive;

pub type Executive = Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

#[cfg(feature = "std")]
use sp_api::impl_runtime_apis;

#[cfg(feature = "std")]
impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> sp_version::RuntimeVersion { Version::get() }
        fn execute_block(block: Block) { Executive::execute_block(block) }
        fn initialize_block(header: &<Block as sp_runtime::traits::Block>::Header) { Executive::initialize_block(header) }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> sp_core::OpaqueMetadata { Runtime::metadata().into() }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as sp_runtime::traits::Block>::Extrinsic) -> sp_runtime::ApplyExtrinsicResult { Executive::apply_extrinsic(extrinsic) }
        fn finalize_block() -> <Block as sp_runtime::traits::Block>::Header { Executive::finalize_block() }
        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as sp_runtime::traits::Block>::Extrinsic> { Executive::inherent_extrinsics(data) }
        fn check_inherents(block: Block, data: sp_inherents::InherentData) -> sp_inherents::CheckInherentsResult { Executive::check_inherents(block, data) }
        fn random_seed() -> <Block as sp_runtime::traits::Block>::Hash { System::random_seed() }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(source: sp_runtime::transaction_validity::TransactionSource, tx: <Block as sp_runtime::traits::Block>::Extrinsic, block_hash: <Block as sp_runtime::traits::Block>::Hash) -> sp_runtime::transaction_validity::TransactionValidity { Executive::validate_transaction(source, tx, block_hash) }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> sp_consensus_aura::SlotDuration { sp_consensus_aura::SlotDuration::from_millis((MinimumPeriod::get() as u64) * 2) }
        fn authorities() -> Vec<AuraId> { Aura::authorities() }
    }

    impl sp_finality_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> sp_finality_grandpa::AuthorityList { Grandpa::grandpa_authorities() }
        fn current_set_id() -> sp_finality_grandpa::SetId { Grandpa::current_set_id() }
        fn submit_report_equivocation_unsigned_extrinsic(_equivocation_proof: sp_finality_grandpa::EquivocationProof<<Block as sp_runtime::traits::Block>::Hash, sp_finality_grandpa::AuthorityId>, _key_owner_proof: sp_finality_grandpa::OpaqueKeyOwnershipProof) -> Option<()> { None }
        fn generate_key_ownership_proof(_set_id: sp_finality_grandpa::SetId, _authority_id: sp_finality_grandpa::AuthorityId) -> Option<sp_finality_grandpa::OpaqueKeyOwnershipProof> { None }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(uxt: <Block as sp_runtime::traits::Block>::Extrinsic, len: u32) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(uxt: <Block as sp_runtime::traits::Block>::Extrinsic, len: u32) -> pallet_transaction_payment_rpc_runtime_api::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
    }

}
