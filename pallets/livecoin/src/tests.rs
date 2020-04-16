// Tests to be written here
use super::*;

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn can_add_minter() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		Owner::<Test>::put(owner);
		let minter = 2;

		assert_ok!(Livecoin::add_minter(Origin::signed(owner), minter));
		assert!(Livecoin::is_minter(minter));
	});
}

#[test]
fn can_remove_minter() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		Owner::<Test>::put(owner);
		let minter = 2;
		Minters::<Test>::insert(minter, true);

		assert_ok!(Livecoin::remove_minter(Origin::signed(owner), minter));
		assert!(!Livecoin::is_minter(minter));
	});
}

#[test]
fn only_owner_can_add_minter() {
	new_test_ext().execute_with(|| {
		let non_owner = 1;
		let minter = 2;
		assert_noop!(
			Livecoin::add_minter(Origin::signed(non_owner), minter),
			Error::<Test>::NotOwner
		);
	});
}
