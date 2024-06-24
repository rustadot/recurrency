
//! Autogenerated weights for `pallet_time_release`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-06-24, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-173-4-164`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("frequency-bench")`, DB CACHE: `1024`

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_time-release
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --additional-trie-layers=5
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/time-release/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_time_release`.
pub trait WeightInfo {
	fn transfer() -> Weight;
	fn claim(i: u32, ) -> Weight;
	fn update_release_schedules(i: u32, ) -> Weight;
}

/// Weights for `pallet_time_release` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TimeRelease::ReleaseSchedules` (r:1 w:1)
	/// Proof: `TimeRelease::ReleaseSchedules` (`max_values`: None, `max_size`: Some(1449), added: 3924, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `6399`
		// Minimum execution time: 40_913_000 picoseconds.
		Weight::from_parts(41_256_000, 6399)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TimeRelease::ReleaseSchedules` (r:1 w:1)
	/// Proof: `TimeRelease::ReleaseSchedules` (`max_values`: None, `max_size`: Some(1449), added: 3924, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 50]`.
	fn claim(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `6399`
		// Minimum execution time: 26_883_000 picoseconds.
		Weight::from_parts(27_795_461, 6399)
			// Standard Error: 1_814
			.saturating_add(Weight::from_parts(3_760, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `TimeRelease::ReleaseSchedules` (r:0 w:1)
	/// Proof: `TimeRelease::ReleaseSchedules` (`max_values`: None, `max_size`: Some(1449), added: 3924, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 50]`.
	fn update_release_schedules(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `39`
		//  Estimated: `6249`
		// Minimum execution time: 21_550_000 picoseconds.
		Weight::from_parts(22_217_831, 6249)
			// Standard Error: 1_926
			.saturating_add(Weight::from_parts(47_853, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TimeRelease::ReleaseSchedules` (r:1 w:1)
	/// Proof: `TimeRelease::ReleaseSchedules` (`max_values`: None, `max_size`: Some(1449), added: 3924, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `6399`
		// Minimum execution time: 40_913_000 picoseconds.
		Weight::from_parts(41_256_000, 6399)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TimeRelease::ReleaseSchedules` (r:1 w:1)
	/// Proof: `TimeRelease::ReleaseSchedules` (`max_values`: None, `max_size`: Some(1449), added: 3924, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 50]`.
	fn claim(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `6399`
		// Minimum execution time: 26_883_000 picoseconds.
		Weight::from_parts(27_795_461, 6399)
			// Standard Error: 1_814
			.saturating_add(Weight::from_parts(3_760, 0).saturating_mul(i.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:1)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:0)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `TimeRelease::ReleaseSchedules` (r:0 w:1)
	/// Proof: `TimeRelease::ReleaseSchedules` (`max_values`: None, `max_size`: Some(1449), added: 3924, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 50]`.
	fn update_release_schedules(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `39`
		//  Estimated: `6249`
		// Minimum execution time: 21_550_000 picoseconds.
		Weight::from_parts(22_217_831, 6249)
			// Standard Error: 1_926
			.saturating_add(Weight::from_parts(47_853, 0).saturating_mul(i.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}


#[cfg(test)]
mod tests {
  use frame_support::{traits::Get, weights::Weight, dispatch::DispatchClass};
  use common_runtime::constants::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
  use common_runtime::weights::extrinsic_weights::ExtrinsicBaseWeight;

  struct BlockWeights;
  impl Get<frame_system::limits::BlockWeights> for BlockWeights {
  	fn get() -> frame_system::limits::BlockWeights {
  		frame_system::limits::BlockWeights::builder()
  			.base_block(Weight::zero())
  			.for_class(DispatchClass::all(), |weights| {
  				weights.base_extrinsic = ExtrinsicBaseWeight::get().into();
  			})
  			.for_class(DispatchClass::non_mandatory(), |weights| {
  				weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
  			})
  			.build_or_panic()
  	}
  }

	#[test]
	fn test_transfer() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6399
		);
	}
	#[test]
	fn test_claim() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6399
		);
	}
	#[test]
	fn test_update_release_schedules() {
		assert!(
			BlockWeights::get()
				.per_class
				.get(frame_support::dispatch::DispatchClass::Normal)
				.max_extrinsic
				.unwrap_or_else(<Weight as sp_runtime::traits::Bounded>::max_value)
				.proof_size()
				> 6249
		);
	}
}
