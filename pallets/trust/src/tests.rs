use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

#[test]
fn test_set_trust_parameter() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::set_trust_parameter(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0
        ));
        System::assert_last_event(
            crate::Event::TrustParameterSet { who: 1 }.into()
        );
    });
}

#[test]
fn test_issue_trust() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(
            crate::Event::TrustIssued { issuer: 1, target: 1 }.into()
        );
        assert_eq!(TrustModule::get_current_trust_count(), 1);
    });
}

#[test]
fn test_issue_trust_error() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(
            crate::Event::TrustIssued { issuer: 1, target: 1 }.into()
        );
        assert_eq!(TrustModule::get_current_trust_count(), 1);
        assert_noop!(
            TrustModule::issue_trust(RuntimeOrigin::signed(1), 1),
            Error::<Test>::TrustExists
        );
    });
}

#[test]
fn test_request_and_cancel_trust() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::request_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(
            crate::Event::TrustRequest { requester: 1, target: 1 }.into()
        );
        assert_eq!(TrustModule::get_current_trust_requests(), 1);
        assert_ok!(TrustModule::cancel_trust_request(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(
            crate::Event::TrustRequestRemoved { requester: 1, target: 1 }.into()
        );
        assert_eq!(TrustModule::get_current_trust_requests(), 0);
    });
}

#[test]
fn test_remove_trust() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
        assert_ok!(TrustModule::remove_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(
            crate::Event::TrustIssuanceRemoved { issuer: 1, target: 1 }.into()
        );
        assert_eq!(TrustModule::get_current_trust_count(), 0);
    });
}

#[test]
fn test_revoke_and_remove_revoked_trust() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(
            crate::Event::TrustRevoked { issuer: 1, target: 1 }.into()
        );
        assert_ok!(TrustModule::remove_revoked_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(
            crate::Event::TrustRevocationRemoved { issuer: 1, target: 1 }.into()
        );
    });
}

#[test]
fn test_errors() {
    new_test_ext().execute_with(|| {
        // cancel non‑existent request
        assert_noop!(
            TrustModule::cancel_trust_request(RuntimeOrigin::signed(1), 1),
            Error::<Test>::TrustRequestNotFound
        );
        // remove non‑existent trust
        assert_noop!(
            TrustModule::remove_trust(RuntimeOrigin::signed(1), 1),
            Error::<Test>::TrustNotFound
        );
        // remove non‑existent revocation
        assert_noop!(
            TrustModule::remove_revoked_trust(RuntimeOrigin::signed(1), 1),
            Error::<Test>::TrustRevocationNotFound
        );
    });
}
