//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;

use crate::{
    instructions::{
        CloseOrderAndClaimTip as CloseOrderAndClaimTipIxAccounts,
        CreateOrder as CreateOrderIxAccounts, CreateOrderInstructionArgs as CreateOrderIxData,
        FlashTakeOrderEnd as FlashTakeOrderEndIxAccounts,
        FlashTakeOrderEndInstructionArgs as FlashTakeOrderEndIxData,
        FlashTakeOrderStart as FlashTakeOrderStartIxAccounts,
        FlashTakeOrderStartInstructionArgs as FlashTakeOrderStartIxData,
        InitializeGlobalConfig as InitializeGlobalConfigIxAccounts,
        InitializeVault as InitializeVaultIxAccounts,
        LogUserSwapBalancesEnd as LogUserSwapBalancesEndIxAccounts,
        LogUserSwapBalancesEndInstructionArgs as LogUserSwapBalancesEndIxData,
        LogUserSwapBalancesStart as LogUserSwapBalancesStartIxAccounts,
        LogUserSwapBalancesStartInstructionArgs as LogUserSwapBalancesStartIxData,
        TakeOrder as TakeOrderIxAccounts, TakeOrderInstructionArgs as TakeOrderIxData,
        UpdateGlobalConfig as UpdateGlobalConfigIxAccounts,
        UpdateGlobalConfigAdmin as UpdateGlobalConfigAdminIxAccounts,
        UpdateGlobalConfigInstructionArgs as UpdateGlobalConfigIxData,
        WithdrawHostTip as WithdrawHostTipIxAccounts,
    },
    ID,
};

/// Limo Instructions
#[derive(Debug)]
pub enum LimoProgramIx {
    InitializeGlobalConfig(InitializeGlobalConfigIxAccounts),
    InitializeVault(InitializeVaultIxAccounts),
    CreateOrder(CreateOrderIxAccounts, CreateOrderIxData),
    CloseOrderAndClaimTip(CloseOrderAndClaimTipIxAccounts),
    TakeOrder(TakeOrderIxAccounts, TakeOrderIxData),
    FlashTakeOrderStart(FlashTakeOrderStartIxAccounts, FlashTakeOrderStartIxData),
    FlashTakeOrderEnd(FlashTakeOrderEndIxAccounts, FlashTakeOrderEndIxData),
    UpdateGlobalConfig(UpdateGlobalConfigIxAccounts, UpdateGlobalConfigIxData),
    UpdateGlobalConfigAdmin(UpdateGlobalConfigAdminIxAccounts),
    WithdrawHostTip(WithdrawHostTipIxAccounts),
    LogUserSwapBalancesStart(
        LogUserSwapBalancesStartIxAccounts,
        LogUserSwapBalancesStartIxData,
    ),
    LogUserSwapBalancesEnd(
        LogUserSwapBalancesEndIxAccounts,
        LogUserSwapBalancesEndIxData,
    ),
}

#[derive(Debug, Copy, Clone)]
pub struct InstructionParser;

impl yellowstone_vixen_core::Parser for InstructionParser {
    type Input = yellowstone_vixen_core::instruction::InstructionUpdate;
    type Output = LimoProgramIx;

    fn id(&self) -> std::borrow::Cow<str> { "Limo::InstructionParser".into() }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .transaction_accounts([ID])
            .build()
            .unwrap()
    }

    async fn parse(
        &self,
        ix_update: &yellowstone_vixen_core::instruction::InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
        if ix_update.program.equals_ref(ID) {
            InstructionParser::parse_impl(ix_update)
        } else {
            Err(yellowstone_vixen_core::ParseError::Filtered)
        }
    }
}

impl yellowstone_vixen_core::ProgramParser for InstructionParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { ID.to_bytes().into() }
}

