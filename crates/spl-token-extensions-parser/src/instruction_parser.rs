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
        ReallocateArgs, SetAuthorityArgs, WithdrawExcessLamportsAccounts,
    },
    token_extension_program_instruction::{self},
    AuthorityTypeProto, ExtensionInstructionParser, TokenExtensionProgramInstruction,
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
        use token_extension_program_instruction as TEPI;

        let accounts_len = ix.accounts.len();

        // helper: wrap a oneof variant into the envelope struct
        let envelope = |ix_oneof: TEPI::Ix| TokenExtensionProgramInstruction { ix: Some(ix_oneof) };

        match SplTokenInstruction::unpack(&ix.data) {
            Ok(token_ix) => match token_ix {
                SplTokenInstruction::TransferFeeExtension => {
                    let parsed = TransferFeeInstruction::try_parse(ix)?;

                    Ok(envelope(TEPI::Ix::TransferFee(TEPI::TransferFee {
                        ix: Some(parsed),
                    })))
                },

                SplTokenInstruction::ConfidentialTransferExtension => {
                    let parsed = ConfidentialTransferInstruction::try_parse(ix)?;

                    Ok(envelope(TEPI::Ix::ConfidentialTransfer(
                        TEPI::ConfidentialTransfer { ix: Some(parsed) },
                    )))
                },

                SplTokenInstruction::ConfidentialTransferFeeExtension => {
                    let parsed = ConfidentialTransferFeeInstruction::try_parse(ix)?;

                    Ok(envelope(TEPI::Ix::ConfidentialTransferFee(
                        TEPI::ConfidentialTransferFee { ix: Some(parsed) },
                    )))
                },

                SplTokenInstruction::CpiGuardExtension => {
                    let parsed = CommonExtensionInstructions::try_parse_extension_instruction(
                        ExtensionWithCommonInstruction::CpiGuard,
                        ix,
                    )?;

                    Ok(envelope(TEPI::Ix::CpiGuard(TEPI::CpiGuard {
                        ix: Some(parsed),
                    })))
                },

                SplTokenInstruction::DefaultAccountStateExtension => {
                    let parsed = CommonExtensionInstructions::try_parse_extension_instruction(
                        ExtensionWithCommonInstruction::DefaultAccountState,
                        ix,
                    )?;

                    Ok(envelope(TEPI::Ix::DefaultAccountState(
                        TEPI::DefaultAccountState { ix: Some(parsed) },
                    )))
                },

                SplTokenInstruction::InterestBearingMintExtension => {
                    let parsed = CommonExtensionInstructions::try_parse_extension_instruction(
                        ExtensionWithCommonInstruction::InterestBearingMint,
                        ix,
                    )?;

                    Ok(envelope(TEPI::Ix::InterestBearingMint(
                        TEPI::InterestBearingMint { ix: Some(parsed) },
                    )))
                },

                SplTokenInstruction::MemoTransferExtension => {
                    let parsed = CommonExtensionInstructions::try_parse_extension_instruction(
                        ExtensionWithCommonInstruction::MemoTransfer,
                        ix,
                    )?;

                    Ok(envelope(TEPI::Ix::MemoTransfer(TEPI::MemoTransfer {
                        ix: Some(parsed),
                    })))
                },

                SplTokenInstruction::GroupMemberPointerExtension => {
                    let parsed = CommonExtensionInstructions::try_parse_extension_instruction(
                        ExtensionWithCommonInstruction::GroupMemberPointer,
                        ix,
                    )?;

                    Ok(envelope(TEPI::Ix::GroupMemberPointer(
                        TEPI::GroupMemberPointer { ix: Some(parsed) },
                    )))
                },

                SplTokenInstruction::GroupPointerExtension => {
                    let parsed = CommonExtensionInstructions::try_parse_extension_instruction(
                        ExtensionWithCommonInstruction::GroupPointer,
                        ix,
                    )?;

                    Ok(envelope(TEPI::Ix::GroupPointer(TEPI::GroupPointer {
                        ix: Some(parsed),
                    })))
                },

                SplTokenInstruction::MetadataPointerExtension => {
                    let parsed = CommonExtensionInstructions::try_parse_extension_instruction(
                        ExtensionWithCommonInstruction::MetadataPointer,
                        ix,
                    )?;

                    Ok(envelope(TEPI::Ix::MetadataPointer(TEPI::MetadataPointer {
                        ix: Some(parsed),
                    })))
                },

                SplTokenInstruction::TransferHookExtension => {
                    let parsed = CommonExtensionInstructions::try_parse_extension_instruction(
                        ExtensionWithCommonInstruction::TransferHook,
                        ix,
                    )?;

                    Ok(envelope(TEPI::Ix::TransferHook(TEPI::TransferHook {
                        ix: Some(parsed),
                    })))
                },

                SplTokenInstruction::SetAuthority {
                    authority_type,
                    new_authority,
                } => {
                    check_min_accounts_req(accounts_len, 2)?;

                    // IMPORTANT: ix.accounts are Pubkey (bytes), and your proto structs want Vec<u8>
                    let accounts = SetAuthorityAccounts {
                        account: ix.accounts[0].to_vec(),
                        current_authority: ix.accounts[1].to_vec(),
                        multisig_signers: ix.accounts[2..].iter().map(|pk| pk.to_vec()).collect(),
                    };

                    let args = SetAuthorityArgs {
                        authority_type: AuthorityTypeProto::from(authority_type) as i32,
                        new_authority: new_authority.map(|pk| pk.to_bytes().to_vec()).into(),
                    };

                    Ok(envelope(TEPI::Ix::SetAuthority(TEPI::SetAuthority {
                        accounts: Some(accounts),
                        args: Some(args),
                    })))
                },

                SplTokenInstruction::CreateNativeMint => {
                    check_min_accounts_req(accounts_len, 2)?;

                    let accounts = CreateNativeMintAccounts {
                        funding_account: ix.accounts[0].to_vec(),
                        mint: ix.accounts[1].to_vec(),
                    };

                    Ok(envelope(TEPI::Ix::CreateNativeMint(
                        TEPI::CreateNativeMint {
                            accounts: Some(accounts),
                        },
                    )))
                },

                SplTokenInstruction::InitializeMintCloseAuthority { close_authority } => {
                    check_min_accounts_req(accounts_len, 1)?;

                    let accounts = InitializeMintCloseAuthorityAccounts {
                        mint: ix.accounts[0].to_vec(),
                    };

                    let args = InitializeMintCloseAuthorityArgs {
                        close_authority: close_authority.map(|pk| pk.to_bytes().to_vec()).into(),
                    };

                    Ok(envelope(TEPI::Ix::InitializeMintCloseAuthority(
                        TEPI::InitializeMintCloseAuthority {
                            accounts: Some(accounts),
                            args: Some(args),
                        },
                    )))
                },

                SplTokenInstruction::InitializeNonTransferableMint => {
                    check_min_accounts_req(accounts_len, 1)?;

                    let accounts = InitializeNonTransferableMintAccounts {
                        mint: ix.accounts[0].to_vec(),
                    };

                    Ok(envelope(TEPI::Ix::InitializeNonTransferableMint(
                        TEPI::InitializeNonTransferableMint {
                            accounts: Some(accounts),
                        },
                    )))
                },

                SplTokenInstruction::Reallocate { extension_types } => {
                    check_min_accounts_req(accounts_len, 4)?;

                    let accounts = ReallocateAccounts {
                        account: ix.accounts[0].to_vec(),
                        payer: ix.accounts[1].to_vec(),
                        owner: ix.accounts[3].to_vec(),
                        multisig_signers: ix.accounts[4..].iter().map(|pk| pk.to_vec()).collect(),
                    };

                    let args = ReallocateArgs {
                        extension_types: extension_types.into_iter().map(|t| t as u32).collect(),
                    };

                    Ok(envelope(TEPI::Ix::Reallocate(TEPI::Reallocate {
                        accounts: Some(accounts),
                        args: Some(args),
                    })))
                },

                SplTokenInstruction::InitializePermanentDelegate { delegate } => {
                    check_min_accounts_req(accounts_len, 1)?;

                    let accounts = InitializePermanentDelegateAccounts {
                        account: ix.accounts[0].to_vec(),
                    };

                    let args = InitializePermanentDelegateArgs {
                        delegate: delegate.to_bytes().to_vec(),
                    };

                    Ok(envelope(TEPI::Ix::InitializePermanentDelegate(
                        TEPI::InitializePermanentDelegate {
                            accounts: Some(accounts),
                            args: Some(args),
                        },
                    )))
                },

                SplTokenInstruction::WithdrawExcessLamports => {
                    check_min_accounts_req(accounts_len, 3)?;

                    let accounts = WithdrawExcessLamportsAccounts {
                        source_account: ix.accounts[0].to_vec(),
                        destination_account: ix.accounts[1].to_vec(),
                        authority: ix.accounts[2].to_vec(),
                        multisig_signers: ix.accounts[3..].iter().map(|pk| pk.to_vec()).collect(),
                    };

                    Ok(envelope(TEPI::Ix::WithdrawExcessLamports(
                        TEPI::WithdrawExcessLamports {
                            accounts: Some(accounts),
                        },
                    )))
                },

                // Anything else: fallback to SPL token parser
                _ => {
                    let token_instruction = TokenProgramInstructionParser::parse_impl(ix)
                        .parse_err(
                            "Error parsing token extension instruction as token instruction",
                        )?;

                    Ok(envelope(TEPI::Ix::TokenProgram(TEPI::TokenProgram {
                        ix: Some(token_instruction),
                    })))
                },
            },

            Err(e) => {
                if SplTokenMetadataInstruction::unpack(&ix.data).is_ok() {
                    let parsed = TokenMetadataInstruction::try_parse(ix)?;

                    return Ok(envelope(TEPI::Ix::TokenMetadata(TEPI::TokenMetadata {
                        ix: Some(parsed),
                    })));
                }

                if SplTokenGroupInstruction::unpack(&ix.data).is_ok() {
                    let parsed = TokenGroupInstruction::try_parse(ix)?;

                    return Ok(envelope(TEPI::Ix::TokenGroup(TEPI::TokenGroup {
                        ix: Some(parsed),
                    })));
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

    use super::{InstructionParser, Parser, TokenExtensionProgramInstruction};

    #[tokio::test]
    async fn test_mint_to_checked_ix_parsing() {
        let parser = InstructionParser;

        let ixs = tx_fixture!(
            "44gWEyKUkeUabtJr4eT3CQEkFGrD4jMdwUV6Ew5MR5K3RGizs9iwbkb5Q4T3gnAaSgHxn3ERQ8g5YTXuLP1FrWnt",
            &parser
        );

        let Some(TokenExtensionProgramInstruction {
            ix:
                Some(crate::instructions::token_extension_program_instruction::Ix::TokenProgram(
                    crate::instructions::token_extension_program_instruction::TokenProgram {
                        ix:
                            Some(yellowstone_vixen_spl_token_parser::TokenProgramInstruction {
                                ix: Some(yellowstone_vixen_spl_token_parser::token_program_instruction::Ix::MintToChecked(
                                    yellowstone_vixen_spl_token_parser::token_program_instruction::MintToChecked { args: Some(args), .. }
                                )),
                            }),
                    },
                )),
        }) = &ixs[0]
        else {
            panic!("Invalid Instruction");
        };

        assert_eq!(args.decimals, 9);
        assert_eq!(args.amount, 100.mul(10u64.pow(args.decimals)));
    }
}
