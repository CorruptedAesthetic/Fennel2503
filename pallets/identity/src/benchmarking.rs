#![cfg(feature = "runtime-benchmarks")]
use super::*;
use crate::Pallet as Identity;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::BoundedVec;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_identity() -> Result<(), BenchmarkError> {
        // Worst-case: signing and creating a new identity
        let caller: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()));

        // Verify that the identity was stored correctly
        let id = Identity::<T>::identity_number().saturating_sub(1);
        assert_eq!(Identity::<T>::identity_list(id), Some(caller.clone()));
        Ok(())
    }

    #[benchmark]
    fn revoke_identity() -> Result<(), BenchmarkError> {
        // Setup: create an identity to revoke
        let caller: T::AccountId = whitelisted_caller();
        Identity::<T>::create_identity(RawOrigin::Signed(caller.clone()))?;
        let id = Identity::<T>::identity_number().saturating_sub(1);

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), id);

        // Verify removal from storage
        assert!(Identity::<T>::identity_list(id).is_none());
        Ok(())
    }

    #[benchmark(extra)]
    fn revoke_identity_not_owned() -> Result<(), BenchmarkError> {
        // Attempt to revoke a non-existent identity (should error early)
        let caller: T::AccountId = whitelisted_caller();
        let bogus_id = 999u32;
        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), bogus_id);
        Ok(())
    }

    #[benchmark]
    fn add_or_update_identity_trait(l: Linear<1, T::MaxSize>) -> Result<(), BenchmarkError> {
        // Setup: create identity and prepare a key/value pair
        let caller: T::AccountId = whitelisted_caller();
        Identity::<T>::create_identity(RawOrigin::Signed(caller.clone()))?;
        let id = Identity::<T>::identity_number().saturating_sub(1);
        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; l as usize].try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize> = vec![1u8; l as usize].try_into().unwrap();

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), id, key.clone(), value.clone());

        // Verify storage update
        assert_eq!(Identity::<T>::identity_trait_list(id, key.clone()), value.clone());
        Ok(())
    }

    #[benchmark]
    fn remove_identity_trait(l: Linear<1, T::MaxSize>) -> Result<(), BenchmarkError> {
        // Setup: create identity, add a trait, then remove it
        let caller: T::AccountId = whitelisted_caller();
        Identity::<T>::create_identity(RawOrigin::Signed(caller.clone()))?;
        let id = Identity::<T>::identity_number().saturating_sub(1);
        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; l as usize].try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize> = vec![1u8; l as usize].try_into().unwrap();
        Identity::<T>::add_or_update_identity_trait(
            RawOrigin::Signed(caller.clone()),
            id,
            key.clone(),
            value.clone(),
        )?;

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), id, key.clone());

        // Verify removal
        assert_eq!(Identity::<T>::identity_trait_list(id, key.clone()), BoundedVec::<u8, T::MaxSize>::default());
        Ok(())
    }

    impl_benchmark_test_suite!(Identity, crate::mock::new_test_ext(), crate::mock::Test);
}
