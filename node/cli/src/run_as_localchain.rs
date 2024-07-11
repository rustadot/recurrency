use crate::cli::Cli;
use frequency_service::block_sealing::frequency_dev_sealing;
use sc_cli::SubstrateCli;

pub fn run_as_localchain(cli: Cli) -> sc_service::Result<(), sc_cli::Error> {
	let runner = cli.create_runner(&cli.run.normalize())?;

	runner.run_node_until_exit(|config| async move {
		frequency_dev_sealing(
			config,
			cli.sealing,
			u16::from(cli.sealing_interval),
			cli.sealing_create_empty_blocks,
		)
		.map_err(Into::into)
	})
}
