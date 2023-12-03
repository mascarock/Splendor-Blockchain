use splendor_chain_runtime::{
    AccountId, BalancesConfig, DifficultyAdjustmentConfig, EVMChainIdConfig, GenesisConfig,
    SystemConfig, WASM_BINARY,
};
use hex_literal::hex;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "Development",
        "dev",
        sc_service::ChainType::Development,
        || {
            genesis(
                wasm_binary,
                vec![],
                4_000_000,
            )
        },
        vec![],
        None,
        None,
        None,
        None,
        None,
    ))
}

pub fn testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "Testnet",
        "testnet",
        sc_service::ChainType::Local,
        || {
            genesis(
                wasm_binary,
                vec![],
                4_000_000,
            )
        },
        vec![],
        None,
        None,
        None,
        None,
        None,
    ))
}

fn genesis(
    wasm_binary: &[u8],
    endowed_accounts: Vec<AccountId>,
    initial_difficulty: u32,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        difficulty_adjustment: DifficultyAdjustmentConfig {
            initial_difficulty: initial_difficulty.into(),
        },

        evm: Default::default(),
        evm_chain_id: EVMChainIdConfig { chain_id: 1118 },
        ethereum: Default::default(),
        base_fee: Default::default(),
        transaction_payment: Default::default(),
    }
}
