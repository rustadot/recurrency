//! # MSA Pallet
//!
//! The MSA pallet provides functionality for handling Message Source Accounts.
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The MSA pallet provides functions for:
//!
//! - Creating, reading, updating, and deleting operations for MSAs.
//! - Managing delegation relationships for MSAs.
//! - Managing keys associated with MSA.
//!
//! ### Terminology
//! * **MSA** - Message Source Account.  A Source or Provider Account for Frequency Messages. It may or may not have `Capacity` token.  It must have at least one `AccountId` (public key) associated with it.
//! Created by generating a new MSA ID number and associating it with a Substrate `AccountID`.
//! An MSA is required for sending Capacity-based messages and for creating Delegations.
//! * **MSA ID** - This is the ID number created for a new Message Source Account and associated with a Substrate `AccountId`.
//! * **Delegator** - a Message Source Account that has provably delegated certain actions to a Provider, typically sending a `Message`
//! * **Provider** - the actor that a Delegator has delegated specific actions to.
//! * **Delegation** - A stored Delegator-Provider association between MSAs which permits the Provider to perform specific actions on the Delegator's behalf.
//!
//! ### Implementations
//!
//! The MSA pallet implements the following traits:
//!
//! - [`AccountProvider`](common_primitives::msa::AccountProvider): Functions for accessing and validating MSAs.  This implementation is what is used in the runtime.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! - `add_key_to_msa` - Associates a key to an MSA ID in a signed payload.
//! - `add_provider_to_msa` - Creates a delegation relationship between a `Provider` and MSA.
//! - `create` - Creates an MSA for the `Origin`.
//! - `create_sponsored_account_with_delegation` - `Origin` creates an account for a given `AccountId` and sets themselves as a `Provider`.
//! - `revoke_delegation_by_provider` - `Provider` MSA terminates a Delegation with Delegator MSA by expiring it.
//! - `revoke_msa_delegation_by_delegator` - Delegator MSA terminates a Delegation with the `Provider` MSA by expiring it.
//! - `delete_msa_key` - Removes the given key by from storage against respective MSA.
//!
//! ### Assumptions
//!
//! * Total MSA keys should be less than the constant `Config::MSA::MaxKeys`.
//! * Maximum schemas, for which provider has publishing rights, be less than `Config::MSA::MaxSchemaGrants`
//!

#![cfg_attr(not(feature = "std"), no_std)]
// Strong Documentation Lints
#![deny(
	rustdoc::broken_intra_doc_links,
	rustdoc::missing_crate_level_docs,
	rustdoc::invalid_codeblock_attributes,
	missing_docs
)]

use codec::{Decode, Encode};
use common_primitives::{
	msa::{
		AccountProvider, Delegator, OrderedSetExt, Provider, ProviderInfo, ProviderMetadata,
		EXPIRATION_BLOCK_VALIDITY_GAP,
	},
	node::BlockNumber,
	schema::SchemaId,
};
use frame_support::{dispatch::DispatchResult, ensure, traits::IsSubType, weights::DispatchInfo};
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{Convert, DispatchInfoOf, Dispatchable, SignedExtension, Verify, Zero},
	DispatchError, MultiSignature,
};

use sp_core::crypto::AccountId32;
pub mod types;
pub use types::{AddKeyData, AddProvider};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub mod weights;

pub use weights::*;

