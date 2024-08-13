use helpers::ExtensionIxParser;
use spl_token_2022::instruction::TokenInstruction;
use spl_token_group_interface::instruction::TokenGroupInstruction;
use spl_token_metadata_interface::instruction::TokenMetadataInstruction;

use super::{extensions::*, token_extensions_ix::TokenExtensionProgramIx};
use crate::ix_parser::{
    token_program::TokenProgramIxParser,
    vixen_ix::structure::{InstructionParser, InstructionUpdate},
};

#[derive(Debug)]
pub struct TokenExtensionProgramIxParser;

impl InstructionParser<TokenExtensionProgramIx> for TokenExtensionProgramIxParser {
    fn parse_readable_ix(ix_update: &InstructionUpdate) -> Result<TokenExtensionProgramIx, String> {
        match TokenInstruction::unpack(&ix_update.data) {
            Ok(ix) => match ix {
                TokenInstruction::TransferFeeExtension => {
                    Ok(TokenExtensionProgramIx::TransferFeeIx(
                        TransferFeeIx::try_parse_extension_ix(ix_update)?,
                    ))
                },
                TokenInstruction::ConfidentialTransferExtension => {
                    Ok(TokenExtensionProgramIx::ConfidentialTransferIx(
                        ConfidentaltransferIx::try_parse_extension_ix(ix_update)?,
                    ))
                },
                TokenInstruction::ConfidentialTransferFeeExtension => {
                    Ok(TokenExtensionProgramIx::ConfidentialtransferFeeIx(
                        ConfidentaltransferFeeIx::try_parse_extension_ix(ix_update)?,
                    ))
                },
                TokenInstruction::CpiGuardExtension => Ok(TokenExtensionProgramIx::CpiGuardIx(
                    CommonExtIxs::try_parse_extension_ix(
                        ExtensionWithCommonIxs::CpiGuard,
                        ix_update,
                    )?,
                )),

                TokenInstruction::DefaultAccountStateExtension => {
                    Ok(TokenExtensionProgramIx::DefaultAccountStateIx(
                        CommonExtIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::DefaultAccountState,
                            ix_update,
                        )?,
                    ))
                },
                TokenInstruction::InterestBearingMintExtension => {
                    Ok(TokenExtensionProgramIx::InterestBearingMintIx(
                        CommonExtIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::InterestBearingMint,
                            ix_update,
                        )?,
                    ))
                },
                TokenInstruction::MemoTransferExtension => Ok(
                    TokenExtensionProgramIx::MemoTransferIx(CommonExtIxs::try_parse_extension_ix(
                        ExtensionWithCommonIxs::MemoTransfer,
                        ix_update,
                    )?),
                ),

                TokenInstruction::GroupMemberPointerExtension => {
                    Ok(TokenExtensionProgramIx::GroupMemberPointerIx(
                        CommonExtIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::GroupMemberPointer,
                            ix_update,
                        )?,
                    ))
                },

                TokenInstruction::GroupPointerExtension => Ok(
                    TokenExtensionProgramIx::GroupPointerIx(CommonExtIxs::try_parse_extension_ix(
                        ExtensionWithCommonIxs::GroupPointer,
                        ix_update,
                    )?),
                ),

                TokenInstruction::MetadataPointerExtension => {
                    Ok(TokenExtensionProgramIx::MetadataPointerIx(
                        CommonExtIxs::try_parse_extension_ix(
                            ExtensionWithCommonIxs::MetadataPointer,
                            ix_update,
                        )?,
                    ))
                },

                TokenInstruction::TransferHookExtension => Ok(
                    TokenExtensionProgramIx::TransferHookIx(CommonExtIxs::try_parse_extension_ix(
                        ExtensionWithCommonIxs::TransferHook,
                        ix_update,
                    )?),
                ),

                _ => Ok(TokenExtensionProgramIx::TokenProgramIx(
                    TokenProgramIxParser::parse_readable_ix(ix_update)?,
                )),
            },
            Err(e) => {
                if let Ok(_) = TokenMetadataInstruction::unpack(&ix_update.data) {
                    return Ok(TokenExtensionProgramIx::TokenMetadataIx(
                        TokenMetadataIx::try_parse_extension_ix(ix_update)?,
                    ));
                }

                if let Ok(_) = TokenGroupInstruction::unpack(&ix_update.data) {
                    return Ok(TokenExtensionProgramIx::TokenGroupIx(
                        TokenGroupIx::try_parse_extension_ix(ix_update)?,
                    ));
                }

                Err(e.to_string())
            },
        }
    }
}
