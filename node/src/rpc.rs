use jsonrpsee::RpcModule;
use sc_client_api::AuxStore;
use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use std::sync::Arc;

pub type Block = zkghost_runtime::Block;

pub fn create_full<C>(
    client: Arc<C>,
    deny_unsafe: DenyUnsafe,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block> + AuxStore + Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, zkghost_runtime::AccountId, zkghost_runtime::Index>
        + pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, zkghost_runtime::Balance>,
{
    let mut module = RpcModule::new(());

    module.merge(substrate_frame_rpc_system::FullSystem::new(client.clone(), None).into_rpc())?;
    module.merge(pallet_transaction_payment_rpc::TransactionPayment::new(client).into_rpc())?;

    Ok(module)
}
