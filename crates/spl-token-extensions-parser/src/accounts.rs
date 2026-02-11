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

fn get_extension_d<'data, T: BaseState + Pack>(
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
    get_extension_d(state_with_ex, extension_type)
}

pub fn mint_account_extensions_data_bytes<'data>(
    state_with_ex: &'data StateWithExtensions<Mint>,
    extension_type: ExtensionType,
) -> Result<&'data [u8], ProgramError> {
    get_extension_d(state_with_ex, extension_type)
}

pub fn parse_extension_data<E: Extension + Pod>(d: &[u8]) -> Result<E, ProgramError> {
    let extension = pod_from_bytes::<E>(d)?;

    Ok(extension.to_owned())
}

pub fn parse_token_metadata_extension(d: &[u8]) -> Result<TokenMetadata, ProgramError> {
    let token_metadata = TokenMetadata::unpack_from_slice(d)?;

    Ok(token_metadata.clone())
}

#[derive(Debug, Clone, PartialEq)]
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
        // d = extension data bytes
        let (extension_type, d) = value;

        // uses aliases for better readability (One liners)
        use parse_extension_data as parse;
        use ExtensionData as ED;
        use ExtensionType as ET;

        match extension_type {
            ET::ImmutableOwner => Ok(ED::ImmutableOwner(parse(d)?)),
            ET::TransferFeeAmount => Ok(ED::TransferFeeAmount(parse(d)?)),
            ET::ConfidentialTransferAccount => Ok(ED::ConfidentialTransferAccount(parse(d)?)),
            ET::MemoTransfer => Ok(ED::MemoTransfer(parse(d)?)),
            ET::NonTransferableAccount => Ok(ED::NonTransferableAccount(parse(d)?)),
            ET::TransferHookAccount => Ok(ED::TransferHookAccount(parse(d)?)),
            ET::CpiGuard => Ok(ED::CpiGuard(parse(d)?)),
            ET::ConfidentialTransferFeeAmount => Ok(ED::ConfidentialTransferFeeAmount(parse(d)?)),
            ET::TransferFeeConfig => Ok(ED::TransferFeeConfig(parse(d)?)),
            ET::MintCloseAuthority => Ok(ED::MintCloseAuthority(parse(d)?)),
            ET::ConfidentialTransferMint => Ok(ED::ConfidentialTransferMint(parse(d)?)),
            ET::DefaultAccountState => Ok(ED::DefaultAccountState(parse(d)?)),
            ET::NonTransferable => Ok(ED::NonTransferable(parse(d)?)),
            ET::InterestBearingConfig => Ok(ED::InterestBearingConfig(parse(d)?)),
            ET::PermanentDelegate => Ok(ED::PermanentDelegate(parse(d)?)),
            ET::TransferHook => Ok(ED::TransferHook(parse(d)?)),
            ET::ConfidentialTransferFeeConfig => Ok(ED::ConfidentialTransferFeeConfig(parse(d)?)),
            ET::MetadataPointer => Ok(ED::MetadataPointer(parse(d)?)),
            ET::TokenMetadata => Ok(ED::TokenMetadata(parse_token_metadata_extension(d)?)),
            ET::GroupPointer => Ok(ED::GroupPointer(parse(d)?)),
            ET::TokenGroup => Ok(ED::TokenGroup(parse(d)?)),
            ET::GroupMemberPointer => Ok(ED::GroupMemberPointer(parse(d)?)),
            ET::TokenGroupMember => Ok(ED::TokenGroupMember(parse(d)?)),
            ET::ConfidentialMintBurn => Ok(ED::ConfidentialMintBurn(parse(d)?)),
            ET::ScaledUiAmount => Ok(ED::ScaledUiAmountConfig(parse(d)?)),
            ET::Pausable => Ok(ED::PausableConfig(parse(d)?)),
            ET::PausableAccount => Ok(ED::PausableAccount(parse(d)?)),

            ET::Uninitialized => Err(ProgramError::InvalidArgument),
        }
    }
}
