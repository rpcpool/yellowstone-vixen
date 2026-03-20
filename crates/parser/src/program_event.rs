//! Constants and helpers for program event support, used by proc-macro generated code.

/// 8-byte self-CPI event prefix.
///
/// When a program emits an event via self-CPI, the instruction data
/// starts with these 8 bytes, followed by the event discriminator and borsh payload.
///
/// Value: first 8 bytes of `sha256("anchor:event")`, stored as little-endian u64.
/// Defined in [`anchor-lang/src/event.rs`](https://github.com/coral-xyz/anchor/blob/v0.30.1/lang/src/event.rs#L55).
pub const EVENT_IX_TAG: [u8; 8] = 0x1d9a_cb51_2ea5_45e4_u64.to_le_bytes();

/// Check whether instruction data starts with the self-CPI event prefix.
#[inline]
pub fn is_cpi_event(data: &[u8]) -> bool { data.len() >= 8 && data[..8] == EVENT_IX_TAG }
