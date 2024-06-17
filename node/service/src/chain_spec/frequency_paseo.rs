#![allow(missing_docs)]
use common_primitives::node::AccountId;
use common_runtime::constants::{
	currency::EXISTENTIAL_DEPOSIT, FREQUENCY_LOCAL_TOKEN, TOKEN_DECIMALS,
};
use cumulus_primitives_core::ParaId;
use frequency_runtime::{AuraId, CouncilConfig, Ss58Prefix, SudoConfig, TechnicalCommitteeConfig};
use polkadot_service::chain_spec::Extensions as RelayChainExtensions;
use sc_service::ChainType;
use sp_runtime::traits::AccountIdConversion;

use super::{get_account_id_from_seed, get_collator_keys_from_seed, get_properties, Extensions};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<frequency_runtime::RuntimeGenesisConfig, Extensions>;
use sp_core::sr25519;

// Generic chain spec, in case when we don't have the native runtime.
pub type RelayChainSpec = sc_service::GenericChainSpec<(), RelayChainExtensions>;

#[allow(clippy::unwrap_used)]
/// Generates the Frequency Paseo chain spec from the raw json
pub fn load_frequency_paseo_spec() -> ChainSpec {
	ChainSpec::from_json_bytes(
		&include_bytes!("../../../../resources/frequency-paseo.raw.json")[..],
	)
	.unwrap()
}

// TODO: Remove once on a Polkadot-SDK with Paseo
#[allow(clippy::unwrap_used)]
/// Generates the Paseo Relay chain spec from the json
pub fn load_paseo_spec() -> RelayChainSpec {
	RelayChainSpec::from_json_bytes(&include_bytes!("../../../../resources/paseo.json")[..])
		.unwrap()
}

// TODO: Remove once on a Polkadot-SDK with Paseo-Local
#[allow(clippy::unwrap_used)]
/// Generates the Paseo-Local Relay chain spec from the json
pub fn load_paseo_local_spec() -> RelayChainSpec {
	RelayChainSpec::from_json_bytes(&include_bytes!("../../../../resources/paseo-local.json")[..])
		.unwrap()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
fn template_session_keys(keys: AuraId) -> frequency_runtime::SessionKeys {
	frequency_runtime::SessionKeys { aura: keys }
}

pub mod public_testnet_keys {
	pub const COLLATOR_1_SR25519: &str =
		"0x5c0f55ba602f76d69b5cc075d81f6d27db9157c90dc4be492f4edbe7d7c96d18";
	pub const COLLATOR_2_SR25519: &str =
		"0x202be6542ed50679271280b8c37140681bbd5acfd0e508668297029f871b9f0c";
	pub const ROCOCO_FRQ_SUDO: &str =
		"0xccca4a5b784105460c5466cbb8d11b34f29ffcf6c725d07b65940e697763763c";
	pub const TECH_COUNCIL1: &str =
		"0x847c1ac02474b90cf1e9d8e722318b75fd56d370e6f35e9c983fe671e788d23a";
	pub const TECH_COUNCIL2: &str =
		"0x52b580c22c5ff6f586a0966fbd2373de279d1aa1b2d05dff47616b5a338fce27";
	pub const TECH_COUNCIL3: &str =
		"0x6a13f08b279cb33b249954190bcee832747b9aa9dc14cc290f82d73d496cfc0a";
	pub const FRQ_COUNCIL1: &str =
		"0xa608f3e0030c157b6e2a540c5f0c7dbd6004793813cad2c9fbda0c84c093c301";
	pub const FRQ_COUNCIL2: &str =
		"0x52d76db441043a5d47d9bf83e6fd2d5acb86b8547062571ee7b68255b6bada10";
	pub const FRQ_COUNCIL3: &str =
		"0x809d0a4e6683ebff9d74c7f6ba9fe504a64a7227d74eb45ee85556cc01013a63";
	pub const FRQ_COUNCIL4: &str =
		"0x8e47c13fd0f028f56378e202523fa44508fd64df89fddb482fc0b128989e9f0b";
	pub const FRQ_COUNCIL5: &str =
		"0xf23d555b95ca8c752b531e48848bfb4d3aa2b4eea407484ccee947501e77d04f";
	pub const FRQ_COUNCIL6: &str =
		"0xe87a126794cb727b5a7760922f81fbf0f80fd64b7e86e6ae4fee0be4289c7512";
	pub const FRQ_COUNCIL7: &str =
		"0x14a6bff08e9637457a165779765417feca01a2119dec98ec134f8ae470111318";
	pub const FRQ_COUNCIL8: &str =
		"0x140c17ced6e4fba8b62a6935052cfb7c5a8ad8ecc43dee1f4fc7c30c1ca3cb14";
	pub const FRQ_COUNCIL9: &str =
		"0xfc61655783e14b361d2b9601c657c3c5361a2cf32aa1a448fc83b1a356808a1a";
}

/// Generates the chain spec for a local testnet
pub fn local_paseo_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let properties =
		get_properties(FREQUENCY_LOCAL_TOKEN, TOKEN_DECIMALS as u32, Ss58Prefix::get().into());

	ChainSpec::builder(
		frequency_runtime::wasm_binary_unwrap(),
		Extensions {
			relay_chain: "paseo-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
	.with_name("Frequency Local Testnet")
	.with_protocol_id("frequency-paseo-local")
	.with_properties(properties)
	.with_chain_type(ChainType::Local)
	.with_genesis_config(testnet_genesis(
		// initial collators.
		vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_collator_keys_from_seed("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_collator_keys_from_seed("Bob"),
			),
		],
		// Sudo
		Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
		// Endowed Accounts
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			common_runtime::constants::TREASURY_PALLET_ID.into_account_truncating(),
		],
		// Council members
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
		],
		// Technical Committee members
		vec![
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
		],
		// ParaId
		2000.into(),
	))
	.build()
}

#[allow(clippy::unwrap_used)]
fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	root_key: Option<AccountId>,
	endowed_accounts: Vec<AccountId>,
	council_members: Vec<AccountId>,
	technical_committee_members: Vec<AccountId>,
	id: ParaId,
) -> serde_json::Value {
	let genesis = frequency_runtime::RuntimeGenesisConfig {
		system: frequency_runtime::SystemConfig { ..Default::default() },
		balances: frequency_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: frequency_runtime::ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		collator_selection: frequency_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: frequency_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		#[cfg(any(not(feature = "frequency-no-relay"), feature = "frequency-lint-check"))]
		parachain_system: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		schemas: Default::default(),
		time_release: Default::default(),
		democracy: Default::default(),
		treasury: Default::default(),
		council: CouncilConfig { phantom: Default::default(), members: council_members },
		technical_committee: TechnicalCommitteeConfig {
			phantom: Default::default(),
			members: technical_committee_members,
		},
	};

	serde_json::to_value(&genesis).unwrap()
}
