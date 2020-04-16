// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn can_set_supply() {
	new_test_ext().execute_with(|| {
		// Just a dummy test for the dummy function `do_something`
		// calling the `do_something` function with a value 42
		assert_ok!(Livecoin::set_supply(Origin::signed(1), 42));
		// asserting that the stored value is equal to what we stored
		assert_eq!(Livecoin::total_supply(), 42);
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the correct error is thrown on None value
// 		assert_noop!(
// 			Livecoin::cause_error(Origin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }
