use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

#[cfg(not(any(
	feature = "recurrency",
	feature = "recurrency-local",
	feature = "recurrency-no-relay",
	feature = "recurrency-testnet"
)))]
compile_error!(
	r#"You must enable one of these features:
- Mainnet: "recurrency"
- Recurrency Paseo: "recurrency-testnet"
- Local: "recurrency-local"
- No Relay: "recurrency-no-relay",
- All: "recurrency-lint-check"#
);

// Don't allow more than one main feature (except for benchmark/lint/check) so that we always have a good mainnet runtime
#[cfg(all(
	not(feature = "recurrency-lint-check"),
	feature = "recurrency",
	any(
		feature = "recurrency-no-relay",
		feature = "recurrency-local",
		feature = "recurrency-testnet"
	)
))]
compile_error!("\"Only one main feature can be enabled except for benchmark/lint/check with \"recurrency-lint-check\"");

#[cfg(all(
	not(feature = "recurrency-lint-check"),
	feature = "recurrency-no-relay",
	any(feature = "recurrency", feature = "recurrency-local", feature = "recurrency-testnet")
))]
compile_error!("\"Only one main feature can be enabled except for benchmark/lint/check with \"recurrency-lint-check\"");

#[cfg(all(
	not(feature = "recurrency-lint-check"),
	feature = "recurrency-local",
	any(feature = "recurrency", feature = "recurrency-no-relay", feature = "recurrency-testnet")
))]
compile_error!("\"Only one main feature can be enabled except for benchmark/lint/check with \"recurrency-lint-check\"");

#[cfg(all(
	not(feature = "recurrency-lint-check"),
	feature = "recurrency-testnet",
	any(feature = "recurrency", feature = "recurrency-no-relay", feature = "recurrency-local",)
))]
compile_error!("\"Only one main feature can be enabled except for benchmark/lint/check with \"recurrency-lint-check\"");

fn main() {
	generate_cargo_keys();

	rerun_if_git_head_changed();
}
