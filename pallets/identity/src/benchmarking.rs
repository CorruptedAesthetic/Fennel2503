//! FRAME benchmarking for pallet-identity.

#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::{traits::ConstU32, BoundedVec};

use crate::{Call, Config, Pallet as Identity};
use super::mock_runtime::Test;
use crate as pallet_identity;

#[benchmarks(
    // Force T::MaxSize = 1024 so that Linear<1, 1024> parses
    where
        T: Config<MaxSize = ConstU32<1024>>,
)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_identity() -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        #[extrinsic_call] _(RawOrigin::Signed(who.clone()));
        let last = Identity::<T>::identity_number().saturating_sub(1);
        assert_eq!(Identity::<T>::identity_list(last), Some(who));
        Ok(())
    }

    #[benchmark]
    fn revoke_identity() -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        // pre-create
        Identity::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let last = Identity::<T>::identity_number().saturating_sub(1);

        #[extrinsic_call] _(RawOrigin::Signed(who.clone()), last);
        assert!(Identity::<T>::identity_list(last).is_none());
        Ok(())
    }

    #[benchmark]
    fn add_or_update_identity_trait(l: Linear<1, 1024>) -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Identity::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let last = Identity::<T>::identity_number().saturating_sub(1);

        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; l as usize].try_into().unwrap();
        let val: BoundedVec<u8, T::MaxSize> = vec![1u8; l as usize].try_into().unwrap();

        #[extrinsic_call] _(RawOrigin::Signed(who.clone()), last, key.clone(), val.clone());
        assert_eq!(Identity::<T>::identity_trait_list(last, key), val);
        Ok(())
    }

    #[benchmark]
    fn remove_identity_trait(l: Linear<1, 1024>) -> Result<(), BenchmarkError> {
        let who: T::AccountId = whitelisted_caller();
        Identity::<T>::create_identity(RawOrigin::Signed(who.clone()).into())?;
        let last = Identity::<T>::identity_number().saturating_sub(1);

        let key: BoundedVec<u8, T::MaxSize> = vec![0u8; l as usize].try_into().unwrap();
        let val: BoundedVec<u8, T::MaxSize> = vec![1u8; l as usize].try_into().unwrap();
        Identity::<T>::add_or_update_identity_trait(
            RawOrigin::Signed(who.clone()).into(),
            last,
            key.clone(),
            val,
        )?;

        #[extrinsic_call] _(RawOrigin::Signed(who.clone()), last, key.clone());
        assert_eq!(
            Identity::<T>::identity_trait_list(last, key),
            BoundedVec::<u8, T::MaxSize>::default()
        );
        Ok(())
    }

    impl_benchmark_test_suite!(
        Identity,
        crate::mock::new_test_ext(),
        Test
    );
}
