pub const PROTOBUF_SCHEMA: &str = r#"syntax = "proto3";

package spl_token;

enum AuthorityTypeProto {
  AUTHORITY_MINT_TOKENS = 0;
  AUTHORITY_FREEZE_ACCOUNT = 1;
  AUTHORITY_ACCOUNT_OWNER = 2;
  AUTHORITY_CLOSE_ACCOUNT = 3;
}

message TransferAccounts {
  bytes source = 1;
  bytes destination = 2;
  bytes owner = 3;
  repeated bytes multisig_signers = 4;
}

message InitializeMintAccounts {
  bytes mint = 1;
}

message InitializeAccountAccounts {
  bytes account = 1;
  bytes mint = 2;
  bytes owner = 3;
}

message InitializeAccount2Accounts {
  bytes account = 1;
  bytes mint = 2;
}

message InitializeMultisigAccounts {
  bytes multisig = 1;
  repeated bytes signers = 2;
}

message ApproveAccounts {
  bytes source = 1;
  bytes delegate = 2;
  bytes owner = 3;
  repeated bytes multisig_signers = 4;
}

message RevokeAccounts {
  bytes source = 1;
  bytes owner = 2;
  repeated bytes multisig_signers = 3;
}

message SetAuthorityAccounts {
  bytes current_authority = 1;
  bytes account = 2;
  repeated bytes multisig_signers = 3;
}

message MintToAccounts {
  bytes mint = 1;
  bytes account = 2;
  bytes mint_authority = 3;
  repeated bytes multisig_signers = 4;
}

message BurnAccounts {
  bytes account = 1;
  bytes mint = 2;
  bytes owner = 3;
  repeated bytes multisig_signers = 4;
}

message CloseAccountAccounts {
  bytes account = 1;
  bytes destination = 2;
  bytes owner = 3;
  repeated bytes multisig_signers = 4;
}

message FreezeAccountAccounts {
  bytes account = 1;
  bytes mint = 2;
  bytes mint_freeze_authority = 3;
  repeated bytes multisig_signers = 4;
}

message ThawAccountAccounts {
  bytes account = 1;
  bytes mint = 2;
  bytes mint_freeze_authority = 3;
  repeated bytes multisig_signers = 4;
}

message TransferCheckedAccounts {
  bytes source = 1;
  bytes mint = 2;
  bytes destination = 3;
  bytes owner = 4;
  repeated bytes multisig_signers = 5;
}

message ApproveCheckedAccounts {
  bytes source = 1;
  bytes mint = 2;
  bytes delegate = 3;
  bytes owner = 4;
  repeated bytes multisig_signers = 5;
}

message MintToCheckedAccounts {
  bytes mint = 1;
  bytes account = 2;
  bytes mint_authority = 3;
  repeated bytes multisig_signers = 4;
}

message BurnCheckedAccounts {
  bytes account = 1;
  bytes mint = 2;
  bytes owner = 3;
  repeated bytes multisig_signers = 4;
}

message SyncNativeAccounts {
  bytes account = 1;
}

message GetAccountDataSizeAccounts {
  bytes mint = 1;
}

message InitializeImmutableOwnerAccounts {
  bytes account = 1;
}

message AmountToUiAmountAccounts {
  bytes mint = 1;
}

message UiAmountToAmountAccounts {
  bytes mint = 1;
}

message TransferArgs {
  uint64 amount = 1;
}

message TransferCheckedArgs {
  uint64 amount = 1;
  uint32 decimals = 2;
}

message InitializeMintArgs {
  uint32 decimals = 1;
  bytes mint_authority = 2;
  bytes freeze_authority = 3;
}

message InitializeAccount2Args {
  bytes owner = 1;
}

message InitializeMultisigArgs {
  uint32 m = 1;
}

message ApproveArgs {
  uint64 amount = 1;
}

message SetAuthorityArgs {
  AuthorityTypeProto authority_type = 1;
  bytes new_authority = 2;
}

message MintToArgs {
  uint64 amount = 1;
}

