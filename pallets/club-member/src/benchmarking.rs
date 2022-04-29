//! Benchmarking setup for pallet-club-member

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
