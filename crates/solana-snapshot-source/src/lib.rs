#![allow(deprecated)]

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    path::PathBuf,
    sync::mpsc,
};

use async_trait::async_trait;
use bincode::Options;
use serde::{Deserialize, Serialize};
use solana_accounts_db::{
    accounts_file::AccountsFile,
    accounts_hash::{SerdeAccountsDeltaHash, SerdeAccountsHash},
    ancestors::AncestorsForSerialization,
    blockhash_queue::BlockhashQueue,
};
use solana_hash::Hash;
use solana_program::clock::{Epoch, Slot, UnixTimestamp};
use solana_pubkey::Pubkey;
use solana_runtime::bank::BankHashStats;
use solana_serde::default_on_eof;
use tar::Archive;
use tempfile::{tempdir, TempDir};
use tokio::task::JoinSet;
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof, SlotStatus, SubscribeUpdate, SubscribeUpdateAccount,
        SubscribeUpdateAccountInfo, SubscribeUpdateSlot,
    },
    tonic::Status,
};
use yellowstone_vixen::{sources::SourceTrait, Error as VixenError};
use yellowstone_vixen_core::Filters;
use zstd::Decoder;

const MAX_STREAM_SIZE: u64 = 32 * 1024 * 1024 * 1024;

pub struct AccountFile {
    pub path: PathBuf,
    pub size: usize,
    pub slot: u64,
    pub id: u64,
}

pub struct SolanaSnapshot {
    pub accounts: Vec<AccountFile>,
    pub version: String,
    pub slot: u64,
    _temp_dir: TempDir,
}

impl SolanaSnapshot {
    pub fn unpack_compressed<P: Into<PathBuf>>(path: P, slot: u64) -> Result<Self, VixenError> {
        let path_buf: PathBuf = path.into();

        let temp_dir = tempdir()?;

        let file = File::open(path_buf)?;

        let decoder = Decoder::new(file)?;

        let mut archive = Archive::new(decoder);
        archive.unpack(temp_dir.path())?;

        let version_path = temp_dir.path().join("version");
        let version = std::fs::read_to_string(version_path)?.trim().to_string();

        // Deserializing the snapshot metadata file
        let snapshots_dir = temp_dir.path().join("snapshots");
        let snapshot_file_name = format!("{}/{}", slot, slot);
        let snapshot_file = File::open(snapshots_dir.join(snapshot_file_name))
            .expect("Snapshot metadatafile not found");

        let mut snapshot_stream = BufReader::new(snapshot_file);

        let bank_fields: DeserializableVersionedBank = bincode::options()
            .with_limit(MAX_STREAM_SIZE)
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .deserialize_from(&mut snapshot_stream)
            .unwrap();

        let accounts_db_fields: AccountsDbFields<SerializableAccountStorageEntry> =
            bincode::options()
                .with_limit(MAX_STREAM_SIZE)
                .with_fixint_encoding()
                .allow_trailing_bytes()
                .deserialize_from(&mut snapshot_stream)
                .unwrap();

        let AccountsDbFields(accounts_metadata, _, accountsdb_fields_slot, ..) = accounts_db_fields;

        assert_eq!(slot, accountsdb_fields_slot);
        assert_eq!(slot, bank_fields.slot);

        // Deserializing the accounts directory files
        let accounts_dir = temp_dir.path().join("accounts");
        let accounts = std::fs::read_dir(accounts_dir)?
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.path();
                let file_size = std::fs::metadata(&path)?.len() as usize;
                let file_name = entry.file_name().to_string_lossy().to_string();

                let (slot_str, id_str) = file_name
                    .split_once('.')
                    .unwrap_or_else(|| panic!("Invalid file name: {}", file_name));
                let slot = slot_str
                    .parse::<u64>()
                    .unwrap_or_else(|_| panic!("Invalid slot: {}", slot_str));
                let id = id_str
                    .parse::<u64>()
                    .unwrap_or_else(|_| panic!("Invalid id: {}", id_str));

                let accounts_metadata = accounts_metadata
                    .get(&slot)
                    .unwrap_or_else(|| panic!("accounts_metadata not found for slot: {}", slot));

                let mut size = None;
                for account in accounts_metadata {
                    if account.id as u64 == id {
                        size = Some(account.accounts_current_len);
                        break;
                    }
                }
                let size = size.unwrap_or_else(|| panic!("size not found for id: {}", id));

                if size != file_size {
                    tracing::warn!("size mismatch for id: {} and slot: {}", id, slot);
                }

                Ok(AccountFile {
                    path,
                    size,
                    slot,
                    id,
                })
            })
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        Ok(Self {
            accounts,
            version,
            slot,
            _temp_dir: temp_dir,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, clap::Args)]
