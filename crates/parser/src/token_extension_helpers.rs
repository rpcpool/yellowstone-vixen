use spl_token_2022::{
    extension::{self, immutable_owner::ImmutableOwner, BaseStateWithExtensions, ExtensionType, StateWithExtensions},
    solana_program::program_error::ProgramError,state::{Account, Mint}
};
// #[cfg(feature = "token-extensions")]
use spl_type_length_value::variable_len_pack::VariableLenPack;
// #[cfg(feature = "token-extensions")]
use spl_pod::bytemuck::pod_from_bytes;
use spl_token_metadata_interface::state::TokenMetadata;
use spl_token_group_interface::state::{TokenGroup, TokenGroupMember};


pub fn get_token_account_extensions_data_bytes<'data>(
    state_with_ex: &'data StateWithExtensions<Account>,
    extention_type: ExtensionType,
) -> Result<&'data [u8], ProgramError> {
    let extension_data = match extention_type {
        ExtensionType::ImmutableOwner => state_with_ex
            .get_extension_bytes::<extension::immutable_owner::ImmutableOwner>()?,
        ExtensionType::TransferFeeAmount => state_with_ex.get_extension_bytes::<extension::transfer_fee::TransferFeeAmount>()?,
        ExtensionType::ConfidentialTransferAccount => state_with_ex.get_extension_bytes::<extension::confidential_transfer::ConfidentialTransferAccount>()?,
        ExtensionType::MemoTransfer => state_with_ex.get_extension_bytes::<extension::memo_transfer::MemoTransfer>()?,
        ExtensionType::NonTransferableAccount => state_with_ex.get_extension_bytes::<extension::non_transferable::NonTransferableAccount>()?,
        ExtensionType::TransferHookAccount => state_with_ex.get_extension_bytes::<extension::transfer_hook::TransferHookAccount>()?,
        ExtensionType::CpiGuard => state_with_ex.get_extension_bytes::<extension::cpi_guard::CpiGuard>()?,
        ExtensionType::ConfidentialTransferFeeAmount => state_with_ex.get_extension_bytes::<extension::confidential_transfer_fee::ConfidentialTransferFeeAmount>()?,
        //return empty slice if these cases dont match 
        _ => &[]
       
    };

    Ok(extension_data)
}

pub fn get_mint_account_extensions_data_bytes<'data>(
    state_with_ex: &'data StateWithExtensions<Mint>,
    extention_type: ExtensionType,
) -> Result<&'data [u8], ProgramError> {
    let extension_data = match extention_type {
        ExtensionType::TransferFeeConfig => state_with_ex.get_extension_bytes::<extension::transfer_fee::TransferFeeConfig>()?,
        ExtensionType::MintCloseAuthority => state_with_ex
            .get_extension_bytes::<extension::mint_close_authority::MintCloseAuthority>(
        )?,
        ExtensionType::ConfidentialTransferMint => state_with_ex.get_extension_bytes::<extension::confidential_transfer::ConfidentialTransferMint>()?,
        ExtensionType::DefaultAccountState => state_with_ex.get_extension_bytes::<extension::default_account_state::DefaultAccountState>()?,
        ExtensionType::NonTransferable => state_with_ex.get_extension_bytes::<extension::non_transferable::NonTransferable>()?,
        ExtensionType::InterestBearingConfig => state_with_ex.get_extension_bytes::<extension::interest_bearing_mint::InterestBearingConfig>()?,
        ExtensionType::PermanentDelegate => state_with_ex.get_extension_bytes::<extension::permanent_delegate::PermanentDelegate>()?,
        ExtensionType::TransferHook => state_with_ex.get_extension_bytes::<extension::transfer_hook::TransferHook>()?,
        ExtensionType::ConfidentialTransferFeeConfig => state_with_ex.get_extension_bytes::<extension::confidential_transfer_fee::ConfidentialTransferFeeConfig>()?,
        ExtensionType::MetadataPointer => state_with_ex.get_extension_bytes::<extension::metadata_pointer::MetadataPointer>()?,
        ExtensionType::TokenMetadata => state_with_ex.get_extension_bytes::<TokenMetadata>()?,
        ExtensionType::GroupPointer => state_with_ex.get_extension_bytes::<extension::group_pointer::GroupPointer>()?,
        ExtensionType::TokenGroup => state_with_ex.get_extension_bytes::<TokenGroup>()?,
        ExtensionType::GroupMemberPointer => state_with_ex.get_extension_bytes::<extension::group_member_pointer::GroupMemberPointer>()?,
        ExtensionType::TokenGroupMember => state_with_ex.get_extension_bytes::<TokenGroupMember>()?,
        //return empty slice if these cases dont match 
        _ => &[]
    };

    Ok(extension_data)
}


