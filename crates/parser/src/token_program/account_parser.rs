use std::borrow::Cow;

use spl_token::{
    solana_program::{program_error::ProgramError, program_pack::Pack},
    state::{Account, Mint, Multisig},
};
use yellowstone_vixen_core::{
    AccountUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};

#[derive(Debug)]
#[cfg_attr(feature = "tracing", derive(strum_macros::Display))]
pub enum TokenProgramState {
    TokenAccount(Account),
    Mint(Mint),
    Multisig(Multisig),
}

impl TokenProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        let acc = match data_bytes.len() {
            Mint::LEN => Mint::unpack_from_slice(data_bytes)
                .map(Self::Mint)
                .map_err(Into::into),
            Account::LEN => Account::unpack_from_slice(data_bytes)
                .map(Self::TokenAccount)
                .map_err(Into::into),
            Multisig::LEN => Multisig::unpack_from_slice(data_bytes)
                .map(Self::Multisig)
                .map_err(Into::into),
            _ => return Err(ParseError::Filtered),
        };

        #[cfg(feature = "tracing")]
        match &acc {
            Ok(acc) => {
                tracing::info!(
                    name: "correctly_parsed_account",
                    name = "account_update",
                    program = spl_token::ID.to_string(),
                    account = acc.to_string()
                );
            },
            Err(e) => {
                tracing::info!(
                    name: "incorrectly_parsed_account",
                    name = "account_update",
                    program = spl_token::ID.to_string(),
                    account = "error",
                    discriminator = ?data_bytes.len(),
                    error = ?e
                );
            },
        }

        acc
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccountParser;

impl Parser for AccountParser {
    type Input = AccountUpdate;
    type Output = TokenProgramState;

    fn id(&self) -> Cow<'static, str> { "token_program::AccountParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([spl_token::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct.account.as_ref().ok_or(ProgramError::InvalidArgument)?;

        TokenProgramState::try_unpack(&inner.data)
    }
}

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey { spl_token::ID.to_bytes().into() }
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_core::proto::ParseProto;
    use yellowstone_vixen_proto::parser::token::{
        program_state as token_program_state_proto, MintProto, MultisigProto,
        ProgramState as TokenProgramStateProto, TokenAccountProto,
    };

    use super::{Account, AccountParser, Mint, Multisig, TokenProgramState};
    use crate::helpers::{
        proto::{FromCOptionPubkeyToOptString, FromVecPubkeyToVecString},
        IntoProto,
    };

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
                signers: self.signers.to_string_vec(),
            }
        }
    }

    impl ParseProto for AccountParser {
        type Message = TokenProgramStateProto;

        fn output_into_message(value: Self::Output) -> Self::Message {
            let state_oneof = match value {
                TokenProgramState::TokenAccount(data) => Some(
                    token_program_state_proto::StateOneof::TokenAccount(data.into_proto()),
                ),
                TokenProgramState::Mint(data) => Some(token_program_state_proto::StateOneof::Mint(
                    data.into_proto(),
                )),
                TokenProgramState::Multisig(data) => Some(
                    token_program_state_proto::StateOneof::Multisig(data.into_proto()),
                ),
            };
            Self::Message { state_oneof }
        }
    }
}
#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::{account_fixture, run_account_parse, FixtureData};

    use super::{AccountParser, Parser, TokenProgramState};

    #[tokio::test]
    async fn test_mint_account_parsing() {
        let parser = AccountParser;

        let account = account_fixture!("3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK", &parser);

        let TokenProgramState::Mint(mint) = account else {
            panic!("Invalid Account");
        };

        assert_eq!(mint.decimals, 10);
    }
}
