use clap::Parser;
use std::path::PathBuf;

mod chain_spec;
mod service;
mod rpc;

#[derive(Debug, Parser)]
#[command(author, version, about = "zkGhost Node (Aura/Grandpa) — scaffold", long_about = None)]
struct Opt {
    /// Use dev chain
    #[arg(long)]
    dev: bool,
    /// Chain spec json path
    #[arg(long)]
    chain: Option<PathBuf>,
}

fn main() {
    let _opt = Opt::parse();
    println!("zkGhost node service is scaffolded. Integrate sc-service per Substrate node-template to run.");
}
