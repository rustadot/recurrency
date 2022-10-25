use super::{mock::*, Event as MessageEvent};
use crate::{BlockMessages, Config, Error, Message, Messages};
use codec::Encode;
use common_primitives::{messages::MessageResponse, schema::*};
use frame_support::{
	assert_err, assert_noop, assert_ok,
	weights::{Pays, PostDispatchInfo},
	BoundedVec,
};
use sp_std::vec::Vec;

/// Populate mocked Messages storage with message data.
///
/// # Arguments
/// * `schema_id` - Registered schema id to which stored messages should adhere
/// * `message_per_block` - A signed transaction origin from the provider
/// * `payload_location` - Determines how a message payload is encoded. PayloadLocation::IPFS
/// 		will encode (mock CID, IPFS_PAYLOAD_LENGTH) on the message payload.
fn populate_messages(
	schema_id: SchemaId,
	message_per_block: Vec<u32>,
	payload_location: PayloadLocation,
) {
	let payload = match payload_location {
		PayloadLocation::OnChain =>
			Vec::from("{'fromId': 123, 'content': '232323114432'}".as_bytes()),
		PayloadLocation::IPFS => (
			Vec::from("bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq".as_bytes()),
			IPFS_PAYLOAD_LENGTH,
		)
			.encode(),
	};

	let mut counter = 0;
	for (idx, count) in message_per_block.iter().enumerate() {
		let mut list = BoundedVec::default();
		for _ in 0..*count {
			list.try_push(Message {
				msa_id: Some(10),
				payload: payload.clone().try_into().unwrap(),
				index: counter,
				provider_msa_id: 1,
			})
			.unwrap();
			counter += 1;
		}
		Messages::<Test>::insert(idx as u64, schema_id, list);
	}
}

#[test]
fn add_message_should_store_message_on_temp_storage() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = 5;
		let caller_2 = 2;
		let schema_id_1: SchemaId = 1;
		let schema_id_2: SchemaId = 2;
		let message_payload_1 = Vec::from("{'fromId': 123, 'content': '232323114432'}".as_bytes());
		let message_payload_2 = Vec::from("{'fromId': 343, 'content': '34333'}".as_bytes());

		// act
		assert_ok!(MessagesPallet::add_onchain_message(
			Origin::signed(caller_1),
			None,
			schema_id_1,
			message_payload_1.clone()
		));
		assert_ok!(MessagesPallet::add_onchain_message(
			Origin::signed(caller_2),
			None,
			schema_id_2,
			message_payload_2.clone()
		));

		// assert
		let list = BlockMessages::<Test>::get().into_inner();
		assert_eq!(list.len(), 2);

		assert_eq!(
			list[0],
			(
				Message {
					msa_id: Some(get_msa_from_account(caller_1)),
					payload: message_payload_1.try_into().unwrap(),
					index: 0,
					provider_msa_id: get_msa_from_account(caller_1)
				},
				schema_id_1
			)
		);

		assert_eq!(
			list[1],
			(
				Message {
					msa_id: Some(get_msa_from_account(caller_2)),
					payload: message_payload_2.try_into().unwrap(),
					index: 1,
					provider_msa_id: get_msa_from_account(caller_2)
				},
				schema_id_2
			)
		);
	});
}

#[test]
fn add_message_with_too_large_message_should_panic() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = 5;
		let schema_id_1: SchemaId = 1;
		let message_payload_1 = Vec::from("{'fromId': 123, 'content': '232323114432'}{'fromId': 123, 'content': '232323114432'}{'fromId': 123, 'content': '232323114432'}".as_bytes());

		// act
		assert_noop!(MessagesPallet::add_onchain_message(Origin::signed(caller_1), None, schema_id_1, message_payload_1), Error::<Test>::ExceedsMaxMessagePayloadSizeBytes);
	});
}

#[test]
fn add_message_with_invalid_msa_account_should_panic() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = 1000;
		let schema_id_1: SchemaId = 1;
		let message_payload_1 = Vec::from(
			"{'fromId': 123, 'content': '232323114432'}{'fromId': 123, 'content': '232323114432'}"
				.as_bytes(),
		);

		// act
		assert_noop!(
			MessagesPallet::add_onchain_message(
				Origin::signed(caller_1),
				None,
				schema_id_1,
				message_payload_1
			),
			Error::<Test>::InvalidMessageSourceAccount
		);
	});
}

