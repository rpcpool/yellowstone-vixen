// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/parser.proto

import type { TransferIxProto as _vixen_parser_TransferIxProto, TransferIxProto__Output as _vixen_parser_TransferIxProto__Output } from '../../vixen/parser/TransferIxProto';
import type { InitializeMintIxProto as _vixen_parser_InitializeMintIxProto, InitializeMintIxProto__Output as _vixen_parser_InitializeMintIxProto__Output } from '../../vixen/parser/InitializeMintIxProto';
import type { InitializeAccountIxProto as _vixen_parser_InitializeAccountIxProto, InitializeAccountIxProto__Output as _vixen_parser_InitializeAccountIxProto__Output } from '../../vixen/parser/InitializeAccountIxProto';
import type { InitializeAccount2IxProto as _vixen_parser_InitializeAccount2IxProto, InitializeAccount2IxProto__Output as _vixen_parser_InitializeAccount2IxProto__Output } from '../../vixen/parser/InitializeAccount2IxProto';
import type { InitializeAccount3IxProto as _vixen_parser_InitializeAccount3IxProto, InitializeAccount3IxProto__Output as _vixen_parser_InitializeAccount3IxProto__Output } from '../../vixen/parser/InitializeAccount3IxProto';
import type { InitializeMultisigIxProto as _vixen_parser_InitializeMultisigIxProto, InitializeMultisigIxProto__Output as _vixen_parser_InitializeMultisigIxProto__Output } from '../../vixen/parser/InitializeMultisigIxProto';
import type { ApproveIxProto as _vixen_parser_ApproveIxProto, ApproveIxProto__Output as _vixen_parser_ApproveIxProto__Output } from '../../vixen/parser/ApproveIxProto';
import type { RevokeIxProto as _vixen_parser_RevokeIxProto, RevokeIxProto__Output as _vixen_parser_RevokeIxProto__Output } from '../../vixen/parser/RevokeIxProto';
import type { SetAuthorityIxProto as _vixen_parser_SetAuthorityIxProto, SetAuthorityIxProto__Output as _vixen_parser_SetAuthorityIxProto__Output } from '../../vixen/parser/SetAuthorityIxProto';
import type { MintToIxProto as _vixen_parser_MintToIxProto, MintToIxProto__Output as _vixen_parser_MintToIxProto__Output } from '../../vixen/parser/MintToIxProto';
import type { BurnIxProto as _vixen_parser_BurnIxProto, BurnIxProto__Output as _vixen_parser_BurnIxProto__Output } from '../../vixen/parser/BurnIxProto';
import type { CloseAccountIxProto as _vixen_parser_CloseAccountIxProto, CloseAccountIxProto__Output as _vixen_parser_CloseAccountIxProto__Output } from '../../vixen/parser/CloseAccountIxProto';
import type { FreezeAccountIxProto as _vixen_parser_FreezeAccountIxProto, FreezeAccountIxProto__Output as _vixen_parser_FreezeAccountIxProto__Output } from '../../vixen/parser/FreezeAccountIxProto';
import type { ThawAccountIxProto as _vixen_parser_ThawAccountIxProto, ThawAccountIxProto__Output as _vixen_parser_ThawAccountIxProto__Output } from '../../vixen/parser/ThawAccountIxProto';
import type { TransferCheckedIxProto as _vixen_parser_TransferCheckedIxProto, TransferCheckedIxProto__Output as _vixen_parser_TransferCheckedIxProto__Output } from '../../vixen/parser/TransferCheckedIxProto';
import type { ApproveCheckedIxProto as _vixen_parser_ApproveCheckedIxProto, ApproveCheckedIxProto__Output as _vixen_parser_ApproveCheckedIxProto__Output } from '../../vixen/parser/ApproveCheckedIxProto';
import type { MintToCheckedIxProto as _vixen_parser_MintToCheckedIxProto, MintToCheckedIxProto__Output as _vixen_parser_MintToCheckedIxProto__Output } from '../../vixen/parser/MintToCheckedIxProto';
import type { BurnCheckedIxProto as _vixen_parser_BurnCheckedIxProto, BurnCheckedIxProto__Output as _vixen_parser_BurnCheckedIxProto__Output } from '../../vixen/parser/BurnCheckedIxProto';
import type { SyncNativeIxProto as _vixen_parser_SyncNativeIxProto, SyncNativeIxProto__Output as _vixen_parser_SyncNativeIxProto__Output } from '../../vixen/parser/SyncNativeIxProto';
import type { GetAccountDataSizeIxProto as _vixen_parser_GetAccountDataSizeIxProto, GetAccountDataSizeIxProto__Output as _vixen_parser_GetAccountDataSizeIxProto__Output } from '../../vixen/parser/GetAccountDataSizeIxProto';
import type { InitializeImmutableOwnerIxProto as _vixen_parser_InitializeImmutableOwnerIxProto, InitializeImmutableOwnerIxProto__Output as _vixen_parser_InitializeImmutableOwnerIxProto__Output } from '../../vixen/parser/InitializeImmutableOwnerIxProto';
import type { AmountToUiAmountIxProto as _vixen_parser_AmountToUiAmountIxProto, AmountToUiAmountIxProto__Output as _vixen_parser_AmountToUiAmountIxProto__Output } from '../../vixen/parser/AmountToUiAmountIxProto';
import type { UiAmountToAmountIxProto as _vixen_parser_UiAmountToAmountIxProto, UiAmountToAmountIxProto__Output as _vixen_parser_UiAmountToAmountIxProto__Output } from '../../vixen/parser/UiAmountToAmountIxProto';

