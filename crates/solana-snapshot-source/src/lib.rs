use std::{collections::HashMap, fs::File, path::PathBuf, sync::Arc};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use solana_accounts_db::accounts_file::AccountsFile;
use tar::Archive;
use tempfile::{tempdir, TempDir};
use tokio::{sync::mpsc, task::JoinSet};
use yellowstone_grpc_proto::{
    geyser::{
        subscribe_update::UpdateOneof, SlotStatus, SubscribeUpdate, SubscribeUpdateAccount,
        SubscribeUpdateAccountInfo, SubscribeUpdateSlot,
    },
    tonic::Status,
};
use yellowstone_vixen::{sources::SourceTrait, Error as VixenError};
use yellowstone_vixen_core::{Filters, Pubkey};
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
    pub channel_size: usize,
}

/// A `Source` implementation for the Solana Snapshot API.
#[derive(Debug)]
pub struct SolanaSnapshotSource {
    filters: Filters,
    config: SolanaSnapshotConfig,
}

#[derive(Clone)]
struct FilterOwnerKeyLookup(Arc<HashMap<Pubkey, Vec<String>>>);

impl FilterOwnerKeyLookup {
    fn new(filters: &Filters) -> Self {
        let mut lookup: HashMap<Pubkey, Vec<String>> = HashMap::new();

        for (key, parser_filter) in filters.parsers_filters.iter() {
            if let Some(account_filter) = parser_filter.account.as_ref() {
                for owner in &account_filter.owners {
                    lookup.entry(*owner).or_default().push(key.clone());
                }
            }
        }

        Self(Arc::new(lookup))
    }

    fn lookup_by_owner(&self, owner: &Pubkey) -> Option<Vec<String>> { self.0.get(owner).cloned() }
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

        let filter_owner_key_lookup = FilterOwnerKeyLookup::new(&filters);

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
        let channel_size = config.channel_size;
        let (sync_tx, mut sync_rx) = mpsc::channel::<Event>(channel_size);
        let mut account_file_workers = JoinSet::new();

        let sender_handle = tokio::spawn(async move {
            while let Some(event) = sync_rx.recv().await {
                match event {
                    Event::AccountUpdate {
                        account_update,
                        filters,
                    } => {
                        if let Err(err) = tx
                            .send(Ok(SubscribeUpdate {
                                filters,
                                created_at: None,
                                update_oneof: Some(UpdateOneof::Account(account_update)),
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
            let worker_sync_tx = sync_tx.clone();
            let slot = solana_snapshot.slot;
            let filter_owner_key_lookup = filter_owner_key_lookup.clone();

            account_file_workers.spawn(async move {
                let blocking_task =
                    tokio::task::spawn_blocking(move || -> Result<(), VixenError> {
                        let (accounts, _usize) = AccountsFile::new_from_file(
                            path,
                            current_len,
                            solana_accounts_db::accounts_file::StorageAccess::default(),
                        )
                        .expect("Unpack account file");

                        accounts.scan_accounts(|_size, account| {
                            let account_owner = Pubkey::try_from(account.owner.as_ref())
                                .expect("Owner address is Pubkey");

                            if let Some(filter_keys) =
                                filter_owner_key_lookup.lookup_by_owner(&account_owner)
                            {
                                worker_sync_tx
                                    .blocking_send(Event::AccountUpdate {
                                        account_update: SubscribeUpdateAccount {
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
                                        filters: filter_keys,
                                    })
                                    .expect("Channel closed while sending account update");
                            }
                        });

                        Ok(())
                    });

                blocking_task.await.map_err(|err| {
                    VixenError::Other(format!("Snapshot worker panicked: {err:?}").into())
                })?
            });

            if account_file_workers.len() >= max_workers {
                if let Some(join_res) = account_file_workers.join_next().await {
                    join_res.map_err(|err| {
                        VixenError::Other(format!("Snapshot worker panicked: {err:?}").into())
                    })??;
                }
            }
        }

        while let Some(join_res) = account_file_workers.join_next().await {
            join_res
                .map_err(|err| VixenError::Other(format!("Snapshot worker panicked: {err:?}").into()))??;
        }

        sync_tx
            .send(Event::SnapshotFinished)
            .await
            .map_err(|_| VixenError::ServerHangup)?;
        drop(sync_tx);
        let _ = sender_handle.await;

        Ok(())
    }
}

enum Event {
    AccountUpdate {
        account_update: SubscribeUpdateAccount,
        filters: Vec<String>,
    },
    SnapshotFinished,
}
