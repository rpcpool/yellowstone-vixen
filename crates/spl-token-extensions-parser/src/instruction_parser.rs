use spl_token_2022::instruction::TokenInstruction as SplTokenInstruction;
use spl_token_group_interface::instruction::TokenGroupInstruction as SplTokenGroupInstruction;
use spl_token_metadata_interface::instruction::TokenMetadataInstruction as SplTokenMetadataInstruction;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};
use yellowstone_vixen_parser::{check_min_accounts_req, Error, Result, ResultExt};
use yellowstone_vixen_spl_token_parser::{
    InstructionParser as TokenProgramInstructionParser, SetAuthorityAccounts,
};

use crate::{
    extensions::{
        CommonExtensionInstructions, ConfidentialTransferFeeInstruction,
        ConfidentialTransferInstruction, ExtensionWithCommonInstruction, TokenGroupInstruction,
        TokenMetadataInstruction, TransferFeeInstruction,
    },
    instructions::{
        CreateNativeMintAccounts, InitializeMintCloseAuthorityAccounts,
        InitializeMintCloseAuthorityArgs, InitializeNonTransferableMintAccounts,
        InitializePermanentDelegateAccounts, InitializePermanentDelegateArgs, ReallocateAccounts,
        ReallocateArgs, WithdrawExcessLamportsAccounts,
    },
    ExtensionInstructionParser, SetAuthorityArgs, TokenExtensionProgramInstruction,
};

#[derive(Debug, Clone, Copy)]
pub struct InstructionParser;

impl Parser for InstructionParser {
    type Input = InstructionUpdate;
    type Output = TokenExtensionProgramInstruction;

    fn id(&self) -> std::borrow::Cow<'static, str> { "token_extensions::InstructionParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_token_2022::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, ix_update: &InstructionUpdate) -> ParseResult<Self::Output> {
        InstructionParser::parse_impl(ix_update).map_err(|e| ParseError::Other(e.into()))
    }
}

impl ProgramParser for InstructionParser {
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_token_2022::ID.to_bytes().into() }
}

impl InstructionParser {
    #[allow(clippy::too_many_lines)]
    fn parse_impl(ix: &InstructionUpdate) -> Result<TokenExtensionProgramInstruction> {
        let accounts_len = ix.accounts.len();
        match SplTokenInstruction::unpack(&ix.data) {
            Ok(token_ix) => match token_ix {
                SplTokenInstruction::TransferFeeExtension => {
                    Ok(TokenExtensionProgramInstruction::TransferFee(
                        TransferFeeInstruction::try_parse(ix)?,
                    ))
                },
                SplTokenInstruction::ConfidentialTransferExtension => {
                    Ok(TokenExtensionProgramInstruction::ConfidentialTransfer(
                        ConfidentialTransferInstruction::try_parse(ix)?,
                    ))
                },
                SplTokenInstruction::ConfidentialTransferFeeExtension => {
                    Ok(TokenExtensionProgramInstruction::ConfidentialTransferFee(
                        ConfidentialTransferFeeInstruction::try_parse(ix)?,
                    ))
                },
                SplTokenInstruction::CpiGuardExtension => {
                    Ok(TokenExtensionProgramInstruction::CpiGuard(
                        CommonExtensionInstructions::try_parse_extension_instruction(
                            ExtensionWithCommonInstruction::CpiGuard,
                            ix,
                        )?,
                    ))
                },

                SplTokenInstruction::DefaultAccountStateExtension => {
                    Ok(TokenExtensionProgramInstruction::DefaultAccountState(
                        CommonExtensionInstructions::try_parse_extension_instruction(
                            ExtensionWithCommonInstruction::DefaultAccountState,
                            ix,
                        )?,
                    ))
                },
                SplTokenInstruction::InterestBearingMintExtension => {
                    Ok(TokenExtensionProgramInstruction::InterestBearingMint(
                        CommonExtensionInstructions::try_parse_extension_instruction(
                            ExtensionWithCommonInstruction::InterestBearingMint,
                            ix,
                        )?,
                    ))
                },
                SplTokenInstruction::MemoTransferExtension => {
                    Ok(TokenExtensionProgramInstruction::MemoTransfer(
                        CommonExtensionInstructions::try_parse_extension_instruction(
                            ExtensionWithCommonInstruction::MemoTransfer,
                            ix,
                        )?,
                    ))
                },

                SplTokenInstruction::GroupMemberPointerExtension => {
                    Ok(TokenExtensionProgramInstruction::GroupMemberPointer(
                        CommonExtensionInstructions::try_parse_extension_instruction(
                            ExtensionWithCommonInstruction::GroupMemberPointer,
                            ix,
                        )?,
                    ))
                },

                SplTokenInstruction::GroupPointerExtension => {
                    Ok(TokenExtensionProgramInstruction::GroupPointer(
                        CommonExtensionInstructions::try_parse_extension_instruction(
                            ExtensionWithCommonInstruction::GroupPointer,
                            ix,
                        )?,
                    ))
                },

                SplTokenInstruction::MetadataPointerExtension => {
                    Ok(TokenExtensionProgramInstruction::MetadataPointer(
                        CommonExtensionInstructions::try_parse_extension_instruction(
                            ExtensionWithCommonInstruction::MetadataPointer,
                            ix,
                        )?,
                    ))
                },

                SplTokenInstruction::TransferHookExtension => {
                    Ok(TokenExtensionProgramInstruction::TransferHook(
                        CommonExtensionInstructions::try_parse_extension_instruction(
                            ExtensionWithCommonInstruction::TransferHook,
                            ix,
                        )?,
                    ))
                },
                SplTokenInstruction::SetAuthority {
                    authority_type,
                    new_authority,
                } => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(TokenExtensionProgramInstruction::SetAuthority {
                        accounts: SetAuthorityAccounts {
                            account: ix.accounts[0],
                            current_authority: ix.accounts[1],
                            multisig_signers: ix.accounts[2..].to_vec(),
                        },
                        args: SetAuthorityArgs {
                            authority_type,
                            new_authority: new_authority.map(|p| p.to_bytes().into()).into(),
                        },
                    })
                },
                SplTokenInstruction::CreateNativeMint => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(TokenExtensionProgramInstruction::CreateNativeMint {
                        accounts: CreateNativeMintAccounts {
                            funding_account: ix.accounts[0],
                            mint: ix.accounts[1],
                        },
                    })
                },

                SplTokenInstruction::InitializeMintCloseAuthority { close_authority } => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(
                        TokenExtensionProgramInstruction::InitializeMintCloseAuthority {
                            accounts: InitializeMintCloseAuthorityAccounts {
                                mint: ix.accounts[0],
                            },
                            args: InitializeMintCloseAuthorityArgs {
                                close_authority: close_authority
                                    .map(|p| p.to_bytes().into())
                                    .into(),
                            },
                        },
                    )
                },

