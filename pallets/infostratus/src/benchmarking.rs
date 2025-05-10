//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Infostratus;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::{sp_runtime::traits::Bounded, traits::Currency, BoundedVec};
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
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[benchmark]
	fn send_submission() -> Result<(), BenchmarkError> {
		let caller = get_origin::<T>("Spock");
		let caller_account: T::AccountId = get_account::<T>("Spock");
		let resource_location = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec()).unwrap();
		T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());

		#[extrinsic_call]
		_(caller, resource_location.clone());

		assert!(SubmissionsList::<T>::contains_key(caller_account.clone(), resource_location.clone()));
		assert_last_event::<T>(Event::SubmissionSent { who: caller_account, resource_location }.into());
		Ok(())
	}

	#[benchmark]
	fn revoke_submission() -> Result<(), BenchmarkError> {
		let caller = get_origin::<T>("Spock");
		let caller_account: T::AccountId = get_account::<T>("Spock");
		let resource_location = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec()).unwrap();
		T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());
		Infostratus::<T>::create_submission_entry(caller.clone().into(), resource_location.clone())?;

		#[extrinsic_call]
		_(caller, caller_account.clone(), resource_location.clone());

		assert!(!SubmissionsList::<T>::contains_key(caller_account.clone(), resource_location.clone()));
		assert_last_event::<T>(Event::SubmissionAssigned { resource_location, who: caller_account }.into());
		Ok(())
	}

	#[benchmark]
	fn create_submission_entry() -> Result<(), BenchmarkError> {
		let caller = get_origin::<T>("Spock");
		let caller_account: T::AccountId = get_account::<T>("Spock");
		let target =
			BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
				.unwrap();

		T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());

		#[extrinsic_call]
		_(caller, target.clone());

		let caller_account_id: T::AccountId = get_account::<T>("Spock");
		assert!(SubmissionsList::<T>::contains_key(caller_account_id, target.clone()));

		Ok(())
	}

	#[benchmark]
	fn create_submission_entry_heavy_storage(m: Linear<0, 100_000>) -> Result<(), BenchmarkError> {
		let caller = get_origin::<T>("Spock");
		let caller_account: T::AccountId = get_account::<T>("Spock");
		let target =
			BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
				.unwrap();

		T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());

		for i in 0..m {
			let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
				format!("TEST{}", i).as_bytes().to_vec(),
			)
			.unwrap();
			Infostratus::<T>::create_submission_entry(caller.clone().into(), loop_target)?;
		}

		#[extrinsic_call]
		_(caller, target.clone());

		let caller_account_id: T::AccountId = get_account::<T>("Spock");
		assert!(SubmissionsList::<T>::contains_key(caller_account_id, target.clone()));

		Ok(())
	}

	#[benchmark]
	fn request_submission_assignment() -> Result<(), BenchmarkError> {
		let caller = get_origin::<T>("Leonard");
		let caller_account: T::AccountId = get_account::<T>("Leonard");
		let second_caller = get_origin::<T>("Montgomery");
		let second_caller_account: T::AccountId = get_account::<T>("Montgomery");
		let target =
			BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
				.unwrap();

		T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());
		T::Currency::make_free_balance_be(&second_caller_account, BalanceOf::<T>::max_value());

		Infostratus::<T>::create_submission_entry(caller.clone().into(), target.clone())?;

		#[extrinsic_call]
		_(second_caller, caller_account, target.clone());

		let caller_account_id: T::AccountId = get_account::<T>("Leonard");
		let second_caller_account_id: T::AccountId = get_account::<T>("Montgomery");
		assert!(SubmissionsList::<T>::contains_key(caller_account_id.clone(), target.clone()));
		assert!(AssignmentsList::<T>::contains_key(
			second_caller_account_id.clone(),
			target.clone()
		));

		Ok(())
	}

	#[benchmark]
	fn request_submission_assignment_heavy_storage(m: Linear<0, 100_000>) -> Result<(), BenchmarkError> {
		let caller = get_origin::<T>("Leonard");
		let caller_account: T::AccountId = get_account::<T>("Leonard");
		let second_caller = get_origin::<T>("Spock");
		let second_caller_account: T::AccountId = get_account::<T>("Spock");
		let target =
			BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from("TEST".as_bytes().to_vec())
				.unwrap();

		T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());
		T::Currency::make_free_balance_be(&second_caller_account, BalanceOf::<T>::max_value());

		Infostratus::<T>::create_submission_entry(caller.clone().into(), target.clone())?;

		for i in 0..m {
			let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
				format!("TEST{}", i).as_bytes().to_vec(),
			)
			.unwrap();
			Infostratus::<T>::create_submission_entry(caller.clone().into(), loop_target)?;
		}

		for i in 0..m {
			let loop_target = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
				format!("TEST{}", i).as_bytes().to_vec(),
			)
			.unwrap();
			Infostratus::<T>::request_submission_assignment(
				get_origin::<T>("Spock").clone().into(),
				caller_account.clone(),
				loop_target,
			)?;
		}

		#[extrinsic_call]
		_(second_caller, caller_account, target.clone());

		let caller_account_id: T::AccountId = get_account::<T>("Leonard");
		let second_caller_account_id: T::AccountId = get_account::<T>("Spock");
		assert!(SubmissionsList::<T>::contains_key(caller_account_id.clone(), target.clone()));
		assert!(AssignmentsList::<T>::contains_key(
			second_caller_account_id.clone(),
			target.clone()
		));

		Ok(())
	}

	impl_benchmark_test_suite!(Infostratus, crate::mock::new_test_ext(), crate::mock::Test);
}
