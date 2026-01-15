//! Common trait for all swap CPI events to extract essential swap data

use solana_pubkey::Pubkey;

use super::{
    SwapCpiEvent, SwapCpiEvent2, SwapToBWithFeesCpiEventV2, SwapToCWithFeesCpiEventV2,
    SwapTobV2CpiEvent2, SwapTocV2CpiEvent2, SwapWithFeesCpiEvent, SwapWithFeesCpiEvent2,
    SwapWithFeesCpiEventEnhanced, SwapWithFeesCpiEventEnhanced2,
};

/// Common trait for all swap CPI events to extract essential swap data
pub trait SwapEventData {
    fn source_mint(&self) -> &Pubkey;
    fn destination_mint(&self) -> &Pubkey;
    fn source_token_account_owner(&self) -> &Pubkey;
    fn source_token_change(&self) -> u64;
    fn destination_token_change(&self) -> u64;
}

impl SwapEventData for SwapCpiEvent {
    fn source_mint(&self) -> &Pubkey {
        &self.source_mint
    }
    fn destination_mint(&self) -> &Pubkey {
        &self.destination_mint
    }
    fn source_token_account_owner(&self) -> &Pubkey {
        &self.source_token_account_owner
    }
    fn source_token_change(&self) -> u64 {
        self.source_token_change
    }
    fn destination_token_change(&self) -> u64 {
        self.destination_token_change
    }
}

impl SwapEventData for SwapCpiEvent2 {
    fn source_mint(&self) -> &Pubkey {
        &self.source_mint
    }
    fn destination_mint(&self) -> &Pubkey {
        &self.destination_mint
    }
    fn source_token_account_owner(&self) -> &Pubkey {
        &self.source_token_account_owner
    }
    fn source_token_change(&self) -> u64 {
        self.source_token_change
    }
    fn destination_token_change(&self) -> u64 {
        self.destination_token_change
    }
}

impl SwapEventData for SwapWithFeesCpiEvent {
    fn source_mint(&self) -> &Pubkey {
        &self.source_mint
    }
    fn destination_mint(&self) -> &Pubkey {
        &self.destination_mint
    }
    fn source_token_account_owner(&self) -> &Pubkey {
        &self.source_token_account_owner
    }
    fn source_token_change(&self) -> u64 {
        self.source_token_change
    }
    fn destination_token_change(&self) -> u64 {
        self.destination_token_change
    }
}

impl SwapEventData for SwapWithFeesCpiEvent2 {
    fn source_mint(&self) -> &Pubkey {
        &self.source_mint
    }
    fn destination_mint(&self) -> &Pubkey {
        &self.destination_mint
    }
    fn source_token_account_owner(&self) -> &Pubkey {
        &self.source_token_account_owner
    }
    fn source_token_change(&self) -> u64 {
        self.source_token_change
    }
    fn destination_token_change(&self) -> u64 {
        self.destination_token_change
    }
}

impl SwapEventData for SwapWithFeesCpiEventEnhanced {
    fn source_mint(&self) -> &Pubkey {
        &self.source_mint
    }
    fn destination_mint(&self) -> &Pubkey {
        &self.destination_mint
    }
    fn source_token_account_owner(&self) -> &Pubkey {
        &self.source_token_account_owner
    }
    fn source_token_change(&self) -> u64 {
        self.source_token_change
    }
    fn destination_token_change(&self) -> u64 {
        self.destination_token_change
    }
}

impl SwapEventData for SwapTocV2CpiEvent2 {
    fn source_mint(&self) -> &Pubkey {
        &self.source_mint
    }
    fn destination_mint(&self) -> &Pubkey {
        &self.destination_mint
    }
    fn source_token_account_owner(&self) -> &Pubkey {
        &self.source_token_account_owner
    }
    fn source_token_change(&self) -> u64 {
        self.source_token_change
    }
    fn destination_token_change(&self) -> u64 {
        self.destination_token_change
    }
}

