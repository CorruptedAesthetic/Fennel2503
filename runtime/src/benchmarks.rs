// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <http://unlicense.org>

use crate::{
	runtime::{
		Balances, Runtime, System, ValidatorManager, Template, /* include the validator manager pallet */
	},
	AccountId,
};
use frame_benchmarking::v2::*;
use frame_support::traits::Currency;
use sp_keyring::Sr25519Keyring;

// Include any individual pallet benchmarks here.
use pallet_template::benchmarking::Pallet as TemplateBenchmarking;

use frame_system::RawOrigin;

fn treasury_account() -> AccountId {
	Sr25519Keyring::Bob.to_account_id()
}

fn alice_account() -> AccountId {
	Sr25519Keyring::Alice.to_account_id()
}

// Create benchmark implementations for the pallet
#[benchmarks]
mod benches {
	use super::*;

	// Add the custom benchmark items here
	// These benchmarks aren't used for anything in the codebase
	// other than testing the macros.
	// Ensure that calling the treasury account does not use a bad origin.
	#[benchmark]
	fn bench_bad_origin() {
		let caller = treasury_account();

		#[extrinsic_call]
		// we use Call::System here to just test the benchmarking macro
		// this extrinsic might error, but the benchmarking will still work
		_(RawOrigin::Signed(caller), frame_system::Call::remark_with_event { remark: vec![] });
	}

	// Ensure extrinsic works with a whitelisted caller
	#[benchmark]
	fn bench_whitelist_call() {
		let caller = alice_account();
		whitelist!(caller);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(caller.clone()),
			frame_system::Call::remark_with_event { remark: vec![] },
		);
		// verify caller had balance transfer.
	}

	// Implementation of all pallet benchmarks.
	impl_benchmark_test_suite!(
		// List all pallets and benchmarks that need to be tested here
		TemplateBenchmarking,
		crate::runtime::Template,
		// Add ValidatorManager benchmarking
		crate::runtime::ValidatorManager,
	);
}

frame_benchmarking::define_benchmarks!(
	[frame_benchmarking, BaselineBench::<Runtime>]
	[frame_system, SystemBench::<Runtime>]
	[frame_system_extensions, SystemExtensionsBench::<Runtime>]
	[pallet_balances, Balances]
	[pallet_timestamp, Timestamp]
	[pallet_sudo, Sudo]
	[pallet_template, Template]
);
