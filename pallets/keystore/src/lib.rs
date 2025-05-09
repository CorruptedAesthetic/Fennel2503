#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub use pallet::*;
use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::One;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
        // Please add one line of comment here about this config
        type MaxSize: Get<u32>;
    }

    #[pallet::type_value]
    pub fn DefaultCurrent<T: Config>() -> u32 {
        0
    }

    #[pallet::storage]
    #[pallet::getter(fn identity_number)]
    /// Tracks the number of identities currently active on the network.
    pub type IdentityNumber<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::getter(fn get_signal_count)]
    /// Tracks the number of signals transmitted to the network.
    pub type SignalCount<T: Config> =
        StorageValue<Value = u32, QueryKind = ValueQuery, OnEmpty = DefaultCurrent<T>>;

    #[pallet::storage]
    #[pallet::getter(fn identity_list)]
    /// Maps accounts to the array of identities it owns.
    pub type IdentityList<T: Config> = StorageMap<_, Blake2_128Concat, u32, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn identity_trait_list)]
    /// Maps identity ID numbers to their key/value attributes.
    pub type IdentityTraitList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        BoundedVec<u8, T::MaxSize>,
        ValueQuery,
    >;

    #[pallet::storage]
    /// This module's main storage will consist of a StorageDoubleMap connecting addresses to the
    /// list of keys they've submitted and not revoked.
    #[pallet::getter(fn key)]
    pub type IssuedKeys<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        BoundedVec<u8, T::MaxSize>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn encryption_key)]
    /// Maps an account to an encryption key that they've issued.
    pub type IssuedEncryptionKeys<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, [u8; 32]>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        IdentityCreated {
            identity_id: u32,
            owner: T::AccountId,
        },
        IdentityRevoked {
            identity_id: u32,
            owner: T::AccountId,
        },
        IdentityUpdated {
            identity_id: u32,
            owner: T::AccountId,
        },
        EncryptionKeyIssued { who: T::AccountId },
        KeyRevoked { key: BoundedVec<u8, T::MaxSize>, who: T::AccountId },
        KeyAnnounced { key: BoundedVec<u8, T::MaxSize>, who: T::AccountId },
    }

    #[pallet::error]
    #[derive(PartialEq, Eq)]
    pub enum Error<T> {
        /// The provided value is too large.
        StorageOverflow,
        /// The current account does not own the identity.
        IdentityNotOwned,
        /// The specified key already exists.
        KeyExists,
        /// The specified key does not exist.
        KeyDoesNotExist,
    }

    impl<T: Config> Pallet<T> {
        fn is_identity_owned_by_sender(account_id: &T::AccountId, identity_id: &u32) -> bool {
            match <IdentityList<T>>::try_get(identity_id) {
                Result::Ok(owner) => owner == *account_id,
                Result::Err(_) => false,
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::create_identity())]
        #[pallet::call_index(0)]
        pub fn create_identity(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_id: u32 = <IdentityNumber<T>>::get();
            <IdentityNumber<T>>::try_mutate(|current_id| -> DispatchResult {
                *current_id = current_id.checked_add(One::one()).ok_or(Error::<T>::StorageOverflow)?;
                Ok(())
            })?;
            let new_id: u32 = <IdentityNumber<T>>::get();
            ensure!(!<IdentityList<T>>::contains_key(&current_id), Error::<T>::StorageOverflow);
            <IdentityList<T>>::try_mutate(&current_id, |owner| -> DispatchResult {
                *owner = Some(who.clone());
                Ok(())
            })?;
            <IdentityNumber<T>>::put(new_id);
            Self::deposit_event(Event::IdentityCreated { identity_id: current_id, owner: who.clone() });
            Ok(().into())
        }
        #[pallet::weight(T::WeightInfo::revoke_identity())]
        #[pallet::call_index(1)]
        pub fn revoke_identity(origin: OriginFor<T>, identity_id: u32) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_identity_owned_by_sender(&who, &identity_id), Error::<T>::IdentityNotOwned);
            <IdentityList<T>>::try_mutate(&identity_id, |owner| -> DispatchResult {
                *owner = None;
                Ok(())
            })?;
            Self::deposit_event(Event::IdentityRevoked { identity_id, owner: who.clone() });
            Ok(().into())
        }
        #[pallet::weight(T::WeightInfo::add_or_update_identity_trait())]
        #[pallet::call_index(2)]
        pub fn add_or_update_identity_trait(
            origin: OriginFor<T>,
            identity_id: u32,
            key: BoundedVec<u8, T::MaxSize>,
            value: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_identity_owned_by_sender(&who, &identity_id), Error::<T>::IdentityNotOwned);
            <IdentityTraitList<T>>::try_mutate(identity_id, key, |v| -> DispatchResult {
                *v = value;
                Ok(())
            })?;
            Self::deposit_event(Event::IdentityUpdated { identity_id, owner: who.clone() });
            Ok(().into())
        }
        #[pallet::weight(T::WeightInfo::remove_identity_trait())]
        #[pallet::call_index(3)]
        pub fn remove_identity_trait(
            origin: OriginFor<T>,
            identity_id: u32,
            key: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_identity_owned_by_sender(&who, &identity_id), Error::<T>::IdentityNotOwned);
            <IdentityTraitList<T>>::remove(identity_id, key);
            Self::deposit_event(Event::IdentityUpdated { identity_id, owner: who.clone() });
            Ok(().into())
        }
        #[pallet::weight(T::WeightInfo::announce_key())]
        #[pallet::call_index(4)]
        pub fn announce_key(
            origin: OriginFor<T>,
            fingerprint: BoundedVec<u8, T::MaxSize>,
            location: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(!<IssuedKeys<T>>::contains_key(&who, &fingerprint), Error::<T>::KeyExists);
            <IssuedKeys<T>>::insert(&who, &fingerprint, &location);
            Self::deposit_event(Event::KeyAnnounced { key: fingerprint, who });
            Ok(().into())
        }
        #[pallet::weight(T::WeightInfo::revoke_key())]
        #[pallet::call_index(5)]
        pub fn revoke_key(
            origin: OriginFor<T>,
            key_index: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            ensure!(<IssuedKeys<T>>::contains_key(&who, &key_index), Error::<T>::KeyDoesNotExist);
            <IssuedKeys<T>>::remove(&who, &key_index);
            Self::deposit_event(Event::KeyRevoked { key: key_index, who });
            Ok(().into())
        }
        #[pallet::weight(T::WeightInfo::issue_encryption_key())]
        #[pallet::call_index(6)]
        pub fn issue_encryption_key(origin: OriginFor<T>, key: [u8; 32]) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            <IssuedEncryptionKeys<T>>::insert(&who, key);
            Self::deposit_event(Event::EncryptionKeyIssued { who });
            Ok(().into())
        }
    }
}