impl SwapEventData for SwapToBWithFeesCpiEventV2 {
    fn source_mint(&self) -> &Pubkey {
        &self.source_mint
    }
    fn destination_mint(&self) -> &Pubkey {
        &self.destination_mint
    }
    fn source_token_account_owner(&self) -> &Pubkey {
        &self.source_token_account_owner
    }
    fn source_token_change(&self) -> u64 {
        self.source_token_change
    }
    fn destination_token_change(&self) -> u64 {
        self.destination_token_change
    }
}

impl SwapEventData for SwapToCWithFeesCpiEventV2 {
    fn source_mint(&self) -> &Pubkey {
        &self.source_mint
    }
    fn destination_mint(&self) -> &Pubkey {
        &self.destination_mint
    }
    fn source_token_account_owner(&self) -> &Pubkey {
        &self.source_token_account_owner
    }
    fn source_token_change(&self) -> u64 {
        self.source_token_change
    }
    fn destination_token_change(&self) -> u64 {
        self.destination_token_change
    }
}

impl SwapEventData for SwapTobV2CpiEvent2 {
    fn source_mint(&self) -> &Pubkey {
        &self.source_mint
    }
    fn destination_mint(&self) -> &Pubkey {
        &self.destination_mint
    }
    fn source_token_account_owner(&self) -> &Pubkey {
        &self.source_token_account_owner
    }
    fn source_token_change(&self) -> u64 {
        self.source_token_change
    }
    fn destination_token_change(&self) -> u64 {
        self.destination_token_change
    }
}

impl SwapEventData for SwapWithFeesCpiEventEnhanced2 {
    fn source_mint(&self) -> &Pubkey {
        &self.source_mint
    }
    fn destination_mint(&self) -> &Pubkey {
        &self.destination_mint
    }
    fn source_token_account_owner(&self) -> &Pubkey {
        &self.source_token_account_owner
    }
    fn source_token_change(&self) -> u64 {
        self.source_token_change
    }
    fn destination_token_change(&self) -> u64 {
        self.destination_token_change
    }
}

/// NOTE: Unified enum for swap CPI events with fallback parsing support.
/// When parsing inner instructions, the primary event type is tried first,
/// then falls back to alternative event types if not found.
/// Primary events are the ones that I found on solscan
#[derive(Clone, Debug)]
pub enum CpiEventWithFallback {
    // Primary events
    SwapCpiEvent2(SwapCpiEvent2),
    SwapWithFeesCpiEvent(SwapWithFeesCpiEvent),
    SwapWithFeesCpiEvent2(SwapWithFeesCpiEvent2),
    SwapWithFeesCpiEventEnhanced(SwapWithFeesCpiEventEnhanced),
    SwapTocV2CpiEvent2(SwapTocV2CpiEvent2),
    // Fallback events
    SwapToBWithFeesCpiEventV2(SwapToBWithFeesCpiEventV2),
    SwapToCWithFeesCpiEventV2(SwapToCWithFeesCpiEventV2),
    SwapTobV2CpiEvent2(SwapTobV2CpiEvent2),
    SwapWithFeesCpiEventEnhanced2(SwapWithFeesCpiEventEnhanced2),
}

impl SwapEventData for CpiEventWithFallback {
    fn source_mint(&self) -> &Pubkey {
        match self {
            CpiEventWithFallback::SwapCpiEvent2(e) => e.source_mint(),
            CpiEventWithFallback::SwapWithFeesCpiEvent(e) => e.source_mint(),
            CpiEventWithFallback::SwapWithFeesCpiEvent2(e) => e.source_mint(),
            CpiEventWithFallback::SwapWithFeesCpiEventEnhanced(e) => e.source_mint(),
            CpiEventWithFallback::SwapTocV2CpiEvent2(e) => e.source_mint(),
            CpiEventWithFallback::SwapToBWithFeesCpiEventV2(e) => e.source_mint(),
            CpiEventWithFallback::SwapToCWithFeesCpiEventV2(e) => e.source_mint(),
            CpiEventWithFallback::SwapTobV2CpiEvent2(e) => e.source_mint(),
            CpiEventWithFallback::SwapWithFeesCpiEventEnhanced2(e) => e.source_mint(),
        }
    }

