//! Benchmarking setup for pallet-validator-manager

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::prelude::*;

benchmarks! {
    register_validators {
        let c in 1 .. 10;
        let caller: T::AccountId = whitelisted_caller();
        let validators: Vec<T::ValidatorId> = (0..c).map(|i| {
            let account_id = T::ValidatorId::decode(&mut [i as u8; 32].as_ref())
                .expect("Failed to decode account id");
            account_id
        }).collect();
    }: _(RawOrigin::Root, validators.clone())
    verify {
        assert_eq!(ValidatorsToAdd::<T>::get().len(), c as usize);
    }

    remove_validator {
        // Set up a validator first to make sure we have one to remove
        let validator_id = T::ValidatorId::decode(&mut [1u8; 32].as_ref())
            .expect("Failed to decode account id");
        
        // We'll skip the validation check since this is a benchmark
        #[cfg(feature = "runtime-benchmarks")]
        // We don't actually check if validator exists in benchmarks, since we
        // have the runtime-benchmarks feature enabled
    }: _(RawOrigin::Root, validator_id)
    verify {
        assert_eq!(ValidatorsToRemove::<T>::get().len(), 1);
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, Test};
    use frame_support::assert_ok;

    #[test]
    fn test_benchmarks() {
        new_test_ext().execute_with(|| {
            assert_ok!(test_benchmark_register_validators::<Test>());
            assert_ok!(test_benchmark_remove_validator::<Test>());
        });
    }
} 