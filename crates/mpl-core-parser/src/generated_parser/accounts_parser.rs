//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::{
    accounts::{AssetV1, CollectionV1, HashedAssetV1, PluginHeaderV1, PluginRegistryV1},
    ID,
};

/// MplCoreProgram Program State
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum MplCoreProgramProgramState {
    PluginHeaderV1(PluginHeaderV1),
    PluginRegistryV1(PluginRegistryV1),
    AssetV1(AssetV1),
    CollectionV1(CollectionV1),
    HashedAssetV1(HashedAssetV1),
}

impl MplCoreProgramProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> yellowstone_vixen_core::ParseResult<Self> {
        let data_len = data_bytes.len();
        const PLUGINREGISTRYV1_LEN: usize = std::mem::size_of::<PluginRegistryV1>();
        const ASSETV1_LEN: usize = std::mem::size_of::<AssetV1>();
        const COLLECTIONV1_LEN: usize = std::mem::size_of::<CollectionV1>();
        match data_len {
            PluginHeaderV1::LEN => Ok(MplCoreProgramProgramState::PluginHeaderV1(
                PluginHeaderV1::from_bytes(data_bytes)?,
            )),
            PLUGINREGISTRYV1_LEN => Ok(MplCoreProgramProgramState::PluginRegistryV1(
                PluginRegistryV1::from_bytes(data_bytes)?,
            )),
            ASSETV1_LEN => Ok(MplCoreProgramProgramState::AssetV1(AssetV1::from_bytes(
                data_bytes,
            )?)),
            COLLECTIONV1_LEN => Ok(MplCoreProgramProgramState::CollectionV1(
                CollectionV1::from_bytes(data_bytes)?,
            )),
            HashedAssetV1::LEN => Ok(MplCoreProgramProgramState::HashedAssetV1(
                HashedAssetV1::from_bytes(data_bytes)?,
            )),
            _ => Err(yellowstone_vixen_core::ParseError::from(
                "Invalid Account data length".to_owned(),
            )),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AccountParser;

impl yellowstone_vixen_core::Parser for AccountParser {
    type Input = yellowstone_vixen_core::AccountUpdate;
    type Output = MplCoreProgramProgramState;

    fn id(&self) -> std::borrow::Cow<str> { "mpl_core_program::AccountParser".into() }

    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
        yellowstone_vixen_core::Prefilter::builder()
            .account_owners([ID])
            .build()
            .unwrap()
    }

    async fn parse(
        &self,
        acct: &yellowstone_vixen_core::AccountUpdate,
    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
        let inner = acct
            .account
            .as_ref()
            .ok_or(solana_program::program_error::ProgramError::InvalidArgument)?;
        MplCoreProgramProgramState::try_unpack(&inner.data)
    }
}

impl yellowstone_vixen_core::ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { ID.to_bytes().into() }
}
