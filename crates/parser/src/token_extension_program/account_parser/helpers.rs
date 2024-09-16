use spl_pod::{bytemuck::pod_from_bytes, solana_program::program_pack::Pack};
use spl_token_2022::{
    extension::{
        self, BaseState, BaseStateWithExtensions, Extension, ExtensionType, StateWithExtensions,
    },
    solana_program::program_error::ProgramError,
    solana_zk_token_sdk::instruction::Pod,
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
        _ => &[],
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
    Ok(token_metadata.to_owned())
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
            _ => Err(ProgramError::InvalidArgument),
        }
    }
}

#[cfg(feature = "proto")]
pub mod token_extensions_proto_parser {
    use extension::{
        group_member_pointer::GroupMemberPointer, group_pointer::GroupPointer,
        immutable_owner::ImmutableOwner, metadata_pointer::MetadataPointer,
    };
    use solana_zk_token_sdk::zk_token_elgamal::pod::ElGamalPubkey;
    use spl_token_2022::state::Multisig;
    use yellowstone_vixen_proto::parser::{extension_data_proto::Data, *};

    use super::*;
    use crate::helpers::{
        bytes_to_elgamal_pubkey, from_coption_to_option, pubkey_to_string, IntoProtoData,
    };

    macro_rules! impl_into_proto_data {
        ($($variant:ident),*) => {
            impl IntoProtoData<ExtensionDataProto> for ExtensionData {
                fn into_proto_data(self) -> ExtensionDataProto {
                    match self {
                        $(
                            ExtensionData::$variant(data) => ExtensionDataProto {
                                data: Some(Data::$variant(data.into_proto_data())),
                            },
                        )*
                    }
                }
            }
        };
    }

    impl_into_proto_data!(
        ImmutableOwner,
        TransferFeeAmount,
        ConfidentialTransferAccount,
        MemoTransfer,
        NonTransferableAccount,
        TransferHookAccount,
        CpiGuard,
        ConfidentialTransferFeeAmount,
        TransferFeeConfig,
        MintCloseAuthority,
        ConfidentialTransferMint,
        DefaultAccountState,
        NonTransferable,
        InterestBearingConfig,
        PermanentDelegate,
        TransferHook,
        MetadataPointer,
        TokenMetadata,
        GroupPointer,
        TokenGroup,
        GroupMemberPointer,
        TokenGroupMember,
        ConfidentialTransferFeeConfig
    );

    impl IntoProtoData<TokenAccountProto> for Account {
        fn into_proto_data(self) -> TokenAccountProto {
            TokenAccountProto {
                mint: pubkey_to_string(self.mint),
                owner: pubkey_to_string(self.owner),
                amount: self.amount,
                delegate: from_coption_to_option(self.delegate.map(|d| pubkey_to_string(d))),
                state: self.state as i32,
                is_native: from_coption_to_option(self.is_native),
                delegated_amount: self.delegated_amount,
                close_authority: from_coption_to_option(
                    self.close_authority.map(|ca| pubkey_to_string(ca)),
                ),
            }
        }
    }

    impl IntoProtoData<MintProto> for Mint {
        fn into_proto_data(self) -> MintProto {
            MintProto {
                mint_authority: from_coption_to_option(
                    self.mint_authority.map(|ma| pubkey_to_string(ma)),
                ),
                supply: self.supply,
                decimals: self.decimals as u64,
                is_initialized: self.is_initialized,
                freeze_authority: from_coption_to_option(
                    self.freeze_authority.map(|fa| pubkey_to_string(fa)),
                ),
            }
        }
    }

    impl IntoProtoData<MultisigProto> for Multisig {
        fn into_proto_data(self) -> MultisigProto {
            MultisigProto {
                m: self.m.into(),
                n: self.n.into(),
                is_initialized: self.is_initialized,
                signers: self
                    .signers
                    .into_iter()
                    .map(|s| pubkey_to_string(s))
                    .collect(),
            }
        }
    }

    impl IntoProtoData<ImmutableOwnerProto> for ImmutableOwner {
        fn into_proto_data(self) -> ImmutableOwnerProto { ImmutableOwnerProto {} }
    }

    impl IntoProtoData<TransferFeeAmountProto> for extension::transfer_fee::TransferFeeAmount {
        fn into_proto_data(self) -> TransferFeeAmountProto {
            TransferFeeAmountProto {
                withheld_amount: self.withheld_amount.into(),
            }
        }
    }

