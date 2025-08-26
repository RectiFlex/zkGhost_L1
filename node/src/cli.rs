use clap::{Parser, Subcommand as ClapSub};
use sc_cli::{ChainSpec, RuntimeVersion, SubstrateCli};

#[derive(Debug, Parser)]
#[command(name = "zkghost-node")]
pub struct Cli {
    #[command(flatten)]
    pub run: sc_cli::RunCmd,
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

#[derive(Debug, ClapSub)]
pub enum Subcommand {
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
    Run(sc_cli::RunCmd),
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
    fn description() -> String { "zkGhost L1 node".into() }
    fn author() -> String { "RectiFlex".into() }
    fn support_url() -> String { "https://github.com/RectiFlex/zkGhost_L1".into() }
    fn copyright_start_year() -> i32 { 2025 }
    fn load_spec(&self, id: &str) -> Result<Box<dyn ChainSpec>, String> {
        match id { _ => Ok(Box::new(crate::chain_spec::development_config()?)) }
    }
    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion { &zkghost_runtime::Version }
}
