#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet as Keystore;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use scale_info::prelude::{format, vec};

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
    RawOrigin::Signed(get_account::<T>(name))
}

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    benchmark_account(name, 0, 0)
}

pub fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn announce_key() -> Result<(), BenchmarkError> {
        let origin = get_origin::<T>("Anakin");
        let key = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
            "fingerprint".as_bytes().to_vec(),
        )
        .unwrap();
        let location = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
            "location".as_bytes().to_vec(),
        )
        .unwrap();

        #[extrinsic_call]
        _(origin.clone(), key.clone(), location.clone());

        let origin_address = get_account::<T>("Anakin");
        assert_eq!(IssuedKeys::<T>::get(&origin_address, &key), Some(location.clone()));
        assert_last_event::<T>(Event::KeyAnnounced { key, who: origin_address }.into());
        Ok(())
    }

    #[benchmark]
    fn revoke_key() -> Result<(), BenchmarkError> {
        let origin = get_origin::<T>("Anakin");
        let key = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
            "somekey".as_bytes().to_vec(),
        )
        .unwrap();

        Keystore::<T>::announce_key(
            origin.clone().into(),
            key.clone(),
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![0; 32]).unwrap(),
        )?;

        #[extrinsic_call]
        _(origin.clone(), key.clone());

        let origin_address = get_account::<T>("Anakin");
        assert_eq!(IssuedKeys::<T>::get(&origin_address, &key), None);
        assert_last_event::<T>(Event::KeyRevoked { key, who: origin_address }.into());
        Ok(())
    }

    #[benchmark]
    fn issue_encryption_key() -> Result<(), BenchmarkError> {
        let origin = get_origin::<T>("Anakin");
        let key = [0; 32];

        #[extrinsic_call]
        _(origin.clone(), key.clone());

        let origin_address = get_account::<T>("Anakin");
        assert_eq!(IssuedEncryptionKeys::<T>::get(&origin_address), Some(key));
        assert_last_event::<T>(Event::EncryptionKeyIssued { who: origin_address }.into());
        Ok(())
    }

    impl_benchmark_test_suite!(Keystore, crate::mock::new_test_ext(), crate::mock::Test);
}
