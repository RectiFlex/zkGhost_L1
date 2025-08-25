use jsonrpsee::RpcModule;
use zkghost_runtime as runtime;

pub fn create_full() -> RpcModule<()> {
    let module = RpcModule::new(());
    // TODO: add pallets' RPCs if any; balances/tx-payment are standard
    module
}
