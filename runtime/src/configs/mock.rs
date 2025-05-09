// mock.rs - Mock runtime for testing

pub mod tests {
    use crate::*;
    use frame_support::runtime;
    use sp_runtime::BuildStorage;
    use sp_core::crypto::AccountId32;

    #[runtime]
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
        // Add other pallets here as needed for your tests
    }

    pub fn new_test_ext() -> sp_io::TestExternalities {
        // Define the initial balances for accounts
        let initial_balances: Vec<(AccountId32, u128)> = vec![
            (AccountId32::from([0u8; 32]), 1_000_000_000_000),
            (AccountId32::from([1u8; 32]), 2_000_000_000_000),
        ];

        let mut t = frame_system::GenesisConfig::<runtime::Test>::default()
            .build_storage()
            .unwrap();

        // Adding balances configuration to the genesis config
        pallet_balances::GenesisConfig::<runtime::Test> {
            balances: initial_balances,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}