export interface TokenProgramIxProto {
  'transfer'?: (_vixen_parser_TransferIxProto | null);
  'initializeMint'?: (_vixen_parser_InitializeMintIxProto | null);
  'initializeAccount'?: (_vixen_parser_InitializeAccountIxProto | null);
  'initializeAccount2'?: (_vixen_parser_InitializeAccount2IxProto | null);
  'initializeAccount3'?: (_vixen_parser_InitializeAccount3IxProto | null);
  'initializeMultisig'?: (_vixen_parser_InitializeMultisigIxProto | null);
  'approve'?: (_vixen_parser_ApproveIxProto | null);
  'revoke'?: (_vixen_parser_RevokeIxProto | null);
  'setAuthority'?: (_vixen_parser_SetAuthorityIxProto | null);
  'mintTo'?: (_vixen_parser_MintToIxProto | null);
  'burn'?: (_vixen_parser_BurnIxProto | null);
  'closeAccount'?: (_vixen_parser_CloseAccountIxProto | null);
  'freezeAccount'?: (_vixen_parser_FreezeAccountIxProto | null);
  'thawAccount'?: (_vixen_parser_ThawAccountIxProto | null);
  'transferChecked'?: (_vixen_parser_TransferCheckedIxProto | null);
  'approveChecked'?: (_vixen_parser_ApproveCheckedIxProto | null);
  'mintToChecked'?: (_vixen_parser_MintToCheckedIxProto | null);
  'burnChecked'?: (_vixen_parser_BurnCheckedIxProto | null);
  'syncNative'?: (_vixen_parser_SyncNativeIxProto | null);
  'getAccountDataSize'?: (_vixen_parser_GetAccountDataSizeIxProto | null);
  'initializeImmutableOwner'?: (_vixen_parser_InitializeImmutableOwnerIxProto | null);
  'amountToUiAmount'?: (_vixen_parser_AmountToUiAmountIxProto | null);
  'uiAmountToAmount'?: (_vixen_parser_UiAmountToAmountIxProto | null);
  'ixOneof'?: "transfer"|"initializeMint"|"initializeAccount"|"initializeAccount2"|"initializeAccount3"|"initializeMultisig"|"approve"|"revoke"|"setAuthority"|"mintTo"|"burn"|"closeAccount"|"freezeAccount"|"thawAccount"|"transferChecked"|"approveChecked"|"mintToChecked"|"burnChecked"|"syncNative"|"getAccountDataSize"|"initializeImmutableOwner"|"amountToUiAmount"|"uiAmountToAmount";
}

export interface TokenProgramIxProto__Output {
  'transfer'?: (_vixen_parser_TransferIxProto__Output | null);
  'initializeMint'?: (_vixen_parser_InitializeMintIxProto__Output | null);
  'initializeAccount'?: (_vixen_parser_InitializeAccountIxProto__Output | null);
  'initializeAccount2'?: (_vixen_parser_InitializeAccount2IxProto__Output | null);
  'initializeAccount3'?: (_vixen_parser_InitializeAccount3IxProto__Output | null);
  'initializeMultisig'?: (_vixen_parser_InitializeMultisigIxProto__Output | null);
  'approve'?: (_vixen_parser_ApproveIxProto__Output | null);
  'revoke'?: (_vixen_parser_RevokeIxProto__Output | null);
  'setAuthority'?: (_vixen_parser_SetAuthorityIxProto__Output | null);
  'mintTo'?: (_vixen_parser_MintToIxProto__Output | null);
  'burn'?: (_vixen_parser_BurnIxProto__Output | null);
  'closeAccount'?: (_vixen_parser_CloseAccountIxProto__Output | null);
  'freezeAccount'?: (_vixen_parser_FreezeAccountIxProto__Output | null);
  'thawAccount'?: (_vixen_parser_ThawAccountIxProto__Output | null);
  'transferChecked'?: (_vixen_parser_TransferCheckedIxProto__Output | null);
  'approveChecked'?: (_vixen_parser_ApproveCheckedIxProto__Output | null);
  'mintToChecked'?: (_vixen_parser_MintToCheckedIxProto__Output | null);
  'burnChecked'?: (_vixen_parser_BurnCheckedIxProto__Output | null);
  'syncNative'?: (_vixen_parser_SyncNativeIxProto__Output | null);
  'getAccountDataSize'?: (_vixen_parser_GetAccountDataSizeIxProto__Output | null);
  'initializeImmutableOwner'?: (_vixen_parser_InitializeImmutableOwnerIxProto__Output | null);
  'amountToUiAmount'?: (_vixen_parser_AmountToUiAmountIxProto__Output | null);
  'uiAmountToAmount'?: (_vixen_parser_UiAmountToAmountIxProto__Output | null);
  'ixOneof': "transfer"|"initializeMint"|"initializeAccount"|"initializeAccount2"|"initializeAccount3"|"initializeMultisig"|"approve"|"revoke"|"setAuthority"|"mintTo"|"burn"|"closeAccount"|"freezeAccount"|"thawAccount"|"transferChecked"|"approveChecked"|"mintToChecked"|"burnChecked"|"syncNative"|"getAccountDataSize"|"initializeImmutableOwner"|"amountToUiAmount"|"uiAmountToAmount";
}
