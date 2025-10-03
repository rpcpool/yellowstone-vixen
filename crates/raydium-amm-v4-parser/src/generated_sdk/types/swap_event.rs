use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapBaseInEvent {
    pub amount_in: u64,
    pub minimum_out: u64,
    pub direction: u64,
    pub user_source: u64,
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub out_amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapBaseOutEvent {
    pub max_in: u64,
    pub amount_out: u64,
    pub direction: u64,
    pub user_source: u64,
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub direct_in: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SwapEvent {
    BaseIn(SwapBaseInEvent),
    BaseOut(SwapBaseOutEvent),
}

impl SwapEvent {
    /// Parse SwapEvent from program logs
    pub fn from_logs(logs: &[&str]) -> Option<Self> {
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

        // Log format: "Program log: ray_log: <base64_encoded_data>"
        if let Some(data_part) = log.strip_prefix("Program log: ray_log: ") {
            if let Ok(decoded) = general_purpose::STANDARD.decode(data_part) {
                if !decoded.is_empty() {
                    let discriminator = decoded[0];
                    let event_data = &decoded[1..]; // Skip the discriminator (1 byte)

                    match discriminator {
                        3 => {
                            // SwapBaseIn event
                            if let Ok(base_in_event) = SwapBaseInEvent::try_from_slice(event_data) {
                                return Some(SwapEvent::BaseIn(base_in_event));
                            }
                        },
                        4 => {
                            // SwapBaseOut event
                            if let Ok(base_out_event) = SwapBaseOutEvent::try_from_slice(event_data)
                            {
                                return Some(SwapEvent::BaseOut(base_out_event));
                            }
                        },
                        _ => {
                            // Unknown discriminator, skip
                        },
                    }
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
    fn test_parse_ray_log_data() {
        // Test data from ray_log
        let log = "Program log: ray_log: \
                   AzAbDwAAAAAAEvwdAAAAAAABAAAAAAAAADAbDwAAAAAAMaooRkgSAADAOkxwAQkAACeXHgAAAAAA";

        let result = SwapEvent::from_log(log);
        assert!(result.is_some(), "Should successfully parse ray_log data");

        let swap_event = result.unwrap();

        // Check if it's the correct variant based on discriminator
        match swap_event {
            SwapEvent::BaseIn(base_in_event) => {
                // Verify the specific fields from the test data
                assert_eq!(base_in_event.amount_in, 990000);
                assert_eq!(base_in_event.out_amount, 2004775);
                println!("Parsed as BaseIn: {base_in_event:?}");
            },
            SwapEvent::BaseOut(base_out_event) => {
                panic!("Expected BaseIn event, got BaseOut: {base_out_event:?}");
            },
        }
    }

    #[test]
    fn test_invalid_log_format() {
        let invalid_log = "Invalid log format";
        let result = SwapEvent::from_log(invalid_log);
        assert!(result.is_none(), "Should not parse invalid log format");
    }

    #[test]
    fn test_invalid_ray_log_discriminator() {
        let log_with_invalid_discriminator =
            "Program log: ray_log: AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=";
        let result = SwapEvent::from_log(log_with_invalid_discriminator);
        assert!(
            result.is_none(),
            "Should not parse with invalid discriminator"
        );
    }
}
