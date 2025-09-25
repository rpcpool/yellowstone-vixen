//! Constants for common program addresses and utilities.

use crate::Pubkey;

/// Jupiter aggregator program address
pub const JUPITER_AGGREGATOR_ADDRESS: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";

/// OKX DEX aggregator program address
pub const OKX_AGGREGATOR_ADDRESS: &str = "6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma";

/// Check if a pubkey is a known aggregator (Jupiter or OKX).
pub fn is_known_aggregator(pubkey: &Pubkey) -> bool {
    let pubkey_str = pubkey.to_string();
    pubkey_str == JUPITER_AGGREGATOR_ADDRESS || pubkey_str == OKX_AGGREGATOR_ADDRESS
}
