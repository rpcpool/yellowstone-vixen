use bytemuck::Pod;
use solana_program_error::ProgramError;
use solana_program_pack::Pack;
use spl_pod::bytemuck::pod_from_bytes;
use spl_token_2022::{
    extension::{
        self, BaseState, BaseStateWithExtensions, Extension, ExtensionType, StateWithExtensions,
    },
    state::{Account, Mint},
};
use spl_token_group_interface::state::{TokenGroup, TokenGroupMember};
use spl_token_metadata_interface::state::TokenMetadata;
use spl_type_length_value::variable_len_pack::VariableLenPack;

fn get_extension_data_bytes<'data, T: BaseState + Pack>(
    state_with_ex: &'data StateWithExtensions<T>,
    extension_type: ExtensionType,
) -> Result<&'data [u8], ProgramError> {
    let extension_data = match extension_type {
        ExtensionType::ImmutableOwner => state_with_ex.get_extension_bytes::<extension::immutable_owner::ImmutableOwner>()?,
        ExtensionType::TransferFeeAmount => state_with_ex.get_extension_bytes::<extension::transfer_fee::TransferFeeAmount>()?,
        ExtensionType::ConfidentialTransferAccount => state_with_ex.get_extension_bytes::<extension::confidential_transfer::ConfidentialTransferAccount>()?,
        ExtensionType::MemoTransfer => state_with_ex.get_extension_bytes::<extension::memo_transfer::MemoTransfer>()?,
        ExtensionType::NonTransferableAccount => state_with_ex.get_extension_bytes::<extension::non_transferable::NonTransferableAccount>()?,
        ExtensionType::TransferHookAccount => state_with_ex.get_extension_bytes::<extension::transfer_hook::TransferHookAccount>()?,
        ExtensionType::CpiGuard => state_with_ex.get_extension_bytes::<extension::cpi_guard::CpiGuard>()?,
        ExtensionType::ConfidentialTransferFeeAmount => state_with_ex.get_extension_bytes::<extension::confidential_transfer_fee::ConfidentialTransferFeeAmount>()?,
        ExtensionType::TransferFeeConfig => state_with_ex.get_extension_bytes::<extension::transfer_fee::TransferFeeConfig>()?,
        ExtensionType::MintCloseAuthority => state_with_ex.get_extension_bytes::<extension::mint_close_authority::MintCloseAuthority>()?,
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
        ExtensionType::ConfidentialMintBurn => state_with_ex.get_extension_bytes::<extension::confidential_mint_burn::ConfidentialMintBurn>()?,
        ExtensionType::ScaledUiAmount => state_with_ex.get_extension_bytes::<extension::scaled_ui_amount::ScaledUiAmountConfig>()?,
        ExtensionType::Pausable => state_with_ex.get_extension_bytes::<extension::pausable::PausableConfig>()?,
        ExtensionType::PausableAccount => state_with_ex.get_extension_bytes::<extension::pausable::PausableAccount>()?,
        ExtensionType::Uninitialized => &[],
    };

    Ok(extension_data)
}

pub fn token_account_extensions_data_bytes<'data>(
    state_with_ex: &'data StateWithExtensions<Account>,
    extension_type: ExtensionType,
) -> Result<&'data [u8], ProgramError> {
    get_extension_data_bytes(state_with_ex, extension_type)
}

pub fn mint_account_extensions_data_bytes<'data>(
    state_with_ex: &'data StateWithExtensions<Mint>,
    extension_type: ExtensionType,
) -> Result<&'data [u8], ProgramError> {
    get_extension_data_bytes(state_with_ex, extension_type)
}

pub fn parse_extension_data<E: Extension + Pod>(data_bytes: &[u8]) -> Result<E, ProgramError> {
    let extension = pod_from_bytes::<E>(data_bytes)?;
    Ok(extension.to_owned())
}

