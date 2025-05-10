//! Benchmarking setup for pallet-validator-manager

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use codec::Decode;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn register_validators(c: Linear<1, 10>) -> Result<(), BenchmarkError> {
        let validators: Vec<T::ValidatorId> = (0..c).map(|i| {
            T::ValidatorId::decode(&mut [i as u8; 32].as_ref()).expect("Failed to decode account id")
        }).collect();

        #[extrinsic_call]
        _(RawOrigin::Root, validators.clone());

        assert_eq!(ValidatorsToAdd::<T>::get().len(), c as usize);
        Ok(())
    }

    #[benchmark]
    fn remove_validator() -> Result<(), BenchmarkError> {
        let validator_id = T::ValidatorId::decode(&mut [1u8; 32].as_ref()).expect("Failed to decode account id");

        #[extrinsic_call]
        _(RawOrigin::Root, validator_id);

        assert_eq!(ValidatorsToRemove::<T>::get().len(), 1);
        Ok(())
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{new_test_ext, Test};
    use frame_support::assert_ok;

    // #[test]
    // fn test_benchmarks() {
    //     new_test_ext().execute_with(|| {
    //         assert_ok!(test_benchmark_register_validators::<Test>());
    //         assert_ok!(test_benchmark_remove_validator::<Test>());
    //     });
    // }
}