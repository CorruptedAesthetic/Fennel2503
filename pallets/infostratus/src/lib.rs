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

const SUBMISSION_EXISTS: bool = true;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;

    use crate::{weights::WeightInfo, SUBMISSION_EXISTS};

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
        /// The identifier for the lock used to store infostratus deposits.
        type LockId: Get<LockIdentifier>;
        /// The price of a submission lock.
        type LockPrice: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn submissions_list)]
    /// Maps accounts to the array of submissions they own.
    pub type SubmissionsList<T: Config> = StorageDoubleMap<
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
        /// A submission was sent.
        SubmissionSent {
            /// The sender of the submission.
            sender: T::AccountId,
            /// The recipient of the submission.
            recipient: T::AccountId,
        },
        /// A submission was revoked.
        SubmissionRevoked {
            /// The sender of the submission.
            sender: T::AccountId,
            /// The recipient of the submission.
            recipient: T::AccountId,
        },
        /// A submission lock was set.
        SubmissionLock {
            /// The account whose balance was locked.
            account: <T as frame_system::Config>::AccountId,
            /// The locked balance.
            amount: BalanceOf<T>,
        },
        /// A submission lock was removed.
        SubmissionUnlock {
            /// The account whose balance was unlocked.
            account: <T as frame_system::Config>::AccountId,
            /// The unlocked balance.
            amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The current account does not own the submission.
        SubmissionNotOwned,
        /// The submission already exists.
        SubmissionExists,
        InsufficientBalance,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates an on-chain event with a Submission payload defined as part of the transaction
        /// and commits the details to storage.
        #[pallet::weight(T::WeightInfo::send_submission())]
        #[pallet::call_index(0)]
        pub fn send_submission(origin: OriginFor<T>, recipient: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::total_balance(&who) < T::Currency::minimum_balance() {
                return Err(Error::<T>::InsufficientBalance.into());
            }

            ensure!(
                !SubmissionsList::<T>::contains_key(&who, &recipient),
                Error::<T>::SubmissionExists
            );
            // Insert a placeholder value into storage - if the pair (who, recipient) exists, we
            // know there's a submission present for the pair, regardless of value.
            T::Currency::set_lock(T::LockId::get(), &who, 10u32.into(), WithdrawReasons::all());

            Self::deposit_event(Event::SubmissionLock {
                account: who.clone(),
                amount: T::Currency::free_balance(&who),
            });

            <SubmissionsList<T>>::try_mutate(
                &who,
                recipient.clone(),
                |submission| -> DispatchResult {
                    *submission = SUBMISSION_EXISTS;
                    Ok(())
                },
            )?;

            Self::deposit_event(Event::SubmissionSent {
                sender: who.clone(),
                recipient: recipient.clone(),
            });

            Ok(())
        }

        #[pallet::weight(T::WeightInfo::revoke_submission())]
        #[pallet::call_index(1)]
        /// Revokes the submission with the given recipient, as long as the submission is owned by
        /// origin.
        pub fn revoke_submission(origin: OriginFor<T>, recipient: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::total_balance(&who) < T::Currency::minimum_balance() {
                return Err(Error::<T>::InsufficientBalance.into());
            }

            ensure!(
                SubmissionsList::<T>::contains_key(&who, &recipient),
                Error::<T>::SubmissionNotOwned
            );

            T::Currency::remove_lock(T::LockId::get(), &who);
            Self::deposit_event(Event::SubmissionUnlock {
                account: who.clone(),
                amount: T::Currency::free_balance(&who),
            });

            <SubmissionsList<T>>::try_mutate(
                &who,
                recipient.clone(),
                |submission| -> DispatchResult {
                    *submission = !SUBMISSION_EXISTS;
                    Ok(())
                },
            )?;

            Self::deposit_event(Event::SubmissionRevoked {
                sender: who.clone(),
                recipient: recipient.clone(),
            });

            Ok(())
        }
    }
}
