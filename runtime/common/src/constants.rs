use crate::prod_or_testnet_or_local;
use common_primitives::{
	node::{Balance, BlockNumber},
	schema::SchemaId,
};

use frame_support::{
	parameter_types,
	sp_runtime::{Perbill, Permill},
	traits::{ConstU32, ConstU8},
	weights::{constants::WEIGHT_PER_SECOND, Weight},
	PalletId,
};

pub const FREQUENCY_ROCOCO_TOKEN: &str = "XRQCY";
pub const FREQUENCY_LOCAL_TOKEN: &str = "UNIT";
pub const FREQUENCY_TOKEN: &str = "FRQCY";
pub const TOKEN_DECIMALS: u8 = 8;

parameter_types! {
	/// Clone + Debug + Eq  implementation for u32 types
	pub const MaxDataSize: u32 = 30;
}

impl Clone for MaxDataSize {
	fn clone(&self) -> Self {
		MaxDataSize {}
	}
}

impl Eq for MaxDataSize {
	fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for MaxDataSize {
	fn eq(&self, other: &Self) -> bool {
		self == other
	}
}

impl sp_std::fmt::Debug for MaxDataSize {
	#[cfg(feature = "std")]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// Unit = the base number of indivisible units for balances
pub mod currency {
	use common_primitives::node::Balance;

	/// The existential deposit. Set to be 1/100th of a token.
	pub const EXISTENTIAL_DEPOSIT: Balance = CENTS;

	pub const UNITS: Balance = 10u128.saturating_pow(super::TOKEN_DECIMALS as u32);
	pub const DOLLARS: Balance = UNITS; // 100_000_000
	pub const CENTS: Balance = DOLLARS / 100; // 1_000_000
	pub const MILLICENTS: Balance = CENTS / 1_000; // 1_000

	/// Generates an balance based on amount of items and bytes
	/// Items are each worth 20 Dollars
	/// Bytes each cost 1/1_000 of a Dollar
	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 20 * DOLLARS + (bytes as Balance) * 100 * MILLICENTS
	}
}

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 0.5 of a second of compute with a 12 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND.saturating_div(2);

pub type ZERO = ConstU32<0>;
pub type FIFTY = ConstU32<50>;
pub type HUNDRED = ConstU32<100>;

// --- Frame System Pallet ---
pub type FrameSystemMaxConsumers = ConstU32<16>;
// -end- Frame System Pallet ---

// --- MSA Pallet ---
pub type MsaMaxPublicKeysPerMsa = ConstU8<25>;
pub type MsaMaxProviderNameSize = ConstU32<16>;
// -end- MSA Pallet ---

// --- Schemas Pallet ---
parameter_types! {
	pub const SchemasMaxRegistrations: SchemaId = 65_000;
}
pub type SchemasMinModelSizeBytes = ConstU32<8>;
pub type SchemasMaxBytesBoundedVecLimit = ConstU32<65_500>;
// -end- Schemas Pallet ---

// --- Orml Vesting Pallet ---
parameter_types! {
	pub const VestingPalletId: PalletId = PalletId(*b"py/vstng");
}

parameter_types! {
	pub const MinVestedTransfer: Balance = 0;
}

pub const ORML_MAX_VESTING_SCHEDULES: u32 = 50;
// -end- Orml Vesting Pallet ---

// --- Timestamp Pallet ---
parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}
// -end- Timestamp Pallet ---

// --- Authorship Pallet ---
pub type AuthorshipUncleGenerations = ZERO;
// -end- Authorship Pallet ---

// --- Balances Pallet ---
pub type BalancesMaxLocks = FIFTY;
pub type BalancesMaxReserves = FIFTY;
// -end- Balances Pallet ---

// --- Scheduler Pallet ---
pub type SchedulerMaxScheduledPerBlock = FIFTY;
// -end- Scheduler Pallet ---

// --- Preimage Pallet ---
/// Preimage maximum size set to 4 MB
/// Expected to be removed in Polkadot v0.9.31
pub type PreimageMaxSize = ConstU32<{ 4096 * 1024 }>;

parameter_types! {
	pub const PreimageBaseDeposit: Balance = currency::deposit(10, 64);
	pub const PreimageByteDeposit: Balance = currency::deposit(0, 1);
}
// -end- Preimage Pallet ---

// --- Council ---
pub type CouncilMaxProposals = ConstU32<25>;

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 5 * DAYS;
}
// -end- Council ---

// --- Technical Committee ---
pub type TCMaxProposals = ConstU32<25>;
pub type TCMaxMembers = ConstU32<3>;

parameter_types! {
	pub const TCMotionDuration: BlockNumber = 5 * DAYS;
}
// -end- Technical Committee ---

