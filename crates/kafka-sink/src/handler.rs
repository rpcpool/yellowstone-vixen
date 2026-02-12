use yellowstone_vixen::{self as vixen, HandlerResult};
use yellowstone_vixen_block_coordinator::{CoordinatorHandle, RecordSortKey};
use yellowstone_vixen_core::{
    instruction::{InstructionUpdate, Path},
    TransactionUpdate,
};

use crate::{
    events::PreparedRecord,
    sink::{ConfiguredParsers, ParsedInstruction},
};

/// Handler that parses transaction instructions eagerly (at processed commitment)
/// and sends the resulting `PreparedRecord`s to the `BlockMachineCoordinator`.
///
/// After processing all instructions in a transaction, signals `TransactionParsed`
/// so the coordinator can track the fully-parsed gate.
#[derive(Clone)]
pub struct BufferingHandler {
    parsers: ConfiguredParsers,
    handle: CoordinatorHandle<PreparedRecord>,
}

impl std::fmt::Debug for BufferingHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferingHandler").finish()
    }
}

fn sort_key(tx_index: u64, path: &Path) -> RecordSortKey {
    RecordSortKey::new(
        tx_index,
        path.as_slice().iter().map(|&i| i as usize).collect(),
    )
}

impl BufferingHandler {
    pub fn new(parsers: ConfiguredParsers, handle: CoordinatorHandle<PreparedRecord>) -> Self {
        Self { parsers, handle }
    }

    /// Run secondary filters on an instruction and send any additional records to the coordinator.
    async fn apply_secondary_filters(
        &self,
        slot: u64,
        tx_index: u64,
        path: &Path,
        ix: &InstructionUpdate,
        primary_parsed: Option<&ParsedInstruction>,
    ) {
        for filter in self.parsers.secondary_filters() {
            if let Some(filtered) = filter.filter(ix, primary_parsed).await {
                let record = self.parsers.prepare_decoded_record(
                    slot,
                    &ix.shared.signature,
                    path,
                    filtered,
                    filter.label(),
                    filter.topic(),
                );
                if let Err(e) = self
                    .handle
                    .send_parsed(slot, sort_key(tx_index, path), record)
                    .await
                {
                    tracing::error!(?e, slot, "Failed to send secondary record");
                }
            }
        }
    }
}

impl vixen::Handler<TransactionUpdate, TransactionUpdate> for BufferingHandler {
    async fn handle(
        &self,
        update: &TransactionUpdate,
        _raw: &TransactionUpdate,
    ) -> HandlerResult<()> {
        let slot = update.slot;
        let Some(ref tx_info) = update.transaction else {
            // Still count this as parsed so the gate count matches.
            let _ = self.handle.send_transaction_parsed(slot).await;
            return Ok(());
        };
        let tx_index = tx_info.index;

        let instructions = match InstructionUpdate::parse_from_txn(update) {
            Ok(ixs) => ixs,
            Err(e) => {
                tracing::warn!(?e, slot, tx_index, "Failed to parse instructions");
                let _ = self.handle.send_transaction_parsed(slot).await;
                return Ok(());
            },
        };

        for ix_update in &instructions {
            for ix in ix_update.visit_all() {
                let (record, primary_parsed) = self
                    .parsers
                    .parse_instruction(slot, &ix.shared.signature, &ix.path, ix)
                    .await;

                if let Err(e) = self
                    .handle
                    .send_parsed(slot, sort_key(tx_index, &ix.path), record)
                    .await
                {
                    tracing::error!(?e, slot, tx_index, "Failed to send parsed record");
                }

                self.apply_secondary_filters(slot, tx_index, &ix.path, ix, primary_parsed.as_ref())
                    .await;
            }
        }

        // Signal this transaction is fully parsed.
        let _ = self.handle.send_transaction_parsed(slot).await;
        Ok(())
    }
}
