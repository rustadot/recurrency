#![cfg_attr(not(feature = "std"), no_std)]

pub mod constants;
pub mod extensions;
pub mod fee;
pub mod weights;

/// Macro to set a value (e.g. when using the `parameter_types` macro) to either a production value
/// or to an environment variable or testing value (in case the `frequency-rococo-local` feature is selected or in instant-sealing mode).
/// Note that the environment variable is evaluated _at compile time_.
///
/// Usage:
/// ```Rust
/// parameter_types! {
/// 	// Note that the env variable version parameter cannot be const.
/// 	pub LaunchPeriod: BlockNumber = prod_or_local_or_env!(7 * DAYS, 1 * MINUTES, "FRQCY_LAUNCH_PERIOD");
/// 	pub const VotingPeriod: BlockNumber = prod_or_local_or_env!(7 * DAYS, 1 * MINUTES);
/// }
/// ```
#[macro_export]
macro_rules! prod_or_local_or_env {
	($prod:expr, $test:expr) => {
		if cfg!(feature = "frequency-rococo-local") {
			$test
		} else {
			$prod
		}
	};
	($prod:expr, $test:expr, $env:expr) => {
		if cfg!(feature = "frequency-rococo-local") {
			core::option_env!($env).map(|s| s.parse().ok()).flatten().unwrap_or($test)
		} else {
			$prod
		}
	};
}
