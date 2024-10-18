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
            ExtensionType::Uninitialized => Err(ProgramError::InvalidArgument),
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
    use spl_token_2022::state::{Account, Mint, Multisig};
    use spl_token_group_interface::state::{TokenGroup, TokenGroupMember};
    use spl_token_metadata_interface::state::TokenMetadata;
    #[allow(clippy::wildcard_imports)]
    use yellowstone_vixen_proto::parser::{
        extension_data_proto::Data, ConfidentialTransferAccountProto,
        ConfidentialTransferFeeAmountProto, ConfidentialTransferFeeConfigProto,
        ConfidentialTransferMintProto, CpiGuardProto, DefaultAccountStateProto, ExtensionDataProto,
        GroupMemberPointerProto, GroupPointerProto, ImmutableOwnerProto,
        InterestBearingConfigProto, KeyValue, MemoTransferProto, MetadataPointerProto,
        MintCloseAuthorityProto, MintProto, MultisigProto, NonTransferableAccountProto,
        NonTransferableProto, PermanentDelegateProto, TokenAccountProto, TokenGroupMemberProto,
        TokenGroupProto, TokenMetadataProto, TransferFeeAmountProto, TransferFeeConfigProto,
        TransferFeeProto, TransferHookAccountProto, TransferHookProto,
    };

    use super::{extension, ExtensionData};
    use crate::helpers::{
        proto::{ElGamalPubkeyBytes, FromCOptionPubkeyToOptString},
        IntoProto,
    };

    macro_rules! impl_into_proto {
        ($($variant:ident),*) => {
            impl IntoProto<ExtensionDataProto> for ExtensionData {
                fn into_proto(self) -> ExtensionDataProto {
                    match self {
                        $(
                            ExtensionData::$variant(data) => ExtensionDataProto {
                                data: Some(Data::$variant(data.into_proto())),
                            },
                        )*
                    }
                }
            }
        };
    }

    impl_into_proto!(
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

    impl IntoProto<TokenAccountProto> for Account {
        fn into_proto(self) -> TokenAccountProto {
            TokenAccountProto {
                mint: self.mint.to_string(),
                owner: self.owner.to_string(),
                amount: self.amount,
                delegate: self.delegate.to_opt_string(),
                state: self.state as i32,
                is_native: self.is_native.into(),
                delegated_amount: self.delegated_amount,
                close_authority: self.close_authority.to_opt_string(),
            }
        }
    }

    impl IntoProto<MintProto> for Mint {
        fn into_proto(self) -> MintProto {
            MintProto {
                mint_authority: self.mint_authority.to_opt_string(),

                supply: self.supply,
                decimals: self.decimals.into(),
                is_initialized: self.is_initialized,
                freeze_authority: self.freeze_authority.to_opt_string(),
            }
        }
    }

    impl IntoProto<MultisigProto> for Multisig {
        fn into_proto(self) -> MultisigProto {
            MultisigProto {
                m: self.m.into(),
                n: self.n.into(),
                is_initialized: self.is_initialized,
                signers: self.signers.into_iter().map(|s| s.to_string()).collect(),
            }
        }
    }

    impl IntoProto<ImmutableOwnerProto> for ImmutableOwner {
        fn into_proto(self) -> ImmutableOwnerProto { ImmutableOwnerProto {} }
    }

    impl IntoProto<TransferFeeAmountProto> for extension::transfer_fee::TransferFeeAmount {
        fn into_proto(self) -> TransferFeeAmountProto {
            TransferFeeAmountProto {
                withheld_amount: self.withheld_amount.into(),
            }
        }
    }

    impl IntoProto<ConfidentialTransferAccountProto>
        for extension::confidential_transfer::ConfidentialTransferAccount
    {
        fn into_proto(self) -> ConfidentialTransferAccountProto {
            ConfidentialTransferAccountProto {
                approved: self.approved.into(),
                elgamal_pubkey: ElGamalPubkeyBytes::new(self.elgamal_pubkey.0).to_string(),
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

    impl IntoProto<MemoTransferProto> for extension::memo_transfer::MemoTransfer {
        fn into_proto(self) -> MemoTransferProto {
            MemoTransferProto {
                require_incoming_transfer_memos: self.require_incoming_transfer_memos.into(),
            }
        }
    }

    impl IntoProto<NonTransferableAccountProto>
        for extension::non_transferable::NonTransferableAccount
    {
        fn into_proto(self) -> NonTransferableAccountProto { NonTransferableAccountProto {} }
    }

    impl IntoProto<TransferHookAccountProto> for extension::transfer_hook::TransferHookAccount {
        fn into_proto(self) -> TransferHookAccountProto {
            TransferHookAccountProto {
                transferring: self.transferring.into(),
            }
        }
    }

    impl IntoProto<CpiGuardProto> for extension::cpi_guard::CpiGuard {
        fn into_proto(self) -> CpiGuardProto {
            CpiGuardProto {
                lock_cpi: self.lock_cpi.into(),
            }
        }
    }

    impl IntoProto<ConfidentialTransferFeeAmountProto>
        for extension::confidential_transfer_fee::ConfidentialTransferFeeAmount
    {
        fn into_proto(self) -> ConfidentialTransferFeeAmountProto {
            ConfidentialTransferFeeAmountProto {
                withheld_amount: self.withheld_amount.to_string(),
            }
        }
    }

    impl IntoProto<TransferFeeConfigProto> for extension::transfer_fee::TransferFeeConfig {
        fn into_proto(self) -> TransferFeeConfigProto {
            TransferFeeConfigProto {
                transfer_fee_config_authority: self.transfer_fee_config_authority.0.to_string(),

                withdraw_withheld_authority: self.withdraw_withheld_authority.0.to_string(),
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

    impl IntoProto<MintCloseAuthorityProto> for extension::mint_close_authority::MintCloseAuthority {
        fn into_proto(self) -> MintCloseAuthorityProto {
            MintCloseAuthorityProto {
                close_authority: self.close_authority.0.to_string(),
            }
        }
    }

    impl IntoProto<ConfidentialTransferMintProto>
        for extension::confidential_transfer::ConfidentialTransferMint
    {
        fn into_proto(self) -> ConfidentialTransferMintProto {
            ConfidentialTransferMintProto {
                authority: self.authority.0.to_string(),
                auditor_elgamal_pubkey: Into::<Option<ElGamalPubkey>>::into(
                    self.auditor_elgamal_pubkey,
                )
                .map(|x| ElGamalPubkeyBytes::new(x.0).to_string()),
                auto_approve_new_accounts: self.auto_approve_new_accounts.into(),
            }
        }
    }

    impl IntoProto<DefaultAccountStateProto> for extension::default_account_state::DefaultAccountState {
        fn into_proto(self) -> DefaultAccountStateProto {
            DefaultAccountStateProto {
                state: self.state.into(),
            }
        }
    }

    impl IntoProto<NonTransferableProto> for extension::non_transferable::NonTransferable {
        fn into_proto(self) -> NonTransferableProto { NonTransferableProto {} }
    }

    impl IntoProto<InterestBearingConfigProto>
        for extension::interest_bearing_mint::InterestBearingConfig
    {
        fn into_proto(self) -> InterestBearingConfigProto {
            InterestBearingConfigProto {
                rate_authority: self.rate_authority.0.to_string(),
                initialization_timestamp: self.initialization_timestamp.into(),
                pre_update_average_rate: Into::<i16>::into(self.pre_update_average_rate).into(),
                last_update_timestamp: self.last_update_timestamp.into(),
                current_rate: Into::<i16>::into(self.current_rate).into(),
            }
        }
    }

    impl IntoProto<PermanentDelegateProto> for extension::permanent_delegate::PermanentDelegate {
        fn into_proto(self) -> PermanentDelegateProto {
            PermanentDelegateProto {
                delegate: self.delegate.0.to_string(),
            }
        }
    }

    impl IntoProto<TransferHookProto> for extension::transfer_hook::TransferHook {
        fn into_proto(self) -> TransferHookProto {
            TransferHookProto {
                authority: self.authority.0.to_string(),
                program_id: self.program_id.0.to_string(),
            }
        }
    }

    impl IntoProto<ConfidentialTransferFeeConfigProto>
        for extension::confidential_transfer_fee::ConfidentialTransferFeeConfig
    {
        fn into_proto(self) -> ConfidentialTransferFeeConfigProto {
            ConfidentialTransferFeeConfigProto {
                authority: self.authority.0.to_string(),
                withheld_amount: self.withheld_amount.to_string(),
                withdraw_withheld_authority_elgamal_pubkey: ElGamalPubkeyBytes::new(
                    self.withdraw_withheld_authority_elgamal_pubkey.0,
                )
                .to_string(),
                harvest_to_mint_enabled: self.harvest_to_mint_enabled.into(),
            }
        }
    }

    impl IntoProto<MetadataPointerProto> for MetadataPointer {
        fn into_proto(self) -> MetadataPointerProto {
            MetadataPointerProto {
                authority: self.authority.0.to_string(),
                metadata_address: self.metadata_address.0.to_string(),
            }
        }
    }

    impl IntoProto<TokenMetadataProto> for TokenMetadata {
        fn into_proto(self) -> TokenMetadataProto {
            TokenMetadataProto {
                update_authority: self.update_authority.0.to_string(),
                mint: self.mint.to_string(),
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

    impl IntoProto<GroupPointerProto> for GroupPointer {
        fn into_proto(self) -> GroupPointerProto {
            GroupPointerProto {
                authority: self.authority.0.to_string(),
                group_address: self.group_address.0.to_string(),
            }
        }
    }

    impl IntoProto<TokenGroupProto> for TokenGroup {
        fn into_proto(self) -> TokenGroupProto {
            TokenGroupProto {
                update_authority: self.update_authority.0.to_string(),
                mint: self.mint.to_string(),
                size: self.size.into(),
                max_size: self.max_size.into(),
            }
        }
    }

    impl IntoProto<GroupMemberPointerProto> for GroupMemberPointer {
        fn into_proto(self) -> GroupMemberPointerProto {
            GroupMemberPointerProto {
                authority: self.authority.0.to_string(),
                member_address: self.member_address.0.to_string(),
            }
        }
    }

    impl IntoProto<TokenGroupMemberProto> for TokenGroupMember {
        fn into_proto(self) -> TokenGroupMemberProto {
            TokenGroupMemberProto {
                member_number: Into::<u32>::into(self.member_number).into(),
                mint: self.mint.to_string(),
                group: self.group.to_string(),
            }
        }
    }
}