    impl IntoProtoData<ConfidentialTransferAccountProto>
        for extension::confidential_transfer::ConfidentialTransferAccount
    {
        fn into_proto_data(self) -> ConfidentialTransferAccountProto {
            ConfidentialTransferAccountProto {
                approved: self.approved.into(),
                elgamal_pubkey: bytes_to_elgamal_pubkey(self.elgamal_pubkey.0),
                pending_balance: self.pending_balance_lo.to_string(),
                pending_balance_lo: self.pending_balance_lo.to_string(),
                pending_balance_hi: self.pending_balance_hi.to_string(),
                available_balance: self.available_balance.to_string(),
                decryptable_available_balance: self.decryptable_available_balance.to_string(),
                allow_confidential_credits: self.allow_confidential_credits.into(),
                pending_balance_credit_counter: self.pending_balance_credit_counter.into(),
                maximum_pending_balance_credit_counter: self
                    .maximum_pending_balance_credit_counter
                    .into(),
                expected_pending_balance_credit_counter: self
                    .expected_pending_balance_credit_counter
                    .into(),
                actual_pending_balance_credit_counter: self
                    .actual_pending_balance_credit_counter
                    .into(),
            }
        }
    }

    impl IntoProtoData<MemoTransferProto> for extension::memo_transfer::MemoTransfer {
        fn into_proto_data(self) -> MemoTransferProto {
            MemoTransferProto {
                require_incoming_transfer_memos: self.require_incoming_transfer_memos.into(),
            }
        }
    }

    impl IntoProtoData<NonTransferableAccountProto>
        for extension::non_transferable::NonTransferableAccount
    {
        fn into_proto_data(self) -> NonTransferableAccountProto { NonTransferableAccountProto {} }
    }

    impl IntoProtoData<TransferHookAccountProto> for extension::transfer_hook::TransferHookAccount {
        fn into_proto_data(self) -> TransferHookAccountProto {
            TransferHookAccountProto {
                transferring: self.transferring.into(),
            }
        }
    }

    impl IntoProtoData<CpiGuardProto> for extension::cpi_guard::CpiGuard {
        fn into_proto_data(self) -> CpiGuardProto {
            CpiGuardProto {
                lock_cpi: self.lock_cpi.into(),
            }
        }
    }

    impl IntoProtoData<ConfidentialTransferFeeAmountProto>
        for extension::confidential_transfer_fee::ConfidentialTransferFeeAmount
    {
        fn into_proto_data(self) -> ConfidentialTransferFeeAmountProto {
            ConfidentialTransferFeeAmountProto {
                withheld_amount: self.withheld_amount.to_string(),
            }
        }
    }

    impl IntoProtoData<TransferFeeConfigProto> for extension::transfer_fee::TransferFeeConfig {
        fn into_proto_data(self) -> TransferFeeConfigProto {
            TransferFeeConfigProto {
                transfer_fee_config_authority: pubkey_to_string(
                    self.transfer_fee_config_authority.0,
                ),
                withdraw_withheld_authority: pubkey_to_string(self.withdraw_withheld_authority.0),
                withheld_amount: self.withheld_amount.into(),
                older_transfer_fee: Some(TransferFeeProto {
                    epoch: self.older_transfer_fee.epoch.into(),
                    maximum_fee: self.older_transfer_fee.maximum_fee.into(),
                    transfer_fee_basis_points: Into::<u16>::into(
                        self.older_transfer_fee.transfer_fee_basis_points,
                    )
                    .into(),
                }),
                newer_transfer_fee: Some(TransferFeeProto {
                    epoch: self.newer_transfer_fee.epoch.into(),
                    maximum_fee: self.newer_transfer_fee.maximum_fee.into(),
                    transfer_fee_basis_points: Into::<u16>::into(
                        self.newer_transfer_fee.transfer_fee_basis_points,
                    )
                    .into(),
                }),
            }
        }
    }

    impl IntoProtoData<MintCloseAuthorityProto>
        for extension::mint_close_authority::MintCloseAuthority
    {
        fn into_proto_data(self) -> MintCloseAuthorityProto {
            MintCloseAuthorityProto {
                close_authority: pubkey_to_string(self.close_authority.0),
            }
        }
    }

    impl IntoProtoData<ConfidentialTransferMintProto>
        for extension::confidential_transfer::ConfidentialTransferMint
    {
        fn into_proto_data(self) -> ConfidentialTransferMintProto {
            ConfidentialTransferMintProto {
                authority: pubkey_to_string(self.authority.0),
                auditor_elgamal_pubkey: Into::<Option<ElGamalPubkey>>::into(
                    self.auditor_elgamal_pubkey,
                )
                .map(|x| bytes_to_elgamal_pubkey(x.0)),
                auto_approve_new_accounts: self.auto_approve_new_accounts.into(),
            }
        }
    }