// --- Democracy Pallet ---
// Config from
// https://github.com/paritytech/substrate/blob/367dab0d4bd7fd7b6c222dd15c753169c057dd42/bin/node/runtime/src/lib.rs#L880
parameter_types! {
	pub LaunchPeriod: BlockNumber = prod_or_testnet_or_local!(7 * DAYS, 1 * DAYS, 5 * MINUTES);
	pub VotingPeriod: BlockNumber = prod_or_testnet_or_local!(7 * DAYS, 1 * DAYS, 5 * MINUTES);
	pub FastTrackVotingPeriod: BlockNumber = prod_or_testnet_or_local!(3 * HOURS, 30 * MINUTES, 5 * MINUTES);
	pub EnactmentPeriod: BlockNumber = prod_or_testnet_or_local!(8 * DAYS, 30 * HOURS, 10 * MINUTES);
	pub CooloffPeriod: BlockNumber = prod_or_testnet_or_local!(7 * DAYS, 1 * DAYS, 5 * MINUTES);
	pub MinimumDeposit: Balance = prod_or_testnet_or_local!(currency::deposit(5, 0), 100 * currency::deposit(5, 0), 100 * currency::deposit(5, 0));
	pub SpendPeriod: BlockNumber = prod_or_testnet_or_local!(7 * DAYS, 10 * MINUTES, 10 * MINUTES);
}
pub type DemocracyMaxVotes = ConstU32<100>;
pub type DemocracyMaxProposals = HUNDRED;
// -end- Democracy Pallet ---

// --- Treasury Pallet ---
/// Generates the pallet "account"
/// 5EYCAe5ijiYfyeZ2JJCGq56LmPyNRAKzpG4QkoQkkQNB5e6Z
pub const TREASURY_PALLET_ID: PalletId = PalletId(*b"py/trsry");

// https://wiki.polkadot.network/docs/learn-treasury
// https://paritytech.github.io/substrate/master/pallet_treasury/pallet/trait.Config.html
parameter_types! {

	/// Keyless account that holds the money for the treasury
	pub const TreasuryPalletId: PalletId = TREASURY_PALLET_ID;

	/// Bond amount a treasury request must put up to make the proposal
	/// This will be transferred to OnSlash if the proposal is rejected
	pub const ProposalBondPercent: Permill = Permill::from_percent(5);

	/// Minimum bond for a treasury proposal
	pub const ProposalBondMinimum: Balance = 100 * currency::DOLLARS;

	/// Minimum bond for a treasury proposal
	pub const ProposalBondMaximum: Balance = 1_000 * currency::DOLLARS;

	/// How much of the treasury to burn, if funds remain at the end of the SpendPeriod
	/// Set to zero until the economic system is setup and stabilized
	pub const Burn: Permill = Permill::zero();

	/// Maximum number of approved proposals per Spending Period
	/// Set to 64 or 16 per week
	pub const MaxApprovals: u32 = 64;
}
// -end- Treasury Pallet ---

// --- Transaction Payment Pallet ---
pub type TransactionPaymentOperationalFeeMultiplier = ConstU8<5>;

parameter_types! {
	/// Relay Chain `TransactionByteFee` / 10
	pub const TransactionByteFee: Balance = 10 * currency::MILLICENTS;
}
// -end- Transaction Payment Pallet ---

// --- Session Pallet ---
pub type SessionPeriod = ConstU32<{ 6 * HOURS }>;
pub type SessionOffset = ZERO;
// -end- Session Pallet ---

// --- Aura Pallet ---
pub type AuraMaxAuthorities = ConstU32<100_000>;
// -end- Aura Pallet ---

// --- Collator Selection Pallet ---
// Values for each runtime environment are independently configurable.
// Example CollatorMaxInvulnerables are 16 in production(mainnet),
// 5 in rococo testnet and 5 in rococo local
parameter_types! {
	pub CollatorMaxCandidates: u32 = prod_or_testnet_or_local!(0, 0, 0);
	pub CollatorMinCandidates: u32 = prod_or_testnet_or_local!(0, 0, 0);
	pub CollatorMaxInvulnerables: u32 = prod_or_testnet_or_local!(16, 5, 5);
	pub CollatorKickThreshold: BlockNumber = prod_or_testnet_or_local!(
		6 * HOURS,
		6 * HOURS,
		6 * HOURS
	);
	pub const NeverDepositIntoId: PalletId = PalletId(*b"NeverDep");
	pub const MessagesMaxPayloadSizeBytes: u32 = 1024 * 50; // 50K
}
// -end- Collator Selection Pallet ---

// --- Messages Pallet ---
pub type MessagesMaxPerBlock = ConstU32<7000>;

impl Clone for MessagesMaxPayloadSizeBytes {
	fn clone(&self) -> Self {
		MessagesMaxPayloadSizeBytes {}
	}
}
// -end- Messages Pallet ---

pub type MSAMortalityWindowSize = ConstU32<100>;
pub type MSAMaxSignaturesPerBucket = ConstU32<50_000>;
pub type MSANumberOfBuckets = ConstU32<2>;

parameter_types! {
	/// SS58 Prefix for the for Frequency Network
	/// 90 is the prefix for the Frequency Network on Polkadot
	/// 42 is the prefix for the Frequency Network on Rococo
	pub const Ss58Prefix: u16 = prod_or_testnet_or_local!(90, 42, 42);
}