#[test]
fn add_message_with_maxed_out_storage_should_panic() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = 5;
		let schema_id_1: SchemaId = 1;
		let message_payload_1 = Vec::from("{'fromId': 123, 'content': '232323114432'}".as_bytes());

		// act
		for _ in 0..<Test as Config>::MaxMessagesPerBlock::get() {
			assert_ok!(MessagesPallet::add_onchain_message(
				Origin::signed(caller_1),
				None,
				schema_id_1,
				message_payload_1.clone()
			));
		}
		assert_noop!(
			MessagesPallet::add_onchain_message(
				Origin::signed(caller_1),
				None,
				schema_id_1,
				message_payload_1
			),
			Error::<Test>::TooManyMessagesInBlock
		);
	});
}

#[test]
fn on_initialize_should_add_messages_into_storage_and_clean_temp() {
	new_test_ext().execute_with(|| {
		// arrange
		let current_block = 1;
		let caller_1 = 5;
		let caller_2 = 2;
		let schema_id_1: SchemaId = 1;
		let schema_id_2: SchemaId = 2;
		let message_payload_1 = Vec::from("{'fromId': 123, 'content': '232323114432'}".as_bytes());
		let message_payload_2 = Vec::from("{'fromId': 343, 'content': '34333'}".as_bytes());
		assert_ok!(MessagesPallet::add_onchain_message(
			Origin::signed(caller_1),
			None,
			schema_id_1,
			message_payload_1.clone()
		));
		assert_ok!(MessagesPallet::add_onchain_message(
			Origin::signed(caller_2),
			None,
			schema_id_1,
			message_payload_1
		));
		assert_ok!(MessagesPallet::add_onchain_message(
			Origin::signed(caller_2),
			None,
			schema_id_2,
			message_payload_2
		));

		// act
		run_to_block(current_block + 1);

		// assert
		assert_eq!(BlockMessages::<Test>::get().len(), 0);

		let list_1 = MessagesPallet::get_messages_by_schema_and_block(
			schema_id_1,
			PayloadLocation::OnChain,
			current_block,
		);
		assert_eq!(list_1.len(), 2);
		let list_2 = MessagesPallet::get_messages_by_schema_and_block(
			schema_id_2,
			PayloadLocation::OnChain,
			current_block,
		);
		assert_eq!(list_2.len(), 1);

		let events_occured = System::events();
		assert_eq!(events_occured.len(), 2);

		assert_eq!(
			events_occured[0].event,
			Event::MessagesPallet(MessageEvent::MessagesStored {
				block_number: current_block,
				schema_id: schema_id_1,
				count: 2
			}),
		);

		assert_eq!(
			events_occured[1].event,
			Event::MessagesPallet(MessageEvent::MessagesStored {
				block_number: current_block,
				schema_id: schema_id_2,
				count: 1
			}),
		);
	});
}

/// Assert that MessageResponse for IPFS messages returns the payload_length of the offchain message.
#[test]
fn get_messages_by_schema_with_ipfs_payload_location_should_return_offchain_payload_length() {
	new_test_ext().execute_with(|| {
		// Setup
		let schema_id: SchemaId = IPFS_SCHEMA_ID;
		let current_block = 1;

		// Populate
		populate_messages(schema_id, vec![1], PayloadLocation::IPFS);

		// Run to the block +
		run_to_block(current_block + 1);

		let list =
			MessagesPallet::get_messages_by_schema_and_block(schema_id, PayloadLocation::IPFS, 0);

		let cid =
			Vec::from("bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq".as_bytes());

		// IPFS messages should return the payload length that was encoded in a tuple along
		// with the CID: (cid, payload_length).
		assert_eq!(list.len(), 1);
		assert_eq!(
			list[0],
			MessageResponse {
				payload: None,
				index: 0,
				provider_msa_id: 1,
				block_number: 0,
				payload_length: Some(IPFS_PAYLOAD_LENGTH),
				msa_id: None,
				cid: Some(cid)
			}
		);
	});
}

#[test]
fn get_messages_by_schema_with_ipfs_payload_location_should_fail_bad_schema() {
	new_test_ext().execute_with(|| {
		let bad_message: Message<MaxSchemaGrantsPerDelegation> = Message {
			payload: BoundedVec::try_from(
				vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].to_vec(),
			)
			.unwrap(),
			msa_id: Some(0),
			provider_msa_id: 1,
			index: 0,
		};
		let mapped_response = bad_message.map_to_response(0, PayloadLocation::IPFS);
		assert_eq!(mapped_response.cid, Some(Vec::new()));
	});
}