    impl IntoProtoData<DefaultAccountStateProto>
        for extension::default_account_state::DefaultAccountState
    {
        fn into_proto_data(self) -> DefaultAccountStateProto {
            DefaultAccountStateProto {
                state: self.state.into(),
            }
        }
    }

    impl IntoProtoData<NonTransferableProto> for extension::non_transferable::NonTransferable {
        fn into_proto_data(self) -> NonTransferableProto { NonTransferableProto {} }
    }

    impl IntoProtoData<InterestBearingConfigProto>
        for extension::interest_bearing_mint::InterestBearingConfig
    {
        fn into_proto_data(self) -> InterestBearingConfigProto {
            InterestBearingConfigProto {
                rate_authority: pubkey_to_string(self.rate_authority.0),
                initialization_timestamp: self.initialization_timestamp.into(),
                pre_update_average_rate: Into::<i16>::into(self.pre_update_average_rate).into(),
                last_update_timestamp: self.last_update_timestamp.into(),
                current_rate: Into::<i16>::into(self.current_rate).into(),
            }
        }
    }

    impl IntoProtoData<PermanentDelegateProto> for extension::permanent_delegate::PermanentDelegate {
        fn into_proto_data(self) -> PermanentDelegateProto {
            PermanentDelegateProto {
                delegate: pubkey_to_string(self.delegate.0),
            }
        }
    }

    impl IntoProtoData<TransferHookProto> for extension::transfer_hook::TransferHook {
        fn into_proto_data(self) -> TransferHookProto {
            TransferHookProto {
                authority: pubkey_to_string(self.authority.0),
                program_id: pubkey_to_string(self.program_id.0),
            }
        }
    }

    impl IntoProtoData<ConfidentialTransferFeeConfigProto>
        for extension::confidential_transfer_fee::ConfidentialTransferFeeConfig
    {
        fn into_proto_data(self) -> ConfidentialTransferFeeConfigProto {
            ConfidentialTransferFeeConfigProto {
                authority: pubkey_to_string(self.authority.0),
                withheld_amount: self.withheld_amount.to_string(),
                withdraw_withheld_authority_elgamal_pubkey: bytes_to_elgamal_pubkey(
                    self.withdraw_withheld_authority_elgamal_pubkey.0,
                ),
                harvest_to_mint_enabled: self.harvest_to_mint_enabled.into(),
            }
        }
    }

    impl IntoProtoData<MetadataPointerProto> for MetadataPointer {
        fn into_proto_data(self) -> MetadataPointerProto {
            MetadataPointerProto {
                authority: pubkey_to_string(self.authority.0),
                metadata_address: pubkey_to_string(self.metadata_address.0),
            }
        }
    }

    impl IntoProtoData<TokenMetadataProto> for TokenMetadata {
        fn into_proto_data(self) -> TokenMetadataProto {
            TokenMetadataProto {
                update_authority: pubkey_to_string(self.update_authority.0),
                mint: pubkey_to_string(self.mint),
                name: self.name,
                symbol: self.symbol,
                uri: self.uri,
                additional_metadata: self
                    .additional_metadata
                    .into_iter()
                    .map(|x| KeyValue {
                        key: x.0,
                        value: x.1,
                    })
                    .collect(),
            }
        }
    }

    impl IntoProtoData<GroupPointerProto> for GroupPointer {
        fn into_proto_data(self) -> GroupPointerProto {
            GroupPointerProto {
                authority: pubkey_to_string(self.authority.0),
                group_address: pubkey_to_string(self.group_address.0),
            }
        }
    }

    impl IntoProtoData<TokenGroupProto> for TokenGroup {
        fn into_proto_data(self) -> TokenGroupProto {
            TokenGroupProto {
                update_authority: pubkey_to_string(self.update_authority.0),
                mint: pubkey_to_string(self.mint),
                size: self.size.into(),
                max_size: self.max_size.into(),
            }
        }
    }

    impl IntoProtoData<GroupMemberPointerProto> for GroupMemberPointer {
        fn into_proto_data(self) -> GroupMemberPointerProto {
            GroupMemberPointerProto {
                authority: pubkey_to_string(self.authority.0),
                member_address: pubkey_to_string(self.member_address.0),
            }
        }
    }

    impl IntoProtoData<TokenGroupMemberProto> for TokenGroupMember {
        fn into_proto_data(self) -> TokenGroupMemberProto {
            TokenGroupMemberProto {
                member_number: Into::<u32>::into(self.member_number).into(),
                mint: pubkey_to_string(self.mint),
                group: pubkey_to_string(self.group),
            }
        }
    }
}
