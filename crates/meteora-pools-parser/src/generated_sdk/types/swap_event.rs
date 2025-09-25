use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapEvent {
    pub in_amount: u64,
    pub out_amount: u64,
    pub trade_fee: u64,
    pub protocol_fee: u64,
    pub host_fee: u64,
}

impl SwapEvent {
    /// SwapEvent discriminator bytes
    pub const DISCRIMINATOR: [u8; 8] = [0x51, 0x6c, 0xe3, 0xbe, 0xcd, 0xd0, 0x0a, 0xc4];

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
        assert_eq!(
            SwapEvent::DISCRIMINATOR,
            [0x51, 0x6c, 0xe3, 0xbe, 0xcd, 0xd0, 0x0a, 0xc4]
        );
    }

    #[test]
    fn test_parse_real_log_data() {
        // Real program log data from a meteora pools swap transaction
        let log = "Program data: UWzjvs3QCsSPHnQAAAAAAHs/lNQBAAAAdDsAAAAAAADdDgAAAAAAAAAAAAAAAAAA";

        let result = SwapEvent::from_log(log);
        assert!(
            result.is_some(),
            "Should successfully parse real log data"
        );

        let swap_event = result.unwrap();

        // Verify the parsing worked by checking some fields
        assert_eq!(swap_event.in_amount, 7609999);
        assert_eq!(swap_event.out_amount, 7861452667);
    }

    #[test]
    fn test_invalid_log_format() {
        let invalid_log = "Invalid log format";
        let result = SwapEvent::from_log(invalid_log);
        assert!(result.is_none(), "Should not parse invalid log format");
    }

    #[test]
    fn test_invalid_discriminator() {
        let log_with_invalid_discriminator = "Program data: AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=";
        let result = SwapEvent::from_log(log_with_invalid_discriminator);
        assert!(result.is_none(), "Should not parse with invalid discriminator");
    }
}