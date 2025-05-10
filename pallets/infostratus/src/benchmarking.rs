//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Infostratus;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use scale_info::prelude::format;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    let account: T::AccountId = benchmark_account(name, 0, 0);
    account
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_submission_entry() -> Result<(), BenchmarkError> {
        let who = get_account::<T>("Alice");
        let data: BoundedVec<u8, T::MaxSize> = b"testdata".to_vec().try_into().unwrap();
        #[extrinsic_call]
        _(RawOrigin::Signed(who.clone()), data.clone());
        // Assert that the submission was created in storage
        assert!(Infostratus::<T>::submissions_list(&who, &data));
        Ok(())
    }

    #[benchmark]
    fn request_submission_assignment() -> Result<(), BenchmarkError> {
        let who = get_account::<T>("Alice");
        let poster = get_account::<T>("Bob");
        let data: BoundedVec<u8, T::MaxSize> = b"testdata".to_vec().try_into().unwrap();
        // Pre-insert a submission for the poster
        Infostratus::<T>::create_submission_entry(RawOrigin::Signed(poster.clone()).into(), data.clone()).ok();
        #[extrinsic_call]
        _(RawOrigin::Signed(who.clone()), poster.clone(), data.clone());
        // Assert that the assignment was processed in storage
        assert!(Infostratus::<T>::assignments_list(&who, &data));
        assert!(Infostratus::<T>::submissions_list(&poster, &data));
        Ok(())
    }

    impl_benchmark_test_suite!(Infostratus, crate::mock::new_test_ext(), crate::mock::Test);
}
