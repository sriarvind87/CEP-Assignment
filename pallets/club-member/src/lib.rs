#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn clubmembers)]
	pub type ClubMembers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The member is add in the club.
		MemberAdded,
		/// The member is removed from the club.
		MemberRemoved,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Already a member.
		AlreadyMember,
		/// Not a member.
		NotMember,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Add a member in the club.
		#[pallet::weight(40_000_000)]
		pub fn add_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			let mut club_members = ClubMembers::<T>::get();
			let location = club_members.binary_search(&who).err().ok_or(Error::<T>::AlreadyMember)?;

			club_members.insert(location, who.clone());

			ClubMembers::<T>::put(&club_members);


			Self::deposit_event(Event::MemberAdded);
			Ok(())
		}

		/// Remove a member from the club.
		#[pallet::weight(40_000_000)]
		pub fn remove_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			let mut club_members = ClubMembers::<T>::get();
			let location = club_members.binary_search(&who).ok().ok_or(Error::<T>::NotMember)?;
			club_members.remove(location);

			ClubMembers::<T>::put(&club_members);

			Self::deposit_event(Event::MemberRemoved);
			Ok(())
		}
	}
}


#[cfg(feature = "runtime-benchmarks")]
mod benchmark {
	use super::{Pallet as ClubMember, *};
	use frame_benchmarking::{account, benchmarks_instance_pallet, whitelist};
	use frame_support::{assert_ok, traits::EnsureOrigin};
	use frame_system::RawOrigin;

	const SEED: u32 = 0;

	benchmarks_instance_pallet! {
		add_member {
			let members = (0..m).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			set_members::<T>(members.clone(), None);
			let new_member = account::<T::AccountId>("add", m, SEED);
		}: {
			assert_ok!(ClubMember::<T>::add_member(T::AddOrigin::successful_origin(), new_member.clone()));
		}
		verify {
			assert!(ClubMembers::<T>::get().contains(&new_member));
			#[cfg(test)] crate::tests::clean();
		}


		remove_member {
			let members = (0..m).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			set_members::<T>(members.clone(), Some(members.len() - 1));
			let to_remove = members.first().cloned().unwrap();
		}: {
			assert_ok!(ClubMembers<T>::remove_member(T::RemoveOrigin::successful_origin(), to_remove.clone()));
		} verify {
			assert!(!ClubMembers<T>::get().contains(&to_remove));
			#[cfg(test)] crate::tests::clean();
		}
	}
}

