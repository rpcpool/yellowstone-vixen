//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

#[cfg(feature = "shared-data")]
use std::sync::Arc;

#[cfg(feature = "shared-data")]
use yellowstone_vixen_core::InstructionUpdateOutput;

use crate::{
    deserialize_checked,
    instructions::{
        AddStrategy as AddStrategyIxAccounts, CollectDust as CollectDustIxAccounts,
        Deposit as DepositIxAccounts, DepositInstructionArgs as DepositIxData,
        DepositStrategy as DepositStrategyIxAccounts,
        DepositStrategyInstructionArgs as DepositStrategyIxData,
        EnableVault as EnableVaultIxAccounts, EnableVaultInstructionArgs as EnableVaultIxData,
        Initialize as InitializeIxAccounts, InitializeStrategy as InitializeStrategyIxAccounts,
        InitializeStrategyInstructionArgs as InitializeStrategyIxData,
        RemoveStrategy as RemoveStrategyIxAccounts, RemoveStrategy2 as RemoveStrategy2IxAccounts,
        RemoveStrategy2InstructionArgs as RemoveStrategy2IxData,
        SetOperator as SetOperatorIxAccounts, Withdraw as WithdrawIxAccounts,
        Withdraw2 as Withdraw2IxAccounts, Withdraw2InstructionArgs as Withdraw2IxData,
        WithdrawDirectlyFromStrategy as WithdrawDirectlyFromStrategyIxAccounts,
        WithdrawDirectlyFromStrategyInstructionArgs as WithdrawDirectlyFromStrategyIxData,
        WithdrawInstructionArgs as WithdrawIxData, WithdrawStrategy as WithdrawStrategyIxAccounts,
        WithdrawStrategyInstructionArgs as WithdrawStrategyIxData,
    },
    ID,
};

/// Vault Instructions
#[derive(Debug)]
#[cfg_attr(feature = "tracing", derive(strum_macros::Display))]
pub enum VaultProgramIx {
    Initialize(InitializeIxAccounts),
    EnableVault(EnableVaultIxAccounts, EnableVaultIxData),
    SetOperator(SetOperatorIxAccounts),
    InitializeStrategy(InitializeStrategyIxAccounts, InitializeStrategyIxData),
    RemoveStrategy(RemoveStrategyIxAccounts),
    RemoveStrategy2(RemoveStrategy2IxAccounts, RemoveStrategy2IxData),
    CollectDust(CollectDustIxAccounts),
    AddStrategy(AddStrategyIxAccounts),
    DepositStrategy(DepositStrategyIxAccounts, DepositStrategyIxData),
    WithdrawStrategy(WithdrawStrategyIxAccounts, WithdrawStrategyIxData),
    Withdraw2(Withdraw2IxAccounts, Withdraw2IxData),
    Deposit(DepositIxAccounts, DepositIxData),
    Withdraw(WithdrawIxAccounts, WithdrawIxData),
    WithdrawDirectlyFromStrategy(
        WithdrawDirectlyFromStrategyIxAccounts,
        WithdrawDirectlyFromStrategyIxData,
    ),
}

#[derive(Debug, Copy, Clone)]
pub struct InstructionParser;

impl yellowstone_vixen_core::Parser for InstructionParser {
    type Input = yellowstone_vixen_core::instruction::InstructionUpdate;
    #[cfg(not(feature = "shared-data"))]
    type Output = VaultProgramIx;
    #[cfg(feature = "shared-data")]
    type Output = InstructionUpdateOutput<VaultProgramIx>;

    fn id(&self) -> std::borrow::Cow<str> { "Vault::InstructionParser".into() }

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
            let res = InstructionParser::parse_impl(ix_update);

            #[cfg(feature = "tracing")]
            if let Err(e) = &res {
                let ix_discriminator: [u8; 8] = ix_update.data[0..8].try_into()?;

                tracing::info!(
                    name: "incorrectly_parsed_instruction",
                    name = "ix_update",
                    program = ID.to_string(),
                    ix = "deserialization_error",
                    discriminator = ?ix_discriminator,
                    error = ?e
                );
            }