pub fn parse_immutable_owner_extension(data_bytes: &[u8]) -> Result<ImmutableOwner,ProgramError> {
    let immutable_owner = pod_from_bytes::<extension::immutable_owner::ImmutableOwner>(data_bytes)?;
    Ok(immutable_owner.to_owned())
}


pub fn parse_transfer_fee_amount_extension(data_bytes: &[u8]) -> Result<extension::transfer_fee::TransferFeeAmount,ProgramError> {
    let transfer_fee_amount = pod_from_bytes::<extension::transfer_fee::TransferFeeAmount>(data_bytes)?;
    Ok(transfer_fee_amount.to_owned())
}

pub fn parse_confidential_transfer_account_extension(data_bytes: &[u8]) -> Result<extension::confidential_transfer::ConfidentialTransferAccount,ProgramError> {
    let confidential_transfer_account = pod_from_bytes::<extension::confidential_transfer::ConfidentialTransferAccount>(data_bytes)?;
    Ok(confidential_transfer_account.to_owned())
}

pub fn parse_memo_transfer_extension(data_bytes: &[u8]) -> Result<extension::memo_transfer::MemoTransfer,ProgramError> {
    let memo_transfer = pod_from_bytes::<extension::memo_transfer::MemoTransfer>(data_bytes)?;
    Ok(memo_transfer.to_owned())
}

pub fn parse_non_transferable_account_extension(data_bytes: &[u8]) -> Result<extension::non_transferable::NonTransferableAccount,ProgramError> {
    let non_transferable_account = pod_from_bytes::<extension::non_transferable::NonTransferableAccount>(data_bytes)?;
    Ok(non_transferable_account.to_owned())
}

pub fn parse_transfer_hook_account_extension(data_bytes: &[u8]) -> Result<extension::transfer_hook::TransferHookAccount,ProgramError> {
    let transfer_hook_account = pod_from_bytes::<extension::transfer_hook::TransferHookAccount>(data_bytes)?;
    Ok(transfer_hook_account.to_owned())
}

pub fn parse_cpi_guard_extension(data_bytes: &[u8]) -> Result<extension::cpi_guard::CpiGuard,ProgramError> {
    let cpi_guard = pod_from_bytes::<extension::cpi_guard::CpiGuard>(data_bytes)?;
    Ok(cpi_guard.to_owned())
}

pub fn parse_confidential_transfer_fee_amount_extension(data_bytes: &[u8]) -> Result<extension::confidential_transfer_fee::ConfidentialTransferFeeAmount,ProgramError> {
    let confidential_transfer_fee_amount = pod_from_bytes::<extension::confidential_transfer_fee::ConfidentialTransferFeeAmount>(data_bytes)?;
    Ok(confidential_transfer_fee_amount.to_owned())
}


pub fn parse_transfer_fee_config_extension(data_bytes: &[u8]) -> Result<extension::transfer_fee::TransferFeeConfig,ProgramError> {
    let transfer_fee_config = pod_from_bytes::<extension::transfer_fee::TransferFeeConfig>(data_bytes)?;
    Ok(transfer_fee_config.to_owned())
}

pub fn parse_mint_close_authority_extension(data_bytes: &[u8]) -> Result<extension::mint_close_authority::MintCloseAuthority,ProgramError> {
    let mint_close_authority = pod_from_bytes::<extension::mint_close_authority::MintCloseAuthority>(data_bytes)?;
    Ok(mint_close_authority.to_owned())
}

pub fn parse_confidential_transfer_mint_extension(data_bytes: &[u8]) -> Result<extension::confidential_transfer::ConfidentialTransferMint,ProgramError> {
    let confidential_transfer_mint = pod_from_bytes::<extension::confidential_transfer::ConfidentialTransferMint>(data_bytes)?;
    Ok(confidential_transfer_mint.to_owned())
}


pub fn parse_default_account_state_extension(data_bytes: &[u8]) -> Result<extension::default_account_state::DefaultAccountState,ProgramError> {
    let default_account_state = pod_from_bytes::<extension::default_account_state::DefaultAccountState>(data_bytes)?;
    Ok(default_account_state.to_owned())
}


pub fn parse_non_transferable_extension(data_bytes: &[u8]) -> Result<extension::non_transferable::NonTransferable,ProgramError> {
    let non_transferable = pod_from_bytes::<extension::non_transferable::NonTransferable>(data_bytes)?;
    Ok(non_transferable.to_owned())
}