pub use common_primitives::{msa::MessageSourceId, utils::wrap_binary_data};

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// AccountId truncated to 32 bytes
		type ConvertIntoAccountId32: Convert<Self::AccountId, AccountId32>;

		/// Maximum count of keys allowed per MSA
		#[pallet::constant]
		type MaxKeys: Get<u8>;

		/// Maximum count of schemas granted for publishing data per Provider
		#[pallet::constant]
		type MaxSchemaGrants: Get<u32> + Clone + sp_std::fmt::Debug + Eq;
		/// Maximum provider name size allowed per MSA association
		#[pallet::constant]
		type MaxProviderNameSize: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Storage type for MSA identifier
	/// - Value: The current maximum MSA Id
	#[pallet::storage]
	#[pallet::getter(fn get_identifier)]
	pub type MsaIdentifier<T> = StorageValue<_, MessageSourceId, ValueQuery>;

	/// Storage type for mapping the relationship between a Delegator and its Provider.
	/// - Keys: Delegator MSA, Provider MSA
	/// - Value: [`ProviderInfo`](common_primitives::msa::ProviderInfo)
	#[pallet::storage]
	#[pallet::getter(fn get_provider_info)]
	pub type ProviderInfoOf<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		Delegator,
		Twox64Concat,
		Provider,
		ProviderInfo<T::BlockNumber, T::MaxSchemaGrants>,
		OptionQuery,
	>;

	/// Provider registration information
	/// - Key: Provider MSA Id
	/// - Value: [`ProviderMetadata`](common_primitives::msa::ProviderMetadata)
	#[pallet::storage]
	#[pallet::getter(fn get_provider_metadata)]
	pub type ProviderRegistry<T: Config> = StorageMap<
		_,
		Twox64Concat,
		Provider,
		ProviderMetadata<T::MaxProviderNameSize>,
		OptionQuery,
	>;

	/// Storage type for key to MSA information
	/// - Key: AccountId
	/// - Value: [`MessageSourceId`]
	#[pallet::storage]
	#[pallet::getter(fn get_msa_by_account_id)]
	pub type MessageSourceIdOf<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, MessageSourceId, OptionQuery>;

	/// Storage type for a reference counter of the number of keys assocaited to an MSA
	/// - Key: MSA Id
	/// - Value: [`u8`] Counter of Keys associated with the MSA
	#[pallet::storage]
	#[pallet::getter(fn get_msa_key_count)]
	pub(super) type MsaInfoOf<T: Config> =
		StorageMap<_, Twox64Concat, MessageSourceId, u8, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new Message Service Account was created with a new MessageSourceId
		MsaCreated {
			/// The MSA for the Event
			msa_id: MessageSourceId,
			/// The key added to the MSA
			key: T::AccountId,
		},
		/// An AccountId has been associated with a MessageSourceId
		KeyAdded {
			/// The MSA for the Event
			msa_id: MessageSourceId,
			/// The key added to the MSA
			key: T::AccountId,
		},
		/// An AccountId had all permissions revoked from its MessageSourceId
		KeyRemoved {
			/// The key no longer approved for the associated MSA
			key: T::AccountId,
		},
		/// A delegation relationship was added with the given provider and delegator
		ProviderAdded {
			/// The Provider MSA Id
			provider: Provider,
			/// The Delegator MSA Id
			delegator: Delegator,
		},
		/// A Provider-MSA relationship was registered
		ProviderRegistered {
			/// The MSA id associated with the provider
			provider_msa_id: MessageSourceId,
		},
		/// The Delegator revoked its delegation to the Provider
		DelegatorRevokedDelegation {
			/// The Provider MSA Id
			provider: Provider,
			/// The Delegator MSA Id
			delegator: Delegator,
		},
		/// The Provider revoked itself as delegate for the Delegator
		ProviderRevokedDelegation {
			/// The Provider MSA Id
			provider: Provider,
			/// The Delegator MSA Id
			delegator: Delegator,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Tried to add a key that was already registered to an MSA
		KeyAlreadyRegistered,
		/// MsaId values have reached the maximum
		MsaIdOverflow,
		/// Cryptographic signature verification failed for adding a key to MSA
		AddKeySignatureVerificationFailed,
		/// Ony the MSA Owner may perform the operation
		NotMsaOwner,
		/// Cryptographic signature failed verification
		InvalidSignature,
		/// Only the KeyOwner may perform the operation
		NotKeyOwner,
		/// An operation was attempted with an unknown Key
		NoKeyExists,
		/// The number of key values has reached its maximum
		KeyLimitExceeded,
		/// A transaction's Origin (AccountId) may not remove itself
		InvalidSelfRemoval,
		/// An MSA may not be its own delegate
		InvalidSelfProvider,
		/// The delegation relationship already exists for the given MSA Ids
		DuplicateProvider,
		/// Cryptographic signature verification failed for adding the Provider as delegate
		AddProviderSignatureVerificationFailed,
		/// Origin attempted to add a delegate for someone else's MSA
		UnauthorizedDelegator,
		/// Origin attempted to add a different delegate than what was in the payload
		UnauthorizedProvider,
		/// The operation was attempted with a revoked delegation
		DelegationRevoked,
		/// The operation was attempted with an unknown delegation
		DelegationNotFound,
		/// The operation was attempted with an expired delegation
		DelegationExpired,
		/// The MSA id submitted for provider creation has already been associated with a provider
		DuplicateProviderMetadata,
		/// The maximum length for a provider name has been exceeded
		ExceedsMaxProviderNameSize,
		/// The maximum number of schema grants has been exceeded
		ExceedsMaxSchemaGrants,
		/// Provider is not permitted to publish for given schema_id
		SchemaNotGranted,
		/// The operation was attempted with a non-provider MSA
		ProviderNotRegistered,
		/// The submited proof has expired; the current block is less the expiration block
		ProofHasExpired,
		/// The submitted proof expiration block is too far in the future
		ProofNotYetValid,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates an MSA for the Origin (sender of the transaction).  Origin is assigned an MSA ID.
		/// Deposits [`MsaCreated`](Event::MsaCreated) event, and returns `Ok(())` on success, otherwise returns an error.
		///
		/// ### Errors
		///
		/// - Returns [`KeyLimitExceeded`](Error::KeyLimitExceeded) if MSA has registered `MaxKeys`.
		/// - Returns [`KeyAlreadyRegistered`](Error::KeyAlreadyRegistered) if MSA is already registered to the Origin.
		///
		#[pallet::weight(T::WeightInfo::create(10_000))]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let (_, _) = Self::create_account(who.clone(), |new_msa_id| -> DispatchResult {
				Self::deposit_event(Event::MsaCreated { msa_id: new_msa_id, key: who });
				Ok(())
			})?;

			Ok(())
		}

		/// `Origin` MSA creates an MSA on behalf of `delegator_key`, creates a Delegation with the `delegator_key`'s MSA as the Delegator and `origin` as `Provider`. Deposits events [`MsaCreated`](Event::MsaCreated) and [`ProviderAdded`](Event::ProviderAdded).
		/// Returns `Ok(())` on success, otherwise returns an error.
		///
		/// ## Errors
		///
		/// - Returns [`UnauthorizedProvider`](Error::UnauthorizedProvider) if payload's MSA does not match given provider MSA.
		/// - Returns [`InvalidSignature`](Error::InvalidSignature) if `proof` verification fails; `delegator_key` must have signed `add_provider_payload`
		/// - Returns [`NoKeyExists`](Error::NoKeyExists) if there is no MSA for `origin`.
		/// - Returns [`KeyAlreadyRegistered`](Error::KeyAlreadyRegistered) if there is already an MSA for `delegator_key`.
		/// - Returns [`ProviderNotRegistered`](Error::ProviderNotRegistered) if the a non-provider MSA is used as the provider
		///
		#[pallet::weight(T::WeightInfo::create_sponsored_account_with_delegation())]
		pub fn create_sponsored_account_with_delegation(
			origin: OriginFor<T>,
			delegator_key: T::AccountId,
			proof: MultiSignature,
			add_provider_payload: AddProvider,
		) -> DispatchResult {
			let provider_key = ensure_signed(origin)?;

			Self::verify_signature(proof, delegator_key.clone(), add_provider_payload.encode())?;

			let provider_msa_id = Self::ensure_valid_msa_key(&provider_key)?;
			ensure!(
				add_provider_payload.authorized_msa_id == provider_msa_id,
				Error::<T>::UnauthorizedProvider
			);

			// Verify that the provider is a registered provider
			ensure!(
				Self::is_registered_provider(provider_msa_id),
				Error::<T>::ProviderNotRegistered
			);

			let granted_schemas = add_provider_payload.schema_ids;
			ensure!(
				granted_schemas.len() <= T::MaxSchemaGrants::get().try_into().unwrap(),
				Error::<T>::ExceedsMaxSchemaGrants
			);
			let (_, _) =
				Self::create_account(delegator_key.clone(), |new_msa_id| -> DispatchResult {
					Self::add_provider(provider_msa_id.into(), new_msa_id.into(), granted_schemas)?;

					Self::deposit_event(Event::MsaCreated {
						msa_id: new_msa_id,
						key: delegator_key.clone(),
					});

					Self::deposit_event(Event::ProviderAdded {
						delegator: new_msa_id.into(),
						provider: provider_msa_id.into(),
					});
					Ok(())
				})?;

			Ok(())
		}

		/// Adds an association between MSA id and ProviderMetadata. As of now, the
		/// only piece of metadata we are recording is provider name.
		///
		/// ## Errors
		/// - Returns
		///   [`DuplicateProviderMetadata`](Error::DuplicateProviderMetadata) if there is already a ProviderMetadata associated with the given MSA id.
		#[pallet::weight(T::WeightInfo::register_provider())]
		pub fn register_provider(origin: OriginFor<T>, provider_name: Vec<u8>) -> DispatchResult {
			let provider_key = ensure_signed(origin)?;
			let bounded_name: BoundedVec<u8, T::MaxProviderNameSize> =
				provider_name.try_into().map_err(|_| Error::<T>::ExceedsMaxProviderNameSize)?;

			let provider_msa_id = Self::ensure_valid_msa_key(&provider_key)?;
			ProviderRegistry::<T>::try_mutate(
				Provider(provider_msa_id),
				|maybe_metadata| -> DispatchResult {
					ensure!(maybe_metadata.take().is_none(), Error::<T>::DuplicateProviderMetadata);
					*maybe_metadata = Some(ProviderMetadata { provider_name: bounded_name });
					Ok(())
				},
			)?;
			Self::deposit_event(Event::ProviderRegistered { provider_msa_id });
			Ok(())
		}

		/// Creates a new Delegation for an existing MSA, with `origin` as the Provider and `delegator_key` is the delegator.
		/// Since it is being sent on the Delegator's behalf, it requires the Delegator to authorize the new Delegation.
		/// Returns `Ok(())` on success, otherwise returns an error. Deposits event [`ProviderAdded`](Event::ProviderAdded).
		///
		/// ## Errors
		/// - Returns [`AddProviderSignatureVerificationFailed`](Error::AddProviderSignatureVerificationFailed) if `origin`'s MSA ID does not equal `add_provider_payload.authorized_msa_id`.
		/// - Returns [`DuplicateProvider`](Error::DuplicateProvider) if there is already a Delegation for `origin` MSA and `delegator_key` MSA.
		/// ## Errors
		/// - Returns [`UnauthorizedProvider`](Error::UnauthorizedProvider) if `add_provider_payload.authorized_msa_id`  does not match MSA ID of `delegator_key`.
		/// - Returns [`InvalidSignature`](Error::InvalidSignature) if `proof` verification fails; `delegator_key` must have signed `add_provider_payload`
		/// - Returns [`NoKeyExists`](Error::NoKeyExists) if there is no MSA for `origin`.
		/// - Returns [`ProviderNotRegistered`](Error::ProviderNotRegistered) if the a non-provider MSA is used as the provider
		/// - Returns [`UnauthorizedDelegator`](Error::UnauthorizedDelegator) if Origin attempted to add a delegate for someone else's MSA
		#[pallet::weight(T::WeightInfo::add_provider_to_msa())]
		pub fn add_provider_to_msa(
			origin: OriginFor<T>,
			delegator_key: T::AccountId,
			proof: MultiSignature,
			add_provider_payload: AddProvider,
		) -> DispatchResult {
			let provider_key = ensure_signed(origin)?;

			// delegator must have signed the payload.
			Self::verify_signature(proof, delegator_key.clone(), add_provider_payload.encode())
				.map_err(|_| Error::<T>::AddProviderSignatureVerificationFailed)?;

			let (provider, delegator) =
				Self::ensure_valid_registered_provider(&delegator_key, &provider_key)?;

			ensure!(
				add_provider_payload.authorized_msa_id == provider.0,
				Error::<T>::UnauthorizedDelegator
			);

			let granted_schemas = add_provider_payload.schema_ids;
			ensure!(
				granted_schemas.len() <= T::MaxSchemaGrants::get().try_into().unwrap(),
				Error::<T>::ExceedsMaxSchemaGrants
			);

			Self::add_provider(provider, delegator, granted_schemas)?;

			Self::deposit_event(Event::ProviderAdded { delegator, provider });

			Ok(())
		}

		/// Delegator (Origin) MSA terminates a delegation relationship with the `Provider` MSA. Deposits event[`DelegatorRevokedDelegation`](Event::DelegatorRevokedDelegation).
		/// Returns `Ok(())` on success, otherwise returns an error.
		///
		/// ### Errors
		///
		/// - Returns [`DelegationRevoked`](Error::DelegationRevoked) if the delegation has already been revoked.
		/// - Returns [`DelegationNotFound`](Error::DelegationNotFound) if there is not delegation relationship between Origin and Delegator or Origin and Delegator are the same.
		/// - May also return []
		///
		#[pallet::weight((T::WeightInfo::revoke_msa_delegation_by_delegator(), DispatchClass::Normal, Pays::No))]
		pub fn revoke_msa_delegation_by_delegator(
			origin: OriginFor<T>,
			provider_msa_id: MessageSourceId,
		) -> DispatchResult {
			let delegator_key = ensure_signed(origin)?;

			let delegator_msa_id: Delegator = Self::ensure_valid_msa_key(&delegator_key)?.into();
			let provider_msa_id = Provider(provider_msa_id);

			Self::revoke_provider(provider_msa_id, delegator_msa_id)?;

			Self::deposit_event(Event::DelegatorRevokedDelegation {
				delegator: delegator_msa_id,
				provider: provider_msa_id,
			});

			Ok(())
		}

		/// Adds a given key to Origin's MSA, which must match the MSA in `add_key_payload`. Deposits event [`KeyAdded'](Event::KeyAdded).
		/// Returns `Ok(())` on success, otherwise returns an error.
		///
		/// ### Errors
		///
		/// - Returns [`AddKeySignatureVerificationFailed`](Error::AddKeySignatureVerificationFailed) if `key` is not a valid signer of the provided `add_key_payload`.
		/// - Returns [`NoKeyExists`](Error::NoKeyExists) if the MSA id for the account in `add_key_payload` does not exist.
		/// - Returns ['NotMsaOwner'](Error::NotMsaOwner) if Origin's MSA is not the same as 'add_key_payload` MSA. Essentially you can only add a key to your own MSA.
		/// - Returns ['ProofHasExpired'](Error::ProofHasExpired) if the current block is less than the `expired` bock number set in `AddKeyData`.
		/// - Returns ['ProofNotYetValid'](Error::ProofNotYetValid) if the `expired` bock number set in `AddKeyData` is greater than the current block number plus EXPIRATION_BLOCK_VALIDITY_GAP.
		#[pallet::weight(T::WeightInfo::add_key_to_msa())]
		pub fn add_key_to_msa(
			origin: OriginFor<T>,
			key: T::AccountId,
			proof: MultiSignature,
			add_key_payload: AddKeyData,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::verify_signature(proof, key.clone(), add_key_payload.encode())
				.map_err(|_| Error::<T>::AddKeySignatureVerificationFailed)?;

			Self::ensure_block_is_valid(add_key_payload.expiration)?;

			let msa_id = add_key_payload.msa_id;

			Self::ensure_msa_owner(&who, msa_id)?;

			Self::add_key(msa_id, &key.clone(), |new_msa_id| -> DispatchResult {
				Self::deposit_event(Event::KeyAdded { msa_id: new_msa_id, key });
				Ok(())
			})?;

			Ok(())
		}

		/// Remove a key associated with an MSA by expiring it at the current block.
		/// Returns `Ok(())` on success, otherwise returns an error. Deposits event [`KeyRemoved`](Event::KeyRemoved).
		///
		/// ### Errors
		/// - Returns [`InvalidSelfRemoval`](Error::InvalidSelfRemoval) if `origin` and `key` are the same.
		/// - Returns [`NotKeyOwner`](Error::NotKeyOwner) if `origin` does not own the MSA ID associated with `key`.
		/// - Returns [`NotKeyExists`](Error::NoKeyExists) if `origin` or `key` are not associated with `origin`'s MSA ID.
		///
		/// ### Remarks
		/// - Removal of key deletes the association of the key with the MSA.
		/// - The key can be re-added to same or another MSA if needed.
		#[pallet::weight((T::WeightInfo::delete_msa_key(), DispatchClass::Normal, Pays::No))]
		pub fn delete_msa_key(origin: OriginFor<T>, key: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(who != key, Error::<T>::InvalidSelfRemoval);

			let who = Self::try_get_msa_from_account_id(&who)?;
			let account_msa_id = Self::try_get_msa_from_account_id(&key)?;
			ensure!(who == account_msa_id, Error::<T>::NotKeyOwner);

			Self::delete_key_for_msa(who, &key)?;

			Self::deposit_event(Event::KeyRemoved { key });

			Ok(())
		}

		/// Provider MSA terminates Delegation with a Delegator MSA by expiring the Delegation at the current block.
		/// Returns `Ok(())` on success, otherwise returns an error. Deposits events [`ProviderRevokedDelegation`](Event::ProviderRevokedDelegation).
		///
		/// ### Errors
		///
		/// - Returns [`NoKeyExists`](Error::NoKeyExists) if `provider_key` does not have an MSA key.
		/// - Returns [`DelegationNotFound`](Error::DelegationNotFound) if there is no Delegation between origin MSA and provider MSA.
		///
		#[pallet::weight((T::WeightInfo::revoke_delegation_by_provider(20_000), DispatchClass::Normal, Pays::No))]
		pub fn revoke_delegation_by_provider(
			origin: OriginFor<T>,
			delegator: MessageSourceId,
		) -> DispatchResult {
			let provider_key = ensure_signed(origin)?;

			// Remover should have valid keys (non expired and exists)
			let key_info = Self::ensure_valid_msa_key(&provider_key)?;

			let provider_msa_id = Provider(key_info);
			let delegator_msa_id = Delegator(delegator);

			Self::revoke_provider(provider_msa_id, delegator_msa_id)?;

			Self::deposit_event(Event::ProviderRevokedDelegation {
				provider: provider_msa_id,
				delegator: delegator_msa_id,
			});

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Create the account for the `key`
	pub fn create_account<F>(
		key: T::AccountId,
		on_success: F,
	) -> Result<(MessageSourceId, T::AccountId), DispatchError>
	where
		F: FnOnce(MessageSourceId) -> DispatchResult,
	{
		let next_msa_id = Self::get_next_msa_id()?;
		Self::add_key(next_msa_id, &key, on_success)?;
		let _ = Self::set_msa_identifier(next_msa_id);

		Ok((next_msa_id, key))
	}

	/// Generate the next MSA Id
	pub fn get_next_msa_id() -> Result<MessageSourceId, DispatchError> {
		let next = Self::get_identifier().checked_add(1).ok_or(Error::<T>::MsaIdOverflow)?;

		Ok(next)
	}

	/// Set the current identifier in storage
	pub fn set_msa_identifier(identifier: MessageSourceId) -> DispatchResult {
		MsaIdentifier::<T>::set(identifier);

		Ok(())
	}

	/// Add a new key to the MSA
	pub fn add_key<F>(msa_id: MessageSourceId, key: &T::AccountId, on_success: F) -> DispatchResult
	where
		F: FnOnce(MessageSourceId) -> DispatchResult,
	{
		MessageSourceIdOf::<T>::try_mutate(key, |maybe_msa_id| {
			ensure!(maybe_msa_id.is_none(), Error::<T>::KeyAlreadyRegistered);
			*maybe_msa_id = Some(msa_id);

			// Incremement the key counter
			<MsaInfoOf<T>>::try_mutate(msa_id, |key_count| {
				// key_count:u8 should default to 0 if it does not exist
				let incremented_key_count: u8 = *key_count + 1;
				ensure!(incremented_key_count <= T::MaxKeys::get(), Error::<T>::KeyLimitExceeded);

				*key_count = incremented_key_count;
				on_success(msa_id)
			})
		})
	}

	/// Returns if provider is registered by checking if the [`ProviderRegistry`] contains the MSA id
	pub fn is_registered_provider(msa_id: MessageSourceId) -> bool {
		ProviderRegistry::<T>::contains_key(Provider(msa_id))
	}

	/// Checks that a provider and delegator keys are valid
	/// and that a provider and delegator are not the same
	/// and that a provider has authorized a delegator to create a delegation relationship.
	/// - Returns [`ProviderNotRegistered`](Error::ProviderNotRegistered) if the a non-provider MSA is used as the provider
	/// - Returns [`InvalidSelfProvider`](Error::InvalidSelfProvider) if the delegator is the provider
	pub fn ensure_valid_registered_provider(
		delegator_key: &T::AccountId,
		provider_key: &T::AccountId,
	) -> Result<(Provider, Delegator), DispatchError> {
		let provider_msa_id = Self::ensure_valid_msa_key(provider_key)?;
		let delegator_msa_id = Self::ensure_valid_msa_key(delegator_key)?;

		// Ensure that the delegator is not the provider.  You cannot delegate to yourself.
		ensure!(delegator_msa_id != provider_msa_id, Error::<T>::InvalidSelfProvider);

		// Verify that the provider is a registered provider
		ensure!(Self::is_registered_provider(provider_msa_id), Error::<T>::ProviderNotRegistered);

		Ok((provider_msa_id.into(), delegator_msa_id.into()))
	}

	/// Checks that the MSA for `who` is the same as `msa_id`
	pub fn ensure_msa_owner(who: &T::AccountId, msa_id: MessageSourceId) -> DispatchResult {
		let provider_msa_id = Self::get_owner_of(who).ok_or(Error::<T>::NoKeyExists)?;

		ensure!(provider_msa_id == msa_id, Error::<T>::NotMsaOwner);

		Ok(())
	}

	/// Verify the `signature` was signed by `signer` on `payload` by a wallet
	/// Note the `wrap_binary_data` follows the Polkadot wallet pattern of wrapping with `<Byte>` tags.
	pub fn verify_signature(
		signature: MultiSignature,
		signer: T::AccountId,
		payload: Vec<u8>,
	) -> DispatchResult {
		let key = T::ConvertIntoAccountId32::convert(signer);
		let wrapped_payload = wrap_binary_data(payload);
		ensure!(signature.verify(&wrapped_payload[..], &key), Error::<T>::InvalidSignature);

		Ok(())
	}

	/// Ensure that the expiration block number has not already passed and is also not too far into the future.
	///
	/// # Arguments
	/// * `expiration` - A block number that which validity or expiration is being checked.
	///
	/// # Returns
	/// * [`DispatchResult`]
	///
	/// # Errors
	/// * [Error::ProofHasExpired] - If the current block is less than the `expired` bock number set in `AddKeyData`.
	/// * [Error::ProofNotYetValid] - If the `expired` bock number set in `AddKeyData` is greater than the current block number plus EXPIRATION_BLOCK_VALIDITY_GAP.
	pub fn ensure_block_is_valid(expiration: BlockNumber) -> DispatchResult {
		let current_block: BlockNumber =
			frame_system::Pallet::<T>::block_number().try_into().ok().unwrap();
		ensure!(current_block < expiration.into(), Error::<T>::ProofHasExpired);

		// If gap between the current block and the expiration block is larger than EXPIRATION_BLOCK_VALIDITY_GAP,
		// the proof is too var into the future and return Error::<T>::ProofNotYetValid.
		let blocks_until_expiration: BlockNumber = expiration - current_block;
		ensure!(
			blocks_until_expiration < EXPIRATION_BLOCK_VALIDITY_GAP,
			Error::<T>::ProofNotYetValid
		);

		Ok(())
	}

	/// Add a provider to a delegator with the default permissions
	pub fn add_provider(
		provider: Provider,
		delegator: Delegator,
		schemas: Vec<SchemaId>,
	) -> DispatchResult {
		ProviderInfoOf::<T>::try_mutate(delegator, provider, |maybe_info| -> DispatchResult {
			ensure!(maybe_info.take() == None, Error::<T>::DuplicateProvider);
			let granted_schemas = BoundedVec::<SchemaId, T::MaxSchemaGrants>::try_from(schemas)
				.map_err(|_| Error::<T>::ExceedsMaxSchemaGrants)?;
			let info = ProviderInfo {
				permission: Default::default(),
				expired: Default::default(),
				schemas: OrderedSetExt::<SchemaId, T::MaxSchemaGrants>::from(granted_schemas),
			};

			*maybe_info = Some(info);

			Ok(())
		})?;

		Ok(())
	}

	/// Check that the delegator has an active delegation to the provider
	/// # Arguments
	/// * `provider`
	/// * `delegate`
	/// # Returns
	/// * [`ProviderInfo`]
	/// # Errors
	/// * [`Error::<T>::DelegationNotFound`] - If no delegation
	/// * [`Error::<T>::DelegationExpired`] - If delegation revoked
	pub fn ensure_valid_delegation(
		provider: Provider,
		delegator: Delegator,
	) -> Result<ProviderInfo<T::BlockNumber, T::MaxSchemaGrants>, DispatchError> {
		let info = Self::get_provider_info_of(delegator, provider)
			.ok_or(Error::<T>::DelegationNotFound)?;
		if info.expired == T::BlockNumber::zero() {
			return Ok(info)
		}
		let current_block = frame_system::Pallet::<T>::block_number();
		ensure!(info.expired >= current_block, Error::<T>::DelegationExpired);
		Ok(info)
	}

	/// Deletes a key associated with a given MSA
	/// # Arguments
	/// * `msa_id` - The MSA for which the key needs to be removed
	/// * `key` - The key to be removed from the MSA
	/// # Returns
	/// * [`DispatchResult`]
	/// # Errors
	/// * [`Error::<T>::NoKeyExists`] - If the key does not exist in the MSA
	pub fn delete_key_for_msa(msa_id: MessageSourceId, key: &T::AccountId) -> DispatchResult {
		MessageSourceIdOf::<T>::try_mutate_exists(key, |maybe_msa_id| {
			ensure!(maybe_msa_id.is_some(), Error::<T>::NoKeyExists);

			// Delete the key if it exists
			*maybe_msa_id = None;

			<MsaInfoOf<T>>::try_mutate(msa_id, |key_count| {
				*key_count = *key_count - 1u8;
				Ok(())
			})
		})
	}

	/// Revoke the grant for permissions from the delegator to the provider
	/// # Arguments
	/// * `provider_msa_id` - The provider to remove the grant for
	/// * `delegator_msa_id` - The delegator that is removing the grant
	/// # Returns
	/// * [`DispatchResult`]
	///
	/// # Errors
	/// * [`Error::<T>::DelegationRevoked`] - Already revoked
	/// * [`Error::<T>::DelegationNotFound`] - No delegation
	pub fn revoke_provider(
		provider_msa_id: Provider,
		delegator_msa_id: Delegator,
	) -> DispatchResult {
		ProviderInfoOf::<T>::try_mutate_exists(
			delegator_msa_id,
			provider_msa_id,
			|maybe_info| -> DispatchResult {
				let mut info = maybe_info.take().ok_or(Error::<T>::DelegationNotFound)?;

				ensure!(info.expired == T::BlockNumber::default(), Error::<T>::DelegationRevoked);

				let current_block = frame_system::Pallet::<T>::block_number();

				info.expired = current_block;

				*maybe_info = Some(info);

				Ok(())
			},
		)?;

		Ok(())
	}

	/// Attempts to retrieve the MSA id for an account
	/// # Arguments
	/// * `key` - The `AccountId` you want to attempt to get information on
	/// # Returns
	/// * [`MessageSourceId`]
	pub fn try_get_msa_from_account_id(
		key: &T::AccountId,
	) -> Result<MessageSourceId, DispatchError> {
		let info = Self::get_msa_by_account_id(key).ok_or(Error::<T>::NoKeyExists)?;
		Ok(info)
	}

	/// Retrieves the MSA Id for a given `AccountId`
	/// # Arguments
	/// * `key` - The `AccountId` you want to attempt to get information on
	/// # Returns
	/// * [`MessageSourceId`]
	pub fn get_owner_of(key: &T::AccountId) -> Option<MessageSourceId> {
		Self::get_msa_by_account_id(&key)
	}

	// *Temporarily Removed* until https://github.com/LibertyDSNP/frequency/issues/418
	//
	// Fetches all the keys associated with a message Source Account
	// NOTE: This should only be called from RPC due to heavy database reads
	// pub fn fetch_msa_keys(msa_id: MessageSourceId) -> Vec<KeyInfoResponse<T::AccountId>> {
	// 	let mut response = Vec::new();
	// 	for key in Self::get_msa_keys(msa_id) {
	// 		if let Ok(_info) = Self::try_get_msa_from_account_id(&key) {
	// 			response.push(KeyInfoResponse { key, msa_id });
	// 		}
	// 	}

	// 	response
	// }

	/// Checks that a key is associated to an MSA and has not been revoked.
	pub fn ensure_valid_msa_key(key: &T::AccountId) -> Result<MessageSourceId, DispatchError> {
		let msa_id = Self::try_get_msa_from_account_id(key)?;

		Ok(msa_id)
	}

	/// Check if provider is allowed to publish for a given schema_id for a given delegator
	/// # Arguments
	/// * `provider` - The provider account
	/// * `delegator` - The delegator account
	/// * `schema_id` - The schema id
	/// # Returns
	/// * [`DispatchResult`]
	/// # Errors
	/// * [`ensure_valid_delegation`] Errors
	/// * [`Error::SchemaNotGranted`]
	pub fn ensure_valid_schema_grant(
		provider: Provider,
		delegator: Delegator,
		schema_id: SchemaId,
	) -> DispatchResult {
		let provider_info = Self::ensure_valid_delegation(provider, delegator)?;

		ensure!(provider_info.schemas.0.contains(&schema_id), Error::<T>::SchemaNotGranted);
		Ok(())
	}

	/// Get a list of ```schema_id```s that a provider has been granted access to
	/// # Arguments
	/// * `provider` - The provider account
	/// * `delegator` - The delegator account
	/// # Returns
	/// * [`Vec<SchemaId>`]
	/// # Errors
	/// * [`Error::DelegationNotFound`]
	/// * [`Error::SchemaNotGranted`]
	pub fn get_granted_schemas(
		delegator: Delegator,
		provider: Provider,
	) -> Result<Option<Vec<SchemaId>>, DispatchError> {
		let provider_info = Self::get_provider_info_of(delegator, provider)
			.ok_or(Error::<T>::DelegationNotFound)?;
		let schemas = provider_info.schemas.0;
		if schemas.0.is_empty() {
			return Err(Error::<T>::SchemaNotGranted.into())
		}
		Ok(Some(schemas.0.into()))
	}
}

