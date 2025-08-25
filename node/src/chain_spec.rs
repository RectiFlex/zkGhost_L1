use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_keyring::AccountKeyring;
use zkghost_runtime::{self as runtime, AccountId, Signature, AuraId, Balance};

type AccountPublic = <Signature as sp_runtime::traits::Verify>::Signer;

pub type ChainSpec = sc_service::GenericChainSpec<runtime::GenesisConfig>;

fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<sr25519::Public>,
{
    let pair = sr25519::Pair::from_string(&format!("//{}", seed), None).expect("valid static seed");
    AccountPublic::from(pair.public()).into_account()
}

fn get_aura_keys_from_seed(seed: &str) -> AuraId {
    sr25519::Pair::from_string(&format!("//{}", seed), None).expect("valid static seed").public().into()
}

pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        // Name
        "zkGhost Development",
        // ID
        "zkghost-dev",
        ChainType::Development,
        move || testnet_genesis(
            // initial authorities
            vec![get_aura_keys_from_seed("Alice")],
            // sudo
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            // endowed
            vec![
                (get_account_id_from_seed::<sr25519::Public>("Alice"), 1_000_000_000_000_000_000u128),
                (get_account_id_from_seed::<sr25519::Public>("Bob"), 1_000_000_000_000_000_000u128),
            ],
            true,
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
    _enable_println: bool,
) -> runtime::GenesisConfig {
    runtime::GenesisConfig {
        system: runtime::SystemConfig { code: zkghost_runtime::WASM_BINARY.to_vec(), ..Default::default() },
        balances: runtime::BalancesConfig { balances: endowed_accounts },
        sudo: runtime::SudoConfig { key: Some(root_key) },
        aura: runtime::AuraConfig { authorities: initial_authorities },
        grandpa: Default::default(),
        transaction_payment: Default::default(),
        timestamp: Default::default(),
        zkghost: Default::default(),
    }
}
