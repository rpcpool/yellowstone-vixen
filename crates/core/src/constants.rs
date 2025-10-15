//! Constants for common program addresses and utilities.

use crate::Pubkey;

/// Jupiter aggregator program address
pub const JUPITER_AGGREGATOR_ADDRESS: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";

/// OKX DEX aggregator program address (v1)
pub const OKX_AGGREGATOR_ADDRESS: &str = "6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma";

/// OKX DEX aggregator program address (v2)
pub const OKX_AGGREGATOR_V2_ADDRESS: &str = "proVF4pMXVaYqmy4NjniPh4pqKNfMmsihgd4wdkCX3u";

/// Check if a pubkey is a known aggregator (Jupiter or OKX).
#[must_use]
pub fn is_known_aggregator(pubkey: &Pubkey) -> bool {
    let pubkey_str = pubkey.to_string();
    pubkey_str == JUPITER_AGGREGATOR_ADDRESS
        || pubkey_str == OKX_AGGREGATOR_ADDRESS
        || pubkey_str == OKX_AGGREGATOR_V2_ADDRESS
}
