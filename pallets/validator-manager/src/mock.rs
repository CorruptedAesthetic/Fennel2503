//! Test utilities for validator-manager pallet

use crate as pallet_validator_manager;
use crate::*;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64, GenesisBuild, OneSessionHandler},
};
// We only import sp-io in dev-dependencies
#[cfg(test)]
use sp_io;
use sp_core::{crypto::key_types::DUMMY, H256};
use sp_runtime::{
    impl_opaque_keys,
    testing::{Header, UintAuthorityId},
    traits::{BlakeTwo256, IdentityLookup, OpaqueKeys},
    KeyTypeId,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        ValidatorManager: pallet_validator_manager,
        Session: pallet_session,
    }
);

parameter_types! {
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(frame_support::weights::Weight::from_parts(1024, 0));
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl_opaque_keys! {
    pub struct MockSessionKeys {
        pub dummy: UintAuthorityId,
    }
}

impl From<UintAuthorityId> for MockSessionKeys {
    fn from(dummy: UintAuthorityId) -> Self {
        Self { dummy }
    }
}

parameter_types! {
    pub const MinAuthorities: u32 = 2;
    pub static ValidatorCount: u32 = 3;
}

pub struct TestSessionHandler;
impl OneSessionHandler<u64> for TestSessionHandler {
    type Key = UintAuthorityId;

    fn on_genesis_session<'a, I: 'a>(_: I)
    where
        I: Iterator<Item = (&'a u64, Self::Key)>,
    {
    }

    fn on_new_session<'a, I: 'a>(_: bool, _: I, _: I)
    where
        I: Iterator<Item = (&'a u64, Self::Key)>,
    {
    }

    fn on_disabled(_: u32) {}
}

impl pallet_session::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = u64;
    type ValidatorIdOf = crate::ValidatorOf<Test>;
    type ShouldEndSession = pallet_session::PeriodicSessions<ConstU64<1>, ConstU64<0>>;
    type NextSessionRotation = pallet_session::PeriodicSessions<ConstU64<1>, ConstU64<0>>;
    type SessionManager = ValidatorManager;
    type SessionHandler = (TestSessionHandler,);
    type Keys = MockSessionKeys;
    type WeightInfo = ();
    type DisablingStrategy = ();
}

pub struct PrivilegedAccount;
impl frame_support::traits::EnsureOrigin<RuntimeOrigin> for PrivilegedAccount {
    type Success = ();

    fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
        match o {
            RuntimeOrigin::signed(1) => Ok(()),
            _ => Err(o),
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_origin() -> RuntimeOrigin {
        RuntimeOrigin::signed(1)
    }
}

impl pallet_validator_manager::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type PrivilegedOrigin = PrivilegedAccount;
    type MinAuthorities = MinAuthorities;
    type ValidatorOf = pallet_session::historical::ValidatorOf<Self, u64>;
    type WeightInfo = ();
}

#[cfg(test)]
// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

    pallet_validator_manager::GenesisConfig::<Test> {
        initial_validators: vec![1, 2, 3],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);
    });
    ext
} 