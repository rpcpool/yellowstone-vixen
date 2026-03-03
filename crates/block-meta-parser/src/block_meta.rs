use std::borrow::Cow;

use yellowstone_grpc_proto::geyser::SubscribeUpdateBlockMeta;
use yellowstone_vixen_core::{KeyBytes, ParseResult, Parser, Prefilter, ProgramParser};
use yellowstone_vixen_proc_macro::vixen;

#[vixen]
#[derive(Clone, PartialEq)]
pub struct Reward {
    pub pubkey: String,
    pub lamports: i64,
    pub post_balance: u64,
    pub reward_type: i32,
    pub commission: String,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct Rewards {
    pub rewards: Vec<Reward>,
    pub num_partitions: ::core::option::Option<u64>,
}

#[vixen]
#[derive(Clone, PartialEq)]
pub struct BlockMetaUpdate {
    pub slot: u64,
    pub blockhash: String,
    pub rewards: ::core::option::Option<Rewards>,
    pub block_time: ::core::option::Option<i64>,
    pub block_height: ::core::option::Option<u64>,
    pub parent_slot: u64,
    pub parent_blockhash: String,
    pub executed_transaction_count: u64,
    pub entries_count: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockMetaParser;

impl Parser for BlockMetaParser {
    type Input = SubscribeUpdateBlockMeta;
    type Output = BlockMetaUpdate;

    fn id(&self) -> Cow<'static, str> { "yellowstone::BlockMetaParser".into() }

    fn prefilter(&self) -> Prefilter { Prefilter::builder().block_metas().build().unwrap() }

    async fn parse(&self, block_meta: &SubscribeUpdateBlockMeta) -> ParseResult<Self::Output> {
        let rewards = block_meta.rewards.as_ref().map(|r| Rewards {
            rewards: r
                .rewards
                .iter()
                .map(|reward| Reward {
                    pubkey: reward.pubkey.clone(),
                    lamports: reward.lamports,
                    post_balance: reward.post_balance,
                    reward_type: reward.reward_type,
                    commission: reward.commission.clone(),
                })
                .collect(),
            num_partitions: r.num_partitions.map(|n| n.num_partitions),
        });

        Ok(BlockMetaUpdate {
            slot: block_meta.slot,
            blockhash: block_meta.blockhash.clone(),
            rewards,
            block_time: block_meta.block_time.map(|t| t.timestamp),
            block_height: block_meta.block_height.map(|h| h.block_height),
            parent_slot: block_meta.parent_slot,
            parent_blockhash: block_meta.parent_blockhash.clone(),
            executed_transaction_count: block_meta.executed_transaction_count,
            entries_count: block_meta.entries_count,
        })
    }
}

impl ProgramParser for BlockMetaParser {
    /// "B111111111111111111111111111111111111111112"
    #[inline]
    fn program_id(&self) -> KeyBytes<32> {
        KeyBytes::<32>::new([
            2, 143, 206, 223, 9, 17, 53, 163, 33, 32, 251, 255, 120, 243, 177, 49, 160, 203, 100,
            118, 223, 255, 122, 65, 91, 88, 104, 0, 0, 0, 0, 1,
        ])
    }
}
