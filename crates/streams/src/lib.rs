pub(crate) use vixen::vixen_core;
use vixen::{vixen_core::InstructionsUpdate, DynHandlerPack};
use vixen_core::AccountUpdate;

pub fn run<
    A: DynHandlerPack<AccountUpdate> + Send + Sync + 'static,
    X: DynHandlerPack<InstructionsUpdate> + Send + Sync + 'static,
>() {
    vixen::Runtime::<A, X, _>::builder().build().run()
}
