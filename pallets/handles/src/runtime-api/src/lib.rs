#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]
// Strong Documentation Lints
#![deny(
	rustdoc::broken_intra_doc_links,
	rustdoc::missing_crate_level_docs,
	rustdoc::invalid_codeblock_attributes,
	missing_docs
)]

//! Runtime API definition for [Stateful  Storage](../pallet_stateful_storage/index.html)
//!
//! This api must be implemented by the node runtime.
//! Runtime APIs Provide:
//! - An interface between the runtime and Custom RPCs.
//! - Runtime interfaces for end users beyond just State Queries

use common_primitives::{
	handles::{Handle, HandleResponse, PresumptiveSuffixesResponse},
	msa::MessageSourceId,
};

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime files (the `runtime` folder)
sp_api::decl_runtime_apis! {

	/// Runtime Version for Stateful Storage
	/// - MUST be incremented if anything changes
	/// - Also update in js/api-augment
	/// - See: https://paritytech.github.io/polkadot/doc/polkadot_primitives/runtime_api/index.html
	#[api_version(1)]

	/// Runtime APIs for [Stateful Storage](../pallet_stateful_storage/index.html)
	pub trait HandlesRuntimeApi
	{
		/// Retrieve handle for a particular msa
		fn get_handle_for_msa(msa_id: MessageSourceId) -> Option<HandleResponse>;

		/// Retrieve the next `n` suffixes for a given handle
		fn get_next_suffixes(base_handle: Handle, count: u16) -> PresumptiveSuffixesResponse;

		/// Retrieve msa for a particular handle
		fn get_msa_for_handle(display_handle: Handle) -> Option<MessageSourceId>;
	}
}
