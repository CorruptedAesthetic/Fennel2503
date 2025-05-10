use crate::{mock::*, Error, IdentityNumber, Event};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn issue_identity() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(1)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 0, owner: 1 }.into());
	});
}

#[test]
fn issue_identity_increments_by_number_of_times_called() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(1)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 0, owner: 1 }.into());
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(2)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 1, owner: 2 }.into());
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(3)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 2, owner: 3 }.into());

		assert_eq!(Pallet::identity_number(), 3);
	});
}

#[test]
fn issue_identity_registers_different_account_ids_with_new_identities() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 0, owner: 300 }.into());
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(200)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 1, owner: 200 }.into());

		assert_eq!(Pallet::identity_list(0).unwrap(), 300);
		assert_eq!(Pallet::identity_list(1).unwrap(), 200);
	});
}

#[test]
fn issue_identity_registers_same_account_id_with_multiple_new_identities() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 0, owner: 300 }.into());
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 1, owner: 300 }.into());

		assert_eq!(Pallet::identity_list(0).unwrap(), 300);
		assert_eq!(Pallet::identity_list(1).unwrap(), 300);
	});
}

#[test]
fn revoke_identity() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 0, owner: 300 }.into());
		assert_ok!(Pallet::revoke_identity(RuntimeOrigin::signed(300), 0));
		System::assert_last_event(Event::IdentityRevoked { identity_id: 0, owner: 300 }.into());
		assert!(Pallet::identity_list(0).is_none());
	});
}

#[test]
fn revoke_identity_multiple_from_different_accounts() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 0, owner: 300 }.into());
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(200)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 1, owner: 200 }.into());

		assert_ok!(Pallet::revoke_identity(RuntimeOrigin::signed(300), 0));
		System::assert_last_event(Event::IdentityRevoked { identity_id: 0, owner: 300 }.into());
		assert!(Pallet::identity_list(0).is_none());
		assert_ok!(Pallet::revoke_identity(RuntimeOrigin::signed(200), 1));
		System::assert_last_event(Event::IdentityRevoked { identity_id: 1, owner: 200 }.into());
		assert!(Pallet::identity_list(1).is_none());
	});
}

#[test]
fn revoke_identity_multiple_from_same_account() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 0, owner: 300 }.into());
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 1, owner: 300 }.into());

		assert_ok!(Pallet::revoke_identity(RuntimeOrigin::signed(300), 1));
		System::assert_last_event(Event::IdentityRevoked { identity_id: 1, owner: 300 }.into());
		assert!(Pallet::identity_list(1).is_none());
		assert_ok!(Pallet::revoke_identity(RuntimeOrigin::signed(300), 0));
		System::assert_last_event(Event::IdentityRevoked { identity_id: 0, owner: 300 }.into());
		assert!(Pallet::identity_list(0).is_none());
		// Check that identity_number is not decremented after revocation
		assert_eq!(Pallet::identity_number(), 2);
	});
}

#[test]
fn revoke_identity_from_non_owning_account() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // Create identity 0 owned by 300
        assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
        // Account 200 tries to revoke it
        assert_noop!(
            Pallet::revoke_identity(RuntimeOrigin::signed(200), 0),
            Error::<Test>::IdentityNotOwned
        );
    });
}

#[test]
fn revoke_nonexistent_identity_fails() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            Pallet::revoke_identity(RuntimeOrigin::signed(1), 999),
            Error::<Test>::IdentityNotFound
        );
    });
}

#[test]
fn add_or_update_identity_trait() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let account_id = 300;
		type MaxSize = <Test as pallet_identity::Config>::MaxSize;
		let key = BoundedVec::<u8, MaxSize>::try_from("name".as_bytes().to_vec()).unwrap();

		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(account_id)));
		System::assert_last_event(
			Event::IdentityCreated { identity_id: 0, owner: account_id.try_into().unwrap() }.into(),
		);

		let luke = BoundedVec::<u8, MaxSize>::try_from("Luke Skywalker".as_bytes().to_vec())
			.unwrap();
		assert_ok!(Pallet::add_or_update_identity_trait(
			RuntimeOrigin::signed(account_id),
			0,
			key.clone(),
			luke.clone()
		));
		System::assert_last_event(
			Event::IdentityUpdated { identity_id: 0, owner: account_id.try_into().unwrap() }.into(),
		);
		assert_eq!(Pallet::identity_trait_list(0, key.clone()), luke.clone());

		let anakin =
			BoundedVec::<u8, MaxSize>::try_from("Anakin Skywalker".as_bytes().to_vec())
				.unwrap();
		assert_ok!(Pallet::add_or_update_identity_trait(
			RuntimeOrigin::signed(300),
			0,
			key.clone(),
			anakin.clone()
		));
		System::assert_last_event(
			Event::IdentityUpdated { identity_id: 0, owner: account_id.try_into().unwrap() }.into(),
		);
		assert_eq!(Pallet::identity_trait_list(0, key.clone()), anakin.clone());
	});
}

