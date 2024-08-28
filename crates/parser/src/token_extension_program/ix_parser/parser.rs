use helpers::ExtensionIxParser;
use spl_token_2022::instruction::TokenInstruction;
use spl_token_group_interface::instruction::TokenGroupInstruction;
use spl_token_metadata_interface::instruction::TokenMetadataInstruction;
use yellowstone_vixen_core::{
    Instruction, InstructionParser, InstructionsUpdate, ParseError, ParseResult, Parser, Prefilter,
    ReadableInstruction,
};

use super::{extensions::*, ixs::*};
use crate::{
    helpers::{
        check_min_accounts_req, check_pubkeys_match, get_multisig_signers,
        to_supported_coption_pubkey, to_supported_pubkey,
    },
    token_program::ix_parser::{SetAuthorityAccounts, TokenProgramIxParser},
};

#[derive(Debug)]
pub struct TokenExtensionProgramIxParser;

impl Parser for TokenExtensionProgramIxParser {
    type Input = InstructionsUpdate;
    type Output = Vec<TokenExtensionProgramIx>;

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_token_2022::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, ixs_update: &InstructionsUpdate) -> ParseResult<Self::Output> {
        let mut parsed_ixs: Vec<TokenExtensionProgramIx> = Vec::new();
        for outer_ixs in ixs_update.instructions.iter() {
            if check_pubkeys_match(&outer_ixs.outer_ix.program_id, &spl_token_2022::ID) {
                let parsed_ix = TokenExtensionProgramIxParser::parse_ix(&outer_ixs.outer_ix)
                    .map_err(|e| ParseError::Other(e.into()))?;
                parsed_ixs.push(parsed_ix);
            }
            for inner_ix in outer_ixs.inner_ixs.iter() {
                if check_pubkeys_match(&inner_ix.program_id, &spl_token_2022::ID) {
                    let parsed_ix = TokenExtensionProgramIxParser::parse_ix(inner_ix)
                        .map_err(|e| ParseError::Other(e.into()))?;
                    parsed_ixs.push(parsed_ix);
                }
            }
        }
        if parsed_ixs.len() == 0 {
            return Err(ParseError::Other(
                "No token extension program instructions found to parse"
                    .to_string()
                    .into(),
            ));
        }
        Ok(parsed_ixs)
    }
}

impl InstructionParser<TokenExtensionProgramIx> for TokenExtensionProgramIxParser {
    fn parse_ix(ix: &Instruction) -> Result<TokenExtensionProgramIx, String> {
        let accounts_len = ix.accounts.len();
        match TokenInstruction::unpack(&ix.data) {
            Ok(token_ix) => match token_ix {
                TokenInstruction::TransferFeeExtension => {
                    Ok(TokenExtensionProgramIx::TransferFeeIx(
                        TransferFeeIx::try_parse_extension_ix(ix)?,
                    ))
                },
                TokenInstruction::ConfidentialTransferExtension => {
                    Ok(TokenExtensionProgramIx::ConfidentialTransferIx(
                        ConfidentaltransferIx::try_parse_extension_ix(ix)?,
                    ))
                },
                TokenInstruction::ConfidentialTransferFeeExtension => {
                    Ok(TokenExtensionProgramIx::ConfidentialtransferFeeIx(
                        ConfidentaltransferFeeIx::try_parse_extension_ix(ix)?,
                    ))
                },
                TokenInstruction::CpiGuardExtension => Ok(TokenExtensionProgramIx::CpiGuardIx(
                    CommonExtensionIxs::try_parse_extension_ix(
                        ExtensionWithCommonIxs::CpiGuard,
                        ix,
                    )?,
                )),

                TokenInstruction::DefaultAccountStateExtension => {
                    Ok(TokenExtensionProgramIx::DefaultAccountStateIx(
                        CommonExtensionIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::DefaultAccountState,
                            ix,
                        )?,
                    ))
                },
                TokenInstruction::InterestBearingMintExtension => {
                    Ok(TokenExtensionProgramIx::InterestBearingMintIx(
                        CommonExtensionIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::InterestBearingMint,
                            ix,
                        )?,
                    ))
                },
                TokenInstruction::MemoTransferExtension => {
                    Ok(TokenExtensionProgramIx::MemoTransferIx(
                        CommonExtensionIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::MemoTransfer,
                            ix,
                        )?,
                    ))
                },

