use std::borrow::Cow;

use yellowstone_vixen_core::{
    BlockMetaUpdate, ParseResult, Parser, Prefilter, ProgramParser, Pubkey,
};

#[derive(Debug, Clone)]
pub struct Reward {
    pub pubkey: String,
    pub lamports: i64,
    pub post_balance: u64,
    pub reward_type: i32,
    pub commission: String,
}

#[derive(Debug, Clone)]
pub struct Rewards {
    pub rewards: Vec<Reward>,
    pub num_partitions: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct BlockUpdate {
    pub slot: u64,
    pub blockhash: String,
    pub rewards: Option<Rewards>,
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
    pub parent_slot: u64,
    pub parent_blockhash: String,
    pub executed_transaction_count: u64,
    pub entries_count: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockMetaParser;

impl Parser for BlockMetaParser {
    type Input = BlockMetaUpdate;
    type Output = BlockUpdate;

    fn id(&self) -> Cow<str> { "yellowstone::BlockMetaParser".into() }

    fn prefilter(&self) -> Prefilter { Prefilter::builder().build().unwrap() }

    async fn parse(&self, block_meta: &BlockMetaUpdate) -> ParseResult<Self::Output> {
        let rewards = block_meta.rewards.as_ref().map(|reward| Rewards {
            rewards: reward
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
            num_partitions: reward.num_partitions.map(|num| num.num_partitions),
        });

        Ok(BlockUpdate {
            slot: block_meta.slot,
            blockhash: block_meta.blockhash.clone(),
            rewards,
            block_time: block_meta.block_time.map(|block_time| block_time.timestamp),
            block_height: block_meta
                .block_height
                .map(|block_height| block_height.block_height),
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
    fn program_id(&self) -> Pubkey {
        Pubkey::new([
            2, 143, 206, 223, 9, 17, 53, 163, 33, 32, 251, 255, 120, 243, 177, 49, 160, 203, 100,
            118, 223, 255, 122, 65, 91, 88, 104, 0, 0, 0, 0, 1,
        ])
    }
}
