#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

const SIGNAL_EXISTS: bool = true;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;

    use crate::{weights::WeightInfo, SIGNAL_EXISTS};

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
        type Currency: LockableCurrency<
            Self::AccountId,
            Moment = frame_system::pallet_prelude::BlockNumberFor<Self>,
        >;
        /// The identifier for the lock used to store signal deposits.
        type LockId: Get<LockIdentifier>;
        /// The price of a signal lock.
        type LockPrice: Get<u32>;
        /// The maximum size of a signal.
        type MaxSize: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn signal_list)]
    /// Maps accounts to the array of signals they own.
    pub type SignalList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        bool,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A signal was sent.
        SignalSent {
            /// The sender of the signal.
            sender: T::AccountId,
            /// The recipient of the signal.
            recipient: T::AccountId,
        },
        /// A signal was revoked.
        SignalRevoked {
            /// The sender of the signal.
            sender: T::AccountId,
            /// The recipient of the signal.
            recipient: T::AccountId,
        },
        /// A signal lock was set.
        SignalLock {
            /// The account whose balance was locked.
            account: <T as frame_system::Config>::AccountId,
            /// The locked balance.
            amount: BalanceOf<T>,
        },
        /// A signal lock was removed.
        SignalUnlock {
            /// The account whose balance was unlocked.
            account: <T as frame_system::Config>::AccountId,
            /// The unlocked balance.
            amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The current account does not own the signal.
        SignalNotOwned,
        /// The signal already exists.
        SignalExists,
        InsufficientBalance,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates an on-chain event with a Signal payload defined as part of the transaction
        /// and commits the details to storage.
        #[pallet::weight(T::WeightInfo::send_signal())]
        #[pallet::call_index(0)]
        pub fn send_signal(origin: OriginFor<T>, recipient: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::total_balance(&who) < T::Currency::minimum_balance() {
                return Err(Error::<T>::InsufficientBalance.into());
            }

            ensure!(
                !SignalList::<T>::contains_key(&who, &recipient),
                Error::<T>::SignalExists
            );
            // Insert a placeholder value into storage - if the pair (who, recipient) exists, we
            // know there's a signal present for the pair, regardless of value.
            T::Currency::set_lock(T::LockId::get(), &who, 10u32.into(), WithdrawReasons::all());

            Self::deposit_event(Event::SignalLock {
                account: who.clone(),
                amount: T::Currency::free_balance(&who),
            });

            <SignalList<T>>::try_mutate(
                &who,
                recipient.clone(),
                |signal| -> DispatchResult {
                    *signal = SIGNAL_EXISTS;
                    Ok(())
                },
            )?;

            Self::deposit_event(Event::SignalSent {
                sender: who.clone(),
                recipient: recipient.clone(),
            });

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::revoke_rating_signal())]
        #[pallet::call_index(1)]
        /// Revokes the signal with the given recipient, as long as the signal is owned by
        /// origin.
        pub fn revoke_signal(origin: OriginFor<T>, recipient: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::total_balance(&who) < T::Currency::minimum_balance() {
                return Err(Error::<T>::InsufficientBalance.into());
            }

            ensure!(
                SignalList::<T>::contains_key(&who, &recipient),
                Error::<T>::SignalNotOwned
            );

            T::Currency::remove_lock(T::LockId::get(), &who);
            Self::deposit_event(Event::SignalUnlock {
                account: who.clone(),
                amount: T::Currency::free_balance(&who),
            });

            <SignalList<T>>::try_mutate(
                &who,
                recipient.clone(),
                |signal| -> DispatchResult {
                    *signal = !SIGNAL_EXISTS;
                    Ok(())
                },
            )?;

            Self::deposit_event(Event::SignalRevoked {
                sender: who.clone(),
                recipient: recipient.clone(),
            });

            Ok(())
        }
    }
}
