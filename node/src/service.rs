use std::sync::Arc;
use sc_client_api::ExecutorProvider;
use sc_consensus_aura::SlotProportion;
use sc_finality_grandpa as grandpa;
use sc_network::NetworkBlock;
use sc_service::{Configuration, TaskManager, error::Error as ServiceError, PartialComponents, TFullBackend, TFullClient};
use sc_telemetry::{Telemetry, TelemetryWorker, TelemetryWorkerHandle};
use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;

use zkghost_runtime as runtime;

pub type Block = runtime::Block;

pub struct ExecutorDispatch;
impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> { zkghost_runtime::api::dispatch(method, data) }
    fn native_version() -> sc_executor::NativeVersion { sc_executor::NativeVersion { runtime_version: zkghost_runtime::version(), can_author_with: Default::default() } }
}

pub type FullClient = TFullClient<Block, runtime::RuntimeApi, sc_executor::NativeElseWasmExecutor<ExecutorDispatch>>;

pub fn new_partial(config: &Configuration) -> Result<PartialComponents<
    FullClient,
    TFullBackend<Block>,
    sc_consensus::DefaultImportQueue<Block>,
    sc_transaction_pool::FullPool<Block, FullClient>,
    (sc_consensus_aura::AuraBlockImport<FullClient, FullClient>, grandpa::GrandpaBlockImport<TFullBackend<Block>, Block, FullClient, FullClient>, Option<Telemetry>, Option<TelemetryWorkerHandle>)
>, ServiceError> {
    // This follows the node-template pattern; full wiring should be done on a build-capable runner.
    Err(ServiceError::Other("Service wiring to be finalized on build-capable runner".into()))
}

pub fn new_full(config: Configuration) -> Result<(TaskManager, Arc<FullClient>), ServiceError> {
    Err(ServiceError::Other("Full service wiring to be finalized on build-capable runner".into()))
}
