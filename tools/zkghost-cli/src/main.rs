use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use sp_core::{crypto::Ss58Codec, sr25519, Pair};
use std::{fs, str::FromStr};
use subxt::config::polkadot::PlainTip as _; // tip type marker
use subxt::{dynamic::Value, OnlineClient};

#[derive(Parser, Debug)]
#[command(name = "zkghost", about = "zkGhost developer CLI")]
struct Cli {
    /// WebSocket endpoint
    #[arg(long, default_value = "ws://127.0.0.1:9944")]
    ws: String,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show basic chain info
    Info,
    /// Get free balance of an account (SS58)
    Balance { account: String },
    /// Transfer tokens from a dev seed (e.g., //Alice) to a destination SS58
    Transfer { from_seed: String, to: String, amount: u128 },
    /// Sudo: set a verifying key for a circuit id
    SetVk { sudo_seed: String, circuit_id: String, vk_json: String },
    /// Submit a proof for a circuit id
    SubmitProof { seed: String, circuit_id: String, proof_json: String, inputs_json: String },
}

#[derive(Deserialize)]
struct HexBytes { #[serde(rename = "bytesHex")] bytes_hex: String }

fn parse_hex_bytes(path: &str) -> Result<Vec<u8>> {
    let data = fs::read_to_string(path)?;
    let obj: HexBytes = serde_json::from_str(&data)?;
    let s = obj.bytes_hex.trim_start_matches("0x");
    Ok(hex::decode(s)?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let api = OnlineClient::<subxt::config::PolkadotConfig>::from_url(cli.ws.clone()).await?;

    match cli.cmd {
        Commands::Info => {
            let chain = api.rpc().system_chain().await?;
            let name = api.rpc().system_name().await?;
            let version = api.rpc().system_version().await?;
            let hash = api.rpc().block_hash(None).await?.unwrap();
            println!("Chain: {} ({} v{})\nGenesis/Latest: {:?}", chain, name, version, hash);
        }
        Commands::Balance { account } => {
            let who = sp_core::crypto::AccountId32::from_string(&account)?;
            // System.Account storage: returns AccountInfo, but we read free field via dynamic.
            let storage = subxt::dynamic::storage("System", "Account", vec![Value::from_bytes(who.as_ref())]);
            let acc_any = api.storage().at_latest().await?.fetch(&storage).await?;
            if let Some(val) = acc_any { println!("AccountInfo: {:?}", val); } else { println!("No account info found"); }
        }
        Commands::Transfer { from_seed, to, amount } => {
            let pair = sr25519::Pair::from_string(&from_seed, None).map_err(|e| anyhow!("bad seed: {e}"))?;
            let signer = subxt::tx::PairSigner::new(pair);
            let dest = sp_core::crypto::AccountId32::from_string(&to)?;
            let call = subxt::dynamic::tx("Balances", "transfer_keep_alive", vec![Value::from_bytes(dest.as_ref()), Value::u128(amount)]);
            let tx_hash = api.tx().sign_and_submit_default(&call, &signer).await?;
            println!("Submitted transfer, hash: {tx_hash:?}");
        }
        Commands::SetVk { sudo_seed, circuit_id, vk_json } => {
            let pair = sr25519::Pair::from_string(&sudo_seed, None).map_err(|e| anyhow!("bad seed: {e}"))?;
            let signer = subxt::tx::PairSigner::new(pair);
            let vk = parse_hex_bytes(&vk_json)?;
            let inner = subxt::dynamic::tx("Zkghost", "set_vk", vec![Value::from(circuit_id.clone()), Value::from_bytes(&vk)]);
            // wrap with sudo.sudo(call)
            let sudo_call = subxt::dynamic::tx("Sudo", "sudo", vec![Value::from(inner)]);
            let tx_hash = api.tx().sign_and_submit_default(&sudo_call, &signer).await?;
            println!("Submitted set_vk via sudo, hash: {tx_hash:?}");
        }
        Commands::SubmitProof { seed, circuit_id, proof_json, inputs_json } => {
            let pair = sr25519::Pair::from_string(&seed, None).map_err(|e| anyhow!("bad seed: {e}"))?;
            let signer = subxt::tx::PairSigner::new(pair);
            let proof = parse_hex_bytes(&proof_json)?;
            let inputs = parse_hex_bytes(&inputs_json)?;
            let call = subxt::dynamic::tx("Zkghost", "submit_proof", vec![Value::from(circuit_id.clone()), Value::from_bytes(&proof), Value::from_bytes(&inputs)]);
            let tx_hash = api.tx().sign_and_submit_default(&call, &signer).await?;
            println!("Submitted proof, hash: {tx_hash:?}");
        }
    }
    Ok(())
}
