use borsh::BorshDeserialize;
// use yellowstone_vixen_core::Pubkey;
use spl_stake_pool::{
    instruction::{FundingType, PreferredValidatorType},
    solana_program::pubkey::Pubkey,
    state::{Fee, FeeType},
};

// Fee rate as a ratio, minted on `UpdateStakePoolBalance` as a proportion of
// the rewards
// If either the numerator or the denominator is 0, the fee is considered to be
// 0
// #[derive(Debug, BorshDeserialize)]
// pub struct Fee {
//     /// denominator of the fee ratio
//     pub denominator: u64,
//
//     /// numerator of the fee ratio
//     pub numerator: u64,
// }

// #[derive(Debug, BorshDeserialize)]
// pub enum PreferredValidatorType {
//     Deposit,
//     Withdraw,
// }

// #[derive(Debug, BorshDeserialize)]
// pub enum FeeType {
//     SolReferral(u8),
//     StakeReferral(u8),
//     Epoch(Fee),
//     StakeWithdrawal(Fee),
//     SolDeposit(Fee),
//     StakeDeposit(Fee),
//     SolWithdrawal(Fee),
// }

// #[derive(Debug, BorshDeserialize)]
// pub enum FundingType {
//     StakeDeposit,
//     SolDeposit,
//     SolWithdraw,
// }