pub struct SolanaSnapshotConfig {
    pub path: PathBuf,
    pub slot: u64,
    pub max_workers: usize,
}

/// A `Source` implementation for the Solana Snapshot API.
#[derive(Debug)]
pub struct SolanaSnapshotSource {
    filters: Filters,
    config: SolanaSnapshotConfig,
}

#[async_trait]
impl SourceTrait for SolanaSnapshotSource {
    type Config = SolanaSnapshotConfig;

    fn new(config: Self::Config, filters: Filters) -> Self { Self { config, filters } }

    async fn connect(
        &self,
        tx: tokio::sync::mpsc::Sender<Result<SubscribeUpdate, Status>>,
    ) -> Result<(), VixenError> {
        let filters = self.filters.clone();
        let config = self.config.clone();

        let (owners, filter_keys): (Vec<_>, Vec<_>) = filters
            .parsers_filters
            .iter()
            .filter_map(|(key, parser_filter)| {
                parser_filter
                    .account
                    .as_ref()
                    .map(|accounts| (accounts.owners.clone(), key.clone()))
            })
            .fold(
                (Vec::new(), Vec::new()),
                |(mut owners, mut keys), (account_owners, key)| {
                    owners.extend(account_owners);
                    keys.push(key);
                    (owners, keys)
                },
            );

        let solana_snapshot = SolanaSnapshot::unpack_compressed(config.path.clone(), config.slot)?;

        let filters = filters
            .parsers_filters
            .iter()
            .filter_map(|(key, parser_filter)| parser_filter.slot.map(|_| key.to_string()))
            .collect::<Vec<String>>();

        tx.send(Ok(SubscribeUpdate {
            filters,
            created_at: None,
            update_oneof: Some(UpdateOneof::Slot(SubscribeUpdateSlot {
                slot: solana_snapshot.slot,
                parent: None,
                status: SlotStatus::SlotFinalized.into(),
                dead_error: None,
            })),
        }))
        .await
        .map_err(|_| VixenError::ServerHangup)?;

        let max_workers = config.max_workers;
        let (sync_tx, sync_rx) = mpsc::channel::<Event>();
        let mut account_file_workers = JoinSet::new();

        let filter_keys = filter_keys.clone();
        let sender_handle = tokio::spawn(async move {
            while let Ok(event) = sync_rx.recv() {
                match event {
                    Event::AccountUpdate(account) => {
                        let filter_keys = filter_keys.clone();

                        if let Err(err) = tx
                            .send(Ok(SubscribeUpdate {
                                filters: filter_keys,
                                created_at: None,
                                update_oneof: Some(UpdateOneof::Account(account)),
                            }))
                            .await
                        {
                            tracing::error!("Error snapshot sending account update: {:?}", err);
                        }
                    },
                    Event::SnapshotFinished => break,
                }
            }
        });

        for AccountFile {
            path,
            size: current_len,
            slot: account_file_slot,
            id: write_version,
        } in solana_snapshot.accounts
        {
            let sync_tx = sync_tx.clone();
            let owners = owners.clone();

            account_file_workers.spawn(async move {
                let accounts = AccountsFile::new_for_startup(
                    path,
                    current_len,
                    solana_accounts_db::accounts_file::StorageAccess::default(),
                )
                .expect("Unpack account file");

                accounts.scan_accounts(|_size, account| {
                    let account_owner =
                        yellowstone_vixen_core::Pubkey::try_from(account.owner.as_ref())
                            .expect("Owner address is Pubkey");

                    if owners.contains(&account_owner) {
                        let _ = sync_tx.send(Event::AccountUpdate(SubscribeUpdateAccount {
                            account: Some(SubscribeUpdateAccountInfo {
                                pubkey: account.pubkey().to_bytes().to_vec(),
                                lamports: account.lamports,
                                owner: account.owner.to_bytes().to_vec(),
                                executable: account.executable,
                                rent_epoch: account.rent_epoch,
                                data: account.data.to_vec(),
                                write_version,
                                txn_signature: None,
                            }),
                            slot: account_file_slot,
                            is_startup: true,
                        }));
                    }
                });
            });

            if account_file_workers.len() >= max_workers {
                account_file_workers.join_next().await;
            }
        }

        while account_file_workers.join_next().await.is_some() {}

        let _ = sync_tx.send(Event::SnapshotFinished);
        let _ = sender_handle.await;

        Ok(())
    }
}

