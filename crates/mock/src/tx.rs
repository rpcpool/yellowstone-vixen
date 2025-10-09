use std::{fs, path::Path, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_client::rpc_request::RpcRequest;
use solana_sdk::{bs58, signature::Signature};
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedConfirmedTransactionWithStatusMeta,
    EncodedTransaction, EncodedTransactionWithStatusMeta, UiInstruction, UiMessage,
    UiTransactionTokenBalance,
};
use yellowstone_grpc_proto::{
    geyser::{SubscribeUpdateTransaction, SubscribeUpdateTransactionInfo},
    prelude::MessageHeader,
    solana::storage::confirmed_block::{
        CompiledInstruction, InnerInstruction, InnerInstructions, Message as SolanaMessage,
        TokenBalance, Transaction, TransactionStatusMeta,
    },
};
use yellowstone_vixen_core::{instruction::InstructionUpdate, TransactionUpdate};

use crate::{
    decode_bs58_to_bytes, get_rpc_client, maybe_create_fixture_dir, FixtureData,
    SerializablePubkey, FIXTURES_PATH,
};

/// Convert `UiTransactionTokenBalance` to `TokenBalance` protobuf
fn convert_ui_token_balance(ui_balance: &UiTransactionTokenBalance) -> TokenBalance {
    use yellowstone_grpc_proto::solana::storage::confirmed_block::UiTokenAmount;

    TokenBalance {
        account_index: u32::from(ui_balance.account_index),
        mint: ui_balance.mint.clone(),
        ui_token_amount: Some(UiTokenAmount {
            ui_amount: ui_balance.ui_token_amount.ui_amount.unwrap_or_default(),
            decimals: u32::from(ui_balance.ui_token_amount.decimals),
            amount: ui_balance.ui_token_amount.amount.clone(),
            ui_amount_string: ui_balance.ui_token_amount.ui_amount_string.clone(),
        }),
        owner: ui_balance
            .owner
            .as_ref()
            .map(std::string::ToString::to_string)
            .unwrap_or_default(),
        program_id: ui_balance
            .program_id
            .as_ref()
            .map(std::string::ToString::to_string)
            .unwrap_or_default(),
    }
}

/// Convert a vector of `UiTransactionTokenBalance` to `TokenBalance` protobuf
fn convert_token_balances(ui_balances: &[UiTransactionTokenBalance]) -> Vec<TokenBalance> {
    ui_balances.iter().map(convert_ui_token_balance).collect()
}

