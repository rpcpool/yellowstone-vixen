use std::borrow::Cow;

use solana_loader_v3_interface::state::UpgradeableLoaderState;
use yellowstone_vixen_core::{
    AccountUpdate, ParseError, ParseResult, Parser, Prefilter, ProgramParser,
};
use yellowstone_vixen_proc_macro::vixen;

use crate::PublicKey;

#[vixen]
#[derive(Clone, PartialEq)]
pub struct Buffer {
    pub authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct Program {
    pub programdata_address: PublicKey,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct ProgramData {
    pub slot: u64,
    pub upgrade_authority: Option<PublicKey>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct BpfLoaderState {
    #[hint(oneof = "account::State", tags = "1, 2, 3")]
    pub state: ::core::option::Option<account::State>,
}

pub mod account {
    use super::vixen;

    #[vixen(oneof)]
    #[derive(Clone, PartialEq)]
    pub enum State {
        Buffer(super::Buffer),
        Program(super::Program),
        ProgramData(super::ProgramData),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccountParser;

impl Parser for AccountParser {
    type Input = AccountUpdate;
    type Output = BpfLoaderState;

    fn id(&self) -> Cow<'static, str> { "bpf_loader::AccountParser".into() }

    fn prefilter(&self) -> Prefilter {
        Prefilter::builder()
            .account_owners([solana_sdk_ids::bpf_loader_upgradeable::ID])
            .build()
            .unwrap()
    }

    async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
        let inner = acct
            .account
            .as_ref()
            .ok_or(ParseError::Other("Missing account data".into()))?;

        let data = &inner.data;

        let loader_state: UpgradeableLoaderState =
            bincode::deserialize(data).map_err(|e| ParseError::Other(e.into()))?;

        let state = match loader_state {
            UpgradeableLoaderState::Uninitialized => return Err(ParseError::Filtered),

            UpgradeableLoaderState::Buffer { authority_address } => {
                account::State::Buffer(Buffer {
                    authority: authority_address.map(|pk| PublicKey::new(pk.to_bytes())),
                })
            },

            UpgradeableLoaderState::Program {
                programdata_address,
            } => account::State::Program(Program {
                programdata_address: PublicKey::new(programdata_address.to_bytes()),
            }),

            UpgradeableLoaderState::ProgramData {
                slot,
                upgrade_authority_address,
            } => account::State::ProgramData(ProgramData {
                slot,
                upgrade_authority: upgrade_authority_address
                    .map(|pk| PublicKey::new(pk.to_bytes())),
            }),
        };

        Ok(BpfLoaderState { state: Some(state) })
    }
}

impl ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::KeyBytes<32> {
        solana_sdk_ids::bpf_loader_upgradeable::ID.to_bytes().into()
    }
}
