use helpers::ExtensionIxParser;
use spl_token_2022::instruction::TokenInstruction;
use spl_token_group_interface::instruction::TokenGroupInstruction;
use spl_token_metadata_interface::instruction::TokenMetadataInstruction;
use yellowstone_vixen_core::{
    instruction::InstructionUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};

use super::{
    extensions::{
        helpers, CommonExtensionIxs, ConfidentaltransferFeeIx, ConfidentaltransferIx,
        ExtensionWithCommonIxs, TokenGroupIx, TokenMetadataIx, TransferFeeIx,
    },
    ixs::{
        CreateNativeMintAccounts, InitializeMintCloseAuthorityAccounts,
        InitializeMintCloseAuthorityData, InitializeNonTransferableMintAccounts,
        InitializePermanentDelegateAccounts, InitializePermanentDelegateData, ReallocateAccounts,
        ReallocateData, TokenExtensionProgramIx, WithdrawExcessLamportsAccounts,
    },
    SetAuthorityData,
};
use crate::{
    helpers::{check_min_accounts_req, into_vixen_pubkey},
    token_program::ix_parser::{SetAuthorityAccounts, TokenProgramIxParser},
    Error, Result, ResultExt,
};

#[derive(Debug, Clone, Copy)]
pub struct TokenExtensionProgramIxParser;

impl Parser for TokenExtensionProgramIxParser {
    type Input = InstructionUpdate;
    type Output = TokenExtensionProgramIx;

    fn id(&self) -> std::borrow::Cow<str> {
        "yellowstone_vixen_parser::token_extensions::TokenExtensionProgramIxParser".into()
    }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .transaction_accounts([spl_token_2022::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, ix_update: &InstructionUpdate) -> ParseResult<Self::Output> {
        if ix_update.program.equals_ref(spl_token_2022::ID) {
            TokenExtensionProgramIxParser::parse_impl(ix_update)
                .map_err(|e| ParseError::Other(e.into()))
        } else {
            Err(ParseError::Filtered)
        }
    }
}

impl ProgramParser for TokenExtensionProgramIxParser {
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_token_2022::ID.to_bytes().into() }
}

impl TokenExtensionProgramIxParser {
    #[allow(clippy::too_many_lines)]
    fn parse_impl(ix: &InstructionUpdate) -> Result<TokenExtensionProgramIx> {
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
                    Ok(TokenExtensionProgramIx::SetAuthority(
                        SetAuthorityAccounts {
                            account: ix.accounts[0],
                            current_authority: ix.accounts[1],
                            multisig_signers: ix.accounts[2..].to_vec(),
                        },
                        SetAuthorityData {
                            authority_type,
                            new_authority: new_authority.map(into_vixen_pubkey).into(),
                        },
                    ))
                },
                TokenInstruction::CreateNativeMint => {
                    check_min_accounts_req(accounts_len, 2)?;
                    Ok(TokenExtensionProgramIx::CreateNativeMint(
                        CreateNativeMintAccounts {
                            funding_account: ix.accounts[0],
                            mint: ix.accounts[1],
                        },
                    ))
                },

                TokenInstruction::InitializeMintCloseAuthority { close_authority } => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(TokenExtensionProgramIx::InitializeMintCloseAuthority(
                        InitializeMintCloseAuthorityAccounts {
                            mint: ix.accounts[0],
                        },
                        InitializeMintCloseAuthorityData {
                            close_authority: close_authority.map(into_vixen_pubkey).into(),
                        },
                    ))
                },

                TokenInstruction::InitializeNonTransferableMint => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(TokenExtensionProgramIx::InitializeNonTransferableMint(
                        InitializeNonTransferableMintAccounts {
                            mint: ix.accounts[0],
                        },
                    ))
                },

                TokenInstruction::Reallocate { extension_types } => {
                    check_min_accounts_req(accounts_len, 4)?;
                    Ok(TokenExtensionProgramIx::Reallocate(
                        ReallocateAccounts {
                            account: ix.accounts[0],
                            payer: ix.accounts[1],
                            owner: ix.accounts[3],
                            multisig_signers: ix.accounts[4..].to_vec(),
                        },
                        ReallocateData { extension_types },
                    ))
                },

                TokenInstruction::InitializePermanentDelegate { delegate } => {
                    check_min_accounts_req(accounts_len, 1)?;
                    Ok(TokenExtensionProgramIx::InitializePermanentDelegate(
                        InitializePermanentDelegateAccounts {
                            account: ix.accounts[0],
                        },
                        InitializePermanentDelegateData {
                            delegate: into_vixen_pubkey(delegate),
                        },
                    ))
                },

                TokenInstruction::WithdrawExcessLamports => {
                    check_min_accounts_req(accounts_len, 3)?;
                    Ok(TokenExtensionProgramIx::WithdrawExcessLamports(
                        WithdrawExcessLamportsAccounts {
                            source_account: ix.accounts[0],
                            destination_account: ix.accounts[1],
                            authority: ix.accounts[2],
                            multisig_signers: ix.accounts[3..].to_vec(),
                        },
                    ))
                },

                _ => Ok(TokenExtensionProgramIx::TokenProgramIx(
                    TokenProgramIxParser::parse_impl(ix).parse_err(
                        "Error parsing token extension instruction as token instruction",
                    )?,
                )),
            },
            Err(e) => {
                if TokenMetadataInstruction::unpack(&ix.data).is_ok() {
                    return Ok(TokenExtensionProgramIx::TokenMetadataIx(
                        TokenMetadataIx::try_parse_extension_ix(ix)?,
                    ));
                }

                if TokenGroupInstruction::unpack(&ix.data).is_ok() {
                    return Ok(TokenExtensionProgramIx::TokenGroupIx(
                        TokenGroupIx::try_parse_extension_ix(ix)?,
                    ));
                }

                Err(Error::from_inner("Error unpacking instruction data", e))
            },
        }
    }
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_core::proto::ParseProto;
    use yellowstone_vixen_proto::parser::TokenExtensionProgramIxProto;

    use super::TokenExtensionProgramIxParser;
    use crate::helpers::IntoProto;

    impl ParseProto for TokenExtensionProgramIxParser {
        type Message = TokenExtensionProgramIxProto;

        fn output_into_message(value: Self::Output) -> Self::Message { value.into_proto() }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Mul;

    use yellowstone_vixen_mock::tx_fixture;

    use super::{Parser, TokenExtensionProgramIx, TokenExtensionProgramIxParser};
    use crate::token_program::ix_parser::TokenProgramIx;
    #[tokio::test]
    async fn test_mint_to_checked_ix_parsing() {
        let parser = TokenExtensionProgramIxParser;

        let ixs = tx_fixture!("44gWEyKUkeUabtJr4eT3CQEkFGrD4jMdwUV6Ew5MR5K3RGizs9iwbkb5Q4T3gnAaSgHxn3ERQ8g5YTXuLP1FrWnt",&parser);

        let TokenExtensionProgramIx::TokenProgramIx(TokenProgramIx::MintToChecked(_accts, data)) =
            &ixs[0]
        else {
            panic!("Invalid Instruction");
        };

        assert_eq!(data.decimals, 9);
        assert_eq!(data.amount, 100.mul(10u64.pow(data.decimals.into())));
    }
}