message BurnArgs {
  uint64 amount = 1;
}

message ApproveCheckedArgs {
  uint64 amount = 1;
  uint32 decimals = 2;
}

message MintToCheckedArgs {
  uint64 amount = 1;
  uint32 decimals = 2;
}

message BurnCheckedArgs {
  uint64 amount = 1;
  uint32 decimals = 2;
}

message AmountToUiAmountArgs {
  uint64 amount = 1;
}

message UiAmountToAmountArgs {
  string ui_amount = 1;
}

message Transfer {
  TransferAccounts accounts = 1;
  TransferArgs args = 2;
}

message InitializeMint {
  InitializeMintAccounts accounts = 1;
  InitializeMintArgs args = 2;
}

message InitializeAccount {
  InitializeAccountAccounts accounts = 1;
}

message InitializeAccount2 {
  InitializeAccount2Accounts accounts = 1;
  InitializeAccount2Args args = 2;
}

message InitializeAccount3 {
  InitializeAccount2Accounts accounts = 1;
  InitializeAccount2Args args = 2;
}

message InitializeMultisig {
  InitializeMultisigAccounts accounts = 1;
  InitializeMultisigArgs args = 2;
}

message Approve {
  ApproveAccounts accounts = 1;
  ApproveArgs args = 2;
}

message Revoke {
  RevokeAccounts accounts = 1;
}

message SetAuthority {
  SetAuthorityAccounts accounts = 1;
  SetAuthorityArgs args = 2;
}

message MintTo {
  MintToAccounts accounts = 1;
  MintToArgs args = 2;
}

message Burn {
  BurnAccounts accounts = 1;
  BurnArgs args = 2;
}

message CloseAccount {
  CloseAccountAccounts accounts = 1;
}

message FreezeAccount {
  FreezeAccountAccounts accounts = 1;
}

message ThawAccount {
  ThawAccountAccounts accounts = 1;
}

message TransferChecked {
  TransferCheckedAccounts accounts = 1;
  TransferCheckedArgs args = 2;
}

message ApproveChecked {
  ApproveCheckedAccounts accounts = 1;
  ApproveCheckedArgs args = 2;
}

message MintToChecked {
  MintToCheckedAccounts accounts = 1;
  MintToCheckedArgs args = 2;
}

message BurnChecked {
  BurnCheckedAccounts accounts = 1;
  BurnCheckedArgs args = 2;
}

message SyncNative {
  SyncNativeAccounts accounts = 1;
}

message GetAccountDataSize {
  GetAccountDataSizeAccounts accounts = 1;
}

message InitializeImmutableOwner {
  InitializeImmutableOwnerAccounts accounts = 1;
}

message AmountToUiAmount {
  AmountToUiAmountAccounts accounts = 1;
  AmountToUiAmountArgs args = 2;
}

message UiAmountToAmount {
  UiAmountToAmountAccounts accounts = 1;
  UiAmountToAmountArgs args = 2;
}

message TokenProgramInstruction {
  oneof ix {
    Transfer transfer = 1;
    InitializeMint initialize_mint = 2;
    InitializeAccount initialize_account = 3;
    InitializeAccount2 initialize_account2 = 4;
    InitializeAccount3 initialize_account3 = 5;
    InitializeMultisig initialize_multisig = 6;
    Approve approve = 7;
    Revoke revoke = 8;
    SetAuthority set_authority = 9;
    MintTo mint_to = 10;
    Burn burn = 11;
    CloseAccount close_account = 12;
    FreezeAccount freeze_account = 13;
    ThawAccount thaw_account = 14;
    TransferChecked transfer_checked = 15;
    ApproveChecked approve_checked = 16;
    MintToChecked mint_to_checked = 17;
    BurnChecked burn_checked = 18;
    SyncNative sync_native = 19;
    GetAccountDataSize get_account_data_size = 20;
    InitializeImmutableOwner initialize_immutable_owner = 21;
    AmountToUiAmount amount_to_ui_amount = 22;
    UiAmountToAmount ui_amount_to_amount = 23;
  }
}
"#;
