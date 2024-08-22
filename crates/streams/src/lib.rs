pub(crate) use vixen::vixen_core;
use vixen::{config::VixenConfig, DynHandlerPack};
use vixen_core::{AccountUpdate, TransactionUpdate};

pub fn run<
    A: DynHandlerPack<AccountUpdate> + Send + Sync + 'static,
    X: DynHandlerPack<TransactionUpdate> + Send + Sync + 'static,
>(
    config: VixenConfig<vixen::config::NullConfig>,
) {
    // TODO
    vixen::Runtime::<A, X, _>::builder().build(config).run()
}
