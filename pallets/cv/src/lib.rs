#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{inherent::Vec, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use pallet_utils::{String, TypeID, UnixEpoch, WhoAndWhen};
	use scale_info::TypeInfo;
	use serde::{Deserialize, Serialize};

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[cfg_attr(
		feature = "std",
		serde(bound(serialize = "Account: Serialize, BlockNumber: Serialize, Time: Serialize"))
	)]
	#[cfg_attr(
		feature = "std",
		serde(bound(
			deserialize = "Account: Deserialize<'de>, BlockNumber: Deserialize<'de>, Time: Deserialize<'de>"
		))
	)]
	pub struct Item<Account, BlockNumber, Time> {
		pub item_id: TypeID,
		pub owner: Account,
		pub created: WhoAndWhen<Account, BlockNumber, Time>,
		pub period_from: Option<UnixEpoch>,
		pub period_to: Option<UnixEpoch>,
		pub certificate_id: Option<TypeID>,
		pub score: u32,
		pub keywords: Vec<String>,
		pub metadata: String, /* { "key": "value" }, example: {"content": "we need for talents
		                       * from around the world for this position"} */
	}

	impl<Account, BlockNumber, Time> Item<Account, BlockNumber, Time> {
		pub fn new(
			id: TypeID,
			owner: Account,
			created_by: Account,
			period_from: Option<UnixEpoch>,
			period_to: Option<UnixEpoch>,
			certificate_id: Option<TypeID>,
			score: u32,
			keywords: Vec<String>,
			metadata: String,
			block: BlockNumber,
			time: Time,
		) -> Self {
			Item {
				item_id: id,
				owner,
				created: WhoAndWhen::<Account, BlockNumber, Time>::new(created_by, block, time),
				period_from,
				period_to,
				certificate_id,
				score,
				keywords,
				metadata,
			}
		}

		// pub fn ensure_owner(&self, account: &T::AccountId) -> DispatchResult {
		// 	ensure!(self.is_owner(account), Error::<T>::NotAPostOwner);
		// 	Ok(())
		// }

		// pub fn is_owner(&self, account: &T::AccountId) -> bool {
		// 	self.owner == *account
		// }
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum Status {
		Pending,
		Allow,
		Deny,
	}
	impl Default for Status {
		fn default() -> Self {
			Self::Pending
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_utils::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn item_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type ItemId<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn item_by_id)]
	pub type ItemById<T: Config> = StorageMap<
		_,
		Twox64Concat,
		TypeID,
		Item<T::AccountId, T::BlockNumber, <T as pallet_timestamp::Config>::Moment>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn item_status_by_item_id)]
	pub type ItemStatusByItemId<T> = StorageMap<_, Twox64Concat, TypeID, Status, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn items_by_accountid)]
	pub type ItemsByAccountId<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Vec<TypeID>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RevokeSucceed(TypeID),
		CreateSucceed(TypeID),
		SetStatusSucceed(TypeID),
		ItemUpdated(TypeID),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		ItemNotFound,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		OwnerNotFound,

		ItemUpdateFailed,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000)]
		pub fn create_item(
			origin: OriginFor<T>,
			_owner: T::AccountId,
			_metadata: String,
			_period_from: Option<UnixEpoch>,
			_period_to: Option<UnixEpoch>,
			_keywords: Vec<String>,
			_certificated_id: Option<TypeID>,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			let item_id = Self::item_id().checked_add(1).ok_or(Error::<T>::StorageOverflow)?;

			let new_item: Item<
				T::AccountId,
				T::BlockNumber,
				<T as pallet_timestamp::Config>::Moment,
			> = Item::new(
				item_id,
				_owner.clone(),
				who.clone(),
				_period_from,
				_period_to,
				_certificated_id,
				0,
				_keywords,
				_metadata,
				<frame_system::Pallet<T>>::block_number(),
				<pallet_timestamp::Pallet<T>>::now(),
			);

			// add cv item to storage along with its id
			<ItemById<T>>::insert(item_id, new_item);

			// track item by account id
			<ItemsByAccountId<T>>::mutate(who, |x| x.push(item_id));

			// increase item id by one
			<ItemId<T>>::mutate(|n| {
				*n = item_id;
			});

			// Emit an event.
			Self::deposit_event(Event::CreateSucceed(item_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_item(origin: OriginFor<T>, _item_id: TypeID) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			let item_idx = Self::items_by_accountid(&who).iter().position(|x| *x == _item_id);
			ensure!(item_idx != None, Error::<T>::ItemNotFound);
			if let Some(iid) = item_idx {
				<ItemsByAccountId<T>>::mutate(&who, |x| x.swap_remove(iid));
			}
			<ItemById<T>>::remove(_item_id);
			// Emit an event.
			Self::deposit_event(Event::RevokeSucceed(_item_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		#[pallet::weight(1000)]
		pub fn set_status_item(
			origin: OriginFor<T>,
			_item_id: TypeID,
			status: Status,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let item_idx = Self::items_by_accountid(&who).iter().position(|x| *x == _item_id);
			ensure!(item_idx != None, Error::<T>::ItemNotFound);
			match <ItemStatusByItemId<T>>::contains_key(_item_id) {
				true => {
					<ItemStatusByItemId<T>>::mutate(_item_id, |x| *x = status);
				},
				_ => {
					<ItemStatusByItemId<T>>::insert(_item_id, status);
				},
			}
			Self::deposit_event(Event::SetStatusSucceed(_item_id));
			Ok(().into())
		}

		#[pallet::weight(1_000)]
		pub fn update_item(
			origin: OriginFor<T>,
			_item_id: TypeID,
			_owner: T::AccountId,
			_metadata: String,
			_period_from: Option<UnixEpoch>,
			_period_to: Option<UnixEpoch>,
			_keywords: Vec<String>,
			_certificated_id: Option<TypeID>,
		) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;

			// find item by id
			ensure!(ItemById::<T>::contains_key(_item_id), Error::<T>::ItemNotFound);

			let is_owner = Self::check_permission(&_item_id, &from);
			// check permission
			ensure!(is_owner, Error::<T>::OwnerNotFound);

			// update item in storage
			ItemById::<T>::try_mutate(&_item_id, |item| {
				let mut new_item = item.clone().unwrap();

				new_item.owner = _owner;
				new_item.metadata = _metadata;
				new_item.period_from = _period_from;
				new_item.period_to = _period_to;
				new_item.keywords = _keywords;
				new_item.certificate_id = _certificated_id;

				*item = Some(new_item);

				Ok(())
			})
			.map_err(
				|_it: Item<
					T::AccountId,
					T::BlockNumber,
					<T as pallet_timestamp::Config>::Moment,
				>| Error::<T>::ItemUpdateFailed,
			)?;

			Self::deposit_event(Event::<T>::ItemUpdated(_item_id));
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn check_permission(item_id: &TypeID, sender: &T::AccountId) -> bool {
			let item = ItemById::<T>::get(item_id);
			if item.is_some() {
				let item = item.unwrap();
				return item.owner == *sender
			}
			false
		}
		pub fn get_cv(
		) -> Vec<Item<T::AccountId, T::BlockNumber, <T as pallet_timestamp::Config>::Moment>> {
			ItemById::<T>::iter_values().collect()
		}
	}
}
