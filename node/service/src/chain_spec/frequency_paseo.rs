#![allow(missing_docs)]
use common_primitives::node::AccountId;
use common_runtime::constants::{
	currency::EXISTENTIAL_DEPOSIT, RECURRENCY_LOCAL_TOKEN, TOKEN_DECIMALS,
};
use cumulus_primitives_core::ParaId;
use recurrency_runtime::{AuraId, CouncilConfig, Ss58Prefix, SudoConfig, TechnicalCommitteeConfig};
use polkadot_service::chain_spec::Extensions as RelayChainExtensions;
use sc_service::ChainType;
use sp_runtime::traits::AccountIdConversion;

use super::{get_account_id_from_seed, get_collator_keys_from_seed, get_properties, Extensions};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;
use sp_core::sr25519;

// Generic chain spec, in case when we don't have the native runtime.
pub type RelayChainSpec = sc_service::GenericChainSpec<RelayChainExtensions>;

#[allow(clippy::unwrap_used)]
/// Generates the Recurrency Paseo chain spec from the raw json
pub fn load_recurrency_paseo_spec() -> ChainSpec {
	ChainSpec::from_json_bytes(
		&include_bytes!("../../../../resources/recurrency-paseo.raw.json")[..],
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
fn template_session_keys(keys: AuraId) -> recurrency_runtime::SessionKeys {
	recurrency_runtime::SessionKeys { aura: keys }
}

/// Generates the chain spec for a local testnet
pub fn local_paseo_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let properties =
		get_properties(RECURRENCY_LOCAL_TOKEN, TOKEN_DECIMALS as u32, Ss58Prefix::get().into());

	ChainSpec::builder(
		recurrency_runtime::wasm_binary_unwrap(),
		Extensions {
			relay_chain: "paseo-local".into(), // You MUST set this to the correct network!
			para_id: 2000,
		},
	)
	.with_name("Recurrency Local Testnet")
	.with_protocol_id("recurrency-paseo-local")
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
	let genesis = recurrency_runtime::RuntimeGenesisConfig {
		system: recurrency_runtime::SystemConfig { ..Default::default() },
		balances: recurrency_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: recurrency_runtime::ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		collator_selection: recurrency_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: recurrency_runtime::SessionConfig {
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
		#[cfg(any(not(feature = "recurrency-no-relay"), feature = "recurrency-lint-check"))]
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
