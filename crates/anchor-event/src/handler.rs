use yellowstone_vixen::{Handler, HandlerResult};

use crate::AnchorEventOutput;

/// Wraps separate instruction and event handlers
#[derive(Debug, Clone, Copy)]
pub struct AnchorEventHandler<InstructionHandler, EventHandler> {
    instruction_handler: InstructionHandler,
    event_handler: EventHandler,
}

impl<InstructionHandler, EventHandler> AnchorEventHandler<InstructionHandler, EventHandler> {
    #[must_use]
    pub fn new(instruction_handler: InstructionHandler, event_handler: EventHandler) -> Self {
        Self {
            instruction_handler,
            event_handler,
        }
    }
}

impl<InstructionOut, EventOut, InstructionHandler, EventHandler, R>
    Handler<AnchorEventOutput<InstructionOut, EventOut>, R>
    for AnchorEventHandler<InstructionHandler, EventHandler>
where
    R: Sync,
    InstructionOut: Sync,
    EventOut: Sync,
    InstructionHandler: Handler<InstructionOut, R> + Send + Sync,
    EventHandler: Handler<EventOut, R> + Send + Sync,
{
    fn handle(
        &self,
        value: &AnchorEventOutput<InstructionOut, EventOut>,
        raw_event: &R,
    ) -> impl Future<Output = HandlerResult<()>> + Send {
        async move {
            if let Some(ix) = &value.instruction {
                self.instruction_handler.handle(ix, raw_event).await?;
            }

            for evt in &value.events {
                self.event_handler.handle(evt, raw_event).await?;
            }

            Ok(())
        }
    }
}
