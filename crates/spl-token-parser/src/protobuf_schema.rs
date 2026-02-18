#[cfg(feature = "proto")]
pub const PROTOBUF_SCHEMA: &str = r#"syntax = "proto3";

package spl_token;

enum AuthorityType {
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
  optional bytes freeze_authority = 3;
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
  AuthorityType authority_type = 1;
  optional bytes new_authority = 2;
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

message TransferInstruction {
  TransferAccounts accounts = 1;
  TransferArgs args = 2;
}

message InitializeMintInstruction {
  InitializeMintAccounts accounts = 1;
  InitializeMintArgs args = 2;
}

message InitializeAccountInstruction {
  InitializeAccountAccounts accounts = 1;
}

message InitializeAccount2Instruction {
  InitializeAccount2Accounts accounts = 1;
  InitializeAccount2Args args = 2;
}

message InitializeAccount3Instruction {
  InitializeAccount2Accounts accounts = 1;
  InitializeAccount2Args args = 2;
}

message InitializeMultisigInstruction {
  InitializeMultisigAccounts accounts = 1;
  InitializeMultisigArgs args = 2;
}

message ApproveInstruction {
  ApproveAccounts accounts = 1;
  ApproveArgs args = 2;
}

message RevokeInstruction {
  RevokeAccounts accounts = 1;
}

message SetAuthorityInstruction {
  SetAuthorityAccounts accounts = 1;
  SetAuthorityArgs args = 2;
}

message MintToInstruction {
  MintToAccounts accounts = 1;
  MintToArgs args = 2;
}

message BurnInstruction {
  BurnAccounts accounts = 1;
  BurnArgs args = 2;
}

message CloseAccountInstruction {
  CloseAccountAccounts accounts = 1;
}

message FreezeAccountInstruction {
  FreezeAccountAccounts accounts = 1;
}

message ThawAccountInstruction {
  ThawAccountAccounts accounts = 1;
}

message TransferCheckedInstruction {
  TransferCheckedAccounts accounts = 1;
  TransferCheckedArgs args = 2;
}

message ApproveCheckedInstruction {
  ApproveCheckedAccounts accounts = 1;
  ApproveCheckedArgs args = 2;
}

message MintToCheckedInstruction {
  MintToCheckedAccounts accounts = 1;
  MintToCheckedArgs args = 2;
}

message BurnCheckedInstruction {
  BurnCheckedAccounts accounts = 1;
  BurnCheckedArgs args = 2;
}

message SyncNativeInstruction {
  SyncNativeAccounts accounts = 1;
}

message GetAccountDataSizeInstruction {
  GetAccountDataSizeAccounts accounts = 1;
}

message InitializeImmutableOwnerInstruction {
  InitializeImmutableOwnerAccounts accounts = 1;
}

message AmountToUiAmountInstruction {
  AmountToUiAmountAccounts accounts = 1;
  AmountToUiAmountArgs args = 2;
}

message UiAmountToAmountInstruction {
  UiAmountToAmountAccounts accounts = 1;
  UiAmountToAmountArgs args = 2;
}

message TokenProgram {
  oneof ix {
    TransferInstruction transfer = 1;
    InitializeMintInstruction initialize_mint = 2;
    InitializeAccountInstruction initialize_account = 3;
    InitializeAccount2Instruction initialize_account2 = 4;
    InitializeAccount3Instruction initialize_account3 = 5;
    InitializeMultisigInstruction initialize_multisig = 6;
    ApproveInstruction approve = 7;
    RevokeInstruction revoke = 8;
    SetAuthorityInstruction set_authority = 9;
    MintToInstruction mint_to = 10;
    BurnInstruction burn = 11;
    CloseAccountInstruction close_account = 12;
    FreezeAccountInstruction freeze_account = 13;
    ThawAccountInstruction thaw_account = 14;
    TransferCheckedInstruction transfer_checked = 15;
    ApproveCheckedInstruction approve_checked = 16;
    MintToCheckedInstruction mint_to_checked = 17;
    BurnCheckedInstruction burn_checked = 18;
    SyncNativeInstruction sync_native = 19;
    GetAccountDataSizeInstruction get_account_data_size = 20;
    InitializeImmutableOwnerInstruction initialize_immutable_owner = 21;
    AmountToUiAmountInstruction amount_to_ui_amount = 22;
    UiAmountToAmountInstruction ui_amount_to_amount = 23;
  }
}
"#;