impl<T: Config> AccountProvider for Pallet<T> {
	type AccountId = T::AccountId;
	type BlockNumber = T::BlockNumber;
	type MaxSchemaGrants = T::MaxSchemaGrants;
	fn get_msa_id(key: &Self::AccountId) -> Option<MessageSourceId> {
		Self::get_owner_of(key)
	}

	fn get_provider_info_of(
		delegator: Delegator,
		provider: Provider,
	) -> Option<ProviderInfo<Self::BlockNumber, Self::MaxSchemaGrants>> {
		Self::get_provider_info(delegator, provider)
	}

	#[cfg(not(feature = "runtime-benchmarks"))]
	fn ensure_valid_delegation(
		provider: Provider,
		delegation: Delegator,
	) -> Result<ProviderInfo<Self::BlockNumber, Self::MaxSchemaGrants>, DispatchError> {
		Self::ensure_valid_delegation(provider, delegation)
	}

	/// Since benchmarks are using regular runtime, we can not use mocking for this loosely bounded
	/// pallet trait implementation. To be able to run benchmarks successfully for any other pallet
	/// that has dependencies on this one, we would need to define msa accounts on those pallets'
	/// benchmarks, but this will introduce direct dependencies between these pallets, which we
	/// would like to avoid.
	/// To successfully run benchmarks without adding dependencies between pallets we re-defined
	/// this method to return a dummy account in case it does not exist
	#[cfg(feature = "runtime-benchmarks")]
	fn ensure_valid_delegation(
		provider: Provider,
		delegation: Delegator,
	) -> Result<ProviderInfo<Self::BlockNumber, Self::MaxSchemaGrants>, DispatchError> {
		let validation_check = Self::ensure_valid_delegation(provider, delegation);
		if validation_check.is_err() {
			// If the delegation does not exist, we return a ok
			// This is only used for benchmarks, so it is safe to return a dummy account
			// in case the delegation does not exist
			return Ok(ProviderInfo {
				schemas: OrderedSetExt::new(),
				permission: Default::default(),
				expired: Default::default(),
			})
		}
		Ok(ProviderInfo {
			schemas: OrderedSetExt::new(),
			permission: Default::default(),
			expired: Default::default(),
		})
	}

