use std::borrow::Cow;

use spl_token::{
    solana_program::{program_error::ProgramError, program_pack::Pack},
    state::{Account, Mint, Multisig},
};
use yellowstone_vixen_core::{
    AccountUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};

use crate::helpers::{from_coption_to_option, IntoProtoData};

#[derive(Debug)]
pub enum TokenProgramState {
    TokenAccount(Account),
    Mint(Mint),
    Multisig(Multisig),
}

impl TokenProgramState {
    pub fn try_unpack(data_bytes: &[u8]) -> ParseResult<Self> {
        match data_bytes.len() {
            Mint::LEN => Mint::unpack(data_bytes).map(Self::Mint).map_err(Into::into),
            Account::LEN => Account::unpack(data_bytes)
                .map(Self::TokenAccount)
                .map_err(Into::into),
            Multisig::LEN => Multisig::unpack(data_bytes)
                .map(Self::Multisig)
                .map_err(Into::into),
            _ => Err(ParseError::from("Invalid Account data length".to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TokenProgramAccParser;

impl Parser for TokenProgramAccParser {
    type Input = AccountUpdate;
    type Output = TokenProgramState;

    fn id(&self) -> Cow<str> {
        "yellowstone_vixen_parser::token_program::TokenProgramAccParser".into()
    }

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

impl ProgramParser for TokenProgramAccParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        spl_token::ID.to_bytes().into()
    }
}

#[cfg(feature = "proto")]
mod proto_parser {
    use yellowstone_vixen_proto::parser::{
        token_program_state_proto, MintProto, MultisigProto, TokenAccountProto,
        TokenProgramStateProto,
    };

    use super::*;

    impl IntoProtoData<TokenAccountProto> for Account {
        fn into_proto_data(self) -> TokenAccountProto {
            TokenAccountProto {
                mint: self.mint.to_bytes().to_vec(),
                owner: self.owner.to_bytes().to_vec(),
                amount: self.amount,
                delegate: from_coption_to_option(self.delegate.map(|d| d.to_bytes().to_vec())),
                state: self.state as i32,
                is_native: from_coption_to_option(self.is_native),
                delegated_amount: self.delegated_amount,
                close_authority: from_coption_to_option(
                    self.close_authority.map(|ca| ca.to_bytes().to_vec()),
                ),
            }
        }
    }

    impl IntoProtoData<MintProto> for Mint {
        fn into_proto_data(self) -> MintProto {
            MintProto {
                mint_authority: from_coption_to_option(
                    self.mint_authority.map(|ma| ma.to_bytes().to_vec()),
                ),
                supply: self.supply,
                decimals: self.decimals as u64,
                is_initialized: self.is_initialized,
                freeze_authority: from_coption_to_option(
                    self.freeze_authority.map(|fa| fa.to_bytes().to_vec()),
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
                signers: self.signers.iter().map(|s| s.to_bytes().to_vec()).collect(),
            }
        }
    }

    impl crate::proto::IntoProto for TokenProgramAccParser {
        type Proto = TokenProgramStateProto;

        fn into_proto(value: Self::Output) -> Self::Proto {
            let state_oneof = match value {
                TokenProgramState::TokenAccount(data) => Some(
                    token_program_state_proto::StateOneof::TokenAccount(data.into_proto_data()),
                ),
                TokenProgramState::Mint(data) => Some(token_program_state_proto::StateOneof::Mint(
                    data.into_proto_data(),
                )),
                TokenProgramState::Multisig(data) => Some(
                    token_program_state_proto::StateOneof::Multisig(data.into_proto_data()),
                ),
            };
            Self::Proto { state_oneof }
        }
    }
}
#[cfg(test)]
mod tests {
    use yellowstone_vixen_mock::{account_fixture, run_account_parse, FixtureData};

    use super::*;

    #[tokio::test]
    async fn test_mint_account_parsing() {
        let parser = TokenProgramAccParser;

        let fixture_data = account_fixture!("3SmPYPvZfEmroktLiJsgaNENuPEud3Z52zSfLQ1zJdkK");

        if let FixtureData::Account(account) = fixture_data {
            let state = run_account_parse!(parser, account);

            if let TokenProgramState::Mint(mint) = state {
                assert_eq!(mint.decimals, 10);
            } else {
                panic!("Invalid Account");
            }
        } else {
            panic!("Invalid Fixture Data");
        }
    }
}
