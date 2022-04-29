//! Benchmarking setup for pallet-club-member

use super::{Pallet as ClubMember, *};
use frame_benchmarking::{account, benchmarks, benchmarks_instance_pallet, whitelist};
use frame_benchmarking::BenchmarkParameter::m;
use frame_support::{assert_noop, assert_ok, traits::EnsureOrigin};
use frame_system::{Origin, RawOrigin};

const SEED: u32 = 0;

benchmarks! {
		add_member {
			let members = (0..20).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			let new_member = account::<T::AccountId>("add", 20, SEED);
		}: {
			assert_ok!(ClubMember::<T>::add_member(RawOrigin::Root, new_member.clone()));
		}
		verify {
			assert!(ClubMembers::<T>::get().contains(&new_member));
			#[cfg(test)] crate::tests::clean();
		}


		remove_member {

			let members = (0..20).map(|i| account("member", i, SEED)).collect::<Vec<T::AccountId>>();
			let to_remove = members.first().cloned().unwrap();

		}: {
			assert_ok!(ClubMembers::<T>::remove_member(RawOrigin::Root, to_remove.clone()));
		} verify {
			assert_noop!(ClubMembers::<T>::get());
			#[cfg(test)] crate::tests::clean();
		}
	}