pub fn parse_interest_bearing_config_extension(data_bytes: &[u8]) -> Result<extension::interest_bearing_mint::InterestBearingConfig,ProgramError> {
    let interest_bearing_config = pod_from_bytes::<extension::interest_bearing_mint::InterestBearingConfig>(data_bytes)?;
    Ok(interest_bearing_config.to_owned())
}

pub fn parse_permanent_delegate_extension(data_bytes: &[u8]) -> Result<extension::permanent_delegate::PermanentDelegate,ProgramError> {
    let permanent_delegate = pod_from_bytes::<extension::permanent_delegate::PermanentDelegate>(data_bytes)?;
    Ok(permanent_delegate.to_owned())
}

pub fn parse_transfer_hook_extension(data_bytes: &[u8]) -> Result<extension::transfer_hook::TransferHook,ProgramError> {
    let transfer_hook = pod_from_bytes::<extension::transfer_hook::TransferHook>(data_bytes)?;
    Ok(transfer_hook.to_owned())
}

pub fn parse_confidential_transfer_fee_config_extension(data_bytes: &[u8]) -> Result<extension::confidential_transfer_fee::ConfidentialTransferFeeConfig,ProgramError> {
    let confidential_transfer_fee_config = pod_from_bytes::<extension::confidential_transfer_fee::ConfidentialTransferFeeConfig>(data_bytes)?;
    Ok(confidential_transfer_fee_config.to_owned())
}

pub fn parse_metadata_pointer_extension(data_bytes: &[u8]) -> Result<extension::metadata_pointer::MetadataPointer,ProgramError> {
    let metadata_pointer = pod_from_bytes::<extension::metadata_pointer::MetadataPointer>(data_bytes)?;
    Ok(metadata_pointer.to_owned())
}

pub fn parse_group_pointer_extension(data_bytes: &[u8]) -> Result<extension::group_pointer::GroupPointer,ProgramError> {
    let group_pointer = pod_from_bytes::<extension::group_pointer::GroupPointer>(data_bytes)?;
    Ok(group_pointer.to_owned())
}

pub fn parse_group_member_pointer_extension(data_bytes: &[u8]) -> Result<extension::group_member_pointer::GroupMemberPointer,ProgramError> {
    let group_member_pointer = pod_from_bytes::<extension::group_member_pointer::GroupMemberPointer>(data_bytes)?;
    Ok(group_member_pointer.to_owned())
}

pub fn parse_token_metadata_extension(data_bytes: &[u8]) -> Result<TokenMetadata,ProgramError> {
    let token_metadata = TokenMetadata::unpack_from_slice(data_bytes)?;
    Ok(token_metadata.to_owned())
}

pub fn parse_token_group_extension(data_bytes: &[u8]) -> Result<TokenGroup,ProgramError> {
    let token_group = pod_from_bytes::<TokenGroup>(data_bytes)?;
    Ok(token_group.to_owned())
}

pub fn parse_group_member_extension(data_bytes: &[u8]) -> Result<TokenGroupMember,ProgramError> {
    let group_member = pod_from_bytes::<TokenGroupMember>(data_bytes)?;
    Ok(group_member.to_owned())
}




#[derive(Debug,PartialEq)]
pub enum ExtensionData {
    ImmutableOwner(extension::immutable_owner::ImmutableOwner),
    TransferFeeAmount(extension::transfer_fee::TransferFeeAmount),
    ConfidentialTransferAccount(extension::confidential_transfer::ConfidentialTransferAccount),
    MemoTransfer(extension::memo_transfer::MemoTransfer),
    NonTransferableAccount(extension::non_transferable::NonTransferableAccount),
    TransferHookAccount(extension::transfer_hook::TransferHookAccount),
    CpiGuard(extension::cpi_guard::CpiGuard),
    ConfidentialTransferFeeAmount(
        extension::confidential_transfer_fee::ConfidentialTransferFeeAmount,
    ),
    TransferFeeConfig(extension::transfer_fee::TransferFeeConfig),
    MintCloseAuthority(extension::mint_close_authority::MintCloseAuthority),
    ConfidentialTransferMint(extension::confidential_transfer::ConfidentialTransferMint),
    DefaultAccountState(extension::default_account_state::DefaultAccountState),
    NonTransferable(extension::non_transferable::NonTransferable),
    InterestBearingConfig(extension::interest_bearing_mint::InterestBearingConfig),
    PermanentDelegate(extension::permanent_delegate::PermanentDelegate),
    TransferHook(extension::transfer_hook::TransferHook),
    ConfidentialTransferFeeConfig(
        extension::confidential_transfer_fee::ConfidentialTransferFeeConfig,
    ),
    MetadataPointer(extension::metadata_pointer::MetadataPointer),
    TokenMetadata(TokenMetadata),
    GroupPointer(extension::group_pointer::GroupPointer),
    TokenGroup(TokenGroup),
    GroupMemberPointer(extension::group_member_pointer::GroupMemberPointer),
    TokenGroupMember(TokenGroupMember),
}

