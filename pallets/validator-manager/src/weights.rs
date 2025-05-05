#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_validator_manager`.
pub trait WeightInfo {
    fn register_validators(v: u32) -> Weight;
    fn remove_validator() -> Weight;
}

/// Default weights for the validator manager pallet.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // The weights here are largely just placeholders and should be benchmarked
    fn register_validators(v: u32) -> Weight {
        Weight::from_parts(50_000_000, 0)
            .saturating_add(Weight::from_parts(5_000_000, 0).saturating_mul(v as u64))
    }
    
    fn remove_validator() -> Weight {
        Weight::from_parts(50_000_000, 0)
    }
}

// For tests
impl WeightInfo for () {
    fn register_validators(_v: u32) -> Weight {
        Weight::from_parts(10_000_000, 0)
    }
    
    fn remove_validator() -> Weight {
        Weight::from_parts(10_000_000, 0)
    }
} 