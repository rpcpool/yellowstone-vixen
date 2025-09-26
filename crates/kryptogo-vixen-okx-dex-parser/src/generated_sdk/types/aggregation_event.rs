use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Clone)]
pub struct AggregationEvent {
    pub after_source_balance: u64,
    pub after_destination_balance: u64,
    pub source_token_change: u64,
    pub destination_token_change: u64,
}

impl AggregationEvent {
    /// Parse AggregationEvent from a single log message
    pub fn from_log(log: &str) -> Option<Self> { Self::parse_balance_log(log) }

    /// Parse AggregationEvent from the new log format containing balance information
    fn parse_balance_log(log: &str) -> Option<Self> {
        if !log.starts_with("Program log: ") {
            return None;
        }

        let log_content = log.strip_prefix("Program log: ")?;

        // Parse the format: "after_source_balance: 1410699797, after_destination_balance: 18851684497, source_token_change: 1410699795, destination_token_change: 7482284974"
        let mut after_source_balance = None;
        let mut after_destination_balance = None;
        let mut source_token_change = None;
        let mut destination_token_change = None;

        for part in log_content.split(", ") {
            if let Some(value) = part.strip_prefix("after_source_balance: ") {
                after_source_balance = value.parse::<u64>().ok();
            } else if let Some(value) = part.strip_prefix("after_destination_balance: ") {
                after_destination_balance = value.parse::<u64>().ok();
            } else if let Some(value) = part.strip_prefix("source_token_change: ") {
                source_token_change = value.parse::<u64>().ok();
            } else if let Some(value) = part.strip_prefix("destination_token_change: ") {
                destination_token_change = value.parse::<u64>().ok();
            }
        }

        // All fields must be present for a valid parse
        if let (Some(after_source), Some(after_dest), Some(source_change), Some(dest_change)) = (
            after_source_balance,
            after_destination_balance,
            source_token_change,
            destination_token_change,
        ) {
            Some(AggregationEvent {
                after_source_balance: after_source,
                after_destination_balance: after_dest,
                source_token_change: source_change,
                destination_token_change: dest_change,
            })
        } else {
            None
        }
    }

    /// Parse AggregationEvent from a collection of log messages
    pub fn from_logs(logs: &[String]) -> Option<Self> {
        logs.iter().find_map(|log| Self::from_log(log))
    }

