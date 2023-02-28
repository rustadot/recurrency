use crate::{self as pallet_msa, types::EMPTY_FUNCTION, AddProvider};
use common_primitives::{
	msa::MessageSourceId, node::BlockNumber, schema::SchemaId, utils::wrap_binary_data,
};
use frame_support::{
	assert_ok,
	dispatch::DispatchError,
	parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, EitherOfDiverse, OnFinalize, OnInitialize},
};
use frame_system::EnsureRoot;
use pallet_collective;
use sp_core::{sr25519, sr25519::Public, Encode, Pair, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConvertInto, IdentityLookup},
	AccountId32, MultiSignature,
};

pub use common_runtime::constants::*;

pub use pallet_msa::Call as MsaCall;

use common_primitives::node::AccountId;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Msa: pallet_msa::{Pallet, Call, Storage, Event<T>},
		Schemas: pallet_schemas::{Pallet, Call, Storage, Event<T>},
		Council: pallet_collective::<Instance1>::{Pallet, Call, Config<T,I>, Storage, Event<T>, Origin<T>},
	}
);

// See https://paritytech.github.io/substrate/master/pallet_collective/index.html for
// the descriptions of these configs.
type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
}

impl frame_system::Config for Test {
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
	type AccountId = AccountId;
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

impl pallet_schemas::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MinSchemaModelSizeBytes = ConstU32<10>;
	type SchemaModelMaxBytesBoundedVecLimit = ConstU32<10>;
	type MaxSchemaRegistrations = ConstU16<10>;
	// The proposal type
	type Proposal = RuntimeCall;
	// The Council proposal provider interface
	type ProposalProvider = CouncilProposalProvider;
	// The origin that is allowed to create schemas via governance
	// It has to be this way so benchmarks will pass in CI.
	type CreateSchemaViaGovernanceOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
	>;
}

parameter_types! {
	pub const MaxPublicKeysPerMsa: u8 = 255;
	pub const MaxProviderNameSize: u32 = 16;
	pub const MaxSchemas: u32 = 5;
}

parameter_types! {
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
/// Interface to collective pallet to propose a proposal.
pub struct CouncilProposalProvider;

impl pallet_msa::ProposalProvider<AccountId, RuntimeCall> for CouncilProposalProvider {
	fn propose(
		who: AccountId,
		threshold: u32,
		proposal: Box<RuntimeCall>,
	) -> Result<(u32, u32), DispatchError> {
		let length_bound: u32 = proposal.using_encoded(|p| p.len() as u32);
		Council::do_propose_proposed(who, threshold, proposal, length_bound)
	}

	fn propose_with_simple_majority(
		who: AccountId,
		proposal: Box<RuntimeCall>,
	) -> Result<(u32, u32), DispatchError> {
		let threshold: u32 = ((Council::members().len() / 2) + 1) as u32;
		let length_bound: u32 = proposal.using_encoded(|p| p.len() as u32);
		Council::do_propose_proposed(who, threshold, proposal, length_bound)
	}

	#[cfg(any(feature = "runtime-benchmarks", feature = "test"))]
	fn proposal_count() -> u32 {
		Council::proposal_count()
	}
}

impl pallet_msa::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type ConvertIntoAccountId32 = ConvertInto;
	type MaxPublicKeysPerMsa = MaxPublicKeysPerMsa;
	type MaxSchemaGrantsPerDelegation = MaxSchemaGrantsPerDelegation;
	type MaxProviderNameSize = MaxProviderNameSize;
	type SchemaValidator = Schemas;
	type MortalityWindowSize = ConstU32<100>;
	type MaxSignaturesPerBucket = ConstU32<4000>;
	type NumberOfBuckets = ConstU32<2>;
	/// This MUST ALWAYS be MaxSignaturesPerBucket * NumberOfBuckets.
	type MaxSignaturesStored = ConstU32<8000>;
	// The proposal type
	type Proposal = RuntimeCall;
	// The Council proposal provider interface
	type ProposalProvider = CouncilProposalProvider;
	// The origin that is allowed to create providers via governance
	// It has to be this way so benchmarks will pass in CI.
	type CreateProviderViaGovernanceOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureMembers<AccountId, CouncilCollective, 1>,
	>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Msa::on_initialize(System::block_number());
	}
}

/// Create and return a simple test AccountId32 constructed with the desired integer.
pub fn test_public(n: u8) -> AccountId32 {
	AccountId32::new([n; 32])
}

