use std::env;
use std::io::{self, Write};

fn print_help() {
    println!(
        "zkGhost node stub (offline)\n\n\
Usage:\n  zkghost-node [--chain-id <id>] [--role <authority|full|light>] [--help]\n\n\
This is a minimal CLI that prints startup info only. In a real deployment, replace this crate with\
 a Substrate-based node binary (e.g., node-template) and wire the runtime/pallet."
    );
}

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let git = option_env!("GIT_COMMIT_HASH").unwrap_or("unknown");
    let mut chain_id: Option<String> = None;
    let mut role: Option<String> = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--chain-id" => chain_id = args.next(),
            "--role" => role = args.next(),
            _ => {
                eprintln!("Unknown argument: {}", arg);
                print_help();
                std::process::exit(2);
            }
        }
    }

    println!("=== zkGhost Node Stub ===");
    println!("version: {}", version);
    println!("commit: {}", git);
    println!("chain_id: {}", chain_id.as_deref().unwrap_or("dev"));
    println!("role: {}", role.as_deref().unwrap_or("full"));
    println!("");
    println!("Note: This is a placeholder binary without Substrate.");
    println!("Integrate with node-template or your custom Substrate node externally.");

    print!("Starting... ");
    io::stdout().flush().ok();
    std::thread::sleep(std::time::Duration::from_millis(300));
    println!("ok");
}