            res
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
    ) -> yellowstone_vixen_core::ParseResult<<Self as yellowstone_vixen_core::Parser>::Output> {
        let accounts_len = ix.accounts.len();
        let accounts = &mut ix.accounts.iter();

        #[cfg(feature = "shared-data")]
        let shared_data = Arc::clone(&ix.shared);

        let ix_discriminator: [u8; 8] = ix.data[0..8].try_into()?;
        let ix_data = &ix.data[8..];
        let ix = match ix_discriminator {
            [175, 175, 109, 31, 13, 152, 155, 237] => {
                let expected_accounts_len = 8;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = InitializeIxAccounts {
                    vault: next_account(accounts)?,
                    payer: next_account(accounts)?,
                    token_vault: next_account(accounts)?,
                    token_mint: next_account(accounts)?,
                    lp_mint: next_account(accounts)?,
                    rent: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                    system_program: next_account(accounts)?,
                };
                Ok(VaultProgramIx::Initialize(ix_accounts))
            },
            [145, 82, 241, 156, 26, 154, 233, 211] => {
                let expected_accounts_len = 2;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = EnableVaultIxAccounts {
                    vault: next_account(accounts)?,
                    admin: next_account(accounts)?,
                };
                let de_ix_data: EnableVaultIxData =
                    deserialize_checked(ix_data, &ix_discriminator)?;
                Ok(VaultProgramIx::EnableVault(ix_accounts, de_ix_data))
            },
            [238, 153, 101, 169, 243, 131, 36, 1] => {
                let expected_accounts_len = 3;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = SetOperatorIxAccounts {
                    vault: next_account(accounts)?,
                    operator: next_account(accounts)?,
                    admin: next_account(accounts)?,
                };
                Ok(VaultProgramIx::SetOperator(ix_accounts))
            },
            [208, 119, 144, 145, 178, 57, 105, 252] => {
                let expected_accounts_len = 10;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = InitializeStrategyIxAccounts {
                    vault: next_account(accounts)?,
                    strategy_program: next_account(accounts)?,
                    strategy: next_account(accounts)?,
                    reserve: next_account(accounts)?,
                    collateral_vault: next_account(accounts)?,
                    collateral_mint: next_account(accounts)?,
                    admin: next_account(accounts)?,
                    system_program: next_account(accounts)?,
                    rent: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                };
                let de_ix_data: InitializeStrategyIxData =
                    deserialize_checked(ix_data, &ix_discriminator)?;
                Ok(VaultProgramIx::InitializeStrategy(ix_accounts, de_ix_data))
            },
            [185, 238, 33, 91, 134, 210, 97, 26] => {
                let expected_accounts_len = 10;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = RemoveStrategyIxAccounts {
                    vault: next_account(accounts)?,
                    strategy: next_account(accounts)?,
                    strategy_program: next_account(accounts)?,
                    collateral_vault: next_account(accounts)?,
                    reserve: next_account(accounts)?,
                    token_vault: next_account(accounts)?,
                    fee_vault: next_account(accounts)?,
                    lp_mint: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                    admin: next_account(accounts)?,
                };
                Ok(VaultProgramIx::RemoveStrategy(ix_accounts))
            },
            [138, 104, 208, 148, 126, 35, 195, 14] => {
                let expected_accounts_len = 12;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = RemoveStrategy2IxAccounts {
                    vault: next_account(accounts)?,
                    strategy: next_account(accounts)?,
                    strategy_program: next_account(accounts)?,
                    collateral_vault: next_account(accounts)?,
                    reserve: next_account(accounts)?,
                    token_vault: next_account(accounts)?,
                    token_admin_advance_payment: next_account(accounts)?,
                    token_vault_advance_payment: next_account(accounts)?,
                    fee_vault: next_account(accounts)?,
                    lp_mint: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                    admin: next_account(accounts)?,
                };
                let de_ix_data: RemoveStrategy2IxData =
                    deserialize_checked(ix_data, &ix_discriminator)?;
                Ok(VaultProgramIx::RemoveStrategy2(ix_accounts, de_ix_data))
            },
            [246, 149, 21, 82, 160, 74, 254, 240] => {
                let expected_accounts_len = 5;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = CollectDustIxAccounts {
                    vault: next_account(accounts)?,
                    token_vault: next_account(accounts)?,
                    token_admin: next_account(accounts)?,
                    admin: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                };
                Ok(VaultProgramIx::CollectDust(ix_accounts))
            },
            [64, 123, 127, 227, 192, 234, 198, 20] => {
                let expected_accounts_len = 3;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = AddStrategyIxAccounts {
                    vault: next_account(accounts)?,
                    strategy: next_account(accounts)?,
                    admin: next_account(accounts)?,
                };
                Ok(VaultProgramIx::AddStrategy(ix_accounts))
            },
            [246, 82, 57, 226, 131, 222, 253, 249] => {
                let expected_accounts_len = 10;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = DepositStrategyIxAccounts {
                    vault: next_account(accounts)?,
                    strategy: next_account(accounts)?,
                    token_vault: next_account(accounts)?,
                    fee_vault: next_account(accounts)?,
                    lp_mint: next_account(accounts)?,
                    strategy_program: next_account(accounts)?,
                    collateral_vault: next_account(accounts)?,
                    reserve: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                    operator: next_account(accounts)?,
                };
                let de_ix_data: DepositStrategyIxData =
                    deserialize_checked(ix_data, &ix_discriminator)?;
                Ok(VaultProgramIx::DepositStrategy(ix_accounts, de_ix_data))
            },
            [31, 45, 162, 5, 193, 217, 134, 188] => {
                let expected_accounts_len = 10;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = WithdrawStrategyIxAccounts {
                    vault: next_account(accounts)?,
                    strategy: next_account(accounts)?,
                    token_vault: next_account(accounts)?,
                    fee_vault: next_account(accounts)?,
                    lp_mint: next_account(accounts)?,
                    strategy_program: next_account(accounts)?,
                    collateral_vault: next_account(accounts)?,
                    reserve: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                    operator: next_account(accounts)?,
                };
                let de_ix_data: WithdrawStrategyIxData =
                    deserialize_checked(ix_data, &ix_discriminator)?;
                Ok(VaultProgramIx::WithdrawStrategy(ix_accounts, de_ix_data))
            },
            [80, 6, 111, 73, 174, 211, 66, 132] => {
                let expected_accounts_len = 7;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = Withdraw2IxAccounts {
                    vault: next_account(accounts)?,
                    token_vault: next_account(accounts)?,
                    lp_mint: next_account(accounts)?,
                    user_token: next_account(accounts)?,
                    user_lp: next_account(accounts)?,
                    user: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                };
                let de_ix_data: Withdraw2IxData = deserialize_checked(ix_data, &ix_discriminator)?;
                Ok(VaultProgramIx::Withdraw2(ix_accounts, de_ix_data))
            },
            [242, 35, 198, 137, 82, 225, 242, 182] => {
                let expected_accounts_len = 7;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = DepositIxAccounts {
                    vault: next_account(accounts)?,
                    token_vault: next_account(accounts)?,
                    lp_mint: next_account(accounts)?,
                    user_token: next_account(accounts)?,
                    user_lp: next_account(accounts)?,
                    user: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                };
                let de_ix_data: DepositIxData = deserialize_checked(ix_data, &ix_discriminator)?;
                Ok(VaultProgramIx::Deposit(ix_accounts, de_ix_data))
            },
            [183, 18, 70, 156, 148, 109, 161, 34] => {
                let expected_accounts_len = 7;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = WithdrawIxAccounts {
                    vault: next_account(accounts)?,
                    token_vault: next_account(accounts)?,
                    lp_mint: next_account(accounts)?,
                    user_token: next_account(accounts)?,
                    user_lp: next_account(accounts)?,
                    user: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                };
                let de_ix_data: WithdrawIxData = deserialize_checked(ix_data, &ix_discriminator)?;
                Ok(VaultProgramIx::Withdraw(ix_accounts, de_ix_data))
            },
            [201, 141, 146, 46, 173, 116, 198, 22] => {
                let expected_accounts_len = 12;
                check_min_accounts_req(accounts_len, expected_accounts_len)?;
                let ix_accounts = WithdrawDirectlyFromStrategyIxAccounts {
                    vault: next_account(accounts)?,
                    strategy: next_account(accounts)?,
                    reserve: next_account(accounts)?,
                    strategy_program: next_account(accounts)?,
                    collateral_vault: next_account(accounts)?,
                    token_vault: next_account(accounts)?,
                    lp_mint: next_account(accounts)?,
                    fee_vault: next_account(accounts)?,
                    user_token: next_account(accounts)?,
                    user_lp: next_account(accounts)?,
                    user: next_account(accounts)?,
                    token_program: next_account(accounts)?,
                };
                let de_ix_data: WithdrawDirectlyFromStrategyIxData =
                    deserialize_checked(ix_data, &ix_discriminator)?;
                Ok(VaultProgramIx::WithdrawDirectlyFromStrategy(
                    ix_accounts,
                    de_ix_data,
                ))
            },
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Instruction discriminator".to_owned(),
            )),
        };

        #[cfg(feature = "tracing")]
        match &ix {
            Ok(ix) => {
                tracing::info!(
                    name: "correctly_parsed_instruction",
                    name = "ix_update",
                    program = ID.to_string(),
                    ix = ix.to_string()
                );
            },
            Err(e) => {
                tracing::info!(
                    name: "incorrectly_parsed_instruction",
                    name = "ix_update",
                    program = ID.to_string(),
                    ix = "error",
                    discriminator = ?ix_discriminator,
                    error = ?e
                );
            },
        }

        #[cfg(not(feature = "shared-data"))]
        return ix;

        #[cfg(feature = "shared-data")]
        ix.map(|ix| InstructionUpdateOutput {
            parsed_ix: ix,
            shared_data,
        })
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

