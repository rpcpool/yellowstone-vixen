use spl_token_2022::{
    extension::{self, BaseStateWithExtensions, Extension, ExtensionType, StateWithExtensions},
    solana_program::program_error::ProgramError,
    solana_zk_token_sdk::instruction::Pod,
    state::{Account, Mint},
};

pub fn get_token_account_extensions_data_bytes<'data>(
    state_with_ex: &'data StateWithExtensions<Account>,
    extention_type: ExtensionType,
) -> Result<&'data [u8], ProgramError> {
    let extension_data = match extention_type {
        ExtensionType::ConfidentialTransferAccount => state_with_ex.get_extension_bytes::<extension::confidential_transfer::ConfidentialTransferAccount>()?,
        _ => todo!(),
    };

    Ok(extension_data)
}

pub fn get_mint_account_extensions_data_bytes<'data>(
    state_with_ex: &'data StateWithExtensions<Mint>,
    extention_type: ExtensionType,
) -> Result<&'data [u8], ProgramError> {
    let extension_data = match extention_type {
        ExtensionType::MintCloseAuthority => state_with_ex
            .get_extension_bytes::<extension::mint_close_authority::MintCloseAuthority>(
        )?,
        _ => todo!(),
    };

    Ok(extension_data)
}
