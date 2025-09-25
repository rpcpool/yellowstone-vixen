use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TradedEvent {
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub whirlpool: Pubkey,
    pub a_to_b: bool,
    pub pre_sqrt_price: u128,
    pub post_sqrt_price: u128,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub lp_fee: u64,
    pub protocol_fee: u64,
}

impl TradedEvent {
    /// TradedEvent discriminator bytes
    pub const DISCRIMINATOR: [u8; 8] = [0xe1, 0xca, 0x49, 0xaf, 0x93, 0x2b, 0xa0, 0x96];

    /// Parse TradedEvent from a single log message
    pub fn from_log(log: &str) -> Option<Self> {
        log.strip_prefix("Program data: ")
            .and_then(|base64_data| BASE64.decode(base64_data).ok())
            .filter(|data| data.starts_with(&Self::DISCRIMINATOR))
            .and_then(|data| Self::try_from_slice(&data[8..]).ok())
    }

    /// Parse TradedEvent from a collection of log messages
    pub fn from_logs(logs: &[String]) -> Option<Self> {
        logs.iter().find_map(|log| Self::from_log(log))
    }

    /// Parse all TradedEvents from a collection of log messages
    pub fn from_logs_all(logs: &[String]) -> Vec<Self> {
        logs.iter().filter_map(|log| Self::from_log(log)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traded_event_from_log() {
        let log = "Program data: 4cpJr5MroJZgaxfHU++0SJQANeRjkQUgy3JCDwtF3V/SIIs0EW1f1AAL2KOWkOIaBAAAAAAAAAAAvaC4NMeUGwQAAAAAAAAAAKYWAAAAAAAAhsZWAQAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAACAAAAAAAAAA==";

        let traded_event = TradedEvent::from_log(log);

        assert!(traded_event.is_some(), "TradedEvent should be parsed successfully");

        let event = traded_event.unwrap();

        // Log the parsed values for debugging
        println!("Parsed TradedEvent:");
        println!("  whirlpool: {}", event.whirlpool);
        println!("  a_to_b: {}", event.a_to_b);
        println!("  pre_sqrt_price: {}", event.pre_sqrt_price);
        println!("  post_sqrt_price: {}", event.post_sqrt_price);
        println!("  input_amount: {}", event.input_amount);
        println!("  output_amount: {}", event.output_amount);
        println!("  input_transfer_fee: {}", event.input_transfer_fee);
        println!("  output_transfer_fee: {}", event.output_transfer_fee);
        println!("  lp_fee: {}", event.lp_fee);
        println!("  protocol_fee: {}", event.protocol_fee);

        // Verify the discriminator was properly matched
        assert_eq!(event.whirlpool.to_string().len(), 44); // Valid Pubkey length
    }

    #[test]
    fn test_traded_event_from_logs_collection() {
        let logs = vec![
            "Some other log message".to_string(),
            "Program data: 4cpJr5MroJZgaxfHU++0SJQANeRjkQUgy3JCDwtF3V/SIIs0EW1f1AAL2KOWkOIaBAAAAAAAAAAAvaC4NMeUGwQAAAAAAAAAAKYWAAAAAAAAhsZWAQAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAACAAAAAAAAAA==".to_string(),
            "Another log message".to_string(),
        ];

        let traded_event = TradedEvent::from_logs(&logs);

        assert!(traded_event.is_some(), "TradedEvent should be found in logs collection");
    }

    #[test]
    fn test_traded_event_from_invalid_log() {
        let invalid_log = "Program data: invalidbase64data";

        let traded_event = TradedEvent::from_log(invalid_log);

        assert!(traded_event.is_none(), "Invalid log should not parse");
    }

    #[test]
    fn test_traded_event_no_program_data_prefix() {
        let log_without_prefix = "4cpJr5MroJZgaxfHU++0SJQANeRjkQUgy3JCDwtF3V/SIIs0EW1f1AAL2KOWkOIaBAAAAAAAAAAAvaC4NMeUGwQAAAAAAAAAAKYWAAAAAAAAhsZWAQAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAACAAAAAAAAAA==";

        let traded_event = TradedEvent::from_log(log_without_prefix);

        assert!(traded_event.is_none(), "Log without 'Program data: ' prefix should not parse");
    }

    #[test]
    fn test_traded_event_from_logs_all() {
        let logs = vec![
            "Some other log message".to_string(),
            "Program data: 4cpJr5MroJbM9foK0cOUJBZ7j5fJjWhQn9t+TirtBg+CxHGz6hWcGABmSF6SQb7MbwEAAAAAAAAAnYV8hHoJAHEBAAAAAAAAAA+i5QEAAAAA9CDqAAAAAAAAAAAAAAAAAAAAAAAAAAAAEK0AAAAAAADbGQAAAAAAAA==".to_string(),
            "Another log message".to_string(),
            "Program data: 4cpJr5MroJbOM6hJw6X9p4m/l1DaTq0Ed/7X0lBKeShP7RPc8Y6V+QHlvvDmw5HcdwAAAAAAAAAA+KPuJ8yC3HcAAAAAAAAAAPQg6gAAAAAAGFAzAAAAAAAAAAAAAAAAAAAAAAAAAAAA8AwAAAAAAADuAQAAAAAAAA==".to_string(),
            "Program data: 4cpJr5MroJayNpDX0HWNHV2LiVDOx6m018ea6P+1xroNvWKhmDeTWwHOjjQQFj3YdwAAAAAAAAAAnEE7ss7o13cAAAAAAAAAAJkS4lAFAAAArEggKgEAAAAAAAAAAAAAAAAAAAAAAAAABT15AAAAAAC3HRIAAAAAAA==".to_string(),
            "Invalid log".to_string(),
            "Program data: 4cpJr5MroJaEENrNGJ+gupqwy0UT6y818cJJaZc0kJ700LdVaoWZmgD65yoTYwYIAAEAAAAAAAAA4W0jiPcKCAABAAAAAAAAAKxIICoBAAAAAPIFKgEAAAAAAAAAAAAAAAAAAAAAAAAAz6MGAAAAAAD+/QAAAAAAAA==".to_string(),
            "Final log message".to_string(),
        ];

        let traded_events = TradedEvent::from_logs_all(&logs);

        assert_eq!(traded_events.len(), 4, "Should find 4 TradedEvents in logs");

        // Print all parsed events for debugging
        for (i, event) in traded_events.iter().enumerate() {
            println!("TradedEvent {}:", i + 1);
            println!("  whirlpool: {}", event.whirlpool);
            println!("  a_to_b: {}", event.a_to_b);
            println!("  pre_sqrt_price: {}", event.pre_sqrt_price);
            println!("  post_sqrt_price: {}", event.post_sqrt_price);
            println!("  input_amount: {}", event.input_amount);
            println!("  output_amount: {}", event.output_amount);
            println!("  input_transfer_fee: {}", event.input_transfer_fee);
            println!("  output_transfer_fee: {}", event.output_transfer_fee);
            println!("  lp_fee: {}", event.lp_fee);
            println!("  protocol_fee: {}", event.protocol_fee);
            println!();
        }

        // Verify all events are unique (different swap operations)
        for i in 0..traded_events.len() {
            for j in (i + 1)..traded_events.len() {
                assert_ne!(
                    traded_events[i], traded_events[j],
                    "TradedEvent {} and {} should be different", i + 1, j + 1
                );
            }
        }

        // Verify all events have valid whirlpool addresses
        for event in &traded_events {
            assert_eq!(event.whirlpool.to_string().len(), 44, "Valid Pubkey length");
        }
    }

    #[test]
    fn test_discriminator_constant() {
        assert_eq!(
            TradedEvent::DISCRIMINATOR,
            [0xe1, 0xca, 0x49, 0xaf, 0x93, 0x2b, 0xa0, 0x96]
        );
    }
}