                SplTokenInstruction::InitializeNonTransferableMint => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(
                        TokenExtensionProgramInstruction::InitializeNonTransferableMint {
                            accounts: InitializeNonTransferableMintAccounts {
                                mint: ix.accounts[0],
                            },
                        },
                    )
                },

                SplTokenInstruction::Reallocate { extension_types } => {
                    check_min_accounts_req(accounts_len, 4)?;
                    Ok(TokenExtensionProgramInstruction::Reallocate {
                        accounts: ReallocateAccounts {
                            account: ix.accounts[0],
                            payer: ix.accounts[1],
                            owner: ix.accounts[3],
                            multisig_signers: ix.accounts[4..].to_vec(),
                        },
                        args: ReallocateArgs { extension_types },
                    })
                },

                SplTokenInstruction::InitializePermanentDelegate { delegate } => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(
                        TokenExtensionProgramInstruction::InitializePermanentDelegate {
                            accounts: InitializePermanentDelegateAccounts {
                                account: ix.accounts[0],
                            },
                            args: InitializePermanentDelegateArgs {
                                delegate: delegate.to_bytes().into(),
                            },
                        },
                    )
                },

                SplTokenInstruction::WithdrawExcessLamports => {
                    check_min_accounts_req(accounts_len, 3)?;
                    Ok(TokenExtensionProgramInstruction::WithdrawExcessLamports {
                        accounts: WithdrawExcessLamportsAccounts {
                            source_account: ix.accounts[0],
                            destination_account: ix.accounts[1],
                            authority: ix.accounts[2],
                            multisig_signers: ix.accounts[3..].to_vec(),
                        },
                    })
                },

                _ => Ok(TokenExtensionProgramInstruction::TokenProgram(
                    TokenProgramInstructionParser::parse_impl(ix).parse_err(
                        "Error parsing token extension instruction as token instruction",
                    )?,
                )),
            },
            Err(e) => {
                if SplTokenMetadataInstruction::unpack(&ix.data).is_ok() {
                    return Ok(TokenExtensionProgramInstruction::TokenMetadata(
                        TokenMetadataInstruction::try_parse(ix)?,
                    ));
                }

                if SplTokenGroupInstruction::unpack(&ix.data).is_ok() {
                    return Ok(TokenExtensionProgramInstruction::TokenGroup(
                        TokenGroupInstruction::try_parse(ix)?,
                    ));
                }

                Err(Error::from_inner("Error unpacking instruction data", e))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use yellowstone_vixen_mock::tx_fixture;
    use yellowstone_vixen_spl_token_parser::TokenProgramInstruction;

    use super::{InstructionParser, Parser, TokenExtensionProgramInstruction};

    #[tokio::test]
    async fn test_mint_to_checked_ix_parsing() {
        let parser = InstructionParser;

        let ixs = tx_fixture!("44gWEyKUkeUabtJr4eT3CQEkFGrD4jMdwUV6Ew5MR5K3RGizs9iwbkb5Q4T3gnAaSgHxn3ERQ8g5YTXuLP1FrWnt",&parser);

        let TokenExtensionProgramInstruction::TokenProgram(
            TokenProgramInstruction::MintToChecked { args, .. },
        ) = &ixs[0]
        else {
            panic!("Invalid Instruction");
        };

        assert_eq!(args.decimals, 9);
        assert_eq!(args.amount, 100.mul(10u64.pow(args.decimals.into())));
    }
}
