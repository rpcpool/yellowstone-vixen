use super::extensions::*;
use crate::ix_parser::token_program::token_ix::*;

#[derive(Debug)]
pub enum TokenExtensionProgramIx {
    TokenProgramIx(TokenProgramIx),
    TransferFeeIx(TransferFeeIx),
    ConfidentialTransferIx(ConfidentaltransferIx),
    ConfidentialtransferFeeIx(ConfidentaltransferFeeIx),
    CpiGuardIx(CommonExtIxs),
    DefaultAccountStateIx(CommonExtIxs),
    GroupMemberPointerIx(CommonExtIxs),
    GroupPointerIx(CommonExtIxs),
    InterestBearingMintIx(CommonExtIxs),
    MemoTransferIx(CommonExtIxs),
    MetadataPointerIx(CommonExtIxs),
    TransferHookIx(CommonExtIxs),
    TokenMetadataIx(TokenMetadataIx),
    TokenGroupIx(TokenGroupIx),
}
