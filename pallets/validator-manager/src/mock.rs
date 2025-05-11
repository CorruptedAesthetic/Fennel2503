//! Test utilities for validator-manager pallet

use crate as pallet_validator_manager;
use frame_support::derive_impl;
use sp_runtime::BuildStorage;

// Use u64 for AccountId and ValidatorId for simplicity in tests
pub type AccountId = u64;
pub type ValidatorId = u64;

#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Test;

    #[runtime::pallet_index(0)]
    pub type System = frame_system::Pallet<Test>;
    #[runtime::pallet_index(1)]
    pub type Session = pallet_session::Pallet<Test>;
    #[runtime::pallet_index(2)]
    pub type ValidatorManager = pallet_validator_manager::Pallet<Test>;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type AccountId = AccountId;
}

// Minimal Session config for testing
impl pallet_session::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = ValidatorId;
    type ValidatorIdOf = crate::ValidatorOf<Test>;
    type ShouldEndSession = frame_support::traits::Never;
    type NextSessionRotation = frame_support::traits::Never;
    type SessionManager = ValidatorManager;
    type SessionHandler = ();
    type Keys = ();
    type WeightInfo = ();
    type DisablingStrategy = ();
}

impl pallet_validator_manager::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type PrivilegedOrigin = frame_system::EnsureRoot<AccountId>;
    type MinAuthorities = frame_support::traits::ConstU32<2>;
    type ValidatorOf = crate::ValidatorOf<Test>;
    type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
    pallet_validator_manager::GenesisConfig::<Test> {
        initial_validators: vec![1, 2, 3],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}