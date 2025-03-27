//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::accounts::UserState;
use crate::accounts::LendingMarket;
use crate::accounts::Obligation;
use crate::accounts::ReferrerState;
use crate::accounts::ReferrerTokenState;
use crate::accounts::ShortUrl;
use crate::accounts::UserMetadata;
use crate::accounts::Reserve;
use crate::ID;


/// KaminoLending Program State
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum KaminoLendingProgramState {
            UserState(UserState),
            LendingMarket(LendingMarket),
            Obligation(Obligation),
            ReferrerState(ReferrerState),
            ReferrerTokenState(ReferrerTokenState),
            ShortUrl(ShortUrl),
            UserMetadata(UserMetadata),
            Reserve(Reserve),
    }

impl KaminoLendingProgramState {
    pub fn try_unpack(data_bytes:&[u8]) -> yellowstone_vixen_core::ParseResult<Self>{
        let data_len = data_bytes.len();    
                                                                                                                                        const SHORTURL_LEN:usize = std::mem::size_of::<ShortUrl>();
                                                                    match data_len {
                                    UserState::LEN => Ok(
                    KaminoLendingProgramState::UserState(
                        UserState::from_bytes(data_bytes)?
                    )
                    ),
                                                LendingMarket::LEN => Ok(
                    KaminoLendingProgramState::LendingMarket(
                        LendingMarket::from_bytes(data_bytes)?
                    )
                    ),
                                                Obligation::LEN => Ok(
                    KaminoLendingProgramState::Obligation(
                        Obligation::from_bytes(data_bytes)?
                    )
                    ),
                                                ReferrerState::LEN => Ok(
                    KaminoLendingProgramState::ReferrerState(
                        ReferrerState::from_bytes(data_bytes)?
                    )
                    ),
                                                ReferrerTokenState::LEN => Ok(
                    KaminoLendingProgramState::ReferrerTokenState(
                        ReferrerTokenState::from_bytes(data_bytes)?
                    )
                    ),
                                                SHORTURL_LEN => Ok(
                    KaminoLendingProgramState::ShortUrl(
                        ShortUrl::from_bytes(data_bytes)?
                    )
                    ),
                                                UserMetadata::LEN => Ok(
                    KaminoLendingProgramState::UserMetadata(
                        UserMetadata::from_bytes(data_bytes)?
                    )
                    ),
                                                Reserve::LEN => Ok(
                    KaminoLendingProgramState::Reserve(
                        Reserve::from_bytes(data_bytes)?
                    )
                    ),
                            _ => Err(yellowstone_vixen_core::ParseError::from("Invalid Account data length".to_owned())),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AccountParser;

impl yellowstone_vixen_core::Parser for AccountParser {
    type Input = yellowstone_vixen_core::AccountUpdate;
    type Output = KaminoLendingProgramState;

    fn id(&self) -> std::borrow::Cow<str> {
        "kamino_lending::AccountParser".into()
    }

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
        let inner = acct.account.as_ref().ok_or(solana_program::program_error::ProgramError::InvalidArgument)?;
           KaminoLendingProgramState::try_unpack(&inner.data)
    }
}

impl yellowstone_vixen_core::ProgramParser for AccountParser {
    #[inline]
    fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
        ID.to_bytes().into()
    }
}