    fn destination_mint(&self) -> &Pubkey {
        match self {
            CpiEventWithFallback::SwapCpiEvent2(e) => e.destination_mint(),
            CpiEventWithFallback::SwapWithFeesCpiEvent(e) => e.destination_mint(),
            CpiEventWithFallback::SwapWithFeesCpiEvent2(e) => e.destination_mint(),
            CpiEventWithFallback::SwapWithFeesCpiEventEnhanced(e) => e.destination_mint(),
            CpiEventWithFallback::SwapTocV2CpiEvent2(e) => e.destination_mint(),
            CpiEventWithFallback::SwapToBWithFeesCpiEventV2(e) => e.destination_mint(),
            CpiEventWithFallback::SwapToCWithFeesCpiEventV2(e) => e.destination_mint(),
            CpiEventWithFallback::SwapTobV2CpiEvent2(e) => e.destination_mint(),
            CpiEventWithFallback::SwapWithFeesCpiEventEnhanced2(e) => e.destination_mint(),
        }
    }

    fn source_token_account_owner(&self) -> &Pubkey {
        match self {
            CpiEventWithFallback::SwapCpiEvent2(e) => e.source_token_account_owner(),
            CpiEventWithFallback::SwapWithFeesCpiEvent(e) => e.source_token_account_owner(),
            CpiEventWithFallback::SwapWithFeesCpiEvent2(e) => e.source_token_account_owner(),
            CpiEventWithFallback::SwapWithFeesCpiEventEnhanced(e) => e.source_token_account_owner(),
            CpiEventWithFallback::SwapTocV2CpiEvent2(e) => e.source_token_account_owner(),
            CpiEventWithFallback::SwapToBWithFeesCpiEventV2(e) => e.source_token_account_owner(),
            CpiEventWithFallback::SwapToCWithFeesCpiEventV2(e) => e.source_token_account_owner(),
            CpiEventWithFallback::SwapTobV2CpiEvent2(e) => e.source_token_account_owner(),
            CpiEventWithFallback::SwapWithFeesCpiEventEnhanced2(e) => {
                e.source_token_account_owner()
            },
        }
    }

    fn source_token_change(&self) -> u64 {
        match self {
            CpiEventWithFallback::SwapCpiEvent2(e) => e.source_token_change(),
            CpiEventWithFallback::SwapWithFeesCpiEvent(e) => e.source_token_change(),
            CpiEventWithFallback::SwapWithFeesCpiEvent2(e) => e.source_token_change(),
            CpiEventWithFallback::SwapWithFeesCpiEventEnhanced(e) => e.source_token_change(),
            CpiEventWithFallback::SwapTocV2CpiEvent2(e) => e.source_token_change(),
            CpiEventWithFallback::SwapToBWithFeesCpiEventV2(e) => e.source_token_change(),
            CpiEventWithFallback::SwapToCWithFeesCpiEventV2(e) => e.source_token_change(),
            CpiEventWithFallback::SwapTobV2CpiEvent2(e) => e.source_token_change(),
            CpiEventWithFallback::SwapWithFeesCpiEventEnhanced2(e) => e.source_token_change(),
        }
    }

    fn destination_token_change(&self) -> u64 {
        match self {
            CpiEventWithFallback::SwapCpiEvent2(e) => e.destination_token_change(),
            CpiEventWithFallback::SwapWithFeesCpiEvent(e) => e.destination_token_change(),
            CpiEventWithFallback::SwapWithFeesCpiEvent2(e) => e.destination_token_change(),
            CpiEventWithFallback::SwapWithFeesCpiEventEnhanced(e) => e.destination_token_change(),
            CpiEventWithFallback::SwapTocV2CpiEvent2(e) => e.destination_token_change(),
            CpiEventWithFallback::SwapToBWithFeesCpiEventV2(e) => e.destination_token_change(),
            CpiEventWithFallback::SwapToCWithFeesCpiEventV2(e) => e.destination_token_change(),
            CpiEventWithFallback::SwapTobV2CpiEvent2(e) => e.destination_token_change(),
            CpiEventWithFallback::SwapWithFeesCpiEventEnhanced2(e) => e.destination_token_change(),
        }
    }
}
