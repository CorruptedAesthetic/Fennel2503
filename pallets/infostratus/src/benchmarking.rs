//! Benchmarking setup for pallet-infostratus
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_runtime::BoundedVec;
use frame_support::traits::Currency;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_submission_entry() {
        let caller: T::AccountId = whitelisted_caller();
        let resource = BoundedVec::<u8, T::MaxSize>::try_from(b"BENCHMARK_RESOURCE".to_vec()).unwrap();
        // Ensure caller has enough balance
        T::Currency::make_free_balance_be(&caller, 100u32.into());
        #[extrinsic_call]
        create_submission_entry(RawOrigin::Signed(caller.clone()), resource.clone());
        // Assert storage
        assert!(SubmissionsList::<T>::contains_key(&caller, &resource));
    }

    #[benchmark]
    fn request_submission_assignment() {
        let poster: T::AccountId = account("poster", 0, 0);
        let assignee: T::AccountId = whitelisted_caller();
        let resource = BoundedVec::<u8, T::MaxSize>::try_from(b"BENCHMARK_RESOURCE".to_vec()).unwrap();
        // Ensure both have enough balance
        T::Currency::make_free_balance_be(&poster, 100u32.into());
        T::Currency::make_free_balance_be(&assignee, 100u32.into());
        // Poster creates submission
        SubmissionsList::<T>::insert(&poster, &resource, false);
        #[extrinsic_call]
        request_submission_assignment(RawOrigin::Signed(assignee.clone()), poster.clone(), resource.clone());
        // Assert storage
        assert!(AssignmentsList::<T>::contains_key(&assignee, &resource));
        assert!(SubmissionsList::<T>::contains_key(&poster, &resource));
    }

    impl_benchmark_test_suite!(Infostratus, crate::mock::new_test_ext(), crate::mock::Test);
}
