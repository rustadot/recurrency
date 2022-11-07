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

//! Runtime API definition for [MSA](../pallet_msa/index.html)
//!
//! This api must be implemented by the node runtime.
//! Runtime APIs Provide:
//! - An interface between the runtime and Custom RPCs.
//! - Runtime interfaces for end users beyond just State Queries

use codec::Codec;
use common_primitives::{msa::*, node::BlockNumber};
use sp_std::vec::Vec;

// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime files (the `runtime` folder)
sp_api::decl_runtime_apis! {
	/// Runtime API definition for [MSA](../pallet_msa/index.html)
	pub trait MsaRuntimeApi<AccountId> where
		AccountId: Codec,
	{
		// *Temporarily Removed* until https://github.com/LibertyDSNP/frequency/issues/418 is completed
		// fn get_msa_keys(msa_id: MessageSourceId) ->	Vec<KeyInfoResponse<AccountId>>;

		/// Check to see if a delegation existed between the given delegator and provider at a given block
		fn has_delegation(delegator: Delegator, provider: Provider, block_number: Option<BlockNumber>) -> bool;

		/// Get the list of schema ids (if any) that exist in any delegation between the delegator and provider
		fn get_granted_schemas_by_msa_id(delegator: Delegator, provider: Provider) -> Option<Vec<SchemaId>>;
	}
}
