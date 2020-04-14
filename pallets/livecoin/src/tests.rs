// Tests to be written here

use super::*;

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn minting_works() {
	new_test_ext().execute_with(|| {
		let minter = 2;
		let recipient = 4;
		<Minters<Test>>::insert(&minter, true);
		
		assert_ok!(Livecoin::mint(Origin::signed(minter), recipient, 42));
		
		assert_eq!(Livecoin::balance_of(recipient), 42);
	});
}

#[test]
fn can_add_minter() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		let minter = 2;
		<Owner<Test>>::put(owner);
		assert!(!Livecoin::is_minter(minter));

		assert_ok!(Livecoin::add_minter(Origin::signed(owner), minter));

		assert!(!Livecoin::is_minter(owner));
		assert!(Livecoin::is_minter(minter));
		assert!(!Livecoin::is_minter(3));
	});
}

#[test]
fn can_only_remove_existing_minter() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		let minter = 2;
		<Owner<Test>>::put(owner);
		assert!(!Livecoin::is_minter(minter));

		assert_noop!(Livecoin::remove_minter(Origin::signed(owner), minter), Error::<Test>::NotMinter);

		assert_ok!(Livecoin::add_minter(Origin::signed(owner), minter));
		assert!(Livecoin::is_minter(minter));

		assert_ok!(Livecoin::remove_minter(Origin::signed(owner), minter));
	});
}

#[test]
fn only_owner_can_add_and_remove_minters() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		let minter = 2;
		assert_noop!(Livecoin::add_minter(Origin::signed(owner), minter), Error::<Test>::NotOwner);

		<Owner<Test>>::put(owner);
		assert_ok!(Livecoin::add_minter(Origin::signed(owner), minter));
		assert!(Livecoin::is_minter(minter));
	});
}

#[test]
fn burning_works() {
	new_test_ext().execute_with(|| {
		let minter = 2;
		<Minters<Test>>::insert(&minter, true);
		<Balances<Test>>::insert(minter, 42);

		assert_ok!(Livecoin::burn(Origin::signed(minter), 21));

		assert_eq!(Livecoin::balance_of(minter), 21);
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		let minter = 2;
		let recipient = 3;
		<Minters<Test>>::insert(&minter, true);
		<Balances<Test>>::insert(minter, 42);

		assert_ok!(Livecoin::transfer(Origin::signed(minter), recipient, 20));

		assert_eq!(Livecoin::balance_of(minter), 22);
		assert_eq!(Livecoin::balance_of(recipient), 20);
	});
}