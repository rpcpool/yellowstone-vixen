use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapEvent {
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub pool_state: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub sender: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub token_account_0: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub token_account_1: Pubkey,
    pub amount_0: u64,
    pub transfer_fee_0: u64,
    pub amount_1: u64,
    pub transfer_fee_1: u64,
    pub zero_for_one: bool,
    pub sqrt_price_x64: u128,
    pub liquidity: u128,
    pub tick: i32,
}

impl SwapEvent {
    /// SwapEvent discriminator bytes
    pub const DISCRIMINATOR: [u8; 8] = [0x40, 0xc6, 0xcd, 0xe8, 0x26, 0x08, 0x71, 0xe2];

    /// Parse SwapEvent from program logs
    pub fn from_logs(logs: &[String]) -> Option<Self> {
        for log in logs {
            if let Some(swap_event) = Self::from_log(log) {
                return Some(swap_event);
            }
        }
        None
    }

    /// Parse SwapEvent from a single log message
    pub fn from_log(log: &str) -> Option<Self> {
        use base64::{engine::general_purpose, Engine as _};

        // Log format: "Program data: <base64_encoded_data>"
        if let Some(data_part) = log.strip_prefix("Program data: ") {
            if let Ok(decoded) = general_purpose::STANDARD.decode(data_part) {
                // Check if the decoded data starts with SwapEvent discriminator
                if decoded.starts_with(&Self::DISCRIMINATOR) {
                    // Skip the discriminator (8 bytes) and deserialize the SwapEvent
                    return Self::try_from_slice(&decoded[8..]).ok();
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discriminator_constant() {
        assert_eq!(SwapEvent::DISCRIMINATOR, [
            0x40, 0xc6, 0xcd, 0xe8, 0x26, 0x08, 0x71, 0xe2
        ]);
    }

    #[test]
    fn test_parse_real_log_data() {
        // Real program log data from a raydium CLMM swap transaction
        let log = "Program data: QMbN6CYIceIR1k+SYKqPB3yWQGZQmTtdQubbhevD6b4aoY6gi9rcO6pR7nSXYVvM8Eym4QndqRw7kvVco9i1uX2Z0DoAzSy+AfOvOVQfNtMkQIEiQ7eHfGpczhYfBa2AfkwwbCFwdvpKnynX6XRVIAWPTbBhAjJGYva2LiWxDrOG0+PDd5/G85jy+AAAAAAAAAAAAAAAAAA91T4tCQAAAAAAAAAAAAAAAL8Wwkf4EkomMQAAAAAAAADDsC//Fg4AAAAAAAAAAAAATTABAA==";

        let result = SwapEvent::from_log(log);
        assert!(result.is_some(), "Should successfully parse real log data");

        let swap_event = result.unwrap();

        // Verify the parsing worked by checking some fields
        assert_eq!(swap_event.amount_0, 16315032);
        assert_eq!(swap_event.transfer_fee_0, 0);
        assert_eq!(swap_event.amount_1, 39413798205);
        assert_eq!(swap_event.transfer_fee_1, 0);
        assert_eq!(swap_event.zero_for_one, false);
    }

    #[test]
    fn test_invalid_log_format() {
        let invalid_log = "Invalid log format";
        let result = SwapEvent::from_log(invalid_log);
        assert!(result.is_none(), "Should not parse invalid log format");
    }

    #[test]
    fn test_invalid_discriminator() {
        let log_with_invalid_discriminator =
            "Program data: AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=";
        let result = SwapEvent::from_log(log_with_invalid_discriminator);
        assert!(
            result.is_none(),
            "Should not parse with invalid discriminator"
        );
    }
}
