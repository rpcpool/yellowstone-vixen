//! Metrics interface and backend support for the Vixen runtime.

use std::sync::LazyLock;

use prometheus::{IntCounter, Opts, Registry};
use yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof;

use crate::handler::PipelineErrors;

// TRANSACTIONS COUNTERS
pub(crate) static VIXEN_TRANSACTIONS_RECEIVED: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_transactions_received",
        "Total transactions received",
    ))
    .unwrap()
});
pub(crate) static VIXEN_TRANSACTIONS_SUCCESSFUL: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_transactions_successful",
        "Total successfully processed transactions",
    ))
    .unwrap()
});
pub(crate) static VIXEN_TRANSACTIONS_PARSING_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_transactions_parsing_errors",
        "Total parsing errors",
    ))
    .unwrap()
});
pub(crate) static VIXEN_TRANSACTIONS_HANDLER_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_transactions_handler_errors",
        "Total handler errors",
    ))
    .unwrap()
});

// ACCOUNTS COUNTERS
pub(crate) static VIXEN_ACCOUNTS_RECEIVED: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_accounts_received",
        "Total accounts received",
    ))
    .unwrap()
});
pub(crate) static VIXEN_ACCOUNTS_SUCCESSFUL: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_accounts_successful",
        "Total successfully processed accounts",
    ))
    .unwrap()
});
pub(crate) static VIXEN_ACCOUNTS_PARSING_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_accounts_parsing_errors",
        "Total parsing errors",
    ))
    .unwrap()
});
pub(crate) static VIXEN_ACCOUNTS_HANDLER_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_accounts_handler_errors",
        "Total handler errors",
    ))
    .unwrap()
});

// BLOCK METAS COUNTERS
pub(crate) static VIXEN_BLOCK_METAS_RECEIVED: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_block_metas_received",
        "Total block metas received",
    ))
    .unwrap()
});
pub(crate) static VIXEN_BLOCK_METAS_SUCCESSFUL: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_block_metas_successful",
        "Total successfully processed block metas",
    ))
    .unwrap()
});
pub(crate) static VIXEN_BLOCK_METAS_PARSING_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_block_metas_parsing_errors",
        "Total block metas parsing errors",
    ))
    .unwrap()
});
pub(crate) static VIXEN_BLOCK_METAS_HANDLER_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_block_metas_handler_errors",
        "Total block metas handler errors",
    ))
    .unwrap()
});

// INSTRUCTIONS COUNTERS
pub(crate) static VIXEN_INSTRUCTIONS_SUCCESSFUL: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_instructions_successful",
        "Total successfully processed instructions",
    ))
    .unwrap()
});
pub(crate) static VIXEN_INSTRUCTIONS_PARSING_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_instructions_parsing_errors",
        "Total parsing errors",
    ))
    .unwrap()
});
pub(crate) static VIXEN_INSTRUCTIONS_HANDLER_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_instructions_handler_errors",
        "Total handler errors",
    ))
    .unwrap()
});

// SLOTS COUNTERS
pub(crate) static VIXEN_SLOTS_RECEIVED: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new("vixen_slots_received", "Total slots received")).unwrap()
});
pub(crate) static VIXEN_SLOTS_SUCCESSFUL: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_slots_successful",
        "Total successfully processed slots",
    ))
    .unwrap()
});
pub(crate) static VIXEN_SLOTS_PARSING_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_slots_parsing_errors",
        "Total slots parsing errors",
    ))
    .unwrap()
});
pub(crate) static VIXEN_SLOTS_HANDLER_ERRORS: LazyLock<IntCounter> = LazyLock::new(|| {
    IntCounter::with_opts(Opts::new(
        "vixen_slots_handler_errors",
        "Total slots handler errors",
    ))
    .unwrap()
});

#[derive(Clone, Copy)]
pub(crate) enum UpdateType {
    Account,
    Transaction,
    BlockMeta,
    Instruction,
    Unknown,
    Slot,
}

impl From<&UpdateOneof> for UpdateType {
    fn from(update_type: &UpdateOneof) -> Self {
        match update_type {
            UpdateOneof::Account(_) => UpdateType::Account,
            UpdateOneof::Transaction(_) => UpdateType::Transaction,
            UpdateOneof::BlockMeta(_) => UpdateType::BlockMeta,
            UpdateOneof::Slot(_) => UpdateType::Slot,
            _ => UpdateType::Unknown,
        }
    }
}