///   (Staker only) Adds stake account delegated to validator to the pool's
///   list of managed validators.
///
///   The stake account will have the rent-exempt amount plus
///   `max(
///     crate::MINIMUM_ACTIVE_STAKE,
///     solana_program::stake::tools::get_minimum_delegation()
///   )`.
///   It is funded from the stake pool reserve.
///
///  User data: optional non-zero `u32` seed used for generating the
///  validator stake address
#[derive(Debug, BorshDeserialize)]
pub struct AddValidatorToPoolAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[s]` Staker
    pub staker: Pubkey,

    ///   2. `[w]` Reserve stake account
    pub funder: Pubkey,

    ///   3. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw: Pubkey,

    ///   4. `[w]` Validator stake list storage account
    pub validator_list: Pubkey,

    ///   5. `[w]` Stake account to add to the pool
    pub stake: Pubkey,

    ///   6. `[]` Validator this stake account will be delegated to
    pub validator: Pubkey,

    ///   7. `[]` Rent sysvar
    pub rent: Pubkey,

    ///   8. `[]` Clock sysvar
    pub clock: Pubkey,

    ///   9. '[]' Stake history sysvar
    pub sysvar_stake_history: Pubkey,

    ///  10. '[]' Stake config sysvar
    pub stake_config: Pubkey,

    ///  11. `[]` System program
    pub system_program: Pubkey,

    ///  12. `[]` Stake program
    pub stake_program: Pubkey,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct AddValidatorToPoolData {
    pub raw_validator_seed: u32,
}

/// Cleans up validator stake account entries marked as `ReadyForRemoval`
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct CleanupRemovedValidatorEntriesAccounts {
    /// 0. `[]` Stake pool
    pub stake_pool: Pubkey,

    /// 1. `[w]` Validator stake list storage account
    pub validator_list_storage: Pubkey,
}

/// Create token metadata for the stake-pool token in the
/// metaplex-token program
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct CreateTokenMetadataAccounts {
    /// 0. `[]` Stake pool
    pub stake_pool: Pubkey,

    /// 1. `[s]` Manager
    pub manager: Pubkey,

    /// 2. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    /// 3. `[]` Pool token mint account
    pub pool_mint: Pubkey,

    /// 4. `[s, w]` Payer for creation of token metadata account
    pub payer: Pubkey,

    /// 5. `[w]` Token metadata account
    pub token_metadata: Pubkey,

    /// 6. `[]` Metadata program id
    pub mpl_token_metadata: Pubkey,

    /// 7. `[]` System program id
    pub system_program: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct CreateTokenMetadataData {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

/// NOTE: This instruction has been deprecated since version 0.7.0. Please
/// use `DecreaseValidatorStakeWithReserve` instead.
///
/// (Staker only) Decrease active stake on a validator, eventually moving it
/// to the reserve
///
/// Internally, this instruction splits a validator stake account into its
/// corresponding transient stake account and deactivates it.
///
/// In order to rebalance the pool without taking custody, the staker needs
/// a way of reducing the stake on a stake account. This instruction splits
/// some amount of stake, up to the total activated stake, from the
/// canonical validator stake account, into its "transient" stake
/// account.
///
/// The instruction only succeeds if the transient stake account does not
/// exist. The amount of lamports to move must be at least rent-exemption
/// plus `max(crate::MINIMUM_ACTIVE_STAKE,
/// solana_program::stake::tools::get_minimum_delegation())`.
#[derive(Debug, BorshDeserialize)]
pub struct DecreaseValidatorStakeAccounts {
    ///  0. `[]` Stake pool
    pub stake_pool: Pubkey,

    ///  1. `[s]` Stake pool staker
    pub staker: Pubkey,

    ///  2. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///  3. `[w]` Validator list
    pub validator_list: Pubkey,

    ///  4. `[w]` Canonical stake account to split from
    pub validator_stake: Pubkey,

    ///  5. `[w]` Transient stake account to receive split
    pub transient_stake: Pubkey,

    ///  6. `[]` Clock sysvar
    pub clock: Pubkey,

    ///  7. `[]` Rent sysvar
    pub rent: Pubkey,

    ///  8. `[]` System program
    pub system_program: Pubkey,

    ///  9. `[]` Stake program
    pub stake_program: Pubkey,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct DecreaseValidatorStakeData {
    /// amount of lamports to split into the transient stake account
    pub lamports: u64,

    /// seed used to create transient stake account
    pub transient_stake_seed: u64,
}

///   Deposit SOL directly into the pool's reserve account. The output is a
///   "pool" token representing ownership into the pool. Inputs are
///   converted to the current ratio.
#[derive(Debug, BorshDeserialize)]
pub struct DepositSolAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///   2. `[w]` Reserve stake account, to deposit SOL
    pub reserve_stake_account: Pubkey,

    ///   3. `[s]` Account providing the lamports to be deposited into the pool
    pub lamports_from: Pubkey,

    ///   4. `[w]` User account to receive pool tokens
    pub pool_tokens_to: Pubkey,

    ///   5. `[w]` Account to receive fee tokens
    pub manager_fee_account: Pubkey,

    ///   6. `[w]` Account to receive a portion of fee as referral fees
    pub referrer_pool_tokens_account: Pubkey,

    ///   7. `[w]` Pool token mint account
    pub pool_mint: Pubkey,

    ///   8. `[]` System program account
    pub system_program: Pubkey,

    ///   9. `[]` Token program id
    pub token_program: Pubkey,

    ///  10. `[s]` (Optional) Stake pool sol deposit authority.
    pub deposit_authority: Option<Pubkey>,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct DepositSolData {
    pub arg: u64,
}

///   Deposit some stake into the pool. The output is a "pool" token
///   representing ownership into the pool. Inputs are converted to the
///   current ratio.
#[derive(Debug, BorshDeserialize)]
pub struct DepositStakeAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[w]` Validator stake list storage account
    pub validator_list_storage: Pubkey,

    ///   2. `[s]/[]` Stake pool deposit authority
    pub stake_pool_deposit_authority: Pubkey,

    ///   3. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///   4. `[w]` Stake account to join the pool (withdraw authority for the
    ///      stake account should be first set to the stake pool deposit
    ///      authority)
    pub deposit_stake_address: Pubkey,

    ///   5. `[w]` Validator stake account for the stake account to be merged
    ///      with
    pub validator_stake_account: Pubkey,

    ///   6. `[w]` Reserve stake account, to withdraw rent exempt reserve
    pub reserve_stake_account: Pubkey,

    ///   7. `[w]` User account to receive pool tokens
    pub pool_tokens_to: Pubkey,

    ///   8. `[w]` Account to receive pool fee tokens
    pub manager_fee_account: Pubkey,

    ///   9. `[w]` Account to receive a portion of pool fee tokens as referral
    ///      fees
    pub referrer_pool_tokens_account: Pubkey,

    ///   10. `[w]` Pool token mint account
    pub pool_mint: Pubkey,

    ///   11. '[]' Sysvar clock account
    pub clock: Pubkey,

    ///   12. '[]' Sysvar stake history account
    pub sysvar_stake_history: Pubkey,

    ///   13. `[]` Pool token program id,
    pub token_program: Pubkey,

    ///   14. `[]` Stake program id,
    pub stake_program: Pubkey,
}

/// (Staker only) Increase stake on a validator from the reserve account
///
/// Internally, this instruction splits reserve stake into a transient stake
/// account and delegate to the appropriate validator.
/// `UpdateValidatorListBalance` will do the work of merging once it's
/// ready.
///
/// This instruction only succeeds if the transient stake account does not
/// exist. The minimum amount to move is rent-exemption plus
/// `max(crate::MINIMUM_ACTIVE_STAKE,
/// solana_program::stake::tools::get_minimum_delegation())`.
///
/// User data: amount of lamports to increase on the given validator.
///
/// The actual amount split into the transient stake account is:
/// `lamports + stake_rent_exemption`.
///
/// The rent-exemption of the stake account is withdrawn back to the
/// reserve after it is merged.
#[derive(Debug, BorshDeserialize)]
pub struct IncreaseValidatorStakeAccounts {
    ///  0. `[]` Stake pool
    pub stake_pool: Pubkey,

    ///  1. `[s]` Stake pool staker
    pub staker: Pubkey,

    ///  2. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///  3. `[w]` Validator list
    pub validator_list: Pubkey,

    ///  4. `[w]` Stake pool reserve stake
    pub reserve_stake: Pubkey,

    ///  5. `[w]` Transient stake account
    pub transient_stake: Pubkey,

    ///  6. `[]` Validator stake account
    pub validator_stake: Pubkey,

    ///  7. `[]` Validator vote account to delegate to
    pub validator: Pubkey,

    ///  8. '[]' Clock sysvar
    pub clock: Pubkey,

    ///  9. '[]' Rent sysvar
    pub rent: Pubkey,

    /// 10. `[]` Stake History sysvar
    pub sysvar_stake_history: Pubkey,

    /// 11. `[]` Stake Config sysvar
    pub stake_config: Pubkey,

    /// 12. `[]` System program
    pub system_program: Pubkey,

    /// 13. `[]` Stake program
    pub stake_program: Pubkey,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct IncreaseValidatorStakeData {
    /// amount of lamports to increase on the given validator
    pub lamports: u64,

    /// seed used to create transient stake account
    pub transient_stake_seed: u64,
}

///   Initializes a new `StakePool`.
#[derive(Debug, BorshDeserialize)]
pub struct InitializeAccounts {
    ///   0. `[w]` New `StakePool` to create.
    pub stake_pool: Pubkey,

    ///   1. `[s]` Manager
    pub manager: Pubkey,

    ///   2. `[]` Staker
    pub staker: Pubkey,

    ///   3. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///   4. `[w]` Uninitialized validator stake list storage account
    pub validator_list: Pubkey,

    ///   5. `[]` Reserve stake account must be initialized, have zero balance,
    ///      and staker / withdrawer authority set to pool withdraw authority.
    pub reserve_stake: Pubkey,

    ///   6. `[]` Pool token mint. Must have zero supply, owned by withdraw
    ///      authority.
    pub pool_mint: Pubkey,

    ///   7. `[]` Pool account to deposit the generated fee for manager.
    pub manager_pool_account: Pubkey,

    ///   8. `[]` Token program id
    pub token_program: Pubkey,

    ///   9. `[]` (Optional) Deposit authority that must sign all deposits.
    ///      Defaults to the program address generated using
    ///      `find_deposit_authority_program_address`, making deposits
    ///      permissionless.
    pub deposit_authority: Option<Pubkey>,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct InitializeData {
    /// Fee assessed as percentage of perceived rewards
    pub fee: Fee,

    /// Fee charged per withdrawal as percentage of withdrawal
    pub withdrawal_fee: Fee,

    /// Fee charged per deposit as percentage of deposit
    pub deposit_fee: Fee,

    /// Percentage [0-100] of `deposit_fee` that goes to referrer
    pub referral_fee: u8,

    /// Maximum expected number of validators
    pub max_validators: u32,
}

///   (Staker only) Removes validator from the pool, deactivating its stake
///
///   Only succeeds if the validator stake account has the minimum of
///   `max(crate::MINIMUM_ACTIVE_STAKE,
/// solana_program::stake::tools::get_minimum_delegation())`.   plus the
/// rent-exempt amount.
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct RemoveValidatorFromPoolAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[s]` Staker
    pub staker: Pubkey,

    ///   2. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw: Pubkey,

    ///   3. `[w]` Validator stake list storage account
    pub validator_list: Pubkey,

    ///   4. `[w]` Stake account to remove from the pool
    pub stake_account: Pubkey,

    ///   5. `[w]` Transient stake account, to deactivate if necessary
    pub transient_stake_account: Pubkey,

    ///   6. `[]` Sysvar clock
    pub clock: Pubkey,

    ///   7. `[]` Stake program id,
    pub stake_program: Pubkey,
}

///  (Manager only) Update fee
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct SetFeeAccounts {
    ///  0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///  1. `[s]` Manager
    pub manager: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct SetFeeData {
    /// Type of fee to update and value to update it to
    pub fee: FeeType,
}

///  (Manager only) Update SOL deposit, stake deposit, or SOL withdrawal
/// authority.
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct SetFundingAuthorityAccounts {
    ///  0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///  1. `[s]` Manager
    pub manager: Pubkey,

    ///  2. '[]` New authority pubkey or none
    pub auth: Option<Pubkey>,
}

#[derive(Debug, BorshDeserialize)]
pub struct SetFundingAuthorityData {
    pub arg: FundingType,
}

///  (Manager only) Update manager
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct SetManagerAccounts {
    ///  0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///  1. `[s]` Manager
    pub manager: Pubkey,

    ///  2. `[s]` New manager
    pub new_manager: Pubkey,

    ///  3. `[]` New manager fee account
    pub new_fee_receiver: Pubkey,
}

/// (Staker only) Set the preferred deposit or withdraw stake account for
/// the stake pool
///
/// In order to avoid users abusing the stake pool as a free conversion
/// between SOL staked on different validators, the staker can force all
/// deposits and/or withdraws to go to one chosen account, or unset that
/// account.
///
/// Fails if the validator is not part of the stake pool.
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct SetPreferredValidatorAccounts {
    /// 0. `[w]` Stake pool
    pub stake_pool_address: Pubkey,

    /// 1. `[s]` Stake pool staker
    pub staker: Pubkey,

    /// 2. `[]` Validator list
    pub validator_list_address: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct SetPreferredValidatorData {
    /// Affected operation (deposit or withdraw)
    pub validator_type: PreferredValidatorType,

    /// Validator vote account that deposits or withdraws must go through,
    /// unset with None
    pub validator_vote_address: Option<Pubkey>,
}

///  (Manager or staker only) Update staker
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct SetStakerAccounts {
    ///  0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///  1. `[s]` Manager or current staker
    pub set_staker_authority: Pubkey,

    ///  2. '[]` New staker pubkey
    pub new_staker: Pubkey,
}

///   Updates total pool balance based on balances in the reserve and
///   validator list
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct UpdateStakePoolBalanceAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[]` Stake pool withdraw authority
    pub withdraw_authority: Pubkey,

    ///   2. `[w]` Validator stake list storage account
    pub validator_list_storage: Pubkey,

    ///   3. `[]` Reserve stake account
    pub reserve_stake: Pubkey,

    ///   4. `[w]` Account to receive pool fee tokens
    pub manager_fee_account: Pubkey,

    ///   5. `[w]` Pool mint account
    pub stake_pool_mint: Pubkey,

    ///   6. `[]` Pool token program
    pub token_program: Pubkey,
}

/// Update token metadata for the stake-pool token in the
/// metaplex-token program
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct UpdateTokenMetadataAccounts {
    /// 0. `[]` Stake pool
    pub stake_pool: Pubkey,

    /// 1. `[s]` Manager
    pub manager: Pubkey,

    /// 2. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    /// 3. `[w]` Token metadata account
    pub token_metadata: Pubkey,

    /// 4. `[]` Metadata program id
    pub mpl_token_metadata: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct UpdateTokenMetadataData {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

///  Updates balances of validator and transient stake accounts in the pool
///
///  While going through the pairs of validator and transient stake
///  accounts, if the transient stake is inactive, it is merged into the
///  reserve stake account. If the transient stake is active and has
///  matching credits observed, it is merged into the canonical
///  validator stake account. In all other states, nothing is done, and
///  the balance is simply added to the canonical stake account balance.
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct UpdateValidatorListBalanceAccounts {
    ///  0. `[]` Stake pool
    pub stake_pool: Pubkey,

    ///  1. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///  2. `[w]` Validator stake list storage account
    pub validator_list_address: Pubkey,

    ///  3. `[w]` Reserve stake account
    pub reserve_stake: Pubkey,

    ///  4. `[]` Sysvar clock
    pub clock: Pubkey,

    ///  5. `[]` Sysvar stake history
    pub sysvar_stake_history: Pubkey,

    ///  6. `[]` Stake program
    pub stake_program: Pubkey,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct UpdateValidatorListBalanceData {
    /// Index to start updating on the validator list
    pub start_index: u32,

    /// If true, don't try merging transient stake accounts into the reserve
    /// or validator stake account.  Useful for testing or if a
    /// particular stake account is in a bad state, but we still
    /// want to update
    pub no_merge: bool,
}

///   Withdraw SOL directly from the pool's reserve account. Fails if the
///   reserve does not have enough SOL.
#[derive(Debug, BorshDeserialize)]
pub struct WithdrawSolAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///   2. `[s]` User transfer authority, for pool token account
    pub user_transfer_authority: Pubkey,

    ///   3. `[w]` User account to burn pool tokens
    pub pool_tokens_from: Pubkey,

    ///   4. `[w]` Reserve stake account, to withdraw SOL
    pub reserve_stake_account: Pubkey,

    ///   5. `[w]` Account receiving the lamports from the reserve, must be a
    ///      system account
    pub lamports_to: Pubkey,

    ///   6. `[w]` Account to receive pool fee tokens
    pub manager_fee_account: Pubkey,

    ///   7. `[w]` Pool token mint account
    pub pool_mint: Pubkey,

    ///   8. '[]' Clock sysvar
    pub clock: Pubkey,

    ///   9. '[]' Stake history sysvar
    pub sysvar_stake_history: Pubkey,

    ///  10. `[]` Stake program account
    pub stake_program: Pubkey,

    ///  11. `[]` Token program id
    pub token_program: Pubkey,

    ///  12. `[s]` (Optional) Stake pool sol withdraw authority
    pub sol_withdraw_authority: Option<Pubkey>,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct WithdrawSolData {
    pub arg: u64,
}

///   Withdraw the token from the pool at the current ratio.
///
///   Succeeds if the stake account has enough SOL to cover the desired
///   amount of pool tokens, and if the withdrawal keeps the total
///   staked amount above the minimum of rent-exempt amount plus `max(
///     crate::MINIMUM_ACTIVE_STAKE,
///     solana_program::stake::tools::get_minimum_delegation()
///   )`.
///
///   When allowing withdrawals, the order of priority goes:
///
///   * preferred withdraw validator stake account (if set)
///   * validator stake accounts
///   * transient stake accounts
///   * reserve stake account OR totally remove validator stake accounts
///
///   A user can freely withdraw from a validator stake account, and if they
///   are all at the minimum, then they can withdraw from transient stake
///   accounts, and if they are all at minimum, then they can withdraw from
///   the reserve or remove any validator from the pool.
///
///  User data: amount of pool tokens to withdraw
#[derive(Debug, BorshDeserialize)]
pub struct WithdrawStakeAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[w]` Validator stake list storage account
    pub validator_list_storage: Pubkey,

    ///   2. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw: Pubkey,

    ///   3. `[w]` Validator or reserve stake account to split
    pub stake_to_split: Pubkey,

    ///   4. `[w]` Uninitialized stake account to receive withdrawal
    pub stake_to_receive: Pubkey,

    ///   5. `[]` User account to set as a new withdraw authority
    pub user_stake_authority: Pubkey,

    ///   6. `[s]` User transfer authority, for pool token account
    pub user_transfer_authority: Pubkey,

    ///   7. `[w]` User account with pool tokens to burn from
    pub user_pool_token_account: Pubkey,

    ///   8. `[w]` Account to receive pool fee tokens
    pub manager_fee_account: Pubkey,

    ///   9. `[w]` Pool token mint account
    pub pool_mint: Pubkey,

    ///  10. `[]` Sysvar clock account (required)
    pub clock: Pubkey,

    ///  11. `[]` Pool token program id
    pub token_program: Pubkey,

    ///  12. `[]` Stake program id,
    pub stake_program: Pubkey,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct WithdrawStakeData {
    /// amount of pool tokens to withdraw
    pub arg: u64,
}

/// (Staker only) Increase stake on a validator again in an epoch.
///
/// Works regardless if the transient stake account exists.
///
/// Internally, this instruction splits reserve stake into an ephemeral
/// stake account, activates it, then merges or splits it into the
/// transient stake account delegated to the appropriate validator.
/// `UpdateValidatorListBalance` will do the work of merging once it's
/// ready.
///
/// The minimum amount to move is rent-exemption plus
/// `max(crate::MINIMUM_ACTIVE_STAKE,
/// solana_program::stake::tools::get_minimum_delegation())`.
#[derive(Debug, BorshDeserialize)]
pub struct IncreaseAdditionalValidatorStakeAccounts {
    ///  0. `[]` Stake pool
    pub stake_pool: Pubkey,

    ///  1. `[s]` Stake pool staker
    pub staker: Pubkey,

    ///  2. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///  3. `[w]` Validator list
    pub validator_list: Pubkey,

    ///  4. `[w]` Stake pool reserve stake
    pub reserve_stake: Pubkey,

    ///  5. `[w]` Uninitialized ephemeral stake account to receive stake
    pub ephemeral_stake: Pubkey,

    ///  6. `[w]` Transient stake account
    pub transient_stake: Pubkey,

    ///  7. `[]` Validator stake account
    pub validator_stake: Pubkey,

    ///  8. `[]` Validator vote account to delegate to
    pub validator: Pubkey,

    ///  9. '[]' Clock sysvar
    pub clock: Pubkey,

    /// 10. `[]` Stake History sysvar
    pub stake_history: Pubkey,

    /// 11. `[]` Stake Config sysvar
    pub stake_config: Pubkey,

    /// 12. `[]` System program
    pub system_program: Pubkey,

    /// 13. `[]` Stake program
    pub stake_program: Pubkey,
}

/// User data: amount of lamports to increase on the given validator.
///
/// The actual amount split into the transient stake account is:
/// `lamports + stake_rent_exemption`.
///
/// The rent-exemption of the stake account is withdrawn back to the
/// reserve after it is merged.
#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct IncreaseAdditionalValidatorStakeData {
    /// amount of lamports to increase on the given validator
    pub lamports: u64,

    /// seed used to create transient stake account
    pub transient_stake_seed: u64,

    /// seed used to create ephemeral account.
    pub ephemeral_stake_seed: u64,
}

/// (Staker only) Decrease active stake again from a validator, eventually
/// moving it to the reserve
///
/// Works regardless if the transient stake account already exists.
///
/// Internally, this instruction:
///  * withdraws rent-exempt reserve lamports from the reserve into the
///    ephemeral stake
///  * splits a validator stake account into an ephemeral stake account
///  * deactivates the ephemeral account
///  * merges or splits the ephemeral account into the transient stake
///    account delegated to the appropriate validator
///
///  The amount of lamports to move must be at least
/// `max(crate::MINIMUM_ACTIVE_STAKE,
/// solana_program::stake::tools::get_minimum_delegation())`.
#[derive(Debug, BorshDeserialize)]
pub struct DecreaseAdditionalValidatorStakeAccounts {
    ///  0. `[]` Stake pool
    pub stake_pool: Pubkey,

    ///  1. `[s]` Stake pool staker
    pub staker: Pubkey,

    ///  2. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///  3. `[w]` Validator list
    pub validator_list: Pubkey,

    ///  4. `[w]` Reserve stake account, to fund rent exempt reserve
    pub reserve_stake: Pubkey,

    ///  5. `[w]` Canonical stake account to split from
    pub validator_stake: Pubkey,

    ///  6. `[w]` Uninitialized ephemeral stake account to receive stake
    pub ephemeral_stake: Pubkey,

    ///  7. `[w]` Transient stake account
    pub transient_stake: Pubkey,

    ///  8. `[]` Clock sysvar
    pub clock: Pubkey,

    ///  9. '[]' Stake history sysvar
    pub stake_history: Pubkey,

    /// 10. `[]` System program
    pub system_program: Pubkey,

    /// 11. `[]` Stake program
    pub stake_program: Pubkey,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct DecreaseAdditionalValidatorStakeData {
    /// amount of lamports to increase on the given validator
    pub lamports: u64,

    /// seed used to create transient stake account
    pub transient_stake_seed: u64,

    /// seed used to create ephemeral account.
    pub ephemeral_stake_seed: u64,
}

/// (Staker only) Decrease active stake on a validator, eventually moving it
/// to the reserve
///
/// Internally, this instruction:
/// * withdraws enough lamports to make the transient account rent-exempt
/// * splits from a validator stake account into a transient stake account
/// * deactivates the transient stake account
///
/// In order to rebalance the pool without taking custody, the staker needs
/// a way of reducing the stake on a stake account. This instruction splits
/// some amount of stake, up to the total activated stake, from the
/// canonical validator stake account, into its "transient" stake
/// account.
///
/// The instruction only succeeds if the transient stake account does not
/// exist. The amount of lamports to move must be at least rent-exemption
/// plus `max(crate::MINIMUM_ACTIVE_STAKE,
/// solana_program::stake::tools::get_minimum_delegation())`.
#[derive(Debug, BorshDeserialize)]
pub struct DecreaseValidatorStakeWithReserveAccounts {
    ///  0. `[]` Stake pool
    pub stake_pool: Pubkey,

    ///  1. `[s]` Stake pool staker
    pub staker: Pubkey,

    ///  2. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///  3. `[w]` Validator list
    pub validator_list: Pubkey,

    ///  4. `[w]` Reserve stake account, to fund rent exempt reserve
    pub reserve_stake: Pubkey,

    ///  5. `[w]` Canonical stake account to split from
    pub validator_stake: Pubkey,

    ///  6. `[w]` Transient stake account to receive split
    pub transient_stake: Pubkey,

    ///  7. `[]` Clock sysvar
    pub clock: Pubkey,

    ///  8. '[]' Stake history sysvar
    pub stake_history: Pubkey,

    ///  9. `[]` System program
    pub system_program: Pubkey,

    /// 10. `[]` Stake program
    pub stake_program: Pubkey,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct DecreaseValidatorStakeWithReserveData {
    /// amount of lamports to split into the transient stake account
    pub lamports: u64,

    /// seed used to create transient stake account
    pub transient_stake_seed: u64,
}

// #[derive(Debug)]
// pub struct Redelegate {
//     pub stake_pool: Pubkey,
//     pub staker: Pubkey,
//     pub stake_pool_withdraw_authority: Pubkey,
//     pub validator_list: Pubkey,
//     pub reserve_stake: Pubkey,
//     pub source_validator_stake: Pubkey,
//     pub source_transient_stake: Pubkey,
//     pub ephemeral_stake: Pubkey,
//     pub destination_transient_stake: Pubkey,
//     pub destination_validator_stake: Pubkey,
//     pub validator: Pubkey,
//     pub clock: Pubkey,
//     pub stake_history: Pubkey,
//     pub stake_config: Pubkey,
//     pub system_program: Pubkey,
//     pub stake_program: Pubkey,
// }

///   Deposit some stake into the pool, with a specified slippage
///   constraint. The output is a "pool" token representing ownership
///   into the pool. Inputs are converted at the current ratio.
///
///   0. `[w]` Stake pool
///   1. `[w]` Validator stake list storage account
///   2. `[s]/[]` Stake pool deposit authority
///   3. `[]` Stake pool withdraw authority
///   4. `[w]` Stake account to join the pool (withdraw authority for the
///      stake account should be first set to the stake pool deposit
///      authority)
///   5. `[w]` Validator stake account for the stake account to be merged
///      with
///   6. `[w]` Reserve stake account, to withdraw rent exempt reserve
///   7. `[w]` User account to receive pool tokens
///   8. `[w]` Account to receive pool fee tokens
///   9. `[w]` Account to receive a portion of pool fee tokens as referral
///      fees
///   10. `[w]` Pool token mint account
///   11. '[]' Sysvar clock account
///   12. '[]' Sysvar stake history account
///   13. `[]` Pool token program id,
///   14. `[]` Stake program id,
#[derive(Debug, BorshDeserialize)]
pub struct DepositStakeWithSlippageAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[w]` Validator stake list storage account
    pub validator_list_storage: Pubkey,

    ///   2. `[s]/[]` Stake pool deposit authority
    pub stake_pool_deposit_authority: Pubkey,

    ///   3. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///   4. `[w]` Stake account to join the pool (withdraw authority for the
    ///      stake account should be first set to the stake pool deposit
    ///      authority)
    pub deposit_stake_address: Pubkey,

    ///   5. `[w]` Validator stake account for the stake account to be merged
    ///      with
    pub validator_stake_account: Pubkey,

    ///   6. `[w]` Reserve stake account, to withdraw rent exempt reserve
    pub reserve_stake_account: Pubkey,

    ///   7. `[w]` User account to receive pool tokens
    pub pool_tokens_to: Pubkey,

    ///   8. `[w]` Account to receive pool fee tokens
    pub manager_fee_account: Pubkey,

    ///   9. `[w]` Account to receive a portion of pool fee tokens as referral
    ///      fees
    pub referrer_pool_tokens_account: Pubkey,

    ///   10. `[w]` Pool token mint account
    pub pool_mint: Pubkey,

    ///   11. '[]' Sysvar clock account
    pub clock: Pubkey,

    ///   12. '[]' Sysvar stake history account
    pub sysvar_stake_history: Pubkey,

    ///   13. `[]` Pool token program id,
    pub token_program: Pubkey,

    ///   14. `[]` Stake program id,
    pub stake_program: Pubkey,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct DepositStakeWithSlippageData {
    /// Minimum amount of pool tokens that must be received
    pub minimum_pool_tokens_out: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct WithdrawStakeWithSlippageAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[w]` Validator stake list storage account
    pub validator_list_storage: Pubkey,

    ///   2. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw: Pubkey,

    ///   3. `[w]` Validator or reserve stake account to split
    pub stake_to_split: Pubkey,

    ///   4. `[w]` Uninitialized stake account to receive withdrawal
    pub stake_to_receive: Pubkey,

    ///   5. `[]` User account to set as a new withdraw authority
    pub user_stake_authority: Pubkey,

    ///   6. `[s]` User transfer authority, for pool token account
    pub user_transfer_authority: Pubkey,

    ///   7. `[w]` User account with pool tokens to burn from
    pub user_pool_token_account: Pubkey,

    ///   8. `[w]` Account to receive pool fee tokens
    pub manager_fee_account: Pubkey,

    ///   9. `[w]` Pool token mint account
    pub pool_mint: Pubkey,

    ///  10. `[]` Sysvar clock account (required)
    pub clock: Pubkey,

    ///  11. `[]` Pool token program id
    pub token_program: Pubkey,

    ///  12. `[]` Stake program id,
    pub stake_program: Pubkey,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct WithdrawStakeWithSlippageData {
    /// Pool tokens to burn in exchange for lamports
    pub pool_tokens_in: u64,

    /// Minimum amount of lamports that must be received
    pub minimum_lamports_out: u64,
}

///   Deposit SOL directly into the pool's reserve account, with a
///   specified slippage constraint. The output is a "pool" token
///   representing ownership into the pool. Inputs are converted at the
///   current ratio.
#[derive(Debug, BorshDeserialize)]
pub struct DepositSolWithSlippageAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///   2. `[w]` Reserve stake account, to deposit SOL
    pub reserve_stake_account: Pubkey,

    ///   3. `[s]` Account providing the lamports to be deposited into the pool
    pub lamports_from: Pubkey,

    ///   4. `[w]` User account to receive pool tokens
    pub pool_tokens_to: Pubkey,

    ///   5. `[w]` Account to receive fee tokens
    pub manager_fee_account: Pubkey,

    ///   6. `[w]` Account to receive a portion of fee as referral fees
    pub referrer_pool_tokens_account: Pubkey,

    ///   7. `[w]` Pool token mint account
    pub pool_mint: Pubkey,

    ///   8. `[]` System program account
    pub system_program: Pubkey,

    ///   9. `[]` Token program id
    pub token_program: Pubkey,

    ///  10. `[s]` (Optional) Stake pool sol deposit authority.
    pub deposit_authority: Option<Pubkey>,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct DepositSolWithSlippageData {
    /// Amount of lamports to deposit into the reserve
    pub lamports_in: u64,

    /// Minimum amount of pool tokens that must be received
    pub minimum_pool_tokens_out: u64,
}

///   Withdraw SOL directly from the pool's reserve account. Fails if the
///   reserve does not have enough SOL or if the slippage constraint is not
///   met.
#[derive(Debug, BorshDeserialize)]
pub struct WithdrawSolWithSlippageAccounts {
    ///   0. `[w]` Stake pool
    pub stake_pool: Pubkey,

    ///   1. `[]` Stake pool withdraw authority
    pub stake_pool_withdraw_authority: Pubkey,

    ///   2. `[s]` User transfer authority, for pool token account
    pub user_transfer_authority: Pubkey,

    ///   3. `[w]` User account to burn pool tokens
    pub pool_tokens_from: Pubkey,

    ///   4. `[w]` Reserve stake account, to withdraw SOL
    pub reserve_stake_account: Pubkey,

    ///   5. `[w]` Account receiving the lamports from the reserve, must be a
    ///      system account
    pub lamports_to: Pubkey,

    ///   6. `[w]` Account to receive pool fee tokens
    pub manager_fee_account: Pubkey,

    ///   7. `[w]` Pool token mint account
    pub pool_mint: Pubkey,

    ///   8. '[]' Clock sysvar
    pub clock: Pubkey,

    ///   9. '[]' Stake history sysvar
    pub sysvar_stake_history: Pubkey,

    ///  10. `[]` Stake program account
    pub stake_program: Pubkey,

    ///  11. `[]` Token program id
    pub token_program: Pubkey,

    ///  12. `[s]` (Optional) Stake pool sol withdraw authority
    pub sol_withdraw_authority: Option<Pubkey>,
}

#[derive(Clone, Copy, Debug, BorshDeserialize)]
pub struct WithdrawSolWithSlippageData {
    /// Pool tokens to burn in exchange for lamports
    pub pool_tokens_in: u64,

    /// Minimum amount of lamports that must be received
    pub minimum_lamports_out: u64,
}

#[derive(Debug, BorshDeserialize)]
pub enum StakePoolProgramIx {
    Initialize(InitializeAccounts, InitializeData),
    AddValidatorToPool(AddValidatorToPoolAccounts, AddValidatorToPoolData),
    RemoveValidatorFromPool(RemoveValidatorFromPoolAccounts),
    DecreaseValidatorStake(DecreaseValidatorStakeAccounts, DecreaseValidatorStakeData),
    IncreaseValidatorStake(IncreaseValidatorStakeAccounts, IncreaseValidatorStakeData),
    SetPreferredValidator(SetPreferredValidatorAccounts, SetPreferredValidatorData),
    UpdateValidatorListBalance(
        UpdateValidatorListBalanceAccounts,
        UpdateValidatorListBalanceData,
    ),
    UpdateStakePoolBalance(UpdateStakePoolBalanceAccounts),
    CleanupRemovedValidatorEntries(CleanupRemovedValidatorEntriesAccounts),
    DepositStake(DepositStakeAccounts),
    WithdrawStake(WithdrawStakeAccounts, WithdrawStakeData),
    SetManager(SetManagerAccounts),
    SetFee(SetFeeAccounts, SetFeeData),
    SetStaker(SetStakerAccounts),
    DepositSol(DepositSolAccounts, DepositSolData),
    SetFundingAuthority(SetFundingAuthorityAccounts, SetFundingAuthorityData),
    WithdrawSol(WithdrawSolAccounts, WithdrawSolData),
    CreateTokenMetadata(CreateTokenMetadataAccounts, CreateTokenMetadataData),
    UpdateTokenMetadata(UpdateTokenMetadataAccounts, UpdateTokenMetadataData),
    IncreaseAdditionalValidatorStake(
        IncreaseAdditionalValidatorStakeAccounts,
        IncreaseAdditionalValidatorStakeData,
    ),
    DecreaseAdditionalValidatorStake(
        DecreaseAdditionalValidatorStakeAccounts,
        DecreaseAdditionalValidatorStakeData,
    ),
    DecreaseValidatorStakeWithReserve(
        DecreaseValidatorStakeWithReserveAccounts,
        DecreaseValidatorStakeWithReserveData,
    ),
    DepositStakeWithSlippage(
        DepositStakeWithSlippageAccounts,
        DepositStakeWithSlippageData,
    ),
    WithdrawStakeWithSlippage(
        WithdrawStakeWithSlippageAccounts,
        WithdrawStakeWithSlippageData,
    ),
    DepositSolWithSlippage(DepositSolWithSlippageAccounts, DepositSolWithSlippageData),
    WithdrawSolWithSlippage(WithdrawSolWithSlippageAccounts, WithdrawSolWithSlippageData),
}
