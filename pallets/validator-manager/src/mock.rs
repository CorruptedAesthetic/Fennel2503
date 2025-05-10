//! Test utilities for validator-manager pallet

use crate as pallet_validator_manager;
use crate::*;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64, BuildGenesisConfig, OneSessionHandler},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    impl_opaque_keys,
    testing::{Header, UintAuthorityId, TestXt},
    traits::{BlakeTwo256, IdentityLookup},
    AccountId32, BuildStorage, Perbill,
};
use pallet_session::{PeriodicSessions};
use sp_std::prelude::*;
use std::ops::{Deref, DerefMut};

// Define our own TestAccountId type which will be used as AccountId
// This allows us to implement required traits directly
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct TestAccountId(pub u64);

impl From<u64> for TestAccountId {
    fn from(id: u64) -> Self {
        TestAccountId(id)
    }
}

impl From<UintAuthorityId> for TestAccountId {
    fn from(id: UintAuthorityId) -> Self {
        TestAccountId(id.0)
    }
}

// Implement required traits for our TestAccountId to work as a system AccountId
impl core::fmt::Display for TestAccountId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl sp_std::str::FromStr for TestAccountId {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>().map(TestAccountId).map_err(|_| ())
    }
}

impl Default for TestAccountId {
    fn default() -> Self {
        TestAccountId(0)
    }
}

impl_opaque_keys! {
    pub struct MockSessionKeys {
        pub dummy: UintAuthorityId,
    }
}

type UncheckedExtrinsic = system::mocking::MockUncheckedExtrinsic<Test>;
type Block = system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test {
        System: system::{Pallet, Call, Storage, Event<T>},
        Session: pallet_session::{Pallet, Call, Storage, Event<T>},
        ValidatorManager: pallet_validator_manager::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MinAuthorities: u32 = 2;
    pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights::simple_max(frame_support::weights::Weight::from_parts(1024, 0));
}

// Helper functions for converting between types
pub fn account_id_to_authority_id(account: TestAccountId) -> UintAuthorityId {
    UintAuthorityId(account.0)
}

pub fn authority_id_to_account_id(authority: UintAuthorityId) -> TestAccountId {
    TestAccountId(authority.0)
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = TestAccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type RuntimeTask = ();
    type Nonce = u64;
    type Block = Block;
    type ExtensionsWeightInfo = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

// Note: UintAuthorityId already implements Clone and Copy 
// in the sp_runtime::testing module

// We'll use a different approach - test keys
parameter_types! {
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(33);
}

// TestSessionHandler with proper implementation
pub struct TestSessionHandler;
impl sp_runtime::BoundToRuntimeAppPublic for TestSessionHandler {
    type Public = UintAuthorityId;
}

impl OneSessionHandler<TestAccountId> for TestSessionHandler {
    type Key = UintAuthorityId;

    fn on_genesis_session<'a, I: 'a>(_validators: I)
    where
        I: Iterator<Item = (&'a TestAccountId, Self::Key)>,
    {
        // Not needed for our tests
    }

    fn on_new_session<'a, I: 'a>(_changed: bool, _validators: I, _queued_validators: I)
    where
        I: Iterator<Item = (&'a TestAccountId, Self::Key)>,
    {
        // Not needed for our tests
    }

    fn on_disabled(_validator_index: u32) {
        // Not needed for our tests
    }
}

// Validator ID conversion handler
pub struct ValidatorIdOf;
impl sp_runtime::traits::Convert<TestAccountId, Option<UintAuthorityId>> for ValidatorIdOf {
    fn convert(account: TestAccountId) -> Option<UintAuthorityId> {
        // Simply convert the TestAccountId's u64 value to a UintAuthorityId
        Some(UintAuthorityId(account.0))
    }
}

impl pallet_session::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = UintAuthorityId;
    type ValidatorIdOf = ValidatorIdOf;
    type ShouldEndSession = PeriodicSessions<ConstU64<1>, ConstU64<0>>;
    type NextSessionRotation = PeriodicSessions<ConstU64<1>, ConstU64<0>>;
    type SessionManager = ValidatorManager;
    type SessionHandler = (TestSessionHandler,);
    type Keys = MockSessionKeys;
    type WeightInfo = ();
    type DisablingStrategy = ();
}

impl pallet_validator_manager::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type PrivilegedOrigin = frame_system::EnsureRoot<TestAccountId>;
    type MinAuthorities = MinAuthorities;
    type ValidatorOf = ValidatorIdOf;
    type WeightInfo = ();
}

#[cfg(test)]
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = <system::GenesisConfig<Test> as BuildStorage>::build_storage(&system::GenesisConfig::default()).unwrap();
    pallet_validator_manager::GenesisConfig::<Test> {
        initial_validators: vec![UintAuthorityId(1), UintAuthorityId(2), UintAuthorityId(3)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);
    });
    ext
}