pub(crate) fn increment_processed_updates(
    handle_result: &Result<(), PipelineErrors>,
    update_type: UpdateType,
) {
    match handle_result {
        Ok(()) => match update_type {
            UpdateType::Account => VIXEN_ACCOUNTS_SUCCESSFUL.inc(),
            UpdateType::Transaction => VIXEN_TRANSACTIONS_SUCCESSFUL.inc(),
            UpdateType::Instruction => VIXEN_INSTRUCTIONS_SUCCESSFUL.inc(),
            UpdateType::BlockMeta => VIXEN_BLOCK_METAS_SUCCESSFUL.inc(),
            UpdateType::Slot => VIXEN_SLOTS_SUCCESSFUL.inc(),
            _ => (),
        },
        Err(PipelineErrors::Parse(_)) => match update_type {
            UpdateType::Account => VIXEN_ACCOUNTS_PARSING_ERRORS.inc(),
            UpdateType::Transaction => VIXEN_TRANSACTIONS_PARSING_ERRORS.inc(),
            UpdateType::Instruction => VIXEN_INSTRUCTIONS_PARSING_ERRORS.inc(),
            UpdateType::BlockMeta => VIXEN_BLOCK_METAS_PARSING_ERRORS.inc(),
            UpdateType::Slot => VIXEN_SLOTS_PARSING_ERRORS.inc(),
            _ => (),
        },
        Err(PipelineErrors::Handlers(_)) => match update_type {
            UpdateType::Account => VIXEN_ACCOUNTS_HANDLER_ERRORS.inc(),
            UpdateType::Transaction => VIXEN_TRANSACTIONS_HANDLER_ERRORS.inc(),
            UpdateType::Instruction => VIXEN_INSTRUCTIONS_HANDLER_ERRORS.inc(),
            UpdateType::BlockMeta => VIXEN_BLOCK_METAS_HANDLER_ERRORS.inc(),
            UpdateType::Slot => VIXEN_SLOTS_HANDLER_ERRORS.inc(),
            _ => (),
        },
        _ => (),
    }
}

/// Increment accounts, transactions or block total updates received
///  based on the update type.
pub(crate) fn increment_received_updates(update_type: UpdateType) {
    match update_type {
        UpdateType::Account => VIXEN_ACCOUNTS_RECEIVED.inc(),
        UpdateType::Transaction => VIXEN_TRANSACTIONS_RECEIVED.inc(),
        UpdateType::BlockMeta => VIXEN_BLOCK_METAS_RECEIVED.inc(),
        UpdateType::Slot => VIXEN_SLOTS_RECEIVED.inc(),
        _ => (),
    }
}

/// Register the metrics with the provided registry.
/// This function is idempotent - if metrics are already registered, it will not panic.
pub fn register_metrics(registry: &Registry) {
    // Try to register each metric, but ignore "AlreadyReg" errors
    let _ = registry.register(Box::new(VIXEN_TRANSACTIONS_RECEIVED.clone()));
    let _ = registry.register(Box::new(VIXEN_TRANSACTIONS_SUCCESSFUL.clone()));
    let _ = registry.register(Box::new(VIXEN_TRANSACTIONS_PARSING_ERRORS.clone()));
    let _ = registry.register(Box::new(VIXEN_TRANSACTIONS_HANDLER_ERRORS.clone()));

    let _ = registry.register(Box::new(VIXEN_ACCOUNTS_RECEIVED.clone()));
    let _ = registry.register(Box::new(VIXEN_ACCOUNTS_SUCCESSFUL.clone()));
    let _ = registry.register(Box::new(VIXEN_ACCOUNTS_PARSING_ERRORS.clone()));
    let _ = registry.register(Box::new(VIXEN_ACCOUNTS_HANDLER_ERRORS.clone()));

    let _ = registry.register(Box::new(VIXEN_BLOCK_METAS_RECEIVED.clone()));
    let _ = registry.register(Box::new(VIXEN_BLOCK_METAS_SUCCESSFUL.clone()));
    let _ = registry.register(Box::new(VIXEN_BLOCK_METAS_PARSING_ERRORS.clone()));
    let _ = registry.register(Box::new(VIXEN_BLOCK_METAS_HANDLER_ERRORS.clone()));

    let _ = registry.register(Box::new(VIXEN_INSTRUCTIONS_SUCCESSFUL.clone()));
    let _ = registry.register(Box::new(VIXEN_INSTRUCTIONS_PARSING_ERRORS.clone()));
    let _ = registry.register(Box::new(VIXEN_INSTRUCTIONS_HANDLER_ERRORS.clone()));

    let _ = registry.register(Box::new(VIXEN_SLOTS_RECEIVED.clone()));
    let _ = registry.register(Box::new(VIXEN_SLOTS_SUCCESSFUL.clone()));
    let _ = registry.register(Box::new(VIXEN_SLOTS_PARSING_ERRORS.clone()));
    let _ = registry.register(Box::new(VIXEN_SLOTS_HANDLER_ERRORS.clone()));
}
