use std::sync::Arc;

use sc_client_api::ExecutorProvider;
use sc_consensus::SelectChain;
use sc_consensus_aura::sr25519::AuthorityId as AuraId;
use sc_executor::NativeElseWasmExecutor;
use sc_network_common::service::NetworkBlock;
use sc_service::{
    build_network, build_offchain_workers, new_full_parts,
    Configuration, TFullBackend, TFullClient, TaskManager, error::Error,
    PartialComponents,
};
use sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker};
use sp_api::ConstructRuntimeApi;
use sp_runtime::traits::{BlakeTwo256, Block as BlockT};

pub type Block = zkghost_runtime::Block;
pub type RuntimeApi = zkghost_runtime::RuntimeApi;

// Native executor dispatch provided by runtime
pub struct ExecutorDispatch;
impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
    type ExtendHostFunctions = ();

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        zkghost_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        zkghost_runtime::native_version()
    }
}

pub type FullClient = TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
pub type FullBackend = TFullBackend<Block>;

pub fn new_partial(
    config: &Configuration,
) -> Result<
    PartialComponents<
        FullClient,
        FullBackend,
        sc_consensus::LongestChain<FullBackend, Block>,
        sc_consensus::DefaultImportQueue<Block>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (Option<Telemetry>, Option<TelemetryWorker>),
    >,
    Error,
> {
    let telemetry_worker_and_handle = if let Some(endpoints) = config.telemetry_endpoints.clone().filter(|x| !x.is_empty()) {
        let worker = TelemetryWorker::new(16)?;
        (Some(worker), Some(worker.handle().new_telemetry(endpoints)))
    } else {
        (None, None)
    };

    let telemetry = telemetry_worker_and_handle.1.clone();

    let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
        config.runtime_cache_size,
    );

    let (client, backend, keystore_container, mut task_manager) = new_full_parts::<Block, RuntimeApi, _>(
        config,
        None,
        executor,
    )?;
    let client = Arc::new(client);

    if let Some(worker) = telemetry_worker_and_handle.0 {
        task_manager.spawn_handle().spawn("telemetry", None, worker.run());
    }

    let select_chain = sc_consensus::LongestChain::new(backend.blockchain());

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    // GRANDPA block import and link
    let (grandpa_block_import, _grandpa_link) = sc_finality_grandpa::block_import(client.clone(), &select_chain)?;

    // Aura import queue
    let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
    let import_queue = sc_consensus_aura::import_queue::<AuraId, _, _, _>(
        sc_consensus_aura::ImportQueueParams {
            block_import: grandpa_block_import.clone(),
            justification_import: Some(grandpa_block_import.clone()),
            client: client.clone(),
            create_inherent_data_providers: move |_, _| async move {
                let timestamp = pallet_timestamp::InherentDataProvider::from_system_time();
                Ok((timestamp,))
            },
            spawner: &task_manager.spawn_handle(),
            registry: config.prometheus_registry(),
            check_for_equivocation: Default::default(),
            telemetry: telemetry.as_ref().map(|t| t.handle()),
            compatibility_mode: Default::default(),
            slot_duration,
            block_proposal_slot_portion: sc_consensus_aura::SlotProportion::new(2f32 / 3f32),
            max_block_proposal_slot_portion: Some(sc_consensus_aura::SlotProportion::new(3f32 / 4f32)),
        },
    )?;

    Ok(PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (telemetry, telemetry_worker_and_handle.0),
    })
}