enum Event {
    AccountUpdate(SubscribeUpdateAccount),
    SnapshotFinished,
}

// Serializable version of AccountStorageEntry for snapshot format
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize)]
pub struct SerializableAccountStorageEntry {
    id: usize, // SerializedAccountsFileId
    accounts_current_len: usize,
}

#[derive(Clone, Deserialize, Debug)]
#[allow(dead_code)]
struct DeserializableVersionedBank {
    blockhash_queue: BlockhashQueue,
    ancestors: AncestorsForSerialization,
    hash: Hash,
    parent_hash: Hash,
    parent_slot: Slot,
    hard_forks: solana_hard_forks::HardForks,
    transaction_count: u64,
    tick_height: u64,
    signature_count: u64,
    capitalization: u64,
    max_tick_height: u64,
    hashes_per_tick: Option<u64>,
    ticks_per_slot: u64,
    ns_per_slot: u128,
    genesis_creation_time: UnixTimestamp,
    slots_per_year: f64,
    accounts_data_len: u64,
    slot: Slot,
    epoch: Epoch,
    block_height: u64,
    collector_id: Pubkey,
    collector_fees: u64,
    _fee_calculator: solana_fee_calculator::FeeCalculator,
    fee_rate_governor: solana_fee_calculator::FeeRateGovernor,
    collected_rent: u64,
    rent_collector: solana_rent_collector::RentCollector,
    epoch_schedule: solana_epoch_schedule::EpochSchedule,
    inflation: solana_inflation::Inflation,
    stakes: solana_runtime::stakes::Stakes<solana_stake_interface::state::Delegation>,
    #[allow(dead_code)]
    unused_accounts: UnusedAccounts,
    epoch_stakes: HashMap<Epoch, solana_runtime::epoch_stakes::EpochStakes>,
    is_delta: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Deserialize)]
struct UnusedAccounts {
    unused1: HashSet<Pubkey>,
    unused2: HashSet<Pubkey>,
    unused3: HashMap<Pubkey, u64>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct AccountsDbFields<T>(
    /// Careful! This contains entries for all historical slots with accounts, not only the slots
    ///  for the snapshot (even if it's incremental)
    pub HashMap<Slot, Vec<T>>,
    pub u64, // obsolete, formerly write_version
    pub Slot,
    #[allow(private_interfaces)] pub BankHashInfo,
    /// all slots that were roots within the last epoch
    #[serde(deserialize_with = "default_on_eof")]
    pub Vec<Slot>,
    /// slots that were roots within the last epoch for which we care about the hash value
    #[serde(deserialize_with = "default_on_eof")]
    pub Vec<(Slot, Hash)>,
);

#[derive(Clone, Default, Debug, Deserialize, PartialEq, Eq)]
struct BankHashInfo {
    accounts_delta_hash: SerdeAccountsDeltaHash,
    accounts_hash: SerdeAccountsHash,
    stats: BankHashStats,
}