fn next_account<'a, T: Iterator<Item = &'a yellowstone_vixen_core::KeyBytes<32>>>(
    accounts: &mut T,
) -> Result<solana_pubkey::Pubkey, yellowstone_vixen_core::ParseError> {
    accounts
        .next()
        .ok_or(yellowstone_vixen_core::ParseError::from(
            "No more accounts to parse",
        ))
        .map(|acc| acc.0.into())
}

/// Gets the next optional account using the ommited account strategy (account is not passed at all at the instruction).
/// ### Be careful to use this function when more than one account is optional in the Instruction.
///  Only by order there is no way to which ones of the optional accounts are present.
pub fn next_optional_account<'a, T: Iterator<Item = &'a yellowstone_vixen_core::KeyBytes<32>>>(
    accounts: &mut T,
    actual_accounts_len: usize,
    expected_accounts_len: &mut usize,
) -> Result<Option<solana_pubkey::Pubkey>, yellowstone_vixen_core::ParseError> {
    if actual_accounts_len == *expected_accounts_len + 1 {
        *expected_accounts_len += 1;
        Ok(Some(next_account(accounts)?))
    } else {
        Ok(None)
    }
}

/// Gets the next optional account using the traditional Program ID strategy.
///  (If account key is the program ID, means account is not present)
pub fn next_program_id_optional_account<
    'a,
    T: Iterator<Item = &'a yellowstone_vixen_core::KeyBytes<32>>,