	#[cfg(not(feature = "runtime-benchmarks"))]
	fn ensure_valid_msa_key(key: &T::AccountId) -> Result<MessageSourceId, DispatchError> {
		Self::ensure_valid_msa_key(key)
	}

	/// Since benchmarks are using regular runtime, we can not use mocking for this loosely bounded
	/// pallet trait implementation. To be able to run benchmarks successfully for any other pallet
	/// that has dependencies on this one, we would need to define msa accounts on those pallets'
	/// benchmarks, but this will introduce direct dependencies between these pallets, which we
	/// would like to avoid.
	/// To successfully run benchmarks without adding dependencies between pallets we re-defined
	/// this method to return a dummy account in case it does not exist
	#[cfg(feature = "runtime-benchmarks")]
	fn ensure_valid_msa_key(key: &T::AccountId) -> Result<MessageSourceId, DispatchError> {
		let result = Self::ensure_valid_msa_key(key);
		if result.is_err() {
			return Ok(1 as MessageSourceId)
		}
		Ok(result.unwrap())
	}

	/// Check if provider is allowed to publish for a given schema_id for a given delegator
	/// # Arguments
	/// * `provider` - The provider account
	/// * `delegator` - The delegator account
	/// * `schema_id` - The schema id
	/// # Returns
	/// * [`DispatchResult`]
	/// # Errors
	/// * [`Error::DelegationNotFound`]
	/// * [`Error::SchemaNotGranted`]
	#[cfg(not(feature = "runtime-benchmarks"))]
	fn ensure_valid_schema_grant(
		provider: Provider,
		delegator: Delegator,
		schema_id: SchemaId,
	) -> DispatchResult {
		Self::ensure_valid_schema_grant(provider, delegator, schema_id)
	}