impl ExtensionData {
    pub fn from_extension_type_and_data(
        extension_type: ExtensionType,
        data_bytes: &[u8],
    ) -> Result<Self, ProgramError> {
        match extension_type {
            ExtensionType::ImmutableOwner => Ok(ExtensionData::ImmutableOwner(
                parse_immutable_owner_extension(data_bytes)?,
            )),
            ExtensionType::TransferFeeAmount => Ok(ExtensionData::TransferFeeAmount(
                parse_transfer_fee_amount_extension(data_bytes)?,
            )),

            ExtensionType::ConfidentialTransferAccount => Ok(ExtensionData::ConfidentialTransferAccount(
                parse_confidential_transfer_account_extension(data_bytes)?,
            )),

            ExtensionType::MemoTransfer => Ok(ExtensionData::MemoTransfer(
                parse_memo_transfer_extension(data_bytes)?,
            )),

            ExtensionType::NonTransferableAccount => Ok(ExtensionData::NonTransferableAccount(
                parse_non_transferable_account_extension(data_bytes)?,
            )),
            ExtensionType::TransferHookAccount => Ok(ExtensionData::TransferHookAccount(
                parse_transfer_hook_account_extension(data_bytes)?,
            )),

            ExtensionType::CpiGuard => Ok(ExtensionData::CpiGuard(parse_cpi_guard_extension(data_bytes)?)),

            ExtensionType::ConfidentialTransferFeeAmount => Ok(ExtensionData::ConfidentialTransferFeeAmount(
                parse_confidential_transfer_fee_amount_extension(data_bytes)?,
            )),

            ExtensionType::TransferFeeConfig => Ok(ExtensionData::TransferFeeConfig(
                parse_transfer_fee_config_extension(data_bytes)?,
            )),

            ExtensionType::MintCloseAuthority => Ok(ExtensionData::MintCloseAuthority(
                parse_mint_close_authority_extension(data_bytes)?,
            )),

            ExtensionType::ConfidentialTransferMint => Ok(ExtensionData::ConfidentialTransferMint(
                parse_confidential_transfer_mint_extension(data_bytes)?,
            )),

            ExtensionType::DefaultAccountState => Ok(ExtensionData::DefaultAccountState(
                parse_default_account_state_extension(data_bytes)?,
            )),

            ExtensionType::NonTransferable => Ok(ExtensionData::NonTransferable(
                parse_non_transferable_extension(data_bytes)?,
            )),

            ExtensionType::InterestBearingConfig => Ok(ExtensionData::InterestBearingConfig(
                parse_interest_bearing_config_extension(data_bytes)?,
            )),

            ExtensionType::PermanentDelegate => Ok(ExtensionData::PermanentDelegate(
                parse_permanent_delegate_extension(data_bytes)?,
            )),

            ExtensionType::TransferHook => Ok(ExtensionData::TransferHook(
                parse_transfer_hook_extension(data_bytes)?,
            )),


            ExtensionType::ConfidentialTransferFeeConfig => Ok(ExtensionData::ConfidentialTransferFeeConfig(
                parse_confidential_transfer_fee_config_extension(data_bytes)?,
            )),

            ExtensionType::MetadataPointer => Ok(ExtensionData::MetadataPointer(
                parse_metadata_pointer_extension(data_bytes)?,
            )),

            ExtensionType::TokenMetadata => Ok(ExtensionData::TokenMetadata(
                parse_token_metadata_extension(data_bytes)?,
            )),
            ExtensionType::GroupPointer => Ok(ExtensionData::GroupPointer(
                parse_group_pointer_extension(data_bytes)?,
            )),

            ExtensionType::TokenGroup => Ok(ExtensionData::TokenGroup(
                parse_token_group_extension(data_bytes)?,
            )),

            ExtensionType::GroupMemberPointer => Ok(ExtensionData::GroupMemberPointer(
                parse_group_member_pointer_extension(data_bytes)?,
            )),

            ExtensionType::TokenGroupMember => Ok(ExtensionData::TokenGroupMember(
                parse_group_member_extension(data_bytes)?,
            )),

            _ => Err(ProgramError::InvalidArgument),
            
        }
    }
}