>(
    accounts: &mut T,
) -> Result<Option<solana_pubkey::Pubkey>, yellowstone_vixen_core::ParseError> {
    let account_key = next_account(accounts)?;
    if account_key.eq(&ID) {
        Ok(None)
    } else {
        Ok(Some(account_key))
    }
}

// #[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_core::proto::ParseProto;

    use super::{InitializeIxAccounts, InstructionParser, VaultProgramIx};
    use crate::{proto_def, proto_helpers::proto_types_parsers::IntoProto};
    impl IntoProto<proto_def::InitializeIxAccounts> for InitializeIxAccounts {
        fn into_proto(self) -> proto_def::InitializeIxAccounts {
            proto_def::InitializeIxAccounts {
                vault: self.vault.to_string(),
                payer: self.payer.to_string(),
                token_vault: self.token_vault.to_string(),
                token_mint: self.token_mint.to_string(),
                lp_mint: self.lp_mint.to_string(),
                rent: self.rent.to_string(),
                token_program: self.token_program.to_string(),
                system_program: self.system_program.to_string(),
            }
        }
    }
    use super::EnableVaultIxAccounts;
    impl IntoProto<proto_def::EnableVaultIxAccounts> for EnableVaultIxAccounts {
        fn into_proto(self) -> proto_def::EnableVaultIxAccounts {
            proto_def::EnableVaultIxAccounts {
                vault: self.vault.to_string(),
                admin: self.admin.to_string(),
            }
        }
    }
    use super::EnableVaultIxData;
    impl IntoProto<proto_def::EnableVaultIxData> for EnableVaultIxData {
        fn into_proto(self) -> proto_def::EnableVaultIxData {
            proto_def::EnableVaultIxData {
                enabled: self.enabled.into(),
            }
        }
    }
    use super::SetOperatorIxAccounts;
    impl IntoProto<proto_def::SetOperatorIxAccounts> for SetOperatorIxAccounts {
        fn into_proto(self) -> proto_def::SetOperatorIxAccounts {
            proto_def::SetOperatorIxAccounts {
                vault: self.vault.to_string(),
                operator: self.operator.to_string(),
                admin: self.admin.to_string(),
            }
        }
    }
    use super::InitializeStrategyIxAccounts;
    impl IntoProto<proto_def::InitializeStrategyIxAccounts> for InitializeStrategyIxAccounts {
        fn into_proto(self) -> proto_def::InitializeStrategyIxAccounts {
            proto_def::InitializeStrategyIxAccounts {
                vault: self.vault.to_string(),
                strategy_program: self.strategy_program.to_string(),
                strategy: self.strategy.to_string(),
                reserve: self.reserve.to_string(),
                collateral_vault: self.collateral_vault.to_string(),
                collateral_mint: self.collateral_mint.to_string(),
                admin: self.admin.to_string(),
                system_program: self.system_program.to_string(),
                rent: self.rent.to_string(),
                token_program: self.token_program.to_string(),
            }
        }
    }
    use super::InitializeStrategyIxData;
    impl IntoProto<proto_def::InitializeStrategyIxData> for InitializeStrategyIxData {
        fn into_proto(self) -> proto_def::InitializeStrategyIxData {
            proto_def::InitializeStrategyIxData {
                strategy_index: self.strategy_index.into(),
                other_bumps: self.other_bumps.into_iter().map(|x| x.into()).collect(),
                strategy_type: self.strategy_type as i32,
            }
        }
    }
    use super::RemoveStrategyIxAccounts;
    impl IntoProto<proto_def::RemoveStrategyIxAccounts> for RemoveStrategyIxAccounts {
        fn into_proto(self) -> proto_def::RemoveStrategyIxAccounts {
            proto_def::RemoveStrategyIxAccounts {
                vault: self.vault.to_string(),
                strategy: self.strategy.to_string(),
                strategy_program: self.strategy_program.to_string(),
                collateral_vault: self.collateral_vault.to_string(),
                reserve: self.reserve.to_string(),
                token_vault: self.token_vault.to_string(),
                fee_vault: self.fee_vault.to_string(),
                lp_mint: self.lp_mint.to_string(),
                token_program: self.token_program.to_string(),
                admin: self.admin.to_string(),
            }
        }
    }
    use super::RemoveStrategy2IxAccounts;
    impl IntoProto<proto_def::RemoveStrategy2IxAccounts> for RemoveStrategy2IxAccounts {
        fn into_proto(self) -> proto_def::RemoveStrategy2IxAccounts {
            proto_def::RemoveStrategy2IxAccounts {
                vault: self.vault.to_string(),
                strategy: self.strategy.to_string(),
                strategy_program: self.strategy_program.to_string(),
                collateral_vault: self.collateral_vault.to_string(),
                reserve: self.reserve.to_string(),
                token_vault: self.token_vault.to_string(),
                token_admin_advance_payment: self.token_admin_advance_payment.to_string(),
                token_vault_advance_payment: self.token_vault_advance_payment.to_string(),
                fee_vault: self.fee_vault.to_string(),
                lp_mint: self.lp_mint.to_string(),
                token_program: self.token_program.to_string(),
                admin: self.admin.to_string(),
            }
        }
    }
    use super::RemoveStrategy2IxData;
    impl IntoProto<proto_def::RemoveStrategy2IxData> for RemoveStrategy2IxData {
        fn into_proto(self) -> proto_def::RemoveStrategy2IxData {
            proto_def::RemoveStrategy2IxData {
                max_admin_pay_amount: self.max_admin_pay_amount,
            }
        }
    }
    use super::CollectDustIxAccounts;
    impl IntoProto<proto_def::CollectDustIxAccounts> for CollectDustIxAccounts {
        fn into_proto(self) -> proto_def::CollectDustIxAccounts {
            proto_def::CollectDustIxAccounts {
                vault: self.vault.to_string(),
                token_vault: self.token_vault.to_string(),
                token_admin: self.token_admin.to_string(),
                admin: self.admin.to_string(),
                token_program: self.token_program.to_string(),
            }
        }
    }
    use super::AddStrategyIxAccounts;
    impl IntoProto<proto_def::AddStrategyIxAccounts> for AddStrategyIxAccounts {
        fn into_proto(self) -> proto_def::AddStrategyIxAccounts {
            proto_def::AddStrategyIxAccounts {
                vault: self.vault.to_string(),
                strategy: self.strategy.to_string(),
                admin: self.admin.to_string(),
            }
        }
    }
    use super::DepositStrategyIxAccounts;
    impl IntoProto<proto_def::DepositStrategyIxAccounts> for DepositStrategyIxAccounts {
        fn into_proto(self) -> proto_def::DepositStrategyIxAccounts {
            proto_def::DepositStrategyIxAccounts {
                vault: self.vault.to_string(),
                strategy: self.strategy.to_string(),
                token_vault: self.token_vault.to_string(),
                fee_vault: self.fee_vault.to_string(),
                lp_mint: self.lp_mint.to_string(),
                strategy_program: self.strategy_program.to_string(),
                collateral_vault: self.collateral_vault.to_string(),
                reserve: self.reserve.to_string(),
                token_program: self.token_program.to_string(),
                operator: self.operator.to_string(),
            }
        }
    }
    use super::DepositStrategyIxData;
    impl IntoProto<proto_def::DepositStrategyIxData> for DepositStrategyIxData {
        fn into_proto(self) -> proto_def::DepositStrategyIxData {
            proto_def::DepositStrategyIxData {
                amount: self.amount,
            }
        }
    }
    use super::WithdrawStrategyIxAccounts;
    impl IntoProto<proto_def::WithdrawStrategyIxAccounts> for WithdrawStrategyIxAccounts {
        fn into_proto(self) -> proto_def::WithdrawStrategyIxAccounts {
            proto_def::WithdrawStrategyIxAccounts {
                vault: self.vault.to_string(),
                strategy: self.strategy.to_string(),
                token_vault: self.token_vault.to_string(),
                fee_vault: self.fee_vault.to_string(),
                lp_mint: self.lp_mint.to_string(),
                strategy_program: self.strategy_program.to_string(),
                collateral_vault: self.collateral_vault.to_string(),
                reserve: self.reserve.to_string(),
                token_program: self.token_program.to_string(),
                operator: self.operator.to_string(),
            }
        }
    }
    use super::WithdrawStrategyIxData;
    impl IntoProto<proto_def::WithdrawStrategyIxData> for WithdrawStrategyIxData {
        fn into_proto(self) -> proto_def::WithdrawStrategyIxData {
            proto_def::WithdrawStrategyIxData {
                amount: self.amount,
            }
        }
    }
    use super::Withdraw2IxAccounts;
    impl IntoProto<proto_def::Withdraw2IxAccounts> for Withdraw2IxAccounts {
        fn into_proto(self) -> proto_def::Withdraw2IxAccounts {
            proto_def::Withdraw2IxAccounts {
                vault: self.vault.to_string(),
                token_vault: self.token_vault.to_string(),
                lp_mint: self.lp_mint.to_string(),
                user_token: self.user_token.to_string(),
                user_lp: self.user_lp.to_string(),
                user: self.user.to_string(),
                token_program: self.token_program.to_string(),
            }
        }
    }
    use super::Withdraw2IxData;
    impl IntoProto<proto_def::Withdraw2IxData> for Withdraw2IxData {
        fn into_proto(self) -> proto_def::Withdraw2IxData {
            proto_def::Withdraw2IxData {
                unmint_amount: self.unmint_amount,
                min_out_amount: self.min_out_amount,
            }
        }
    }
    use super::DepositIxAccounts;
    impl IntoProto<proto_def::DepositIxAccounts> for DepositIxAccounts {
        fn into_proto(self) -> proto_def::DepositIxAccounts {
            proto_def::DepositIxAccounts {
                vault: self.vault.to_string(),
                token_vault: self.token_vault.to_string(),
                lp_mint: self.lp_mint.to_string(),
                user_token: self.user_token.to_string(),
                user_lp: self.user_lp.to_string(),
                user: self.user.to_string(),
                token_program: self.token_program.to_string(),
            }
        }
    }
    use super::DepositIxData;
    impl IntoProto<proto_def::DepositIxData> for DepositIxData {
        fn into_proto(self) -> proto_def::DepositIxData {
            proto_def::DepositIxData {
                token_amount: self.token_amount,
                minimum_lp_token_amount: self.minimum_lp_token_amount,
            }
        }
    }
    use super::WithdrawIxAccounts;
    impl IntoProto<proto_def::WithdrawIxAccounts> for WithdrawIxAccounts {
        fn into_proto(self) -> proto_def::WithdrawIxAccounts {
            proto_def::WithdrawIxAccounts {
                vault: self.vault.to_string(),
                token_vault: self.token_vault.to_string(),
                lp_mint: self.lp_mint.to_string(),
                user_token: self.user_token.to_string(),
                user_lp: self.user_lp.to_string(),
                user: self.user.to_string(),
                token_program: self.token_program.to_string(),
            }
        }
    }
    use super::WithdrawIxData;
    impl IntoProto<proto_def::WithdrawIxData> for WithdrawIxData {
        fn into_proto(self) -> proto_def::WithdrawIxData {
            proto_def::WithdrawIxData {
                unmint_amount: self.unmint_amount,
                min_out_amount: self.min_out_amount,
            }
        }
    }
    use super::WithdrawDirectlyFromStrategyIxAccounts;
    impl IntoProto<proto_def::WithdrawDirectlyFromStrategyIxAccounts>
        for WithdrawDirectlyFromStrategyIxAccounts
    {
        fn into_proto(self) -> proto_def::WithdrawDirectlyFromStrategyIxAccounts {
            proto_def::WithdrawDirectlyFromStrategyIxAccounts {
                vault: self.vault.to_string(),
                strategy: self.strategy.to_string(),
                reserve: self.reserve.to_string(),
                strategy_program: self.strategy_program.to_string(),
                collateral_vault: self.collateral_vault.to_string(),
                token_vault: self.token_vault.to_string(),
                lp_mint: self.lp_mint.to_string(),
                fee_vault: self.fee_vault.to_string(),
                user_token: self.user_token.to_string(),
                user_lp: self.user_lp.to_string(),
                user: self.user.to_string(),
                token_program: self.token_program.to_string(),
            }
        }
    }
    use super::WithdrawDirectlyFromStrategyIxData;
    impl IntoProto<proto_def::WithdrawDirectlyFromStrategyIxData>
        for WithdrawDirectlyFromStrategyIxData
    {
        fn into_proto(self) -> proto_def::WithdrawDirectlyFromStrategyIxData {
            proto_def::WithdrawDirectlyFromStrategyIxData {
                unmint_amount: self.unmint_amount,
                min_out_amount: self.min_out_amount,
            }
        }
    }

    impl IntoProto<proto_def::ProgramIxs> for VaultProgramIx {
        fn into_proto(self) -> proto_def::ProgramIxs {
            match self {
                VaultProgramIx::Initialize(acc) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::Initialize(
                        proto_def::InitializeIx {
                            accounts: Some(acc.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::EnableVault(acc, data) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::EnableVault(
                        proto_def::EnableVaultIx {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::SetOperator(acc) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::SetOperator(
                        proto_def::SetOperatorIx {
                            accounts: Some(acc.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::InitializeStrategy(acc, data) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::InitializeStrategy(
                        proto_def::InitializeStrategyIx {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::RemoveStrategy(acc) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::RemoveStrategy(
                        proto_def::RemoveStrategyIx {
                            accounts: Some(acc.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::RemoveStrategy2(acc, data) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::RemoveStrategy2(
                        proto_def::RemoveStrategy2Ix {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::CollectDust(acc) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::CollectDust(
                        proto_def::CollectDustIx {
                            accounts: Some(acc.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::AddStrategy(acc) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::AddStrategy(
                        proto_def::AddStrategyIx {
                            accounts: Some(acc.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::DepositStrategy(acc, data) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::DepositStrategy(
                        proto_def::DepositStrategyIx {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::WithdrawStrategy(acc, data) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::WithdrawStrategy(
                        proto_def::WithdrawStrategyIx {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::Withdraw2(acc, data) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::Withdraw2(
                        proto_def::Withdraw2Ix {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::Deposit(acc, data) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::Deposit(
                        proto_def::DepositIx {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::Withdraw(acc, data) => proto_def::ProgramIxs {
                    ix_oneof: Some(proto_def::program_ixs::IxOneof::Withdraw(
                        proto_def::WithdrawIx {
                            accounts: Some(acc.into_proto()),
                            data: Some(data.into_proto()),
                        },
                    )),
                },
                VaultProgramIx::WithdrawDirectlyFromStrategy(acc, data) => proto_def::ProgramIxs {
                    ix_oneof: Some(
                        proto_def::program_ixs::IxOneof::WithdrawDirectlyFromStrategy(
                            proto_def::WithdrawDirectlyFromStrategyIx {
                                accounts: Some(acc.into_proto()),
                                data: Some(data.into_proto()),
                            },
                        ),
                    ),
                },
            }
        }
    }

    impl ParseProto for InstructionParser {
        type Message = proto_def::ProgramIxs;

        fn output_into_message(value: Self::Output) -> Self::Message {
            #[cfg(not(feature = "shared-data"))]
            return value.into_proto();

            #[cfg(feature = "shared-data")]
            value.parsed_ix.into_proto()
        }
    }
}
