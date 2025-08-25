mod chain_spec; mod service; mod rpc; mod cli;
use cli::{Cli, Subcommand};
use clap::Parser;
use sc_cli::{SubstrateCli, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => { let runner = cli.create_runner(cmd)?; runner.sync_run(|config| cmd.run(config.chain_spec, config.network)) }
        Some(Subcommand::CheckBlock(cmd)) => { let runner = cli.create_runner(cmd)?; runner.async_run(|_config| Ok((async move { Ok(()) }, sc_service::TaskManager::new(_config.tokio_handle.clone(), None).unwrap()))) }
        Some(Subcommand::ExportBlocks(cmd)) => { let runner = cli.create_runner(cmd)?; runner.async_run(|_config| Ok((async move { Ok(()) }, sc_service::TaskManager::new(_config.tokio_handle.clone(), None).unwrap()))) }
        Some(Subcommand::ExportState(cmd)) => { let runner = cli.create_runner(cmd)?; runner.sync_run(|_config| cmd.run(_config.chain_spec)) }
        Some(Subcommand::ImportBlocks(cmd)) => { let runner = cli.create_runner(cmd)?; runner.async_run(|_config| Ok((async move { Ok(()) }, sc_service::TaskManager::new(_config.tokio_handle.clone(), None).unwrap()))) }
        Some(Subcommand::PurgeChain(cmd)) => { let runner = cli.create_runner(cmd)?; runner.sync_run(|_config| cmd.run(_config.database)) }
        Some(Subcommand::Revert(cmd)) => { let runner = cli.create_runner(cmd)?; runner.async_run(|_config| Ok((async move { Ok(()) }, sc_service::TaskManager::new(_config.tokio_handle.clone(), None).unwrap()))) }
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::Benchmark(cmd)) => { let runner = cli.create_runner(cmd)?; runner.sync_run(|_config| cmd.run(_config.chain_spec)) }
        None => {
            let runner = cli.create_runner(&sc_cli::RunCmd::new())?;
            runner.run_node_until_exit(|config| async move {
                // Placeholder: return error until service wiring finished
                Err(sc_service::error::Error::Other("node service wiring pending".into()))
            })
        }
    }
}
