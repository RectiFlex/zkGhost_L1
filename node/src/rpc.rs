use jsonrpsee::RpcModule;
use sc_client_api::{AuxStore, HeaderBackend};
use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use std::sync::Arc;

pub type Block = zkghost_runtime::Block;

pub fn create_full<C, P>(
    client: Arc<C>,
    pool: Arc<P>,
    deny_unsafe: DenyUnsafe,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block> + AuxStore + HeaderBackend<Block> + Send + Sync + 'static,
    C::Api:
        substrate_frame_rpc_system::AccountNonceApi<Block, zkghost_runtime::AccountId, zkghost_runtime::Index>
        + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, zkghost_runtime::Balance>,
    P: TransactionPool + 'static,
{
    use substrate_frame_rpc_system::{FullSystem, SystemApiServer};
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};

    let mut module = RpcModule::new(());

    module.merge(FullSystem::new(client.clone(), pool, deny_unsafe).into_rpc())?;
    module.merge(TransactionPayment::new(client).into_rpc())?;

    Ok(module)
}
