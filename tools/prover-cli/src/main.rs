use clap::Parser;
use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "prover-cli", about = "Demo prover for zkGhost")] 
struct Args {
    #[arg(long, default_value = "artifacts/vk.json")] 
    vk_json: PathBuf,
    #[arg(long, default_value = "artifacts/proof.json")] 
    proof_json: PathBuf,
    #[arg(long, default_value = "artifacts/inputs.json")] 
    inputs_json: PathBuf,
}

#[derive(Serialize)]
struct BytesOut<'a> {
    #[serde(rename = "bytesHex")]
    bytes_hex: &'a str,
}

fn ensure_parent(p: &PathBuf) {
    if let Some(parent) = p.parent() { let _ = fs::create_dir_all(parent); }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    ensure_parent(&args.vk_json);
    ensure_parent(&args.proof_json);
    ensure_parent(&args.inputs_json);

    // Deterministic demo bytes (replace with real arkworks path under feature)
    let vk_hex = "0x766b2d64656d6f"; // "vk-demo"
    let proof_hex = "0x70726f6f662d64656d6f"; // "proof-demo"
    let inputs_hex = "0x696e707574732d64656d6f"; // "inputs-demo"

    // If compiled with zk-verify, you could construct real VK/proof here.
    #[cfg(feature = "zk-verify")]
    {
        // Placeholder: keep same demo hex until real circuit wiring is added.
        let _ = ();
    }

    write_json(&args.vk_json, vk_hex)?;
    write_json(&args.proof_json, proof_hex)?;
    write_json(&args.inputs_json, inputs_hex)?;

    println!("Wrote demo artifacts:\n  VK: {}\n  Proof: {}\n  Inputs: {}", args.vk_json.display(), args.proof_json.display(), args.inputs_json.display());
    Ok(())
}

fn write_json(path: &PathBuf, hex_str: &str) -> anyhow::Result<()> {
    let obj = BytesOut { bytes_hex: hex_str };
    let s = serde_json::to_string_pretty(&obj)?;
    fs::write(path, s)?;
    Ok(())
}
