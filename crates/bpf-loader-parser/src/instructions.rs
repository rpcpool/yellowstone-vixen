pub use yellowstone_vixen_core::PublicKey;
use yellowstone_vixen_proc_macro::vixen;

// ── Instruction accounts ───────────────────────────────────────────

#[vixen]
#[derive(Clone, PartialEq)]
pub struct InitializeBufferAccounts {
    pub buffer: PublicKey,
    pub authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WriteAccounts {
    pub buffer: PublicKey,
    pub authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DeployAccounts {
    pub payer: PublicKey,
    pub program_data: PublicKey,
    pub program: PublicKey,
    pub buffer: PublicKey,
    pub rent: PublicKey,
    pub clock: PublicKey,
    pub system_program: PublicKey,
    pub authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct UpgradeAccounts {
    pub program_data: PublicKey,
    pub program: PublicKey,
    pub buffer: PublicKey,
    pub spill: PublicKey,
    pub rent: PublicKey,
    pub clock: PublicKey,
    pub authority: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetAuthorityAccounts {
    pub account: PublicKey,
    pub current_authority: PublicKey,
    pub new_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct CloseAccounts {
    pub close_target: PublicKey,
    pub recipient: PublicKey,
    pub authority: Option<PublicKey>,
    pub program: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ExtendProgramAccounts {
    pub program_data: PublicKey,
    pub program: PublicKey,
    pub system_program: Option<PublicKey>,
    pub payer: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct SetAuthorityCheckedAccounts {
    pub account: PublicKey,
    pub current_authority: PublicKey,
    pub new_authority: PublicKey,
}

// ── Instruction args ───────────────────────────────────────────────

#[vixen]
#[derive(Clone, PartialEq)]
pub struct WriteArgs {
    pub offset: u32,
    pub bytes: Vec<u8>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct DeployArgs {
    pub max_data_len: u64,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ExtendProgramArgs {
    pub additional_bytes: u32,
}

// ── Instruction wrappers ───────────────────────────────────────────

pub mod instruction {
    use super::vixen;

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct InitializeBuffer {
        pub accounts: Option<super::InitializeBufferAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Write {
        pub accounts: Option<super::WriteAccounts>,
        pub args: Option<super::WriteArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Deploy {
        pub accounts: Option<super::DeployAccounts>,
        pub args: Option<super::DeployArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Upgrade {
        pub accounts: Option<super::UpgradeAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct SetAuthority {
        pub accounts: Option<super::SetAuthorityAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct Close {
        pub accounts: Option<super::CloseAccounts>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct ExtendProgram {
        pub accounts: Option<super::ExtendProgramAccounts>,
        pub args: Option<super::ExtendProgramArgs>,
    }

    #[vixen]
    #[derive(Clone, PartialEq)]
    pub struct SetAuthorityChecked {
        pub accounts: Option<super::SetAuthorityCheckedAccounts>,
    }

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum Instruction {
        InitializeBuffer(InitializeBuffer),
        Write(Write),
        Deploy(Deploy),
        Upgrade(Upgrade),
        SetAuthority(SetAuthority),
        Close(Close),
        ExtendProgram(ExtendProgram),
        SetAuthorityChecked(SetAuthorityChecked),
    }
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct BpfLoaderProgram {
    #[hint(oneof = "instruction::Instruction", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub instruction: Option<instruction::Instruction>,
}
