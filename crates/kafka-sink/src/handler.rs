use yellowstone_vixen::{self as vixen, HandlerResult};
use yellowstone_vixen_block_coordinator::{
    AccountRecordSortKey, CoordinatorHandle, ParseStatsKind, InstructionRecordSortKey,
};
use yellowstone_vixen_core::{
    instruction::{InstructionUpdate, Path},
    AccountUpdate, TransactionUpdate,
};

use crate::{
    events::PreparedRecord,
    sink::{KafkaSink, ParsedOutput},
};

/// Handler that parses transaction instructions eagerly (at processed commitment)
/// and sends the resulting `PreparedRecord`s to the `BlockMachineCoordinator`.
///
/// After processing all instructions in a transaction, signals `TransactionParsed`
/// so the coordinator can track the fully-parsed gate.
///
/// Also handles account updates when registered as a handler for `AccountUpdate`.
#[derive(Clone)]
pub struct BufferingHandler {
    parsers: KafkaSink,
    handle: CoordinatorHandle<PreparedRecord>,
}

impl std::fmt::Debug for BufferingHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferingHandler").finish()
    }
}

fn sort_key(tx_index: u64, path: &Path) -> InstructionRecordSortKey {
    InstructionRecordSortKey::new(
        tx_index,
        path.as_slice().iter().map(|&i| i as usize).collect(),
    )
}

impl BufferingHandler {
    pub fn new(parsers: KafkaSink, handle: CoordinatorHandle<PreparedRecord>) -> Self {
        Self { parsers, handle }
    }

    /// Run secondary filters on an instruction and send any additional records to the coordinator.
    async fn apply_secondary_filters_to_transaction(
        &self,
        slot: u64,
        tx_index: u64,
        path: &Path,
        ix: &InstructionUpdate,
        primary_parsed: Option<&ParsedOutput>,
    ) {
        for filter in self.parsers.secondary_filters() {
            if let Some(filtered) = filter.filter(ix, primary_parsed).await {
                let record = self.parsers.prepare_decoded_instruction_record(
                    slot,
                    &ix.shared.signature,
                    path,
                    filtered,
                    filter.topic(),
                );
                if let Err(e) = self
                    .handle
                    .send_instruction_parsed(slot, sort_key(tx_index, path), record)
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
                let (result, had_error) = self
                    .parsers
                    .parse_instruction(slot, &ix.shared.signature, &ix.path, ix)
                    .await;

                let primary_parsed = if let Some((record, parsed)) = result {
                    if let Err(e) = self
                        .handle
                        .send_instruction_parsed(slot, sort_key(tx_index, &ix.path), record)
                        .await
                    {
                        tracing::error!(?e, slot, tx_index, "Failed to send parsed record");
                    }
                    parsed
                } else {
                    let kind = if had_error {
                        ParseStatsKind::InstructionError
                    } else {
                        ParseStatsKind::InstructionFiltered
                    };
                    let _ = self.handle.send_parse_stats(slot, kind).await;
                    None
                };

                self.apply_secondary_filters_to_transaction(slot, tx_index, &ix.path, ix, primary_parsed.as_ref())
                    .await;
            }
        }

        // Signal this transaction is fully parsed.
        let _ = self.handle.send_transaction_parsed(slot).await;
        Ok(())
    }
}

impl vixen::Handler<AccountUpdate, AccountUpdate> for BufferingHandler {
    async fn handle(
        &self,
        update: &AccountUpdate,
        _raw: &AccountUpdate,
    ) -> HandlerResult<()> {
        let slot = update.slot;

        let (record, had_error) = self.parsers.parse_account(slot, update).await;
        if let Some(record) = record {
            let info = update.account.as_ref().expect("parse_account returned Some so account must exist");
            let pubkey: [u8; 32] = match info.pubkey.as_slice().try_into() {
                Ok(pk) => pk,
                Err(_) => {
                    tracing::warn!(slot, pubkey_len = info.pubkey.len(), "Malformed pubkey in account update");
                    let _ = self.handle.send_parse_stats(slot, ParseStatsKind::AccountError).await;
                    return Ok(());
                },
            };
            let ingress_seq = info.write_version;
            let key = AccountRecordSortKey::new(ingress_seq, pubkey);
            if let Err(e) = self.handle.send_account_parsed(slot, key, record).await {
                tracing::error!(?e, slot, "Failed to send account parsed record");
            }
        } else {
            let kind = if had_error {
                ParseStatsKind::AccountError
            } else {
                ParseStatsKind::AccountFiltered
            };
            let _ = self.handle.send_parse_stats(slot, kind).await;
        }
        // No send_transaction_parsed — accounts don't affect the TX gate.
        Ok(())
    }
}
