use crate as pallet_messages;
use common_primitives::{
	msa::{
		Delegation, DelegationValidator, DelegatorId, MessageSourceId, MsaLookup, MsaValidator,
		ProviderId, ProviderLookup, SchemaGrantValidator,
	},
	schema::*,
};

use frame_support::{
	dispatch::DispatchResult,
	parameter_types,
	traits::{ConstU16, ConstU64, OnFinalize, OnInitialize},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	DispatchError,
};
use std::fmt::Formatter;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const INVALID_SCHEMA_ID: SchemaId = 65534;
pub const IPFS_SCHEMA_ID: SchemaId = 50;

pub const IPFS_PAYLOAD_LENGTH: u32 = 1200;

pub const DUMMY_CID: &[u8; 59] = b"bafkreidgvpkjawlxz6sffxzwgooowe5yt7i6wsyg236mfoks77nywkptdq";

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		MessagesPallet: pallet_messages::{Pallet, Call, Storage, Event<T>},
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxMessagesPerBlock: u32 = 500;
	// Max payload size was picked specifically to be large enough to accomodate
	// a CIDv1 using SHA2-256, but too small to accomodate CIDv1 w/SHA2-512.
	// This is purely so that we can test the error condition. Real world configuration
	// should have this set large enough to accomodate the largest possible CID.
	// Take care when adding new tests for on-chain (not IPFS) messages that the payload
	// is not too big.
	pub static MaxMessagePayloadSizeBytes: u32 = 73;
	pub const MaxSchemaGrantsPerDelegation: u32 = 30;
}

impl Clone for MaxSchemaGrantsPerDelegation {
	fn clone(&self) -> Self {
		MaxSchemaGrantsPerDelegation {}
	}
}

impl Eq for MaxSchemaGrantsPerDelegation {
	fn assert_receiver_is_total_eq(&self) -> () {}
}

impl PartialEq for MaxSchemaGrantsPerDelegation {
	fn eq(&self, _other: &Self) -> bool {
		true
	}
}

impl sp_std::fmt::Debug for MaxSchemaGrantsPerDelegation {
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl std::fmt::Debug for MaxMessagePayloadSizeBytes {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("MaxMessagePayloadSizeBytes")
			.field("v", &MaxMessagePayloadSizeBytes::get())
			.finish()
	}
}

impl PartialEq for MaxMessagePayloadSizeBytes {
	fn eq(&self, _other: &Self) -> bool {
		true
	}
}

impl Clone for MaxMessagePayloadSizeBytes {
	fn clone(&self) -> Self {
		MaxMessagePayloadSizeBytes {}
	}
}

pub struct MsaInfoHandler;
pub struct DelegationInfoHandler;
pub struct SchemaGrantValidationHandler;
impl MsaLookup for MsaInfoHandler {
	type AccountId = u64;

	fn get_msa_id(key: &Self::AccountId) -> Option<MessageSourceId> {
		if *key == 1000 {
			return None
		}
		if *key == 2000 {
			return Some(2000 as MessageSourceId)
		}
		Some(get_msa_from_account(*key) as MessageSourceId)
	}
}

impl MsaValidator for MsaInfoHandler {
	type AccountId = u64;

	fn ensure_valid_msa_key(key: &Self::AccountId) -> Result<MessageSourceId, DispatchError> {
		if *key == 1000 {
			return Err(DispatchError::Other("some error"))
		}
		if *key == 2000 {
			return Ok(2000)
		}

		Ok(get_msa_from_account(*key))
	}
}
impl ProviderLookup for DelegationInfoHandler {
	type BlockNumber = u64;
	type MaxSchemaGrantsPerDelegation = MaxSchemaGrantsPerDelegation;
	type SchemaId = SchemaId;

	fn get_delegation_of(
		_delegator: DelegatorId,
		provider: ProviderId,
	) -> Option<Delegation<SchemaId, Self::BlockNumber, MaxSchemaGrantsPerDelegation>> {
		if provider == ProviderId(2000) {
			return None
		};
		Some(Delegation { revoked_at: 100, schema_permissions: Default::default() })
	}
}
impl DelegationValidator for DelegationInfoHandler {
	type BlockNumber = u64;
	type MaxSchemaGrantsPerDelegation = MaxSchemaGrantsPerDelegation;
	type SchemaId = SchemaId;

	fn ensure_valid_delegation(
		provider: ProviderId,
		_delegator: DelegatorId,
		_block_number: Option<Self::BlockNumber>,
	) -> Result<
		Delegation<SchemaId, Self::BlockNumber, Self::MaxSchemaGrantsPerDelegation>,
		DispatchError,
	> {
		if provider == ProviderId(2000) {
			return Err(DispatchError::Other("some delegation error"))
		};

		Ok(Delegation { schema_permissions: Default::default(), revoked_at: Default::default() })
	}
}
impl<BlockNumber> SchemaGrantValidator<BlockNumber> for SchemaGrantValidationHandler {
	fn ensure_valid_schema_grant(
		provider: ProviderId,
		delegator: DelegatorId,
		_schema_id: SchemaId,
		_block_number: BlockNumber,
	) -> DispatchResult {
		match DelegationInfoHandler::get_delegation_of(delegator, provider) {
			Some(_) => Ok(()),
			None => Err(DispatchError::Other("no schema grant or delegation")),
		}
	}
}

pub struct SchemaHandler;
impl SchemaProvider<u16> for SchemaHandler {
	fn get_schema_by_id(schema_id: SchemaId) -> Option<SchemaResponse> {
		if schema_id == INVALID_SCHEMA_ID {
			return None
		}
		if schema_id == IPFS_SCHEMA_ID {
			return Some(SchemaResponse {
				schema_id,
				model: r#"schema"#.to_string().as_bytes().to_vec(),
				model_type: ModelType::Parquet,
				payload_location: PayloadLocation::IPFS,
			})
		}

		Some(SchemaResponse {
			schema_id,
			model: r#"schema"#.to_string().as_bytes().to_vec(),
			model_type: ModelType::AvroBinary,
			payload_location: PayloadLocation::OnChain,
		})
	}
}

impl pallet_messages::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MsaInfoProvider = MsaInfoHandler;
	type SchemaGrantValidator = SchemaGrantValidationHandler;
	type SchemaProvider = SchemaHandler;
	type WeightInfo = ();
	type MaxMessagesPerBlock = MaxMessagesPerBlock;
	type MaxMessagePayloadSizeBytes = MaxMessagePayloadSizeBytes;

	/// A set of helper functions for benchmarking.
	#[cfg(feature = "runtime-benchmarks")]
	type MsaBenchmarkHelper = ();
	#[cfg(feature = "runtime-benchmarks")]
	type SchemaBenchmarkHelper = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			MessagesPallet::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		MessagesPallet::on_initialize(System::block_number());
	}
}

pub fn get_msa_from_account(account_id: u64) -> u64 {
	account_id + 100
}
