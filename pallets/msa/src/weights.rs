// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_msa
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-06, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-runner-wc6w8-pv9rm`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_msa
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --additional-trie-layers=5
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/msa/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_msa.
pub trait WeightInfo {
	fn create() -> Weight;
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight;
	fn revoke_delegation_by_provider() -> Weight;
	fn add_public_key_to_msa() -> Weight;
	fn delete_msa_public_key() -> Weight;
	fn retire_msa() -> Weight;
	fn grant_delegation(s: u32, ) -> Weight;
	fn revoke_delegation_by_delegator() -> Weight;
	fn create_provider() -> Weight;
	fn create_provider_via_governance() -> Weight;
	fn propose_to_be_provider() -> Weight;
	fn revoke_schema_permissions(s: u32, ) -> Weight;
}

/// Weights for pallet_msa using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Msa::CurrentMsaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Msa::CurrentMsaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `4998`
		// Minimum execution time: 15_517_000 picoseconds.
		Weight::from_parts(16_044_000, 4998)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:2 w:2)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Msa::CurrentMsaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Msa::CurrentMsaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1284`
		//  Estimated: `7521`
		// Minimum execution time: 117_466_000 picoseconds.
		Weight::from_parts(121_415_406, 7521)
			// Standard Error: 22_880
			.saturating_add(Weight::from_parts(143_516, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	fn revoke_delegation_by_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `5167`
		// Minimum execution time: 15_874_000 picoseconds.
		Weight::from_parts(16_228_000, 5167)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:4 w:4)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn add_public_key_to_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1502`
		//  Estimated: `10971`
		// Minimum execution time: 171_847_000 picoseconds.
		Weight::from_parts(176_454_000, 10971)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn delete_msa_public_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `254`
		//  Estimated: `7521`
		// Minimum execution time: 26_754_000 picoseconds.
		Weight::from_parts(28_041_000, 7521)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn retire_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `71`
		//  Estimated: `4998`
		// Minimum execution time: 22_131_000 picoseconds.
		Weight::from_parts(22_637_000, 4998)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:2 w:2)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn grant_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1327`
		//  Estimated: `7521`
		// Minimum execution time: 106_204_000 picoseconds.
		Weight::from_parts(109_160_115, 7521)
			// Standard Error: 22_230
			.saturating_add(Weight::from_parts(163_811, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	fn revoke_delegation_by_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `5167`
		// Minimum execution time: 16_226_000 picoseconds.
		Weight::from_parts(16_912_000, 5167)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:1)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	fn create_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `4998`
		// Minimum execution time: 12_713_000 picoseconds.
		Weight::from_parts(13_207_000, 4998)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:1)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	fn create_provider_via_governance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `4998`
		// Minimum execution time: 12_983_000 picoseconds.
		Weight::from_parts(13_240_000, 4998)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn propose_to_be_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `5097`
		// Minimum execution time: 22_650_000 picoseconds.
		Weight::from_parts(23_294_000, 5097)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn revoke_schema_permissions(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316 + s * (6 ±0)`
		//  Estimated: `5167`
		// Minimum execution time: 19_241_000 picoseconds.
		Weight::from_parts(20_319_646, 5167)
			// Standard Error: 4_971
			.saturating_add(Weight::from_parts(110_294, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Msa::CurrentMsaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Msa::CurrentMsaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `4998`
		// Minimum execution time: 15_517_000 picoseconds.
		Weight::from_parts(16_044_000, 4998)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:2 w:2)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Msa::CurrentMsaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Msa::CurrentMsaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn create_sponsored_account_with_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1284`
		//  Estimated: `7521`
		// Minimum execution time: 117_466_000 picoseconds.
		Weight::from_parts(121_415_406, 7521)
			// Standard Error: 22_880
			.saturating_add(Weight::from_parts(143_516, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	fn revoke_delegation_by_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `5167`
		// Minimum execution time: 15_874_000 picoseconds.
		Weight::from_parts(16_228_000, 5167)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:4 w:4)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn add_public_key_to_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1502`
		//  Estimated: `10971`
		// Minimum execution time: 171_847_000 picoseconds.
		Weight::from_parts(176_454_000, 10971)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn delete_msa_public_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `254`
		//  Estimated: `7521`
		// Minimum execution time: 26_754_000 picoseconds.
		Weight::from_parts(28_041_000, 7521)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyCountForMsaId` (r:1 w:1)
	/// Proof: `Msa::PublicKeyCountForMsaId` (`max_values`: None, `max_size`: Some(17), added: 2492, mode: `MaxEncodedLen`)
	fn retire_msa() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `71`
		//  Estimated: `4998`
		// Minimum execution time: 22_131_000 picoseconds.
		Weight::from_parts(22_637_000, 4998)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Msa::PayloadSignatureRegistryList` (r:2 w:2)
	/// Proof: `Msa::PayloadSignatureRegistryList` (`max_values`: Some(50000), `max_size`: Some(144), added: 2124, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PayloadSignatureRegistryPointer` (r:1 w:1)
	/// Proof: `Msa::PayloadSignatureRegistryPointer` (`max_values`: Some(1), `max_size`: Some(140), added: 635, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:2 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:0)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn grant_delegation(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1327`
		//  Estimated: `7521`
		// Minimum execution time: 106_204_000 picoseconds.
		Weight::from_parts(109_160_115, 7521)
			// Standard Error: 22_230
			.saturating_add(Weight::from_parts(163_811, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	fn revoke_delegation_by_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `161`
		//  Estimated: `5167`
		// Minimum execution time: 16_226_000 picoseconds.
		Weight::from_parts(16_912_000, 5167)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:1)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	fn create_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `4998`
		// Minimum execution time: 12_713_000 picoseconds.
		Weight::from_parts(13_207_000, 4998)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::ProviderToRegistryEntry` (r:1 w:1)
	/// Proof: `Msa::ProviderToRegistryEntry` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	fn create_provider_via_governance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `4998`
		// Minimum execution time: 12_983_000 picoseconds.
		Weight::from_parts(13_240_000, 4998)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn propose_to_be_provider() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `5097`
		// Minimum execution time: 22_650_000 picoseconds.
		Weight::from_parts(23_294_000, 5097)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:1)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:0)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 30]`.
	fn revoke_schema_permissions(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `316 + s * (6 ±0)`
		//  Estimated: `5167`
		// Minimum execution time: 19_241_000 picoseconds.
		Weight::from_parts(20_319_646, 5167)
			// Standard Error: 4_971
			.saturating_add(Weight::from_parts(110_294, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