	/// Since benchmarks are using regular runtime, we can not use mocking for this loosely bounded
	/// pallet trait implementation. To be able to run benchmarks successfully for any other pallet
	/// that has dependencies on this one, we would need to define msa accounts on those pallets'
	/// benchmarks, but this will introduce direct dependencies between these pallets, which we
	/// would like to avoid.
	/// To successfully run benchmarks without adding dependencies between pallets we re-defined
	/// this method to return a dummy account in case it does not exist
	/// # Arguments
	/// * `provider` - The provider account
	/// * `delegator` - The delegator account
	/// * `schema_id` - The schema id
	/// # Returns
	/// * [`DispatchResult`]
	/// # Errors
	/// * [`Error::DelegationNotFound`]
	/// * [`Error::SchemaNotGranted`]
	#[cfg(feature = "runtime-benchmarks")]
	fn ensure_valid_schema_grant(
		provider: Provider,
		delegator: Delegator,
		_schema_id: SchemaId,
	) -> DispatchResult {
		let provider_info = Self::get_provider_info_of(delegator, provider);
		if provider_info.is_none() {
			return Ok(())
		}
		Ok(())
	}
}

/// The SignedExtension trait is implemented on CheckFreeExtrinsicUse to validate that a provider
/// has not already been revoked if the calling extrinsic is revoking a provider to an MSA. The
/// purpose of this is to ensure that the revoke_msa_delegation_by_delegator extrinsic cannot be
/// repeatedly called and flood the network.
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckFreeExtrinsicUse<T: Config + Send + Sync>(PhantomData<T>);

