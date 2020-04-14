// Tests to be written here

use super::*;

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn minting_works() {
	new_test_ext().execute_with(|| {
		<Minters<Test>>::insert(&1, true);
		
		assert_ok!(Livecoin::mint(Origin::signed(1), 4, 42));
		
		assert_eq!(Livecoin::balance_of(4), 42);
	});
}

#[test]
fn can_add_minter() {
	new_test_ext().execute_with(|| {
		assert!(!Livecoin::is_minter(2));

		assert_ok!(Livecoin::add_minter(Origin::signed(1), 2));

		assert!(!Livecoin::is_minter(1));
		assert!(Livecoin::is_minter(2));
		assert!(!Livecoin::is_minter(3));
	});
}

#[test]
fn can_only_remove_existing_minter() {
	new_test_ext().execute_with(|| {
		assert!(!Livecoin::is_minter(2));

		assert_eq!(Livecoin::remove_minter(Origin::signed(1), 2), Err(DispatchError::from(Error::<Test>::NotMinter)));

		assert_ok!(Livecoin::add_minter(Origin::signed(1), 2));
		assert!(Livecoin::is_minter(2));

		assert_ok!(Livecoin::remove_minter(Origin::signed(1), 2));
	});
}

#[test]
fn burning_works() {
	new_test_ext().execute_with(|| {
		<Minters<Test>>::insert(&1, true);

		assert_ok!(Livecoin::mint(Origin::signed(1), 1, 42));
		assert_ok!(Livecoin::burn(Origin::signed(1), 21));

		assert_eq!(Livecoin::balance_of(1), 21);
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		<Minters<Test>>::insert(&1, true);

		assert_ok!(Livecoin::mint(Origin::signed(1), 1, 42));
		assert_ok!(Livecoin::transfer(Origin::signed(1), 2, 20));

		assert_eq!(Livecoin::balance_of(1), 22);
		assert_eq!(Livecoin::balance_of(2), 20);
	});
}