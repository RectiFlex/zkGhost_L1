mod chain_spec; mod service; mod rpc; mod cli;
use clap::Parser;
use sc_cli::{SubstrateCli, Result};

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    match &cli.subcommand {
        Some(cli::Subcommand::Run(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.run_node_until_exit(|config| async move {
                let (task_manager, _rpc) = service::new_full(config)?;
                Ok(task_manager)
            })
        }
        Some(cli::Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(cli::Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|_config| Ok((async move { Ok(()) }, sc_service::TaskManager::new(None, None).unwrap())))
        }
        Some(cli::Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|_config| Ok((async move { Ok(()) }, sc_service::TaskManager::new(None, None).unwrap())))
        }
        Some(cli::Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec))
        }
        Some(cli::Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|_config| Ok((async move { Ok(()) }, sc_service::TaskManager::new(None, None).unwrap())))
        }
        Some(cli::Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(cli::Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|_config| Ok((async move { Ok(()) }, sc_service::TaskManager::new(None, None).unwrap())))
        }
        Some(cli::Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(cli::Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec))
        }
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                let (task_manager, _rpc) = service::new_full(config)?;
                Ok(task_manager)
            })
        }
    }
}
