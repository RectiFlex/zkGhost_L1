use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use zkghost_runtime::{self as runtime, AccountId, Signature, AuraId, Balance};

pub type ChainSpec = sc_service::GenericChainSpec<runtime::GenesisConfig>;

type AccountPublic = <Signature as sp_runtime::traits::Verify>::Signer;

fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<sr25519::Public>,
{
    let pair = sr25519::Pair::from_string(&format!("//{}", seed), None).expect("valid seed");
    AccountPublic::from(pair.public()).into_account()
}

fn get_aura_keys_from_seed(seed: &str) -> AuraId {
    let pair = sr25519::Pair::from_string(&format!("//{}", seed), None).expect("valid seed");
    pair.public().into()
}

pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "zkGhost Development",
        "zkghost-dev",
        ChainType::Development,
        move || testnet_genesis(
            vec![get_aura_keys_from_seed("Alice")],
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            vec![
                (get_account_id_from_seed::<sr25519::Public>("Alice"), 1_000_000_000_000_000_000u128),
                (get_account_id_from_seed::<sr25519::Public>("Bob"), 1_000_000_000_000_000_000u128),
            ],
        ),
        vec![],
        None,
        None,
        None,
        Some(sc_service::Properties::from_iter(vec![
            ("tokenSymbol".into(), serde_json::json!("GHOST")),
            ("tokenDecimals".into(), serde_json::json!(12)),
        ])),
        Default::default(),
    )
}

#[allow(clippy::too_many_arguments)]
fn testnet_genesis(
    initial_authorities: Vec<AuraId>,
    root_key: AccountId,
    endowed_accounts: Vec<(AccountId, Balance)>,
) -> runtime::GenesisConfig {
    runtime::GenesisConfig {
        system: runtime::SystemConfig { code: zkghost_runtime::WASM_BINARY.to_vec(), ..Default::default() },
        balances: runtime::BalancesConfig { balances: endowed_accounts },
        sudo: runtime::SudoConfig { key: Some(root_key) },
        aura: runtime::AuraConfig { authorities: initial_authorities },
        grandpa: Default::default(),
        transaction_payment: Default::default(),
        zk_ghost: Default::default(),
    }
}
