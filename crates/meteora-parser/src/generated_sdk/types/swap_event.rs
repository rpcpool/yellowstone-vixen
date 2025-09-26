use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SwapEvent {
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub lb_pair: Pubkey,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub from: Pubkey,
    pub start_bin_id: i32,
    pub end_bin_id: i32,
    pub amount_in: u64,
    pub amount_out: u64,
    pub swap_for_y: bool,
    pub fee: u64,
    pub protocol_fee: u64,
    pub fee_bps: u128,
    pub host_fee: u64,
}

impl SwapEvent {
    /// SwapEvent discriminator bytes
    pub const DISCRIMINATOR: [u8; 8] = [0x51, 0x6c, 0xe3, 0xbe, 0xcd, 0xd0, 0x0a, 0xc4];

    /// Parse SwapEvent from inner instruction data that starts with self CPI log prefix
    pub fn from_inner_instruction_data(data: &[u8]) -> Option<Self> {
        // Check if data starts with self CPI log prefix: 0xe445a52e51cb9a1d
        let cpi_log_prefix = [0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];
        if !data.starts_with(&cpi_log_prefix) {
            return None;
        }

        // Skip the CPI log prefix (8 bytes)
        let remaining_data = &data[8..];

        // Check if the remaining data starts with SwapEvent discriminator
        if !remaining_data.starts_with(&Self::DISCRIMINATOR) {
            return None;
        }

        // Skip the discriminator (8 bytes) and deserialize the SwapEvent
        Self::try_from_slice(&remaining_data[8..]).ok()
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
    fn test_cpi_log_prefix_detection() {
        let cpi_log_prefix = [0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];
        let swap_discriminator = [0x51, 0x6c, 0xe3, 0xbe, 0xcd, 0xd0, 0x0a, 0xc4];

        // Create test data with CPI log prefix + SwapEvent discriminator + mock data
        let mut test_data = Vec::new();
        test_data.extend_from_slice(&cpi_log_prefix);
        test_data.extend_from_slice(&swap_discriminator);
        test_data.extend_from_slice(&[0u8; 64]); // Mock SwapEvent data

        // Should detect the CPI log prefix
        assert!(test_data.starts_with(&cpi_log_prefix));

        // Should detect the SwapEvent discriminator after CPI prefix
        assert!(test_data[8..].starts_with(&swap_discriminator));
    }

    #[test]
    fn test_invalid_cpi_log_prefix() {
        let invalid_prefix = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let swap_discriminator = [0x51, 0x6c, 0xe3, 0xbe, 0xcd, 0xd0, 0x0a, 0xc4];

        let mut test_data = Vec::new();
        test_data.extend_from_slice(&invalid_prefix);
        test_data.extend_from_slice(&swap_discriminator);
        test_data.extend_from_slice(&[0u8; 64]);

        let result = SwapEvent::from_inner_instruction_data(&test_data);
        assert!(
            result.is_none(),
            "Should not parse with invalid CPI log prefix"
        );
    }

    #[test]
    fn test_parse_real_inner_instruction_data() {
        // Real inner instruction data from a meteora swap transaction
        let hex_data = "e445a52e51cb9a1d516ce3becdd00ac47cc2bedc7bfac9a859ed5d3642b1f4fc3e176ed0470ac57de959005a0e41728ab18634b5ad50625ab467a6229b68af9cc4cbf6d5ac3b620e6cb152205b93be6433ffffff32ffffff8096980000000000beaf200000000000013e3c00000000000003030000000000002c8817000000000000000000000000000000000000000000";

        // Convert hex string to bytes
        let data: Vec<u8> = (0..hex_data.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_data[i..i + 2], 16).unwrap())
            .collect();

        let result = SwapEvent::from_inner_instruction_data(&data);
        assert!(
            result.is_some(),
            "Should successfully parse real inner instruction data"
        );

        let swap_event = result.unwrap();

        // Verify the parsing worked by checking some fields
        assert_eq!(swap_event.start_bin_id, -205);
        assert_eq!(swap_event.amount_in, 10000000);
        assert_eq!(swap_event.amount_out, 2142142);
        assert!(swap_event.swap_for_y);
    }
}