#[allow(clippy::too_many_lines)]
fn convert_to_transaction_update(
    value: EncodedConfirmedTransactionWithStatusMeta,
) -> Result<TransactionUpdate, Box<dyn std::error::Error>> {
    let EncodedConfirmedTransactionWithStatusMeta {
        transaction,
        slot,
        block_time: _,
    } = value;
    let EncodedTransactionWithStatusMeta {
        transaction,
        meta,
        version: _,
    } = transaction;

    let mut account_keys: Vec<Vec<u8>> = Vec::new();
    let mut instructions: Vec<CompiledInstruction> = Vec::new();
    let mut inner_instructions: Vec<InnerInstructions> = Vec::new();
    let mut signatures: Vec<Vec<u8>> = Vec::new();
    let message_header: Option<MessageHeader>;
    let recent_blockhash: Vec<u8>;

    if let EncodedTransaction::Json(tx_data) = transaction {
        if let UiMessage::Raw(raw_message) = tx_data.message {
            // Extract and convert message header
            message_header = Some(MessageHeader {
                num_required_signatures: u32::from(raw_message.header.num_required_signatures),
                num_readonly_signed_accounts: u32::from(
                    raw_message.header.num_readonly_signed_accounts,
                ),
                num_readonly_unsigned_accounts: u32::from(
                    raw_message.header.num_readonly_unsigned_accounts,
                ),
            });

            // Extract recent blockhash
            recent_blockhash = decode_bs58_to_bytes(&raw_message.recent_blockhash)?;

            // Convert account keys from strings to bytes
            for key_str in raw_message.account_keys {
                let key_bytes = decode_bs58_to_bytes(&key_str)?;
                account_keys.push(key_bytes);
            }

            // Convert instructions
            for ui_instruction in raw_message.instructions {
                instructions.push(CompiledInstruction {
                    program_id_index: u32::from(ui_instruction.program_id_index),
                    accounts: ui_instruction.accounts,
                    data: decode_bs58_to_bytes(&ui_instruction.data)?,
                });
            }

            // Convert signatures
            for sig_str in tx_data.signatures {
                let sig_bytes = decode_bs58_to_bytes(&sig_str)?;
                signatures.push(sig_bytes);
            }
        } else {
            return Err("Invalid transaction encoding".into());
        }
    } else {
        return Err("Invalid transaction encoding".into());
    }

    let mut loaded_writable_addresses: Vec<Vec<u8>> = Vec::new();
    let mut loaded_readonly_addresses: Vec<Vec<u8>> = Vec::new();

    if let Some(meta) = &meta {
        // Convert inner instructions
        if let OptionSerializer::Some(inner_ixs) = &meta.inner_instructions {
            for inner_ix in inner_ixs {
                let mut converted_instructions: Vec<InnerInstruction> = Vec::new();
                for ui_instruction in &inner_ix.instructions {
                    if let UiInstruction::Compiled(compiled_ix) = ui_instruction {
                        converted_instructions.push(InnerInstruction {
                            program_id_index: u32::from(compiled_ix.program_id_index),
                            accounts: compiled_ix.accounts.clone(),
                            data: decode_bs58_to_bytes(&compiled_ix.data)?,
                            stack_height: compiled_ix.stack_height,
                        });
                    }
                }
                inner_instructions.push(InnerInstructions {
                    index: u32::from(inner_ix.index),
                    instructions: converted_instructions,
                });
            }
        }

        // Convert loaded addresses
        if let OptionSerializer::Some(loaded) = &meta.loaded_addresses {
            for addr_str in &loaded.writable {
                let addr_bytes = decode_bs58_to_bytes(addr_str)?;
                loaded_writable_addresses.push(addr_bytes);
            }
            for addr_str in &loaded.readonly {
                let addr_bytes = decode_bs58_to_bytes(addr_str)?;
                loaded_readonly_addresses.push(addr_bytes);
            }
        }
    }

    let transaction_info = SubscribeUpdateTransactionInfo {
        signature: signatures.first().cloned().unwrap_or_default(),
        is_vote: false,
        transaction: Some(Transaction {
            signatures,
            message: Some(SolanaMessage {
                header: message_header,
                account_keys,
                recent_blockhash,
                instructions,
                versioned: false,
                address_table_lookups: vec![],
            }),
        }),
        meta: meta.map(|m| {
            // Convert token balances properly (this is the FIX!)
            let pre_token_balances = match &m.pre_token_balances {
                OptionSerializer::Some(balances) => convert_token_balances(balances),
                OptionSerializer::None | OptionSerializer::Skip => vec![],
            };

            let post_token_balances = match &m.post_token_balances {
                OptionSerializer::Some(balances) => convert_token_balances(balances),
                OptionSerializer::None | OptionSerializer::Skip => vec![],
            };

            TransactionStatusMeta {
                err: m.err.as_ref().map(|e| {
                    use yellowstone_grpc_proto::solana::storage::confirmed_block::TransactionError;
                    TransactionError {
                        err: bincode::serialize(e).unwrap_or_default(),
                    }
                }),
                fee: m.fee,
                pre_balances: m.pre_balances,
                post_balances: m.post_balances,
                inner_instructions,
                inner_instructions_none: false,
                log_messages: match m.log_messages {
                    OptionSerializer::Some(logs) => logs,
                    OptionSerializer::None | OptionSerializer::Skip => vec![],
                },
                log_messages_none: false,
                pre_token_balances,
                post_token_balances,
                rewards: vec![],
                loaded_writable_addresses,
                loaded_readonly_addresses,
                return_data: None,
                return_data_none: false,
                compute_units_consumed: match m.compute_units_consumed {
                    OptionSerializer::Some(units) => Some(units),
                    OptionSerializer::None | OptionSerializer::Skip => None,
                },
                cost_units: None,
            }
        }),
        index: 0,
    };

    Ok(SubscribeUpdateTransaction {
        slot,
        transaction: Some(transaction_info),
    })
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableTransactionStatusMeta {
    pub err: Option<serde_json::Value>, // human-readable error
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub inner_instructions: Vec<SerializableInnerInstructions>,
    pub log_messages: Vec<String>,
    pub pre_token_balances: Vec<SerializableTokenBalance>,
    pub post_token_balances: Vec<SerializableTokenBalance>,
    pub loaded_writable_addresses: Vec<SerializablePubkey>,
    pub loaded_readonly_addresses: Vec<SerializablePubkey>,
    pub return_data: Option<SerializableReturnData>,
    pub compute_units_consumed: Option<u64>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableTransactionInfo {
    pub signature: String, // base58
    pub is_vote: bool,
    pub transaction: SerializableTransaction,
    pub meta: Option<SerializableTransactionStatusMeta>,
    pub index: u64,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableTransactionUpdate {
    pub slot: u64,
    pub transaction: Option<SerializableTransactionInfo>,
}

impl From<&SubscribeUpdateTransaction> for SerializableTransactionUpdate {
    #[allow(clippy::too_many_lines)]
    fn from(value: &SubscribeUpdateTransaction) -> Self {
        Self {
            slot: value.slot,
            transaction: value
                .transaction
                .as_ref()
                .map(|tx_info| SerializableTransactionInfo {
                    signature: bs58::encode(&tx_info.signature).into_string(),
                    is_vote: tx_info.is_vote,
                    transaction: tx_info.transaction.as_ref().map_or_else(
                        || SerializableTransaction {
                            signatures: vec![],
                            message: SerializableMessage {
                                header: SerializableMessageHeader {
                                    num_required_signatures: 0,
                                    num_readonly_signed_accounts: 0,
                                    num_readonly_unsigned_accounts: 0,
                                },
                                account_keys: vec![],
                                recent_blockhash: String::new(),
                                instructions: vec![],
                                versioned: false,
                                address_table_lookups: vec![],
                            },
                        },
                        |tx| SerializableTransaction {
                            signatures: tx
                                .signatures
                                .iter()
                                .map(|sig| bs58::encode(sig).into_string())
                                .collect(),
                            message: tx.message.as_ref().map_or_else(
                                || SerializableMessage {
                                    header: SerializableMessageHeader {
                                        num_required_signatures: 0,
                                        num_readonly_signed_accounts: 0,
                                        num_readonly_unsigned_accounts: 0,
                                    },
                                    account_keys: vec![],
                                    recent_blockhash: String::new(),
                                    instructions: vec![],
                                    versioned: false,
                                    address_table_lookups: vec![],
                                },
                                |msg| SerializableMessage {
                                    header: msg.header.as_ref().map_or_else(
                                        || SerializableMessageHeader {
                                            num_required_signatures: 0,
                                            num_readonly_signed_accounts: 0,
                                            num_readonly_unsigned_accounts: 0,
                                        },
                                        |h| SerializableMessageHeader {
                                            num_required_signatures: h.num_required_signatures,
                                            num_readonly_signed_accounts: h
                                                .num_readonly_signed_accounts,
                                            num_readonly_unsigned_accounts: h
                                                .num_readonly_unsigned_accounts,
                                        },
                                    ),
                                    account_keys: msg
                                        .account_keys
                                        .iter()
                                        .filter_map(|key| {
                                            if key.len() == 32 {
                                                let mut arr = [0u8; 32];
                                                arr.copy_from_slice(key);
                                                Some(SerializablePubkey(arr))
                                            } else {
                                                None
                                            }
                                        })
                                        .collect(),
                                    recent_blockhash: bs58::encode(&msg.recent_blockhash)
                                        .into_string(),
                                    instructions: msg
                                        .instructions
                                        .iter()
                                        .map(|ix| SerializableCompiledInstruction {
                                            program_id_index: ix.program_id_index,
                                            accounts: ix.accounts.clone(),
                                            data: ix.data.clone(),
                                        })
                                        .collect(),
                                    versioned: msg.versioned,
                                    address_table_lookups: msg
                                        .address_table_lookups
                                        .iter()
                                        .filter_map(|lookup| {
                                            if lookup.account_key.len() == 32 {
                                                let mut arr = [0u8; 32];
                                                arr.copy_from_slice(&lookup.account_key);
                                                Some(SerializableMessageAddressTableLookup {
                                                    account_key: SerializablePubkey(arr),
                                                    writable_indexes: lookup
                                                        .writable_indexes
                                                        .clone(),
                                                    readonly_indexes: lookup
                                                        .readonly_indexes
                                                        .clone(),
                                                })
                                            } else {
                                                None
                                            }
                                        })
                                        .collect(),
                                },
                            ),
                        },
                    ),
                    meta: tx_info
                        .meta
                        .as_ref()
                        .map(|meta| SerializableTransactionStatusMeta {
                            err: meta.err.as_ref().and_then(|proto_err| {
                                use solana_transaction_error::TransactionError;
                                bincode::deserialize::<TransactionError>(&proto_err.err)
                                    .ok()
                                    .and_then(|err| serde_json::to_value(&err).ok())
                            }),
                            fee: meta.fee,
                            pre_balances: meta.pre_balances.clone(),
                            post_balances: meta.post_balances.clone(),
                            inner_instructions: meta
                                .inner_instructions
                                .iter()
                                .map(|inner| SerializableInnerInstructions {
                                    index: inner.index,
                                    instructions: inner
                                        .instructions
                                        .iter()
                                        .map(|ix| SerializableInnerInstruction {
                                            program_id_index: ix.program_id_index,
                                            accounts: ix.accounts.clone(),
                                            data: ix.data.clone(),
                                            stack_height: ix.stack_height,
                                        })
                                        .collect(),
                                })
                                .collect(),
                            log_messages: meta.log_messages.clone(),
                            pre_token_balances: meta
                                .pre_token_balances
                                .iter()
                                .map(|tb| SerializableTokenBalance {
                                    account_index: tb.account_index,
                                    mint: tb.mint.clone(),
                                    ui_token_amount: tb.ui_token_amount.as_ref().map(|amt| {
                                        SerializableUiTokenAmount {
                                            ui_amount: amt.ui_amount,
                                            decimals: amt.decimals,
                                            amount: amt.amount.clone(),
                                            ui_amount_string: amt.ui_amount_string.clone(),
                                        }
                                    }),
                                    owner: tb.owner.clone(),
                                    program_id: tb.program_id.clone(),
                                })
                                .collect(),
                            post_token_balances: meta
                                .post_token_balances
                                .iter()
                                .map(|tb| SerializableTokenBalance {
                                    account_index: tb.account_index,
                                    mint: tb.mint.clone(),
                                    ui_token_amount: tb.ui_token_amount.as_ref().map(|amt| {
                                        SerializableUiTokenAmount {
                                            ui_amount: amt.ui_amount,
                                            decimals: amt.decimals,
                                            amount: amt.amount.clone(),
                                            ui_amount_string: amt.ui_amount_string.clone(),
                                        }
                                    }),
                                    owner: tb.owner.clone(),
                                    program_id: tb.program_id.clone(),
                                })
                                .collect(),
                            loaded_writable_addresses: meta
                                .loaded_writable_addresses
                                .iter()
                                .filter_map(|addr| {
                                    if addr.len() == 32 {
                                        let mut arr = [0u8; 32];
                                        arr.copy_from_slice(addr);
                                        Some(SerializablePubkey(arr))
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                            loaded_readonly_addresses: meta
                                .loaded_readonly_addresses
                                .iter()
                                .filter_map(|addr| {
                                    if addr.len() == 32 {
                                        let mut arr = [0u8; 32];
                                        arr.copy_from_slice(addr);
                                        Some(SerializablePubkey(arr))
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                            return_data: meta.return_data.as_ref().and_then(|rd| {
                                if rd.program_id.len() == 32 {
                                    let mut arr = [0u8; 32];
                                    arr.copy_from_slice(&rd.program_id);
                                    Some(SerializableReturnData {
                                        program_id: SerializablePubkey(arr),
                                        data: rd.data.clone(),
                                    })
                                } else {
                                    None
                                }
                            }),
                            compute_units_consumed: meta.compute_units_consumed,
                        }),
                    index: tx_info.index,
                }),
        }
    }
}

impl TryFrom<&SerializableTransactionUpdate> for SubscribeUpdateTransaction {
    type Error = String;

    #[allow(clippy::too_many_lines)]
    fn try_from(value: &SerializableTransactionUpdate) -> Result<Self, Self::Error> {
        use solana_transaction_error::TransactionError;

        Ok(Self {
            slot: value.slot,
            transaction: value
                .transaction
                .as_ref()
                .map(|tx_info| {
                    let transaction = Some(Transaction {
                        signatures: tx_info
                            .transaction
                            .signatures
                            .iter()
                            .map(|sig| {
                                bs58::decode(sig)
                                    .into_vec()
                                    .map_err(|e| format!("Invalid signature: {e:?}"))
                            })
                            .collect::<Result<Vec<_>, _>>()
                            .unwrap_or_default(),
                        message: Some(SolanaMessage {
                            header: Some(MessageHeader {
                                num_required_signatures: tx_info
                                    .transaction
                                    .message
                                    .header
                                    .num_required_signatures,
                                num_readonly_signed_accounts: tx_info
                                    .transaction
                                    .message
                                    .header
                                    .num_readonly_signed_accounts,
                                num_readonly_unsigned_accounts: tx_info
                                    .transaction
                                    .message
                                    .header
                                    .num_readonly_unsigned_accounts,
                            }),
                            account_keys: tx_info
                                .transaction
                                .message
                                .account_keys
                                .iter()
                                .map(|key| key.0.to_vec())
                                .collect(),
                            recent_blockhash: bs58::decode(&tx_info.transaction.message.recent_blockhash)
                                .into_vec()
                                .unwrap_or_default(),
                            instructions: tx_info
                                .transaction
                                .message
                                .instructions
                                .iter()
                                .map(|ix| CompiledInstruction {
                                    program_id_index: ix.program_id_index,
                                    accounts: ix.accounts.clone(),
                                    data: ix.data.clone(),
                                })
                                .collect(),
                            versioned: tx_info.transaction.message.versioned,
                            address_table_lookups: tx_info
                                .transaction
                                .message
                                .address_table_lookups
                                .iter()
                                .map(|lookup| {
                                    use yellowstone_grpc_proto::solana::storage::confirmed_block::MessageAddressTableLookup;
                                    MessageAddressTableLookup {
                                        account_key: lookup.account_key.0.to_vec(),
                                        writable_indexes: lookup.writable_indexes.clone(),
                                        readonly_indexes: lookup.readonly_indexes.clone(),
                                    }
                                })
                                .collect(),
                        }),
                    });

                    let meta = tx_info.meta.as_ref().map(|meta| {
                        TransactionStatusMeta {
                            err: meta.err.as_ref().map(|err_json| {
                                use yellowstone_grpc_proto::solana::storage::confirmed_block::TransactionError as ProtoError;
                                let err: TransactionError = serde_json::from_value(err_json.clone())
                                    .unwrap_or(TransactionError::AccountNotFound);
                                ProtoError {
                                    err: bincode::serialize(&err).unwrap_or_default(),
                                }
                            }),
                            fee: meta.fee,
                            pre_balances: meta.pre_balances.clone(),
                            post_balances: meta.post_balances.clone(),
                            inner_instructions: meta
                                .inner_instructions
                                .iter()
                                .map(|inner| InnerInstructions {
                                    index: inner.index,
                                    instructions: inner
                                        .instructions
                                        .iter()
                                        .map(|ix| InnerInstruction {
                                            program_id_index: ix.program_id_index,
                                            accounts: ix.accounts.clone(),
                                            data: ix.data.clone(),
                                            stack_height: ix.stack_height,
                                        })
                                        .collect(),
                                })
                                .collect(),
                            inner_instructions_none: false,
                            log_messages: meta.log_messages.clone(),
                            log_messages_none: false,
                            pre_token_balances: meta
                                .pre_token_balances
                                .iter()
                                .map(|tb| TokenBalance {
                                    account_index: tb.account_index,
                                    mint: tb.mint.clone(),
                                    ui_token_amount: tb.ui_token_amount.as_ref().map(|amt| {
                                        use yellowstone_grpc_proto::solana::storage::confirmed_block::UiTokenAmount;
                                        UiTokenAmount {
                                            ui_amount: amt.ui_amount,
                                            decimals: amt.decimals,
                                            amount: amt.amount.clone(),
                                            ui_amount_string: amt.ui_amount_string.clone(),
                                        }
                                    }),
                                    owner: tb.owner.clone(),
                                    program_id: tb.program_id.clone(),
                                })
                                .collect(),
                            post_token_balances: meta
                                .post_token_balances
                                .iter()
                                .map(|tb| TokenBalance {
                                    account_index: tb.account_index,
                                    mint: tb.mint.clone(),
                                    ui_token_amount: tb.ui_token_amount.as_ref().map(|amt| {
                                        use yellowstone_grpc_proto::solana::storage::confirmed_block::UiTokenAmount;
                                        UiTokenAmount {
                                            ui_amount: amt.ui_amount,
                                            decimals: amt.decimals,
                                            amount: amt.amount.clone(),
                                            ui_amount_string: amt.ui_amount_string.clone(),
                                        }
                                    }),
                                    owner: tb.owner.clone(),
                                    program_id: tb.program_id.clone(),
                                })
                                .collect(),
                            rewards: vec![],
                            loaded_writable_addresses: meta
                                .loaded_writable_addresses
                                .iter()
                                .map(|addr| addr.0.to_vec())
                                .collect(),
                            loaded_readonly_addresses: meta
                                .loaded_readonly_addresses
                                .iter()
                                .map(|addr| addr.0.to_vec())
                                .collect(),
                            return_data: meta.return_data.as_ref().map(|rd| {
                                use yellowstone_grpc_proto::solana::storage::confirmed_block::ReturnData;
                                ReturnData {
                                    program_id: rd.program_id.0.to_vec(),
                                    data: rd.data.clone(),
                                }
                            }),
                            return_data_none: false,
                            compute_units_consumed: meta.compute_units_consumed,
                            cost_units: None,
                        }
                    });

                    SubscribeUpdateTransactionInfo {
                        signature: bs58::decode(&tx_info.signature)
                            .into_vec()
                            .unwrap_or_default(),
                        is_vote: tx_info.is_vote,
                        transaction,
                        meta,
                        index: tx_info.index,
                    }
                }),
        })
    }
}

/*
TransactionUpdate mock
*/

/// Create a mock `TransactionUpdate` with fixture caching support
pub async fn create_mock_transaction_update_with_cache(
    signature: &str,
) -> Result<TransactionUpdate, Box<dyn std::error::Error>> {
    maybe_create_fixture_dir()?;

    // Create fixture path with _txupdate.json suffix (using base64-encoded JSON)
    let mut file_name = signature.to_string();
    file_name.push_str("_txupdate.json");
    let path = Path::new(FIXTURES_PATH).join(file_name);

    if path.is_file() {
        // Read from fixture
        let json_str = fs::read_to_string(&path)?;
        let serializable: SerializableTransactionUpdate = serde_json::from_str(&json_str)?;
        let tx_update = SubscribeUpdateTransaction::try_from(&serializable)
            .map_err(|e| format!("Failed to convert: {e}"))?;
        Ok(tx_update)
    } else {
        // Fetch from RPC and save to fixture
        let tx_update = create_mock_transaction_update(signature).await?;
        let serializable = SerializableTransactionUpdate::from(&tx_update);
        let json_str = serde_json::to_string_pretty(&serializable)?;
        fs::write(&path, json_str)?;
        Ok(tx_update)
    }
}

/// Create a mock `TransactionUpdate` from a transaction signature for testing
pub async fn create_mock_transaction_update(
    signature: &str,
) -> Result<TransactionUpdate, Box<dyn std::error::Error>> {
    let sig = Signature::from_str(signature)?;
    let rpc_client = get_rpc_client();

    let params = json!([sig.to_string(), {
        "encoding": "json",
        "maxSupportedTransactionVersion": 0
    }]);

    let tx = rpc_client
        .send(RpcRequest::GetTransaction, params)
        .await
        .map_err(|e| format!("Error fetching tx: {e:?}"))?;

    convert_to_transaction_update(tx)
}

/// Parse instructions from a `TransactionUpdate` using the core `parse_from_txn` logic
pub fn parse_instructions_from_txn_update(
    txn_update: &TransactionUpdate,
) -> Result<Vec<InstructionUpdate>, Box<dyn std::error::Error>> {
    InstructionUpdate::parse_from_txn(txn_update).map_err(Into::into)
}

/*
transaction update fixture
*/

pub fn read_transaction_update_fixture(
    data: &[u8],
) -> Result<FixtureData, Box<dyn std::error::Error>> {
    let json_str = std::str::from_utf8(data)?;
    let serializable: SerializableTransactionUpdate = serde_json::from_str(json_str)?;
    let tx_update = SubscribeUpdateTransaction::try_from(&serializable)
        .map_err(|e| format!("Failed to convert: {e}"))?;
    Ok(FixtureData::TransactionUpdate(Box::new(tx_update)))
}

/*
TransactionUpdate
*/

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableMessageHeader {
    pub num_required_signatures: u32,
    pub num_readonly_signed_accounts: u32,
    pub num_readonly_unsigned_accounts: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableCompiledInstruction {
    pub program_id_index: u32,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableInnerInstruction {
    pub program_id_index: u32,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
    pub stack_height: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableInnerInstructions {
    pub index: u32,
    pub instructions: Vec<SerializableInnerInstruction>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableMessageAddressTableLookup {
    pub account_key: SerializablePubkey,
    pub writable_indexes: Vec<u8>,
    pub readonly_indexes: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableMessage {
    pub header: SerializableMessageHeader,
    pub account_keys: Vec<SerializablePubkey>,
    pub recent_blockhash: String, // base58
    pub instructions: Vec<SerializableCompiledInstruction>,
    pub versioned: bool,
    pub address_table_lookups: Vec<SerializableMessageAddressTableLookup>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableTransaction {
    pub signatures: Vec<String>, // base58
    pub message: SerializableMessage,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableUiTokenAmount {
    pub ui_amount: f64,
    pub decimals: u32,
    pub amount: String,
    pub ui_amount_string: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableTokenBalance {
    pub account_index: u32,
    pub mint: String,
    pub ui_token_amount: Option<SerializableUiTokenAmount>,
    pub owner: String,
    pub program_id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SerializableReturnData {
    pub program_id: SerializablePubkey,
    pub data: Vec<u8>,
}
