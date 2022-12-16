use crate::{
	self as pallet_msa,
	mock::{generate_test_signature, new_test_ext, run_to_block},
	Error,
};
use common_primitives::node::AccountId;
use frame_support::{
	assert_noop, assert_ok, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, Everything},
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConvertInto, IdentityLookup},
};

pub use pallet_msa::Call as MsaCall;

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
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
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
	type MaxConsumers = ConstU32<16>;
}

impl pallet_schemas::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MinSchemaModelSizeBytes = ConstU32<10>;
	type SchemaModelMaxBytesBoundedVecLimit = ConstU32<10>;
	type MaxSchemaRegistrations = ConstU16<10>;
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

impl pallet_msa::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type ConvertIntoAccountId32 = ConvertInto;
	type MaxPublicKeysPerMsa = MaxPublicKeysPerMsa;
	type MaxSchemaGrantsPerDelegation = MaxSchemaGrantsPerDelegation;
	type MaxProviderNameSize = MaxProviderNameSize;
	type SchemaValidator = Schemas;
	type MortalityWindowSize = ConstU32<10>;
	type MaxSignaturesPerBucket = ConstU32<20>;
	type NumberOfBuckets = ConstU32<10>;
	/// This MUST ALWAYS be MaxSignaturesPerBucket * NumberOfBuckets.
	type MaxSignaturesStored = ConstU32<200>;
}

#[test]
fn audit_replay_scenario_fails() {
	new_test_ext().execute_with(|| {
		let current_block = 9;
		System::set_block_number(current_block);

		let sig1 = &generate_test_signature();
		let mortality: u64 = 13;
		assert_ok!(Msa::register_signature(sig1, mortality));

		assert_noop!(
			Msa::register_signature(sig1, mortality),
			Error::<Test>::SignatureAlreadySubmitted,
		);

		run_to_block(10);
		assert_noop!(
			Msa::register_signature(sig1, mortality),
			Error::<Test>::SignatureAlreadySubmitted,
		);

		run_to_block(mortality - 1);
		assert_noop!(
			Msa::register_signature(sig1, mortality),
			Error::<Test>::SignatureAlreadySubmitted,
		);

		run_to_block(mortality);
		assert_noop!(Msa::register_signature(sig1, mortality), Error::<Test>::ProofHasExpired,);
	})
}
