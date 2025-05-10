use crate as pallet_identity;
use frame_support::{derive_impl, parameter_types};
use frame_system as system;
use sp_core::{ConstU32, H256};
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<runtime::Test>;

#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
      RuntimeCall, RuntimeEvent, RuntimeError,
      RuntimeOrigin, RuntimeFreezeReason,
      RuntimeHoldReason, RuntimeSlashReason,
      RuntimeLockId, RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;

    #[runtime::pallet_index(1)]
    pub type Identity = pallet_identity::Pallet<Test>;
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as system::DefaultConfig)]
impl system::Config for runtime::Test {
    type Block = Block;
    type Nonce = u32;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_identity::Config for runtime::Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxSize = ConstU32<1024>;
}

/// Build genesis storage for the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<runtime::Test>::default()
        .build_storage()
        .unwrap()
        .into()
}