                TokenInstruction::GroupMemberPointerExtension => {
                    Ok(TokenExtensionProgramIx::GroupMemberPointerIx(
                        CommonExtensionIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::GroupMemberPointer,
                            ix,
                        )?,
                    ))
                },

                TokenInstruction::GroupPointerExtension => {
                    Ok(TokenExtensionProgramIx::GroupPointerIx(
                        CommonExtensionIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::GroupPointer,
                            ix,
                        )?,
                    ))
                },

                TokenInstruction::MetadataPointerExtension => {
                    Ok(TokenExtensionProgramIx::MetadataPointerIx(
                        CommonExtensionIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::MetadataPointer,
                            ix,
                        )?,
                    ))
                },

                TokenInstruction::TransferHookExtension => {
                    Ok(TokenExtensionProgramIx::TransferHookIx(
                        CommonExtensionIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::TransferHook,
                            ix,
                        )?,
                    ))
                },
                TokenInstruction::SetAuthority {
                    authority_type,
                    new_authority,
                } => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(TokenExtensionProgramIx::SetAuthority(ReadableInstruction {
                        accounts: SetAuthorityAccounts {
                            account: ix.accounts[0],
                            current_authority: ix.accounts[1],
                            multisig_signers: get_multisig_signers(ix, 2),
                        },
                        data: Some(TokenExtSetAutorityData {
                            authority_type,
                            new_authority: to_supported_coption_pubkey(new_authority),
                        }),
                    }))
                },
                TokenInstruction::CreateNativeMint => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(TokenExtensionProgramIx::CreateNativeMint(
                        ReadableInstruction {
                            accounts: CreateNativeMintAccounts {
                                funding_account: ix.accounts[0],
                                mint: ix.accounts[1],
                            },
                            data: None,
                        },
                    ))
                },

                TokenInstruction::InitializeMintCloseAuthority { close_authority } => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(TokenExtensionProgramIx::InitializeMintCloseAuthority(
                        ReadableInstruction {
                            accounts: InitializeMintCloseAuthorityAccounts {
                                mint: ix.accounts[0],
                            },
                            data: Some(InitializeMintCloseAuthorityData {
                                close_authority: to_supported_coption_pubkey(close_authority),
                            }),
                        },
                    ))
                },

                TokenInstruction::InitializeNonTransferableMint => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(TokenExtensionProgramIx::InitializeNonTransferableMint(
                        ReadableInstruction {
                            accounts: InitializeNonTransferableMintAccounts {
                                mint: ix.accounts[0],
                            },
                            data: None,
                        },
                    ))
                },

                TokenInstruction::Reallocate { extension_types } => {
                    check_min_accounts_req(accounts_len, 4)?;
                    Ok(TokenExtensionProgramIx::Reallocate(ReadableInstruction {
                        accounts: ReallocateAccounts {
                            account: ix.accounts[0],
                            payer: ix.accounts[1],
                            owner: ix.accounts[3],
                            multisig_signers: get_multisig_signers(ix, 4),
                        },
                        data: Some(ReallocateData { extension_types }),
                    }))
                },

                TokenInstruction::InitializePermanentDelegate { delegate } => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(TokenExtensionProgramIx::InitializePermanentDelegate(
                        ReadableInstruction {
                            accounts: InitializePermanentDelegateAccounts {
                                account: ix.accounts[0],
                            },
                            data: Some(InitializePermanentDelegateData {
                                delegate: to_supported_pubkey(delegate),
                            }),
                        },
                    ))
                },

                TokenInstruction::WithdrawExcessLamports => {
                    check_min_accounts_req(accounts_len, 3)?;
                    Ok(TokenExtensionProgramIx::WithdrawExcessLamports(
                        ReadableInstruction {
                            accounts: WithdrawExcessLamportsAccounts {
                                source_account: ix.accounts[0],
                                destination_account: ix.accounts[1],
                                authority: ix.accounts[2],
                                multisig_signers: get_multisig_signers(ix, 3),
                            },
                            data: None,
                        },
                    ))
                },

                _ => Ok(TokenExtensionProgramIx::TokenProgramIx(
                    TokenProgramIxParser::parse_ix(ix).map_err(|e| e.to_string())?,
                )),
            },
            Err(e) => {
                if let Ok(_) = TokenMetadataInstruction::unpack(&ix.data) {
                    return Ok(TokenExtensionProgramIx::TokenMetadataIx(
                        TokenMetadataIx::try_parse_extension_ix(ix)?,
                    ));
                }

                if let Ok(_) = TokenGroupInstruction::unpack(&ix.data) {
                    return Ok(TokenExtensionProgramIx::TokenGroupIx(
                        TokenGroupIx::try_parse_extension_ix(ix)?,
                    ));
                }

                Err(format!(
                    "Err while unpacking ix data : {} data : {:?}",
                    e, ix.data
                ))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use yellowstone_vixen_mock::{run_tx_parse, tx_fixture, FixtureData};

    use super::*;
    use crate::token_program::ix_parser::TokenProgramIx;
    #[tokio::test]
    async fn test_mint_to_checked_ix_parsing() {
        let parser = TokenExtensionProgramIxParser;

        let fixture_data = tx_fixture!("44gWEyKUkeUabtJr4eT3CQEkFGrD4jMdwUV6Ew5MR5K3RGizs9iwbkb5Q4T3gnAaSgHxn3ERQ8g5YTXuLP1FrWnt");

        if let FixtureData::Instructions(ixs) = fixture_data {
            let ixs = run_tx_parse!(parser, ixs);
            match &ixs[0] {
                TokenExtensionProgramIx::TokenProgramIx(ix) => {
                    if let TokenProgramIx::MintToChecked(ix) = ix {
                        assert!(ix.data.is_some());
                        let data = ix.data.as_ref().unwrap();
                        assert_eq!(data.decimals, 9);
                        assert_eq!(data.amount, 100.mul(10u64.pow(data.decimals as u32)));
                    } else {
                        panic!("Invalid Instruction")
                    }
                },
                _ => panic!("Invalid Instruction"),
            }
        } else {
            panic!("Invalid Fixture Data")
        }
    }
}