impl<T: Config + Send + Sync> CheckFreeExtrinsicUse<T> {
	/// Validates the delegation by making sure that the MSA ids used are valid
	pub fn validate_delegation_by_delegator(
		account_id: &T::AccountId,
		provider_msa_id: &MessageSourceId,
	) -> TransactionValidity {
		const TAG_PREFIX: &str = "DelegationRevocation";
		let delegator_msa_id: Delegator = Pallet::<T>::ensure_valid_msa_key(account_id)
			.map_err(|_| InvalidTransaction::Custom(ValidityError::InvalidMsaKey as u8))?
			.into();
		let provider_msa_id = Provider(*provider_msa_id);

		Pallet::<T>::ensure_valid_delegation(provider_msa_id, delegator_msa_id)
			.map_err(|_| InvalidTransaction::Custom(ValidityError::InvalidDelegation as u8))?;
		return ValidTransaction::with_tag_prefix(TAG_PREFIX).and_provides(account_id).build()
	}

	/// validates that a key being revoked is both valid and owned by a valid MSA account
	pub fn validate_key_revocation(
		account_id: &T::AccountId,
		key: &T::AccountId,
	) -> TransactionValidity {
		const TAG_PREFIX: &str = "KeyRevocation";
		let _msa_id: Delegator = Pallet::<T>::ensure_valid_msa_key(key)
			.map_err(|_| InvalidTransaction::Custom(ValidityError::InvalidMsaKey as u8))?
			.into();
		return ValidTransaction::with_tag_prefix(TAG_PREFIX).and_provides(account_id).build()
	}
}

