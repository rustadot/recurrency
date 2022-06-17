//! # Messages pallet
//! A pallet for storing messages.
//!
//! This pallet contains functionality for storing, retrieving and eventually removing messages for
//! registered schemas on chain.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Messages Pallet provides functions for:
//!
//! - Adding a message for given schema.
//! - Retrieving messages for a given schema.
//!
//! ### Terminology
//!
//! - **Message:** A message that matches a registered `Schema` (on-chain or off-chain).
//! - **Payload:** The user data in a `Message` that matches a `Schema`.
//! - **MSA Id:** The 64 bit unsigned integer associated with an `Message Source Account`.
//! - **MSA:** Message Source Account. A registered identifier with the MSA pallet.
//! - **Schema:** A registered data structure and the settings around it.
//! - **Schema Id:** A U16 bit identifier for a schema stored on-chain.
//!
//!
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// Strong Documentation Lints
#![deny(
	rustdoc::broken_intra_doc_links,
	rustdoc::missing_crate_level_docs,
	rustdoc::invalid_codeblock_attributes
)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

mod types;

use frame_support::{ensure, pallet_prelude::Weight, traits::Get, BoundedVec};
use sp_runtime::{traits::One, DispatchError};
use sp_std::{collections::btree_map::BTreeMap, convert::TryInto, prelude::*};

pub use pallet::*;
pub use types::*;
pub use weights::*;

