use sc_client_api::ExecutorProvider;
use sc_consensus_aura::SlotProportion;
use sc_finality_grandpa::{self, SharedVoterState};
use sc_network::NetworkBlock;
use sc_service::{Configuration, TFullBackend, TFullClient, KeepBlocks, PruningMode, TaskManager, error::Error as ServiceError, PartialComponents};
use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
use sp_core::Encode;
use sp_runtime::traits::BlakeTwo256;
use zkghost_runtime as runtime;

pub type Block = runtime::Block;

pub fn new_partial(config: &Configuration) -> Result<PartialComponents<TFullClient<Block, runtime::RuntimeApi, sc_executor::NativeElseWasmExecutor<()>>, TFullBackend<Block>, (), sc_consensus::DefaultImportQueue<Block>, sc_transaction_pool::FullPool<Block, TFullClient<Block, runtime::RuntimeApi, sc_executor::NativeElseWasmExecutor<()>>>, (sc_consensus_aura::AuraBlockImport<TFullClient<Block, runtime::RuntimeApi, sc_executor::NativeElseWasmExecutor<()>>, TFullClient<Block, runtime::RuntimeApi, sc_executor::NativeElseWasmExecutor<()>>>, sc_finality_grandpa::GrandpaBlockImport<TFullBackend<Block>, Block, TFullClient<Block, runtime::RuntimeApi, sc_executor::NativeElseWasmExecutor<()>>, TFullClient<Block, runtime::RuntimeApi, sc_executor::NativeElseWasmExecutor<()>>>)>, ServiceError> {
    unimplemented!("Partial service wiring is scaffolded; implement similarly to node-template")
}

pub fn new_full(mut config: Configuration) -> Result<(TaskManager, Arc<TFullClient<Block, runtime::RuntimeApi, sc_executor::NativeElseWasmExecutor<()>>>), ServiceError> {
    unimplemented!("Full service wiring with Aura and Grandpa is scaffolded; integrate per node-template")
}