/// Create and return a simple signed origin from a test_public constructed with the desired integer,
/// for passing to an extrinsic call
pub fn test_origin_signed(n: u8) -> RuntimeOrigin {
	RuntimeOrigin::signed(test_public(n))
}

/// Create a new keypair and an MSA associated with its public key.
/// # Returns
/// (MessageSourceId, Pair) - a tuple with the MSA and the new Account key pair
pub fn create_account() -> (MessageSourceId, sr25519::Pair) {
	let (key_pair, _) = sr25519::Pair::generate();
	let result_key = Msa::create_account(AccountId32::from(key_pair.public()), EMPTY_FUNCTION);
	assert_ok!(&result_key);
	let (msa_id, _) = result_key.unwrap();
	(msa_id, key_pair)
}

/// Creates and signs an `AddProvider` struct using the provided delegator keypair and provider MSA
/// # Returns
/// (MultiSignature, AddProvider) - Returns a tuple with the signature and the AddProvider struct
pub fn create_and_sign_add_provider_payload(
	delegator_pair: sr25519::Pair,
	provider_msa: MessageSourceId,
) -> (MultiSignature, AddProvider) {
	create_and_sign_add_provider_payload_with_schemas(delegator_pair, provider_msa, None)
}

/// Creates and signs an `AddProvider` struct using the provided delegator keypair, provider MSA and schema ids
/// # Returns
/// (MultiSignature, AddProvider) - Returns a tuple with the signature and the AddProvider struct
pub fn create_and_sign_add_provider_payload_with_schemas(
	delegator_pair: sr25519::Pair,
	provider_msa: MessageSourceId,
	schema_ids: Option<Vec<SchemaId>>,
) -> (MultiSignature, AddProvider) {
	let expiration: BlockNumber = 10;
	let add_provider_payload = AddProvider::new(provider_msa, schema_ids, expiration);
	let encode_add_provider_data = wrap_binary_data(add_provider_payload.encode());
	let signature: MultiSignature = delegator_pair.sign(&encode_add_provider_data).into();
	(signature, add_provider_payload)
}

/// Creates a provider and delegator MSA and sets the delegation relationship.
// create and set up delegations for a delegator and provider, but for convenience only
/// # Returns
/// * (u8, Public) - Returns a provider_msa_id and a delegator account.
pub fn create_provider_msa_and_delegator() -> (u64, Public) {
	let (provider_msa_id, _, _, delegator_account) = create_provider_delegator_msas();
	(provider_msa_id, delegator_account)
}

// create and set up delegations for a delegator and provider, but for convenience only
// return delegator msa and provider account for testing delegator-submitted extrinsics
/// # Returns
/// * (u8, Public) - Returns a delegator_msa_id and a provider_account.
pub fn create_delegator_msa_and_provider() -> (u64, Public) {
	let (_, provider_account, delegator_msa_id, _) = create_provider_delegator_msas();
	(delegator_msa_id, provider_account)
}

// create and set up delegations for a delegator and provider and return it all
pub fn create_provider_delegator_msas() -> (u64, Public, u64, Public) {
	let (provider_msa_id, provider_pair) = create_account();
	let provider_account = provider_pair.public();

	let (delegator_msa_id, delegator_pair) = create_account();
	let delegator_account = delegator_pair.public();

	let (delegator_signature, add_provider_payload) =
		create_and_sign_add_provider_payload(delegator_pair, provider_msa_id);

	// Register provider
	assert_ok!(Msa::create_provider(
		RuntimeOrigin::signed(provider_account.into()),
		Vec::from("Foo")
	));

	assert_ok!(Msa::grant_delegation(
		RuntimeOrigin::signed(provider_account.into()),
		delegator_account.into(),
		delegator_signature,
		add_provider_payload
	));
	(provider_msa_id, provider_account, delegator_msa_id, delegator_account)
}

pub fn generate_test_signature() -> MultiSignature {
	let (key_pair, _) = sr25519::Pair::generate();
	let fake_data = H256::random();
	key_pair.sign(fake_data.as_bytes()).into()
}

#[cfg(feature = "runtime-benchmarks")]
pub fn new_test_ext_keystore() -> sp_io::TestExternalities {
	use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStorePtr};
	use sp_std::sync::Arc;

	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.register_extension(KeystoreExt(Arc::new(KeyStore::new()) as SyncCryptoStorePtr));

	ext
}
