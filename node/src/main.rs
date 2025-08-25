mod chain_spec;
mod service;
mod rpc;
mod cli;

use clap::Parser;
use sc_cli::{Result, SubstrateCli};

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    match &cli.subcommand {
        Some(cli::Subcommand::BuildSpec(cmd)) => cli.create_runner(cmd)?.sync_run(|config| cmd.run(config.chain_spec, config.network)),
        Some(cli::Subcommand::CheckBlock(cmd)) => cli.create_runner(cmd)?.async_run(|config| Ok((service::new_full(config)?, sc_service::TaskManager::new(config.tokio_handle.clone(), None).unwrap()))),
        Some(cli::Subcommand::ExportBlocks(cmd)) => cli.create_runner(cmd)?.async_run(|config| Ok((service::new_full(config)?, sc_service::TaskManager::new(config.tokio_handle.clone(), None).unwrap()))),
        Some(cli::Subcommand::ExportState(cmd)) => cli.create_runner(cmd)?.sync_run(|config| cmd.run(config.chain_spec)),
        Some(cli::Subcommand::ImportBlocks(cmd)) => cli.create_runner(cmd)?.async_run(|config| Ok((service::new_full(config)?, sc_service::TaskManager::new(config.tokio_handle.clone(), None).unwrap()))),
        Some(cli::Subcommand::PurgeChain(cmd)) => cli.create_runner(cmd)?.sync_run(|config| cmd.run(config.database)),
        Some(cli::Subcommand::Revert(cmd)) => cli.create_runner(cmd)?.async_run(|config| Ok((service::new_full(config)?, sc_service::TaskManager::new(config.tokio_handle.clone(), None).unwrap()))),
        Some(cli::Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(cli::Subcommand::Benchmark(cmd)) => cli.create_runner(cmd)?.sync_run(|config| cmd.run(config.chain_spec)),
        None => {
            let runner = cli.create_runner(&sc_cli::RunCmd::new())?;
            runner.run_node_until_exit(|config| async move { service::new_full(config) })
        }
    }
}