    /// Parse all AggregationEvents from a collection of log messages
    pub fn from_logs_all(logs: &[String]) -> Vec<Self> {
        logs.iter().filter_map(|log| Self::from_log(log)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregation_event_from_balance_log() {
        let log = "Program log: after_source_balance: 1410699797, after_destination_balance: \
                   18851684497, source_token_change: 1410699795, destination_token_change: \
                   7482284974";

        let aggregation_event = AggregationEvent::from_log(log);

        assert!(
            aggregation_event.is_some(),
            "AggregationEvent should be parsed successfully"
        );

        let event = aggregation_event.unwrap();

        // Verify the parsed values
        assert_eq!(event.after_source_balance, 1410699797);
        assert_eq!(event.after_destination_balance, 18851684497);
        assert_eq!(event.source_token_change, 1410699795);
        assert_eq!(event.destination_token_change, 7482284974);

        // Log the parsed values for debugging
        println!("Parsed AggregationEvent:");
        println!("  after_source_balance: {}", event.after_source_balance);
        println!(
            "  after_destination_balance: {}",
            event.after_destination_balance
        );
        println!("  source_token_change: {}", event.source_token_change);
        println!(
            "  destination_token_change: {}",
            event.destination_token_change
        );
    }

    #[test]
    fn test_aggregation_event_from_logs_collection() {
        let logs = vec![
            "Some other log message".to_string(),
            "Program log: after_source_balance: 1410699797, after_destination_balance: \
             18851684497, source_token_change: 1410699795, destination_token_change: 7482284974"
                .to_string(),
            "Another log message".to_string(),
        ];

        let aggregation_event = AggregationEvent::from_logs(&logs);

        assert!(
            aggregation_event.is_some(),
            "AggregationEvent should be found in logs collection"
        );

        let event = aggregation_event.unwrap();
        assert_eq!(event.after_source_balance, 1410699797);
        assert_eq!(event.after_destination_balance, 18851684497);
        assert_eq!(event.source_token_change, 1410699795);
        assert_eq!(event.destination_token_change, 7482284974);
    }

    #[test]
    fn test_aggregation_event_from_invalid_log() {
        let invalid_log = "Some random log message";

        let aggregation_event = AggregationEvent::from_log(invalid_log);

        assert!(aggregation_event.is_none(), "Invalid log should not parse");
    }

    #[test]
    fn test_aggregation_event_incomplete_balance_log() {
        let incomplete_log =
            "Program log: after_source_balance: 1410699797, after_destination_balance: 18851684497";

        let aggregation_event = AggregationEvent::from_log(incomplete_log);

        assert!(
            aggregation_event.is_none(),
            "Incomplete balance log should not parse"
        );
    }

    #[test]
    fn test_aggregation_event_no_program_log_prefix() {
        let log_without_prefix = "after_source_balance: 1410699797, after_destination_balance: \
                                  18851684497, source_token_change: 1410699795, \
                                  destination_token_change: 7482284974";

        let aggregation_event = AggregationEvent::from_log(log_without_prefix);

        assert!(
            aggregation_event.is_none(),
            "Log without 'Program log: ' prefix should not parse"
        );
    }

    #[test]
    fn test_aggregation_event_from_logs_all() {
        let logs = vec![
            "Some other log message".to_string(),
            "Program log: after_source_balance: 1410699797, after_destination_balance: \
             18851684497, source_token_change: 1410699795, destination_token_change: 7482284974"
                .to_string(),
            "Another log message".to_string(),
            "Program log: after_source_balance: 2500000000, after_destination_balance: \
             1500000000, source_token_change: 100000000, destination_token_change: 50000000"
                .to_string(),
            "Invalid log".to_string(),
            "Program log: after_source_balance: 999999999, after_destination_balance: 111111111, \
             source_token_change: 888888888, destination_token_change: 222222222"
                .to_string(),
            "Final log message".to_string(),
        ];

        let aggregation_events = AggregationEvent::from_logs_all(&logs);

        assert_eq!(
            aggregation_events.len(),
            3,
            "Should find 3 AggregationEvents in logs"
        );

        // Print all parsed events for debugging
        for (i, event) in aggregation_events.iter().enumerate() {
            println!("AggregationEvent {}:", i + 1);
            println!("  after_source_balance: {}", event.after_source_balance);
            println!(
                "  after_destination_balance: {}",
                event.after_destination_balance
            );
            println!("  source_token_change: {}", event.source_token_change);
            println!(
                "  destination_token_change: {}",
                event.destination_token_change
            );
            println!();
        }

        // Verify specific events
        assert_eq!(aggregation_events[0].after_source_balance, 1410699797);
        assert_eq!(aggregation_events[0].after_destination_balance, 18851684497);
        assert_eq!(aggregation_events[0].source_token_change, 1410699795);
        assert_eq!(aggregation_events[0].destination_token_change, 7482284974);

        assert_eq!(aggregation_events[1].after_source_balance, 2500000000);
        assert_eq!(aggregation_events[1].after_destination_balance, 1500000000);
        assert_eq!(aggregation_events[1].source_token_change, 100000000);
        assert_eq!(aggregation_events[1].destination_token_change, 50000000);

        assert_eq!(aggregation_events[2].after_source_balance, 999999999);
        assert_eq!(aggregation_events[2].after_destination_balance, 111111111);
        assert_eq!(aggregation_events[2].source_token_change, 888888888);
        assert_eq!(aggregation_events[2].destination_token_change, 222222222);
    }
}
