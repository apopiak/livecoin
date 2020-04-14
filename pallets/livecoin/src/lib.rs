#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage,
	dispatch::{DispatchError, DispatchResult},
};
use system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		TotalSupply get(total_supply): u64;

		Balances get(balance_of): map hasher(blake2_128_concat) T::AccountId => u64;
		Minters get(is_minter): map hasher(blake2_128_concat) T::AccountId => bool;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		Mint(AccountId, AccountId, u64),
		Burn(AccountId, u64),
		MinterAdded(AccountId),
		MinterRemoved(AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// An account without minting privilege tried to mint.
		NotMinter,
		/// An account does not have enough balance to perform the operation.
		InsufficientBalance,
		BalanceOverflow,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		pub fn mint(origin, to_account: T::AccountId, amount: u64) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let minter = Self::ensure_minter(sender)?;

			<TotalSupply>::mutate(|supply| *supply = supply.saturating_add(amount));
			<Balances<T>>::mutate(&to_account, |balance| *balance = balance.saturating_add(amount));

			Self::deposit_event(RawEvent::Mint(minter, to_account, amount));
			Ok(())
		}

		pub fn add_minter(origin, minter: T::AccountId) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			<Minters<T>>::insert(&minter, true);

			Self::deposit_event(RawEvent::MinterAdded(minter));
			Ok(())
		}

		pub fn remove_minter(origin, minter: T::AccountId) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			let minter = Self::ensure_minter(minter)?;
			<Minters<T>>::remove(&minter);

			Self::deposit_event(RawEvent::MinterRemoved(minter));
			Ok(())
		}

		pub fn burn(origin, amount: u64) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// make sure that only minters can burn coins
			let minter = Self::ensure_minter(sender)?;

			<Balances<T>>::try_mutate(&minter, |balance| -> Result<u64, DispatchError> {
				*balance = balance.checked_sub(amount).ok_or(Error::<T>::InsufficientBalance)?;
				Ok(*balance)
			})?;
			<TotalSupply>::mutate(|supply| *supply = supply.saturating_sub(amount));

			Self::deposit_event(RawEvent::Burn(minter, amount));
			Ok(())
		}

		pub fn transfer(origin, to_account: T::AccountId, amount: u64) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::transfer_from_to(&sender, &to_account, amount)
		}
	}
}

impl<T: Trait> Module<T> {
	fn ensure_minter(account: T::AccountId) -> Result<T::AccountId, DispatchError> {
		if !Self::is_minter(&account) {
			return Err(DispatchError::from(Error::<T>::NotMinter));
		}
		Ok(account)
	}

	/// Transfer `amount` of coins from one account to another.
	///
	/// **Weight:**
	/// - complexity: `O(1)`
	/// - DB access: 2 storage map reads + 2 storage map writes
	fn transfer_from_to(from: &T::AccountId, to: &T::AccountId, amount: u64) -> DispatchResult {
		let from_balance = Self::balance_of(from);
		let updated_from_balance = from_balance
			.checked_sub(amount)
			.ok_or(Error::<T>::InsufficientBalance)?;
		let receiver_balance = Self::balance_of(&to);
		let updated_to_balance = receiver_balance
			.checked_add(amount)
			.ok_or(Error::<T>::BalanceOverflow)?;

		// ↑ verify ↑
		// ↓ update ↓

		// reduce from's balance
		<Balances<T>>::insert(&from, updated_from_balance);
		// increase receiver's balance
		<Balances<T>>::insert(&to, updated_to_balance);

		Ok(())
	}
}