use common_primitives::{messages::*, schema::*};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use common_primitives::msa::{AccountProvider, Delegator, MessageSourceId, Provider};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// A type that will supply account related information.
		type AccountProvider: AccountProvider<AccountId = Self::AccountId>;

		/// The maximum number of messages in a block.
		#[pallet::constant]
		type MaxMessagesPerBlock: Get<u32>;

		/// The maximum size of a message payload bytes.
		#[pallet::constant]
		type MaxMessagePayloadSizeBytes: Get<u32> + Clone;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// A temporary storage of messages, given a schema id, for a duration of block period.
	/// At the start of the next block this storage is cleared and moved into Messages storage.
	#[pallet::storage]
	#[pallet::getter(fn get_block_messages)]
	pub(super) type BlockMessages<T: Config> = StorageValue<
		_,
		BoundedVec<
			(Message<T::AccountId, T::MaxMessagePayloadSizeBytes>, SchemaId),
			T::MaxMessagesPerBlock,
		>,
		ValueQuery,
	>;

	/// A permanent storage for messages mapped by block number and schema id.
	#[pallet::storage]
	#[pallet::getter(fn get_messages)]
	pub(super) type Messages<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::BlockNumber,
		Twox64Concat,
		SchemaId,
		BoundedVec<Message<T::AccountId, T::MaxMessagePayloadSizeBytes>, T::MaxMessagesPerBlock>,
		ValueQuery,
	>;

	#[pallet::error]
	pub enum Error<T> {
		/// Too many messages are added to existing block
		TooManyMessagesInBlock,
		/// Message payload size is too large
		ExceedsMaxMessagePayloadSizeBytes,
		/// Invalid Pagination Request
		InvalidPaginationRequest,
		/// Type Conversion Overflow
		TypeConversionOverflow,
		/// Invalid Message Source Account
		InvalidMessageSourceAccount,
		/// UnAuthorizedDelegate
		UnAuthorizedDelegate,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Messages are stored for a specified schema id and block number
		MessagesStored {
			/// The schema for these messages
			schema_id: SchemaId,
			/// The block number for these messages
			block_number: T::BlockNumber,
			/// Number of messages in this block for this schema
			count: u16,
		},
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(current: T::BlockNumber) -> Weight {
			let prev_block = current - T::BlockNumber::one();
			Self::move_messages_into_final_storage(prev_block)
			// TODO: add retention policy execution
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Gets a messages for a given schema-id and block-number.
		/// # Arguments
		/// * `origin` - A signed transaction origin from the provider
		/// * `on_behalf_of` - Optional. The msa id of delegate.
		/// * `schema_id` - Registered schema id for current message.
		/// * `payload` - Serialized payload data for a given schema.
		/// # Returns
		/// * [DispatchResultWithPostInfo](https://paritytech.github.io/substrate/master/frame_support/dispatch/type.DispatchResultWithPostInfo.html) The return type of a Dispatchable in frame.
		/// When returned explicitly from a dispatchable function it allows overriding the default PostDispatchInfo returned from a dispatch.
		#[pallet::weight(T::WeightInfo::add(payload.len() as u32, 1_000))]
		pub fn add(
			origin: OriginFor<T>,
			on_behalf_of: Option<MessageSourceId>,
			schema_id: SchemaId,
			payload: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let provider_key = ensure_signed(origin)?;

			ensure!(
				payload.len() < T::MaxMessagePayloadSizeBytes::get().try_into().unwrap(),
				Error::<T>::ExceedsMaxMessagePayloadSizeBytes
			);

			let provider = T::AccountProvider::ensure_valid_msa_key(&provider_key)
				.map_err(|_| Error::<T>::InvalidMessageSourceAccount)?;

			let message_source_id = match on_behalf_of {
				Some(delegator_msa_id) => {
					T::AccountProvider::ensure_valid_delegation(
						Provider(provider.msa_id),
						Delegator(delegator_msa_id),
					)
					.map_err(|_| Error::<T>::UnAuthorizedDelegate)?;
					delegator_msa_id
				},
				None => provider.msa_id,
			};

			// TODO: validate schema existence and validity from schema pallet
			<BlockMessages<T>>::try_mutate(|existing_messages| -> DispatchResultWithPostInfo {
				let current_size: u16 = existing_messages
					.len()
					.try_into()
					.map_err(|_| Error::<T>::TypeConversionOverflow)?;
				let payload_size = payload.len();
				let m = Message {
					payload: payload.try_into().unwrap(), // size is checked on top of extrinsic
					provider_key,
					index: current_size,
					msa_id: message_source_id,
				};
				existing_messages
					.try_push((m, schema_id))
					.map_err(|_| Error::<T>::TooManyMessagesInBlock)?;

				Ok(Some(T::WeightInfo::add(payload_size as u32, current_size as u32)).into())
			})
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Gets a messages for a given schema-id and block-number.
	/// # Arguments
	/// * `schema_id` - Registered schema id for current message.
	/// * `pagination` - [`BlockPaginationRequest`]. Request payload to retrieve paginated messages for a given block-range.
	/// # Returns
	/// * `Result<BlockPaginationResponse<T::BlockNumber, MessageResponse<T::AccountId, T::BlockNumber>>, DispatchError>`
	///
	/// Result is a paginator response of type [`BlockPaginationResponse`].
	pub fn get_messages_by_schema(
		schema_id: SchemaId,
		pagination: BlockPaginationRequest<T::BlockNumber>,
	) -> Result<
		BlockPaginationResponse<T::BlockNumber, MessageResponse<T::AccountId, T::BlockNumber>>,
		DispatchError,
	> {
		ensure!(pagination.validate(), Error::<T>::InvalidPaginationRequest);

		let mut response = BlockPaginationResponse::new();
		let from: u32 = pagination
			.from_block
			.try_into()
			.map_err(|_| Error::<T>::TypeConversionOverflow)?;
		let to: u32 =
			pagination.to_block.try_into().map_err(|_| Error::<T>::TypeConversionOverflow)?;
		let mut from_index = pagination.from_index;

		'loops: for bid in from..to {
			let block_number: T::BlockNumber = bid.into();
			let list = <Messages<T>>::get(block_number, schema_id).into_inner();

			let list_size: u32 =
				list.len().try_into().map_err(|_| Error::<T>::TypeConversionOverflow)?;
			for i in from_index..list_size {
				let m = list[i as usize].clone();
				response.content.push(m.map_to_response(block_number));

				if Self::check_end_condition_and_set_next_pagination(
					block_number,
					i,
					list_size,
					&pagination,
					&mut response,
				) {
					break 'loops
				}
			}

			// next block starts from 0
			from_index = 0;
		}

		Ok(response)
	}

	/// Checks the end condition for paginated query and set the `PaginationResponse`
	///
	/// Returns `true` if page is filled
	fn check_end_condition_and_set_next_pagination(
		block_number: T::BlockNumber,
		current_index: u32,
		list_size: u32,
		request: &BlockPaginationRequest<T::BlockNumber>,
		result: &mut BlockPaginationResponse<
			T::BlockNumber,
			MessageResponse<T::AccountId, T::BlockNumber>,
		>,
	) -> bool {
		if result.content.len() as u32 == request.page_size {
			let mut next_block = block_number;
			let mut next_index = current_index + 1;

			// checking if it's end of current list
			if next_index == list_size {
				next_block = block_number + T::BlockNumber::one();
				next_index = 0;
			}

			if next_block < request.to_block {
				result.has_next = true;
				result.next_block = Some(next_block);
				result.next_index = Some(next_index);
			}
			return true
		}

		false
	}

	/// Moves messages from temporary storage `BlockMessages` into final storage `Messages`
	/// and calculates execution weight
	///
	/// * `block_number`: Target Block Number
	///
	/// Returns execution weights
	fn move_messages_into_final_storage(block_number: T::BlockNumber) -> Weight {
		let mut map = BTreeMap::new();
		let block_messages = BlockMessages::<T>::get();
		let message_count = block_messages.len() as u32;
		let mut schema_count = 0u32;

		if message_count == 0 {
			return T::DbWeight::get().reads(1)
		}

		// grouping messages by schema_id
		for (m, schema_id) in block_messages {
			let list = map.entry(schema_id).or_insert(vec![]);
			list.push(m);
		}

		// insert into storage and create events
		for (schema_id, messages) in map {
			let count = messages.len() as u16;
			let bounded_vec: BoundedVec<_, _> = messages.try_into().unwrap();
			Messages::<T>::insert(&block_number, schema_id, &bounded_vec);
			Self::deposit_event(Event::MessagesStored { schema_id, block_number, count });
			schema_count += 1;
		}

		BlockMessages::<T>::set(BoundedVec::default());
		T::WeightInfo::on_initialize(message_count, schema_count)
	}
}
