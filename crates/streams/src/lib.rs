pub(crate) use vixen::vixen_core;
use vixen::DynHandlerPack;
use vixen_core::{AccountUpdate, TransactionUpdate};

pub fn run<
    A: DynHandlerPack<AccountUpdate> + Send + Sync + 'static,
    X: DynHandlerPack<TransactionUpdate> + Send + Sync + 'static,
>() {
    vixen::Runtime::<A, X, _>::builder().build().run()
}
