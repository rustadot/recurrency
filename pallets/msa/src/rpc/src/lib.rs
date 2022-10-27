// Strong Documentation Lints
#![deny(
	rustdoc::broken_intra_doc_links,
	rustdoc::missing_crate_level_docs,
	rustdoc::invalid_codeblock_attributes,
	missing_docs
)]

//! Custom APIs for [MSA](../pallet_msa/index.html)

use codec::Codec;
use common_helpers::rpc::*;
use common_primitives::{
	msa::{Delegator, MessageSourceId, Provider},
	node::BlockNumber,
	schema::SchemaId,
};

use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
};
use pallet_msa_runtime_api::MsaRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

/// Frequency MSA Custom RPC API
#[rpc(client, server)]
pub trait MsaApi<BlockHash, AccountId> {
	/// Check for a list of delegations
	/// Given a single provider, test a list of potential delegators
	/// At a given block number
	#[method(name = "msa_checkDelegations")]
	fn check_delegations(
		&self,
		delegator_msa_ids: Vec<MessageSourceId>,
		provider_msa_id: MessageSourceId,
		block_number: Option<BlockNumber>,
	) -> RpcResult<Vec<(MessageSourceId, bool)>>;

	/// Retrieve the list of currently granted schemas given a delegator and provider pair
	#[method(name = "msa_grantedSchemaIdsByMsaId")]
	fn get_granted_schemas_by_msa_id(
		&self,
		delegator_msa_id: MessageSourceId,
		provider_msa_id: MessageSourceId,
	) -> RpcResult<Option<Vec<SchemaId>>>;
}

/// The client handler for the API used by Frequency Service RPC with `jsonrpsee`
pub struct MsaHandler<C, M> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> MsaHandler<C, M> {
	/// Create new instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

#[async_trait]
impl<C, Block, AccountId> MsaApiServer<<Block as BlockT>::Hash, AccountId> for MsaHandler<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: MsaRuntimeApi<Block, AccountId>,
	AccountId: Codec,
{
	// *Temporarily Removed* until https://github.com/LibertyDSNP/frequency/issues/418 is completed
	// fn get_msa_keys(&self, msa_id: MessageSourceId) -> RpcResult<Vec<KeyInfoResponse<AccountId>>> {
	// 	let api = self.client.runtime_api();
	// 	let at = BlockId::hash(self.client.info().best_hash);
	// 	let runtime_api_result = api.get_msa_keys(&at, msa_id);
	// 	map_rpc_result(runtime_api_result)
	// }

	fn check_delegations(
		&self,
		delegator_msa_ids: Vec<MessageSourceId>,
		provider_msa_id: MessageSourceId,
		block_number: Option<BlockNumber>,
	) -> RpcResult<Vec<(MessageSourceId, bool)>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);

		let provider = Provider(provider_msa_id);

		Ok(delegator_msa_ids
			.iter() // TODO: Change back to par_iter() which has borrow panic
			.map(|&id| {
				let delegator = Delegator(id);
				let has_delegation =
					api.has_delegation(&at, delegator, provider, block_number).unwrap();
				(id, has_delegation)
			})
			.collect())
	}

	fn get_granted_schemas_by_msa_id(
		&self,
		delegator_msa_id: MessageSourceId,
		provider_msa_id: MessageSourceId,
	) -> RpcResult<Option<Vec<SchemaId>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);
		let delegator = Delegator(delegator_msa_id);
		let provider = Provider(provider_msa_id);
		let runtime_api_result = api.get_granted_schemas_by_msa_id(&at, delegator, provider);
		map_rpc_result(runtime_api_result)
	}
}
