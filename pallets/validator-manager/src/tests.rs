//! Tests for the validator-manager pallet

use crate::{mock::*, Config, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::testing::UintAuthorityId;

fn validator_keys(c: &[u64]) -> Vec<UintAuthorityId> {
    c.iter().map(|x| UintAuthorityId(*x)).collect()
}

fn initialize_validators() {
    let keys = validator_keys(&[1, 2, 3]);
    for (i, key) in keys.into_iter().enumerate() {
        let authority_id = (i + 1) as u64;
        Session::set_keys(
            RuntimeOrigin::signed(authority_id),
            MockSessionKeys { dummy: key }.into(),
            Vec::new(),
        )
        .unwrap();
    }
}

#[test]
fn initial_validators_should_be_set() {
    new_test_ext().execute_with(|| {
        initialize_validators();
        
        // Start at session 1 and advance to session 2 to apply initial validators
        Session::on_initialize(1);
        
        assert_eq!(Session::validators(), vec![1, 2, 3]);
    });
}

#[test]
fn add_validators_should_work() {
    new_test_ext().execute_with(|| {
        initialize_validators();
        
        // Start at session 1 
        Session::on_initialize(1);
        assert_eq!(Session::validators(), vec![1, 2, 3]);
        
        // Register a new validator
        assert_ok!(ValidatorManager::register_validators(RuntimeOrigin::signed(1), vec![4]));
        
        // Check that the validator is in the queue
        assert_eq!(ValidatorManager::validators_to_add(), vec![4]);
        
        // Trigger a new session
        Session::on_initialize(2);
        
        // Validators should now include the new one
        assert_eq!(Session::validators(), vec![1, 2, 3, 4]);
        
        // Check the event was emitted
        System::assert_has_event(
            Event::ValidatorsRegistered { validators: vec![4] }.into(),
        );
    });
}

#[test]
fn cannot_add_duplicate_validator() {
    new_test_ext().execute_with(|| {
        initialize_validators();
        
        // Add validator 4 to the pending queue
        assert_ok!(ValidatorManager::register_validators(RuntimeOrigin::signed(1), vec![4]));
        
        // Attempt to add it again should fail
        assert_noop!(
            ValidatorManager::register_validators(RuntimeOrigin::signed(1), vec![4]),
            Error::<Test>::ValidatorAlreadyAdded
        );
    });
}

#[test]
fn remove_validator_should_work() {
    new_test_ext().execute_with(|| {
        initialize_validators();
        
        // Start at session 1 
        Session::on_initialize(1);
        assert_eq!(Session::validators(), vec![1, 2, 3]);
        
        // Remove validator 2
        assert_ok!(ValidatorManager::remove_validator(RuntimeOrigin::signed(1), 2));
        
        // Check that the validator is in the removal queue
        assert_eq!(ValidatorManager::validators_to_remove(), vec![2]);
        
        // Trigger a new session
        Session::on_initialize(2);
        
        // Validators should no longer include the removed one
        assert_eq!(Session::validators(), vec![1, 3]);
        
        // Check the event was emitted
        System::assert_has_event(
            Event::ValidatorRemoved { validator: 2 }.into(),
        );
    });
}

#[test]
fn cannot_remove_nonexistent_validator() {
    new_test_ext().execute_with(|| {
        initialize_validators();
        
        // Start at session 1 
        Session::on_initialize(1);
        
        // Attempt to remove a non-existent validator
        assert_noop!(
            ValidatorManager::remove_validator(RuntimeOrigin::signed(1), 99),
            Error::<Test>::NotValidator
        );
    });
}

#[test]
fn cannot_remove_below_min_validators() {
    new_test_ext().execute_with(|| {
        initialize_validators();
        
        // Start at session 1 
        Session::on_initialize(1);
        assert_eq!(Session::validators(), vec![1, 2, 3]);
        
        // Remove validator 2
        assert_ok!(ValidatorManager::remove_validator(RuntimeOrigin::signed(1), 2));
        
        // Remove validator 3
        assert_noop!(
            ValidatorManager::remove_validator(RuntimeOrigin::signed(1), 3),
            Error::<Test>::TooFewValidators
        );
    });
}

#[test]
fn unauthorized_origin_cannot_add_validators() {
    new_test_ext().execute_with(|| {
        // Use an unauthorized account (not 1)
        assert!(ValidatorManager::register_validators(RuntimeOrigin::signed(2), vec![4]).is_err());
    });
}

#[test]
fn unauthorized_origin_cannot_remove_validators() {
    new_test_ext().execute_with(|| {
        initialize_validators();
        
        // Start at session 1 
        Session::on_initialize(1);
        
        // Use an unauthorized account (not 1)
        assert!(ValidatorManager::remove_validator(RuntimeOrigin::signed(2), 3).is_err());
    });
} 