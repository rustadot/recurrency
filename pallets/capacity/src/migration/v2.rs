use crate::{
	types::{StakingDetails, UnlockChunk},
	BalanceOf, Config, Pallet, StakingAccountLedger, StakingType, UnlockChunkList, UnstakeUnlocks,
};
use frame_support::{
	pallet_prelude::{GetStorageVersion, Weight},
	traits::{Get, OnRuntimeUpgrade, StorageVersion},
};

const LOG_TARGET: &str = "runtime::capacity";

#[cfg(feature = "try-runtime")]
use sp_std::{fmt::Debug, vec::Vec};

/// Only contains V1 storage format
pub mod v1 {
	use super::*;
	use frame_support::{storage_alias, BoundedVec, Twox64Concat};
	use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
	use scale_info::TypeInfo;

	#[derive(Default, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, MaxEncodedLen)]
	/// Old StakingAccountDetails struct
	pub struct StakingAccountDetails<T: Config> {
		/// The amount a Staker has staked, minus the sum of all tokens in `unlocking`.
		pub active: BalanceOf<T>,
		/// The total amount of tokens in `active` and `unlocking`
		pub total: BalanceOf<T>,
		/// Unstaked balances that are thawing or awaiting withdrawal.
		pub unlocking: BoundedVec<
			UnlockChunk<BalanceOf<T>, T::EpochNumber>,
			<T as crate::pallet::Config>::MaxUnlockingChunks,
		>,
	}

	#[storage_alias]
	/// alias to StakingAccountLedger storage
	pub(crate) type StakingAccountLedger<T: Config> = StorageMap<
		Pallet<T>,
		Twox64Concat,
		<T as frame_system::Config>::AccountId,
		StakingAccountDetails<T>,
	>;
}

/// migrate StakingAccountLedger to use new StakingDetails
pub fn migrate_to_v2<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version(); // 1r

	if on_chain_version.lt(&2) {
		log::info!(target: LOG_TARGET, "🔄 StakingAccountLedger migration started");
		let mut maybe_count = 0u32;
		StakingAccountLedger::<T>::translate(
			|key: <T as frame_system::Config>::AccountId,
			 old_details: v1::StakingAccountDetails<T>| {
				let new_account: StakingDetails<T> = StakingDetails {
					active: old_details.active,
					staking_type: StakingType::MaximumCapacity,
				};
				let new_unlocks: UnlockChunkList<T> = old_details.unlocking;
				UnstakeUnlocks::<T>::insert(key, new_unlocks);
				maybe_count += 1;
				log::info!(target: LOG_TARGET,"migrated {:?}", maybe_count);
				Some(new_account)
			},
		);
		StorageVersion::new(2).put::<Pallet<T>>(); // 1 w
		let reads = (maybe_count + 1) as u64;
		let writes = (maybe_count * 2 + 1) as u64;
		log::info!(target: LOG_TARGET, "🔄 migration finished");
		let weight = T::DbWeight::get().reads_writes(reads, writes);
		log::info!(target: LOG_TARGET, "Migration calculated weight = {:?}", weight);
		weight
	} else {
		// storage was already migrated.
		log::info!(target: LOG_TARGET, "Old StorageAccountLedger migration attempted to run. Please remove");
		T::DbWeight::get().reads(1)
	}
}
/// The OnRuntimeUpgrade implementation for this storage migration
pub struct MigrateToV2<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
	fn on_runtime_upgrade() -> Weight {
		migrate_to_v2::<T>()
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
		use frame_support::storage::generator::StorageMap;
		use parity_scale_codec::Encode;
		let pallet_prefix = v1::StakingAccountLedger::<T>::module_prefix();
		let storage_prefix = v1::StakingAccountLedger::<T>::storage_prefix();
		assert_eq!(&b"Capacity"[..], pallet_prefix);
		assert_eq!(&b"StakingAccountLedger"[..], storage_prefix);
		log::info!(target: LOG_TARGET, "Running pre_upgrade...");

		let count = v1::StakingAccountLedger::<T>::iter().count() as u32;
		log::info!(target: LOG_TARGET, "Finish pre_upgrade for {:?} records", count);
		Ok(count.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
		use parity_scale_codec::Decode;
		let pre_upgrade_count: u32 = Decode::decode(&mut state.as_slice()).unwrap_or_default();
		let on_chain_version = Pallet::<T>::on_chain_storage_version();

		assert_eq!(on_chain_version, crate::pallet::STORAGE_VERSION);
		assert_eq!(pre_upgrade_count as usize, StakingAccountLedger::<T>::iter().count());
		assert_eq!(pre_upgrade_count as usize, UnstakeUnlocks::<T>::iter().count());

		log::info!(target: LOG_TARGET, "✅ migration post_upgrade checks passed");
		Ok(())
	}
}
