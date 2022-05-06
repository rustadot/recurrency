use crate::msa::MessageSenderId;
#[cfg(feature = "std")]
use crate::utils;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::{prelude::*, vec};
#[cfg(feature = "std")]
use utils::*;

pub type SchemaId = u16;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
pub struct MessageResponse<AccountId, BlockNumber> {
	#[cfg_attr(feature = "std", serde(with = "as_hex"))]
	pub data: Vec<u8>, //  Serialized data in a user-defined schema format
	pub signer: AccountId,       //  Signature of the signer
	pub msa_id: MessageSenderId, //  Message source account id (the original sender)
	pub index: u16,              // index in block to get total order
	pub block_number: BlockNumber,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
pub struct BlockPaginationRequest<BlockNumber> {
	pub from_block: BlockNumber, // inclusive
	pub from_index: u32,         // starts from 0
	pub to_block: BlockNumber,   // exclusive
	pub page_size: u32,
}

impl<BlockNumber> BlockPaginationRequest<BlockNumber>
where
	BlockNumber: Copy + AtLeast32BitUnsigned,
{
	pub const MAX_PAGE_SIZE: u32 = 10000;
	pub const MAX_BLOCK_RANGE: u32 = 50000; // ~3 days (6 sec per block)= ~7 days (12 sec per block)

	pub fn validate(&self) -> bool {
		self.page_size > 0 &&
			self.page_size <= Self::MAX_PAGE_SIZE &&
			self.from_block < self.to_block &&
			self.to_block.clone().sub(self.from_block.clone()) <=
				BlockNumber::from(Self::MAX_BLOCK_RANGE)
	}
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq)]
pub struct BlockPaginationResponse<BlockNumber, T> {
	pub content: Vec<T>,
	pub has_next: bool,
	#[cfg_attr(feature = "std", serde(skip_serializing_if = "Option::is_none"))]
	pub next_block: Option<BlockNumber>,
	#[cfg_attr(feature = "std", serde(skip_serializing_if = "Option::is_none"))]
	pub next_index: Option<u32>,
}

impl<BlockNumber, T> BlockPaginationResponse<BlockNumber, T> {
	pub const fn new() -> BlockPaginationResponse<BlockNumber, T> {
		BlockPaginationResponse {
			content: vec![],
			has_next: false,
			next_block: None,
			next_index: None,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::messages::BlockPaginationRequest;

	struct TestCase<T> {
		input: BlockPaginationRequest<u32>,
		expected: T,
		message: String,
	}

	#[test]
	fn block_pagination_request_validation_test() {
		let test_cases: Vec<TestCase<bool>> = vec![
			TestCase {
				input: BlockPaginationRequest { from_block: 10, from_index: 0, to_block: 12, page_size: 1 },
				expected: true,
				message: "Should be valid".to_string(),
			},
			TestCase {
				input: BlockPaginationRequest { from_block: 10, from_index: 0, to_block: 12, page_size: 0 },
				expected: false,
				message: "Page with size 0 is invalid".to_string(),
			},
			TestCase {
				input: BlockPaginationRequest { from_block: 10, from_index: 0, to_block: 8, page_size: 1 },
				expected: false,
				message: "from_block should be less than to_block".to_string(),
			},
			TestCase {
				input: BlockPaginationRequest { from_block: 10, from_index: 0, to_block: 8, page_size: 10000 + 1 },
				expected: false,
				message: "page_size should be less than MAX_PAGE_SIZE".to_string(),
			},
			TestCase {
				input: BlockPaginationRequest { from_block: 1, from_index: 0, to_block: 50000 + 2, page_size: 1 },
				expected: false,
				message: "the difference between from_block and to_block should be less than MAX_BLOCK_RANGE".to_string(),
			},
		];

		for tc in test_cases {
			assert_eq!(tc.expected, tc.input.validate(), "{}", tc.message);
		}
	}
}