/// Errors related to the validity of the CheckFreeExtrinsicUse signed extension.
enum ValidityError {
	/// Delegation to provider is not found or expired.
	InvalidDelegation,
	/// MSA key as been revoked.
	InvalidMsaKey,
}

impl<T: Config + Send + Sync> CheckFreeExtrinsicUse<T> {
	/// Create new `SignedExtension` to check runtime version.
	pub fn new() -> Self {
		Self(sp_std::marker::PhantomData)
	}
}

impl<T: Config + Send + Sync> sp_std::fmt::Debug for CheckFreeExtrinsicUse<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "CheckFreeExtrinsicUse<{:?}>", self.0)
	}
	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl<T: Config + Send + Sync> SignedExtension for CheckFreeExtrinsicUse<T>
where
	T::Call: Dispatchable<Info = DispatchInfo> + IsSubType<Call<T>>,
{
	type AccountId = T::AccountId;
	type Call = T::Call;
	type AdditionalSigned = ();
	type Pre = ();
	const IDENTIFIER: &'static str = "CheckFreeExtrinsicUse";

	fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
		Ok(())
	}

	fn pre_dispatch(
		self,
		who: &Self::AccountId,
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		self.validate(who, call, info, len).map(|_| ())
	}

	/// Frequently called by the transaction queue to ensure that the transaction is valid such that:
	/// * The calling extrinsic is 'revoke_msa_delegation_by_delegator'.
	/// * The sender key is associated to an MSA and not revoked.
	/// * The provider MSA is a valid provider to the delegator MSA.
	fn validate(
		&self,
		who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> TransactionValidity {
		match call.is_sub_type() {
			Some(Call::revoke_msa_delegation_by_delegator { provider_msa_id, .. }) =>
				CheckFreeExtrinsicUse::<T>::validate_delegation_by_delegator(who, provider_msa_id),
			Some(Call::delete_msa_key { key, .. }) =>
				CheckFreeExtrinsicUse::<T>::validate_key_revocation(who, key),
			_ => return Ok(Default::default()),
		}
	}
}
