use std::{collections::HashMap, fs::File, path::PathBuf, sync::mpsc};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use solana_accounts_db::accounts_file::AccountsFile;
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

pub struct AccountFile(PathBuf, usize);

pub struct SolanaSnapshot {
    pub accounts: Vec<AccountFile>,
    pub version: String,
    pub slot: u64,
    _temp_dir: TempDir,
}

impl SolanaSnapshot {
    pub fn unpack_compressed<P: Into<PathBuf>>(path: P) -> Result<Self, VixenError> {
        let path_buf: PathBuf = path.into();

        let temp_dir = tempdir()?;

        let file = File::open(path_buf)?;

        let decoder = Decoder::new(file)?;

        let mut archive = Archive::new(decoder);
        archive.unpack(temp_dir.path())?;

        let version_path = temp_dir.path().join("version");
        let version = std::fs::read_to_string(version_path)?.trim().to_string();

        let snapshots_dir = temp_dir.path().join("snapshots");
        let slot = std::fs::read_dir(snapshots_dir)?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
            .filter_map(|name| name.parse::<u64>().ok())
            .max()
            .ok_or_else(|| VixenError::ConfigError)?;

        let accounts_dir = temp_dir.path().join("accounts");
        let accounts = std::fs::read_dir(accounts_dir)?
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.path();
                let size = std::fs::metadata(&path)?.len() as usize;
                Ok(AccountFile(path, size))
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

        let mut owner_to_filters: HashMap<yellowstone_vixen_core::Pubkey, Vec<String>> =
            HashMap::new();
        let mut all_owners = Vec::new();

        for (key, parser_filter) in filters.parsers_filters.iter() {
            if let Some(account_filter) = parser_filter.account.as_ref() {
                for owner in &account_filter.owners {
                    owner_to_filters
                        .entry(*owner)
                        .or_default()
                        .push(key.clone());
                    if !all_owners.contains(owner) {
                        all_owners.push(*owner);
                    }
                }
            }
        }

        let solana_snapshot = SolanaSnapshot::unpack_compressed(config.path.clone())?;

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

        let sender_handle = tokio::spawn(async move {
            while let Ok(event) = sync_rx.recv() {
                match event {
                    Event::AccountUpdate(account, filter_keys) => {
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

        for AccountFile(path, current_len) in solana_snapshot.accounts {
            let sync_tx = sync_tx.clone();
            let slot = solana_snapshot.slot;
            let all_owners = all_owners.clone();
            let owner_to_filters = owner_to_filters.clone();

            account_file_workers.spawn(async move {
                let (accounts, _usize) = AccountsFile::new_from_file(
                    path,
                    current_len,
                    solana_accounts_db::accounts_file::StorageAccess::default(),
                )
                .expect("Unpack account file");

                accounts.scan_accounts(|_size, account| {
                    let account_owner =
                        yellowstone_vixen_core::Pubkey::try_from(account.owner.as_ref())
                            .expect("Owner address is Pubkey");

                    if all_owners.contains(&account_owner) {
                        // Get the specific filter keys for this owner
                        let filter_keys = owner_to_filters
                            .get(&account_owner)
                            .cloned()
                            .unwrap_or_default();

                        let _ = sync_tx.send(Event::AccountUpdate(
                            SubscribeUpdateAccount {
                                account: Some(SubscribeUpdateAccountInfo {
                                    pubkey: account.pubkey().to_bytes().to_vec(),
                                    lamports: account.lamports,
                                    owner: account.owner.to_bytes().to_vec(),
                                    executable: account.executable,
                                    rent_epoch: account.rent_epoch,
                                    data: account.data.to_vec(),
                                    write_version: 0,
                                    txn_signature: None,
                                }),
                                slot,
                                is_startup: true,
                            },
                            filter_keys,
                        ));
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
    AccountUpdate(SubscribeUpdateAccount, Vec<String>),
    SnapshotFinished,
}