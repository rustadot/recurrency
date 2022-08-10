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
//! DATE: 2022-08-10, STEPS: `20`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("./res/genesis/frequency-weights.json"), DB CACHE: 1024

// Executed Command:
// ./target/release/frequency
// benchmark
// pallet
// --chain
// ./res/genesis/frequency-weights.json
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_msa
// --extrinsic
// *
// --steps
// 20
// --repeat
// 5
// --output
// ./pallets/msa/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(
	rustdoc::all,
	missing_docs,
	unused_parens,
	unused_imports
)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_msa.
pub trait WeightInfo {
	fn create(s: u32, ) -> Weight;
	fn create_sponsored_account_with_delegation() -> Weight;
	fn remove_delegation_by_provider(s: u32, ) -> Weight;
	fn add_key_to_msa() -> Weight;
	fn revoke_msa_key() -> Weight;
	fn add_provider_to_msa() -> Weight;
	fn revoke_msa_delegation_by_delegator() -> Weight;
}

/// Weights for pallet_msa using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa KeyInfoOf (r:1 w:1)
	// Storage: Msa MsaKeysOf (r:1 w:1)
	fn create(s: u32, ) -> Weight {
		(51_352_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((19_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:2 w:1)
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa MsaKeysOf (r:1 w:1)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn create_sponsored_account_with_delegation() -> Weight {
		(122_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn remove_delegation_by_provider(s: u32, ) -> Weight {
		(44_511_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((26_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:2 w:1)
	// Storage: Msa MsaKeysOf (r:1 w:1)
	fn add_key_to_msa() -> Weight {
		(108_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:2 w:1)
	fn revoke_msa_key() -> Weight {
		(41_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:2 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn add_provider_to_msa() -> Weight {
		(106_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn revoke_msa_delegation_by_delegator() -> Weight {
		(38_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa KeyInfoOf (r:1 w:1)
	// Storage: Msa MsaKeysOf (r:1 w:1)
	fn create(s: u32, ) -> Weight {
		(51_352_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((19_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:2 w:1)
	// Storage: Msa MsaIdentifier (r:1 w:1)
	// Storage: Msa MsaKeysOf (r:1 w:1)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn create_sponsored_account_with_delegation() -> Weight {
		(122_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn remove_delegation_by_provider(s: u32, ) -> Weight {
		(44_511_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((26_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:2 w:1)
	// Storage: Msa MsaKeysOf (r:1 w:1)
	fn add_key_to_msa() -> Weight {
		(108_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:2 w:1)
	fn revoke_msa_key() -> Weight {
		(41_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:2 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn add_provider_to_msa() -> Weight {
		(106_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Msa KeyInfoOf (r:1 w:0)
	// Storage: Msa ProviderInfoOf (r:1 w:1)
	fn revoke_msa_delegation_by_delegator() -> Weight {
		(38_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
