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
	trait Store for Module<T: Trait> as Livecoin {
		Owner get(fn owner): T::AccountId;

		TotalSupply get(fn total_supply): u64;

		Balances get(fn balance_of): map hasher(blake2_128_concat) T::AccountId => u64;

		Minters get(fn is_minter): map hasher(twox_64_concat) T::AccountId => bool;
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
		NewOwner(AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Tried to call a function that is limited to the owner of the stablecoin
		/// while not being the owner.
		NotOwner,
		/// A non-minter account tried to mint.
		NotMinter,
		SupplyOverflow,
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

		pub fn set_owner(origin, new_owner: T::AccountId) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			<Owner<T>>::put(&new_owner);

			Self::deposit_event(RawEvent::NewOwner(new_owner));
			Ok(())
		}

		/// Add an account as minter.
		pub fn add_minter(origin, new_minter: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _owner = Self::ensure_owner(who)?;

			<Minters<T>>::insert(&new_minter, true);

			Self::deposit_event(RawEvent::MinterAdded(new_minter));
			Ok(())
		}

		/// Remove an account from the set of minters.
		pub fn remove_minter(origin, minter: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let _owner = Self::ensure_owner(who)?;

			<Minters<T>>::remove(&minter);

			Self::deposit_event(RawEvent::MinterRemoved(minter));
			Ok(())
		}

		/// Create `amount` of coins out of thin air and deposit them into `to_account`.
		pub fn mint(origin, to_account: T::AccountId, amount: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let minter = Self::ensure_minter(who)?;

			let supply = Self::total_supply();
			let new_supply = supply.checked_add(amount).ok_or(Error::<T>::SupplyOverflow)?;
			// ^ verify
			// v update
			<TotalSupply>::put(new_supply);
			<Balances<T>>::mutate(&to_account, |balance| {
				*balance = balance.saturating_add(amount);
			});
			Self::deposit_event(RawEvent::Mint(minter, to_account, amount));
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	fn ensure_owner(acc: T::AccountId) -> Result<T::AccountId, DispatchError> {
		if acc != Self::owner() {
			return Err(DispatchError::from(Error::<T>::NotOwner));
		}
		Ok(acc)
	}

	fn ensure_minter(acc: T::AccountId) -> Result<T::AccountId, DispatchError> {
		if !Self::is_minter(&acc) {
			return Err(DispatchError::from(Error::<T>::NotMinter));
		}
		Ok(acc)
	}
}
