use sc_cli::{SubstrateCli, RuntimeVersion, Role, ChainSpec};
use clap::Parser;

use crate::chain_spec;

#[derive(Debug, Parser)]
#[command(name = "zkghost-node", about = "zkGhost Node (Aura/GRANDPA)")]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    BuildSpec(sc_cli::BuildSpecCmd),
    CheckBlock(sc_cli::CheckBlockCmd),
    ExportBlocks(sc_cli::ExportBlocksCmd),
    ExportState(sc_cli::ExportStateCmd),
    ImportBlocks(sc_cli::ImportBlocksCmd),
    PurgeChain(sc_cli::PurgeChainCmd),
    Revert(sc_cli::RevertCmd),
    Key(sc_cli::KeySubcommand),
    Benchmark(sc_cli::BenchmarkCmd),
}

impl SubstrateCli for Cli {
    fn impl_name() -> String { "zkGhost Node".into() }
    fn impl_version() -> String { env!("CARGO_PKG_VERSION").into() }
    fn description() -> String { "A Substrate-based zkGhost node".into() }
    fn author() -> String { env!("CARGO_PKG_AUTHORS").into() }
    fn support_url() -> String { "https://example.com".into() }
    fn copyright_start_year() -> i32 { 2025 }

    fn load_spec(&self, id: &str) -> Result<Box<dyn ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()),
            path => Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path)).map_err(|e| e.to_string())?),
        })
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &zkghost_runtime::version()
    }
}