impl InstructionParser {
    pub(crate) fn parse_impl(
        ix: &yellowstone_vixen_core::instruction::InstructionUpdate,
    ) -> yellowstone_vixen_core::ParseResult<LimoProgramIx> {
        let accounts_len = ix.accounts.len();
        let ix_discriminator: [u8; 8] = ix.data[0..8].try_into()?;
        let mut ix_data = &ix.data[8..];
        match ix_discriminator {
            [113, 216, 122, 131, 225, 209, 22, 55] => {
                check_min_accounts_req(accounts_len, 3)?;
                let ix_accounts = InitializeGlobalConfigIxAccounts {
                    admin_authority: ix.accounts[0].0.into(),
                    pda_authority: ix.accounts[1].0.into(),
                    global_config: ix.accounts[2].0.into(),
                };
                Ok(LimoProgramIx::InitializeGlobalConfig(ix_accounts))
            },
            [48, 191, 163, 44, 71, 129, 63, 164] => {
                check_min_accounts_req(accounts_len, 7)?;
                let ix_accounts = InitializeVaultIxAccounts {
                    payer: ix.accounts[0].0.into(),
                    global_config: ix.accounts[1].0.into(),
                    pda_authority: ix.accounts[2].0.into(),
                    mint: ix.accounts[3].0.into(),
                    vault: ix.accounts[4].0.into(),
                    token_program: ix.accounts[5].0.into(),
                    system_program: ix.accounts[6].0.into(),
                };
                Ok(LimoProgramIx::InitializeVault(ix_accounts))
            },
            [141, 54, 37, 207, 237, 210, 250, 215] => {
                check_min_accounts_req(accounts_len, 13)?;
                let ix_accounts = CreateOrderIxAccounts {
                    maker: ix.accounts[0].0.into(),
                    global_config: ix.accounts[1].0.into(),
                    pda_authority: ix.accounts[2].0.into(),
                    order: ix.accounts[3].0.into(),
                    input_mint: ix.accounts[4].0.into(),
                    output_mint: ix.accounts[5].0.into(),
                    maker_ata: ix.accounts[6].0.into(),
                    input_vault: ix.accounts[7].0.into(),
                    input_token_program: ix.accounts[8].0.into(),
                    output_token_program: ix.accounts[9].0.into(),
                    system_program: ix.accounts[10].0.into(),
                    event_authority: ix.accounts[11].0.into(),
                    program: ix.accounts[12].0.into(),
                };
                let de_ix_data: CreateOrderIxData = BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(LimoProgramIx::CreateOrder(ix_accounts, de_ix_data))
            },
            [244, 27, 12, 226, 45, 247, 230, 43] => {
                check_min_accounts_req(accounts_len, 12)?;
                let ix_accounts = CloseOrderAndClaimTipIxAccounts {
                    maker: ix.accounts[0].0.into(),
                    order: ix.accounts[1].0.into(),
                    global_config: ix.accounts[2].0.into(),
                    pda_authority: ix.accounts[3].0.into(),
                    input_mint: ix.accounts[4].0.into(),
                    output_mint: ix.accounts[5].0.into(),
                    maker_input_ata: ix.accounts[6].0.into(),
                    input_vault: ix.accounts[7].0.into(),
                    input_token_program: ix.accounts[8].0.into(),
                    system_program: ix.accounts[9].0.into(),
                    event_authority: ix.accounts[10].0.into(),
                    program: ix.accounts[11].0.into(),
                };
                Ok(LimoProgramIx::CloseOrderAndClaimTip(ix_accounts))
            },
            [164, 84, 130, 189, 111, 58, 250, 200] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = UpdateGlobalConfigIxAccounts {
                    admin_authority: ix.accounts[0].0.into(),
                    global_config: ix.accounts[1].0.into(),
                };
                let de_ix_data: UpdateGlobalConfigIxData =
                    BorshDeserialize::deserialize(&mut ix_data)?;
                Ok(LimoProgramIx::UpdateGlobalConfig(ix_accounts, de_ix_data))
            },
            [184, 87, 23, 193, 156, 238, 175, 119] => {
                check_min_accounts_req(accounts_len, 2)?;
                let ix_accounts = UpdateGlobalConfigAdminIxAccounts {
                    admin_authority_cached: ix.accounts[0].0.into(),
                    global_config: ix.accounts[1].0.into(),
                };
                Ok(LimoProgramIx::UpdateGlobalConfigAdmin(ix_accounts))
            },
            [140, 246, 105, 165, 80, 85, 143, 18] => {
                check_min_accounts_req(accounts_len, 4)?;
                let ix_accounts = WithdrawHostTipIxAccounts {
                    admin_authority: ix.accounts[0].0.into(),
                    global_config: ix.accounts[1].0.into(),
                    pda_authority: ix.accounts[2].0.into(),
                    system_program: ix.accounts[3].0.into(),
                };
                Ok(LimoProgramIx::WithdrawHostTip(ix_accounts))
            },
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Instruction discriminator".to_owned(),
            )),
        }
    }
}

pub fn check_min_accounts_req(
    actual: usize,
    expected: usize,
) -> yellowstone_vixen_core::ParseResult<()> {
    if actual < expected {
        Err(yellowstone_vixen_core::ParseError::from(format!(
            "Too few accounts provided: expected {expected}, got {actual}"
        )))
    } else {
        Ok(())
    }
}
