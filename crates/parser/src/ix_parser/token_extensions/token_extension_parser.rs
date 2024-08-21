use helpers::ExtensionIxParser;
use spl_token_2022::instruction::TokenInstruction;
use spl_token_group_interface::instruction::TokenGroupInstruction;
use spl_token_metadata_interface::instruction::TokenMetadataInstruction;
use yellowstone_vixen_core::{
    Instruction, InstructionParser, InstructionsUpdate, ParseError, ParseResult, Parser, Prefilter,
};

use super::{extensions::*, token_extensions_ix::TokenExtensionProgramIx};
use crate::ix_parser::token_program::TokenProgramIxParser;

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
            for inner_ix in outer_ixs.instructions.iter() {
                let parsed_ix = TokenExtensionProgramIxParser::parse_ix(inner_ix)
                    .map_err(|e| ParseError::Other(e.into()))?;
                parsed_ixs.push(parsed_ix);
            }
        }
        Ok(parsed_ixs)
    }
}

impl InstructionParser<TokenExtensionProgramIx> for TokenExtensionProgramIxParser {
    fn parse_ix(ix: &Instruction) -> Result<TokenExtensionProgramIx, String> {
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
                    CommonExtIxs::try_parse_extension_ix(ExtensionWithCommonIxs::CpiGuard, ix)?,
                )),

                TokenInstruction::DefaultAccountStateExtension => {
                    Ok(TokenExtensionProgramIx::DefaultAccountStateIx(
                        CommonExtIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::DefaultAccountState,
                            ix,
                        )?,
                    ))
                },
                TokenInstruction::InterestBearingMintExtension => {
                    Ok(TokenExtensionProgramIx::InterestBearingMintIx(
                        CommonExtIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::InterestBearingMint,
                            ix,
                        )?,
                    ))
                },
                TokenInstruction::MemoTransferExtension => Ok(
                    TokenExtensionProgramIx::MemoTransferIx(CommonExtIxs::try_parse_extension_ix(
                        ExtensionWithCommonIxs::MemoTransfer,
                        ix,
                    )?),
                ),

                TokenInstruction::GroupMemberPointerExtension => {
                    Ok(TokenExtensionProgramIx::GroupMemberPointerIx(
                        CommonExtIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::GroupMemberPointer,
                            ix,
                        )?,
                    ))
                },

                TokenInstruction::GroupPointerExtension => Ok(
                    TokenExtensionProgramIx::GroupPointerIx(CommonExtIxs::try_parse_extension_ix(
                        ExtensionWithCommonIxs::GroupPointer,
                        ix,
                    )?),
                ),

                TokenInstruction::MetadataPointerExtension => {
                    Ok(TokenExtensionProgramIx::MetadataPointerIx(
                        CommonExtIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::MetadataPointer,
                            ix,
                        )?,
                    ))
                },

                TokenInstruction::TransferHookExtension => Ok(
                    TokenExtensionProgramIx::TransferHookIx(CommonExtIxs::try_parse_extension_ix(
                        ExtensionWithCommonIxs::TransferHook,
                        ix,
                    )?),
                ),

                _ => Ok(TokenExtensionProgramIx::TokenProgramIx(
                    TokenProgramIxParser::parse_ix(ix)?,
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

                Err(e.to_string())
            },
        }
    }
}