pub fn parse_token_metadata_extension(data_bytes: &[u8]) -> Result<TokenMetadata, ProgramError> {
    let token_metadata = TokenMetadata::unpack_from_slice(data_bytes)?;
    Ok(token_metadata.clone())
}

#[derive(Debug, PartialEq)]
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
    ConfidentialMintBurn(extension::confidential_mint_burn::ConfidentialMintBurn),
    ScaledUiAmountConfig(extension::scaled_ui_amount::ScaledUiAmountConfig),
    PausableConfig(extension::pausable::PausableConfig),
    PausableAccount(extension::pausable::PausableAccount),
}

impl TryFrom<(ExtensionType, &[u8])> for ExtensionData {
    type Error = ProgramError;

    fn try_from(value: (ExtensionType, &[u8])) -> Result<Self, Self::Error> {
        let (extension_type, data_bytes) = value;
        match extension_type {
            ExtensionType::ImmutableOwner => Ok(ExtensionData::ImmutableOwner(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::TransferFeeAmount => Ok(ExtensionData::TransferFeeAmount(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::ConfidentialTransferAccount => Ok(
                ExtensionData::ConfidentialTransferAccount(parse_extension_data(data_bytes)?),
            ),
            ExtensionType::MemoTransfer => Ok(ExtensionData::MemoTransfer(parse_extension_data(
                data_bytes,
            )?)),
            ExtensionType::NonTransferableAccount => Ok(ExtensionData::NonTransferableAccount(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::TransferHookAccount => Ok(ExtensionData::TransferHookAccount(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::CpiGuard => {
                Ok(ExtensionData::CpiGuard(parse_extension_data(data_bytes)?))
            },
            ExtensionType::ConfidentialTransferFeeAmount => Ok(
                ExtensionData::ConfidentialTransferFeeAmount(parse_extension_data(data_bytes)?),
            ),
            ExtensionType::TransferFeeConfig => Ok(ExtensionData::TransferFeeConfig(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::MintCloseAuthority => Ok(ExtensionData::MintCloseAuthority(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::ConfidentialTransferMint => Ok(ExtensionData::ConfidentialTransferMint(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::DefaultAccountState => Ok(ExtensionData::DefaultAccountState(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::NonTransferable => Ok(ExtensionData::NonTransferable(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::InterestBearingConfig => Ok(ExtensionData::InterestBearingConfig(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::PermanentDelegate => Ok(ExtensionData::PermanentDelegate(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::TransferHook => Ok(ExtensionData::TransferHook(parse_extension_data(
                data_bytes,
            )?)),
            ExtensionType::ConfidentialTransferFeeConfig => Ok(
                ExtensionData::ConfidentialTransferFeeConfig(parse_extension_data(data_bytes)?),
            ),
            ExtensionType::MetadataPointer => Ok(ExtensionData::MetadataPointer(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::TokenMetadata => Ok(ExtensionData::TokenMetadata(
                parse_token_metadata_extension(data_bytes)?,
            )),
            ExtensionType::GroupPointer => Ok(ExtensionData::GroupPointer(parse_extension_data(
                data_bytes,
            )?)),
            ExtensionType::TokenGroup => {
                Ok(ExtensionData::TokenGroup(parse_extension_data(data_bytes)?))
            },
            ExtensionType::GroupMemberPointer => Ok(ExtensionData::GroupMemberPointer(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::TokenGroupMember => Ok(ExtensionData::TokenGroupMember(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::ConfidentialMintBurn => Ok(ExtensionData::ConfidentialMintBurn(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::ScaledUiAmount => Ok(ExtensionData::ScaledUiAmountConfig(
                parse_extension_data(data_bytes)?,
            )),
            ExtensionType::Pausable => Ok(ExtensionData::PausableConfig(parse_extension_data(
                data_bytes,
            )?)),
            ExtensionType::PausableAccount => Ok(ExtensionData::PausableAccount(
                parse_extension_data(data_bytes)?,
            )),

            ExtensionType::Uninitialized => Err(ProgramError::InvalidArgument),
        }
    }
}