#[test]
fn add_or_update_identity_trait_non_owner_should_fail() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // Account 300 creates identity 0
        assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
        type MaxSize = <Test as pallet_identity::Config>::MaxSize;
        let key = BoundedVec::<u8, MaxSize>::try_from("name".as_bytes().to_vec()).unwrap();
        let value = BoundedVec::<u8, MaxSize>::try_from("Luke Skywalker".as_bytes().to_vec()).unwrap();
        // Account 200 tries to update trait on identity 0 (should fail)
        assert_noop!(
            Pallet::add_or_update_identity_trait(
                RuntimeOrigin::signed(200),
                0,
                key.clone(),
                value.clone()
            ),
            Error::<Test>::IdentityNotOwned
        );
    });
}

#[test]
fn remove_identity_trait() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
		System::assert_last_event(Event::IdentityCreated { identity_id: 0, owner: 300 }.into());
		type MaxSize = <Test as pallet_identity::Config>::MaxSize;
		let key = BoundedVec::<u8, MaxSize>::try_from("name".as_bytes().to_vec()).unwrap();
		let value = BoundedVec::<u8, MaxSize>::try_from("Luke Skywalker".as_bytes().to_vec()).unwrap();
		assert_ok!(Pallet::add_or_update_identity_trait(
			RuntimeOrigin::signed(300),
			0,
			key.clone(),
			value.clone()
		));
		System::assert_last_event(Event::IdentityUpdated { identity_id: 0, owner: 300 }.into());
		assert_ok!(Pallet::remove_identity_trait(
			RuntimeOrigin::signed(300),
			0,
			key.clone()
		));
		System::assert_last_event(Event::IdentityUpdated { identity_id: 0, owner: 300 }.into());
		// Verify the trait is removed from storage
		assert_eq!(Pallet::identity_trait_list(0, key.clone()), BoundedVec::default());
	});
}

#[test]
fn remove_identity_trait_non_owner_should_fail() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // Account 300 creates identity 0 and adds a trait
        assert_ok!(Pallet::create_identity(RuntimeOrigin::signed(300)));
        type MaxSize = <Test as pallet_identity::Config>::MaxSize;
        let key = BoundedVec::<u8, MaxSize>::try_from("name".as_bytes().to_vec()).unwrap();
        let value = BoundedVec::<u8, MaxSize>::try_from("Luke Skywalker".as_bytes().to_vec()).unwrap();
        assert_ok!(Pallet::add_or_update_identity_trait(
            RuntimeOrigin::signed(300),
            0,
            key.clone(),
            value.clone()
        ));
        // Account 200 tries to remove trait on identity 0 (should fail)
        assert_noop!(
            Pallet::remove_identity_trait(
                RuntimeOrigin::signed(200),
                0,
                key.clone()
            ),
            Error::<Test>::IdentityNotOwned
        );
    });
}

#[test]
fn defaults_are_zeroed() {
    new_test_ext().execute_with(|| {
        assert_eq!(Pallet::identity_number(), 0);
        assert!(Pallet::identity_list(0).is_none());
    });
}

#[test]
fn create_identity_storage_overflow() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        // Set identity_number to u32::MAX using the pallet's storage API
        IdentityNumber::<Test>::put(u32::MAX);
        // Attempt to create a new identity should fail with StorageOverflow
        assert_noop!(
            Pallet::create_identity(RuntimeOrigin::signed(1)),
            Error::<Test>::StorageOverflow
        );
    });
}

#[test]
fn add_trait_on_nonexistent_identity_should_fail() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let bogus_id = 999;
        type MaxSize = <Test as pallet_identity::Config>::MaxSize;
        let key   = BoundedVec::<u8, MaxSize>::try_from(b"foo".to_vec()).unwrap();
        let value = BoundedVec::<u8, MaxSize>::try_from(b"bar".to_vec()).unwrap();
        assert_noop!(
            Pallet::add_or_update_identity_trait(
                RuntimeOrigin::signed(1),
                bogus_id,
                key.clone(),
                value.clone()
            ),
            Error::<Test>::IdentityNotOwned
        );
    });
}

#[test]
fn remove_trait_on_nonexistent_identity_should_fail() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let bogus_id = 42;
        type MaxSize = <Test as pallet_identity::Config>::MaxSize;
        let key = BoundedVec::<u8, MaxSize>::try_from(b"foo".to_vec()).unwrap();
        assert_noop!(
            Pallet::remove_identity_trait(
                RuntimeOrigin::signed(1),
                bogus_id,
                key.clone()
            ),
            Error::<Test>::IdentityNotOwned
        );
    });
}
