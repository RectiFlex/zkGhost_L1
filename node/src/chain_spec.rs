use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public};
use sp_keyring::AccountKeyring;
use sp_runtime::traits::{IdentifyAccount, Verify};
use zkghost_runtime::{AccountId, Signature, BalancesConfig, SudoConfig, AuraConfig, GrandpaConfig, SystemConfig, WASM_BINARY, opaque::SessionKeys};
use zkghost_runtime::GenesisConfig;

pub type AccountPublic = <Signature as Verify>::Signer;

fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None).expect("static values are valid; qed").public()
}

fn authority_keys_from_seed(seed: &str) -> (sr25519::Public, sp_finality_grandpa::AuthorityId) {
    (get_from_seed::<sr25519::Public>(seed), get_from_seed::<sp_finality_grandpa::AuthorityId>(seed))
}

fn endowed() -> Vec<AccountId> {
    use AccountKeyring::*;
    vec![Alice, Bob, Charlie, Dave, Eve, Ferdie].into_iter().map(|k| AccountId::from(k.to_account_id())).collect()
}

pub fn development_config() -> Result<sc_service::GenericChainSpec<GenesisConfig>, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not built. Build the runtime first.".to_string())?;

    let (aura_id, grandpa_id) = authority_keys_from_seed("Alice");

    Ok(sc_service::GenericChainSpec::from_genesis(
        "zkGhost Dev",
        "zkghost_dev",
        ChainType::Development,
        move || testnet_genesis(wasm_binary, vec![(aura_id.clone(), grandpa_id.clone())], AccountKeyring::Alice.to_account_id().into(), endowed()),
        vec![],
        None,
        None,
        None,
        None,
    ))
}

fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(sr25519::Public, sp_finality_grandpa::AuthorityId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig { code: wasm_binary.to_vec(), ..Default::default() },
        balances: BalancesConfig { balances: endowed_accounts.iter().cloned().map(|k| (k, 1u128 << 60)).collect() },
        aura: AuraConfig { authorities: initial_authorities.iter().map(|x| x.0.clone()).collect() },
        grandpa: GrandpaConfig { authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1u64)).collect() },
        sudo: SudoConfig { key: Some(root_key) },
        transaction_payment: Default::default(),
        zkghost: Default::default(),
    }
}
