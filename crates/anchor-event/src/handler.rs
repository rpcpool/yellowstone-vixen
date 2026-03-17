//! Handler wrapper that routes instruction and event outputs to separate handlers.

use yellowstone_vixen::{Handler, HandlerResult};

use crate::AnchorEventOutput;

/// Handler that splits [`AnchorEventOutput`] into separate instruction and event handlers.
///
/// This is useful when you want to route parsed instructions and parsed events
/// to different sinks (e.g., separate Kafka topics).
///
/// # Example
///
/// ```rust,ignore
/// let handler = AnchorEventHandler::new(
///     ix_kafka_handler,   // sends instructions to "pump_fun_instructions" topic
///     evt_kafka_handler,  // sends events to "pump_fun_events" topic
/// );
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AnchorEventHandler<IxH, EvtH> {
    ix_handler: IxH,
    evt_handler: EvtH,
}

impl<IxH, EvtH> AnchorEventHandler<IxH, EvtH> {
    /// Create a new handler that routes instructions and events to separate handlers.
    #[must_use]
    pub fn new(ix_handler: IxH, evt_handler: EvtH) -> Self {
        Self {
            ix_handler,
            evt_handler,
        }
    }
}

impl<IxOut, EvtOut, IxH, EvtH, R> Handler<AnchorEventOutput<IxOut, EvtOut>, R>
    for AnchorEventHandler<IxH, EvtH>
where
    R: Sync,
    IxOut: Sync,
    EvtOut: Sync,
    IxH: Handler<IxOut, R> + Send + Sync,
    EvtH: Handler<EvtOut, R> + Send + Sync,
{
    fn handle(
        &self,
        value: &AnchorEventOutput<IxOut, EvtOut>,
        raw_event: &R,
    ) -> impl Future<Output = HandlerResult<()>> + Send {
        async move {
            if let Some(ix) = &value.instruction {
                self.ix_handler.handle(ix, raw_event).await?;
            }
            for evt in &value.events {
                self.evt_handler.handle(evt, raw_event).await?;
            }
            Ok(())
        }
    }
}