#[test]
fn add_message_via_valid_delegate_should_pass() {
	new_test_ext().execute_with(|| {
		// arrange
		let message_producer = 1;
		let caller_1 = 5;
		let caller_2 = 2;
		let schema_id_1: SchemaId = 1;
		let schema_id_2: SchemaId = 2;
		let message_payload_1 = Vec::from("{'fromId': 123, 'content': '232323114432'}".as_bytes());
		let message_payload_2 = Vec::from("{'fromId': 343, 'content': '34333'}".as_bytes());

		// act
		assert_ok!(MessagesPallet::add_onchain_message(
			Origin::signed(caller_1),
			Some(message_producer),
			schema_id_1,
			message_payload_1.clone()
		));
		assert_ok!(MessagesPallet::add_onchain_message(
			Origin::signed(caller_2),
			Some(message_producer),
			schema_id_2,
			message_payload_2.clone()
		));

		// assert
		let list = BlockMessages::<Test>::get().into_inner();
		assert_eq!(list.len(), 2);

		assert_eq!(
			list[0],
			(
				Message {
					msa_id: Some(message_producer),
					payload: message_payload_1.try_into().unwrap(),
					index: 0,
					provider_msa_id: get_msa_from_account(caller_1)
				},
				schema_id_1
			)
		);

		assert_eq!(
			list[1],
			(
				Message {
					msa_id: Some(message_producer),
					payload: message_payload_2.try_into().unwrap(),
					index: 1,
					provider_msa_id: get_msa_from_account(caller_2)
				},
				schema_id_2
			)
		);
	});
}

#[test]
fn add_message_via_non_delegate_should_fail() {
	new_test_ext().execute_with(|| {
		// arrange
		let message_producer = 1;
		let message_provider = 2000;
		let schema_id_1: SchemaId = 1;
		let message_payload_1 = Vec::from("{'fromId': 123, 'content': '232323114432'}".as_bytes());
		// act
		assert_err!(
			MessagesPallet::add_onchain_message(
				Origin::signed(message_provider),
				Some(message_producer),
				schema_id_1,
				message_payload_1
			),
			Error::<Test>::UnAuthorizedDelegate
		);

		// assert
		let list = BlockMessages::<Test>::get().into_inner();
		assert_eq!(list.len(), 0);
	});
}

#[test]
fn add_message_with_invalid_schema_id_should_error() {
	new_test_ext().execute_with(|| {
		// arrange
		let caller_1 = 5;
		let schema_id_1: SchemaId = INVALID_SCHEMA_ID;
		let message_payload_1 = Vec::from(
			"{'fromId': 123, 'content': '232323114432'}{'fromId': 123, 'content': '232323114432'}"
				.as_bytes(),
		);

		// act
		assert_err!(
			MessagesPallet::add_onchain_message(
				Origin::signed(caller_1),
				None,
				schema_id_1,
				message_payload_1
			),
			Error::<Test>::InvalidSchemaId
		);
	});
}

#[test]
fn valid_payload_location() {
	new_test_ext().execute_with(|| {
		let caller_1 = 5;
		let schema_id_1: SchemaId = IPFS_SCHEMA_ID;
		let info_result = MessagesPallet::add_ipfs_message(
			Origin::signed(caller_1),
			schema_id_1,
			Vec::from("foo"),
			1,
		);

		assert_eq!(info_result.is_ok(), true);
		let info: PostDispatchInfo = info_result.unwrap();

		assert_eq!(info.actual_weight.is_some(), true);
		assert_eq!(info.pays_fee, Pays::Yes);
	});
}

#[test]
fn invalid_payload_location_ipfs() {
	new_test_ext().execute_with(|| {
		let caller_1 = 5;
		let schema_id_1: SchemaId = 1;

		assert_noop!(
			MessagesPallet::add_ipfs_message(
				Origin::signed(caller_1),
				schema_id_1,
				Vec::from("foo"),
				1
			),
			Error::<Test>::InvalidPayloadLocation
		);
	});
}

#[test]
fn invalid_payload_location_onchain() {
	new_test_ext().execute_with(|| {
		let caller_1 = 5;
		let payload: Vec<u8> = Vec::from("foo");

		assert_noop!(
			MessagesPallet::add_onchain_message(
				Origin::signed(caller_1),
				None,
				IPFS_SCHEMA_ID,
				payload,
			),
			Error::<Test>::InvalidPayloadLocation
		);
	});
}
