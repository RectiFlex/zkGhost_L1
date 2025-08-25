use jsonrpsee::RpcModule;
pub fn create_full() -> RpcModule<()> { RpcModule::new(()) }
