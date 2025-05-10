#![cfg(feature = "runtime-benchmarks")]
use super::*;
use crate::Pallet as Identity;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use scale_info::prelude::format;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
	RawOrigin::Signed(get_account::<T>(name))
}

pub fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_identity() -> Result<(), BenchmarkError> {
        let who = get_account::<T>("Anakin");
        #[extrinsic_call]
        _(RawOrigin::Signed(who.clone()));
        // Use correct storage accessor for identity existence
        let id = Identity::<T>::identity_number() - 1;
        assert_eq!(Identity::<T>::identity_list(id), Some(who.clone()));
        Ok(())
    }

    #[benchmark]
    fn revoke_identity() -> Result<(), BenchmarkError> {
        let who = get_account::<T>("Anakin");
        Identity::<T>::create_identity(RawOrigin::Signed(who.clone()).into()).ok();
        let id = Identity::<T>::identity_number() - 1;
        #[extrinsic_call]
        _(RawOrigin::Signed(who.clone()), id);
        assert_eq!(Identity::<T>::identity_list(id), None);
        Ok(())
    }

    #[benchmark]
    fn add_or_update_identity_trait() -> Result<(), BenchmarkError> {
        let who = get_account::<T>("Anakin");
        Identity::<T>::create_identity(RawOrigin::Signed(who.clone()).into()).ok();
        let id = Identity::<T>::identity_number() - 1;
        let name: BoundedVec<u8, T::MaxSize> = "name".as_bytes().to_vec().try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize> = "Skywalker".as_bytes().to_vec().try_into().unwrap();
        #[extrinsic_call]
        _(RawOrigin::Signed(who.clone()), id, name.clone(), value.clone());
        assert_eq!(Identity::<T>::identity_trait_list(id, name.clone()), value);
        Ok(())
    }

    #[benchmark]
    fn remove_identity_trait() -> Result<(), BenchmarkError> {
        let who = get_account::<T>("Anakin");
        Identity::<T>::create_identity(RawOrigin::Signed(who.clone()).into()).ok();
        let id = Identity::<T>::identity_number() - 1;
        let name: BoundedVec<u8, T::MaxSize> = "name".as_bytes().to_vec().try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize> = "Skywalker".as_bytes().to_vec().try_into().unwrap();
        Identity::<T>::add_or_update_identity_trait(RawOrigin::Signed(who.clone()).into(), id, name.clone(), value.clone()).ok();
        #[extrinsic_call]
        _(RawOrigin::Signed(who.clone()), id, name.clone());
        let empty: BoundedVec<u8, T::MaxSize> = BoundedVec::default();
        assert_eq!(Identity::<T>::identity_trait_list(id, name.clone()), empty);
        Ok(())
    }

    impl_benchmark_test_suite!(Identity, crate::mock::new_test_ext(), crate::mock::Test);
}