pub fn new_full(
    mut config: Configuration,
) -> Result<(TaskManager, sc_service::RpcHandlers), Error> {
    use sc_finality_grandpa::{self as grandpa, SharedVoterState};
    use sc_service::SpawnTasksParams;

    config.network.extra_sets.push(grandpa::grandpa_peers_set_config());

    let PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (mut telemetry, telemetry_worker),
    } = new_partial(&config)?;

    let (network, system_rpc_tx, network_starter) = build_network(sc_service::BuildNetworkParams {
        config: &config,
        client: client.clone(),
        transaction_pool: transaction_pool.clone(),
        spawn_handle: task_manager.spawn_handle(),
        import_queue,
        block_announce_validator_builder: None,
        warp_sync: None,
    })?;

    if config.offchain_worker.enabled {
        build_offchain_workers(&config, task_manager.spawn_handle(), client.clone(), network.clone());
    }

    let role = config.role.clone();
    let force_authoring = config.force_authoring;
    let name = config.network.node_name.clone();

    // Proposer factory
    let proposer_factory = sc_basic_authorship::ProposerFactory::new(
        task_manager.spawn_handle(),
        client.clone(),
        transaction_pool.clone(),
        config.prometheus_registry().cloned(),
        None,
    );

    let can_author_with = sc_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());
    let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

    // GRANDPA block import and link
    let (grandpa_block_import, grandpa_link) = grandpa::block_import(client.clone(), &select_chain)?;

    // Start AURA
    let aura_params = sc_consensus_aura::StartAuraParams {
        slot_duration,
        client: client.clone(),
        select_chain: select_chain.clone(),
        block_import: grandpa_block_import.clone(),
        proposer_factory,
        create_inherent_data_providers: move |_, _| async move {
            let timestamp = pallet_timestamp::InherentDataProvider::from_system_time();
            Ok((timestamp,))
        },
        force_authoring,
        backoff_authoring_blocks: None,
        keystore: keystore_container.sync_keystore(),
        sync_oracle: network.clone(),
        justification_sync_link: network.clone(),
        block_proposal_slot_portion: sc_consensus_aura::SlotProportion::new(2f32 / 3f32),
        max_block_proposal_slot_portion: Some(sc_consensus_aura::SlotProportion::new(3f32 / 4f32)),
        telemetry: telemetry.as_ref().map(|x| x.handle()),
        compatibility_mode: Default::default(),
        can_author_with,
    };
    let aura = sc_consensus_aura::start_aura::<AuraId, _, _, _, _>(aura_params)?;

    // Start GRANDPA voter
    let grandpa_config = grandpa::Config {
        gossip_duration: std::time::Duration::from_millis(333),
        justification_generation_period: 512,
        name: Some(name),
        observer_enabled: false,
        keystore: Some(keystore_container.sync_keystore()),
        local_role: role.clone(),
        telemetry: telemetry.as_ref().map(|x| x.handle()),
        protocol_name: grandpa::protocol_standard_name(&client.block_hash(0)?.unwrap_or_default()),
    };

    let grandpa_voter = grandpa::run_grandpa_voter(grandpa::GrandpaParams {
        config: grandpa_config,
        link: grandpa_link,
        network: network.clone(),
        inherent_data_providers: (),
        telemetry: telemetry.as_ref().map(|x| x.handle()),
        voting_rule: grandpa::VotingRulesBuilder::default().build(),
        spawn_handle: task_manager.spawn_handle(),
        prometheus_registry: config.prometheus_registry().cloned(),
        shared_voter_state: SharedVoterState::empty(),
    })?;

    // RPC builder
    let rpc_builder = {
        let client = client.clone();
        move |deny_unsafe, _| {
            super::rpc::create_full::<FullClient>(client.clone(), deny_unsafe)
        }
    };

    let rpc_handlers = sc_service::spawn_tasks(SpawnTasksParams {
        config,
        client: client.clone(),
        backend,
        task_manager: &mut task_manager,
        keystore: keystore_container.sync_keystore(),
        transaction_pool: transaction_pool.clone(),
        network: network.clone(),
        system_rpc_tx,
        telemetry: telemetry.as_ref().map(|x| x.handle()),
        rpc_builder: Box::new(rpc_builder),
    })?;

    network_starter.start_network();

    task_manager.spawn_essential_handle().spawn_blocking("aura", None, aura);
    task_manager.spawn_essential_handle().spawn_blocking("grandpa-voter", None, grandpa_voter);

    Ok((task_manager, rpc_handlers))
}
