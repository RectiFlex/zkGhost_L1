#![cfg_attr(not(feature = "std"), no_std)]

pub use frame_support::{construct_runtime, parameter_types};
use frame_support::traits::Everything;
use frame_system as system;
use sp_core::OpaqueMetadata;
use sp_runtime::{traits::{BlakeTwo256, Block as BlockT, IdentifyAccount, Verify, AccountIdLookup}, generic, MultiSignature};
use sp_version::RuntimeVersion;

pub type BlockNumber = u32;
pub type Signature = MultiSignature;
pub type AccountId = <Signature as Verify>::Signer::AccountId;
pub type Balance = u128;
pub type Index = u32;
pub type Hash = sp_core::H256;

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
}

pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<AccountId, RuntimeCall, Signature, SignedExtra>;
pub type Block = generic::Block<generic::Header<BlockNumber, BlakeTwo256>, UncheckedExtrinsic>;

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

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage},
        Sudo: pallet_sudo::{Pallet, Call, Storage, Event<T>},
        ZkGhost: pallet_zkghost::{Pallet, Call, Storage, Event<T>},
    }
);

// Metadata
pub fn version() -> RuntimeVersion { Version::get() }
pub fn metadata() -> OpaqueMetadata { Runtime::metadata().into() }
