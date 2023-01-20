use codec::{Decode, Encode};
use common_primitives::{
	messages::MessageResponse, msa::MessageSourceId, node::BlockNumber, schema::PayloadLocation,
};
use frame_support::{traits::Get, BoundedVec};
use multibase::Base;
use scale_info::TypeInfo;
use sp_std::prelude::*;

/// Payloads stored offchain contain a tuple of (bytes(the payload reference), payload length).
pub type OffchainPayloadType = (Vec<u8>, u32);

/// A single message type definition.
#[derive(Default, Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
#[scale_info(skip_type_params(MaxDataSize))]
pub struct Message<MaxDataSize>
where
	MaxDataSize: Get<u32> + Clone,
{
	///  Data structured by the associated schema's model.
	pub payload: BoundedVec<u8, MaxDataSize>,
	/// Message source account id of the Provider. This may be the same id as contained in `msa_id`,
	/// indicating that the original source MSA is acting as its own provider. An id differing from that
	/// of `msa_id` indicates that `provider_msa_id` was delegated by `msa_id` to send this message on
	/// its behalf.
	pub provider_msa_id: MessageSourceId,
	///  Message source account id (the original source).
	pub msa_id: Option<MessageSourceId>,
	///  Stores index of message in block to keep total order.
	pub index: u16,
}

impl<MaxDataSize> Message<MaxDataSize>
where
	MaxDataSize: Get<u32> + Clone,
{
	/// Helper function to handle response type [`MessageResponse`] depending on the Payload Location (on chain or IPFS)
	pub fn map_to_response(
		&self,
		block_number: BlockNumber,
		payload_location: PayloadLocation,
	) -> MessageResponse {
		match payload_location {
			PayloadLocation::OnChain => MessageResponse {
				provider_msa_id: self.provider_msa_id,
				index: self.index,
				block_number,
				msa_id: Some(self.msa_id.unwrap_or_default()),
				payload: Some(self.payload.to_vec()),
				cid: None,
				payload_length: None,
			},
			PayloadLocation::IPFS => {
				let (binary_cid, payload_length) =
					OffchainPayloadType::decode(&mut &self.payload[..]).unwrap_or_default();
				MessageResponse {
					provider_msa_id: self.provider_msa_id,
					index: self.index,
					block_number,
					cid: Some(multibase::encode(Base::Base32Lower, binary_cid).as_bytes().to_vec()),
					payload_length: Some(payload_length),
					msa_id: None,
					payload: None,
				}
			},
		}
	}
}
