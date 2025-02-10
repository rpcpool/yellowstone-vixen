// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { InitializeConfidentialMintIxProto as _vixen_parser_InitializeConfidentialMintIxProto, InitializeConfidentialMintIxProto__Output as _vixen_parser_InitializeConfidentialMintIxProto__Output } from '../../vixen/parser/InitializeConfidentialMintIxProto';
import type { UpdateMintIxProto as _vixen_parser_UpdateMintIxProto, UpdateMintIxProto__Output as _vixen_parser_UpdateMintIxProto__Output } from '../../vixen/parser/UpdateMintIxProto';
import type { ConfigureAccountIxProto as _vixen_parser_ConfigureAccountIxProto, ConfigureAccountIxProto__Output as _vixen_parser_ConfigureAccountIxProto__Output } from '../../vixen/parser/ConfigureAccountIxProto';
import type { ApproveAccountIxProto as _vixen_parser_ApproveAccountIxProto, ApproveAccountIxProto__Output as _vixen_parser_ApproveAccountIxProto__Output } from '../../vixen/parser/ApproveAccountIxProto';
import type { EmptyAccountIxProto as _vixen_parser_EmptyAccountIxProto, EmptyAccountIxProto__Output as _vixen_parser_EmptyAccountIxProto__Output } from '../../vixen/parser/EmptyAccountIxProto';
import type { DepositIxProto as _vixen_parser_DepositIxProto, DepositIxProto__Output as _vixen_parser_DepositIxProto__Output } from '../../vixen/parser/DepositIxProto';
import type { WithdrawIxProto as _vixen_parser_WithdrawIxProto, WithdrawIxProto__Output as _vixen_parser_WithdrawIxProto__Output } from '../../vixen/parser/WithdrawIxProto';
import type { ConfidentialTransferIxProto as _vixen_parser_ConfidentialTransferIxProto, ConfidentialTransferIxProto__Output as _vixen_parser_ConfidentialTransferIxProto__Output } from '../../vixen/parser/ConfidentialTransferIxProto';
import type { ApplyPendingBalanceIxProto as _vixen_parser_ApplyPendingBalanceIxProto, ApplyPendingBalanceIxProto__Output as _vixen_parser_ApplyPendingBalanceIxProto__Output } from '../../vixen/parser/ApplyPendingBalanceIxProto';
import type { EnableConfidentialCreditsIxProto as _vixen_parser_EnableConfidentialCreditsIxProto, EnableConfidentialCreditsIxProto__Output as _vixen_parser_EnableConfidentialCreditsIxProto__Output } from '../../vixen/parser/EnableConfidentialCreditsIxProto';
import type { DisableConfidentialCreditsIxProto as _vixen_parser_DisableConfidentialCreditsIxProto, DisableConfidentialCreditsIxProto__Output as _vixen_parser_DisableConfidentialCreditsIxProto__Output } from '../../vixen/parser/DisableConfidentialCreditsIxProto';
import type { EnableNonConfidentialCreditsIxProto as _vixen_parser_EnableNonConfidentialCreditsIxProto, EnableNonConfidentialCreditsIxProto__Output as _vixen_parser_EnableNonConfidentialCreditsIxProto__Output } from '../../vixen/parser/EnableNonConfidentialCreditsIxProto';
import type { DisableNonConfidentialCreditsIxProto as _vixen_parser_DisableNonConfidentialCreditsIxProto, DisableNonConfidentialCreditsIxProto__Output as _vixen_parser_DisableNonConfidentialCreditsIxProto__Output } from '../../vixen/parser/DisableNonConfidentialCreditsIxProto';
import type { TransferWithSplitProofsIxProto as _vixen_parser_TransferWithSplitProofsIxProto, TransferWithSplitProofsIxProto__Output as _vixen_parser_TransferWithSplitProofsIxProto__Output } from '../../vixen/parser/TransferWithSplitProofsIxProto';

export interface ConfidentialTransferExtIxProto {
  'initializeMintIx'?: (_vixen_parser_InitializeConfidentialMintIxProto | null);
  'updateMintIx'?: (_vixen_parser_UpdateMintIxProto | null);
  'configureAccountIx'?: (_vixen_parser_ConfigureAccountIxProto | null);
  'approveAccountIx'?: (_vixen_parser_ApproveAccountIxProto | null);
  'emptyAccountIx'?: (_vixen_parser_EmptyAccountIxProto | null);
  'depositIx'?: (_vixen_parser_DepositIxProto | null);
  'withdrawIx'?: (_vixen_parser_WithdrawIxProto | null);
  'transferIx'?: (_vixen_parser_ConfidentialTransferIxProto | null);
  'applyPendingBalanceIx'?: (_vixen_parser_ApplyPendingBalanceIxProto | null);
  'enableConfidentialCreditsIx'?: (_vixen_parser_EnableConfidentialCreditsIxProto | null);
  'disableConfidentialCreditsIx'?: (_vixen_parser_DisableConfidentialCreditsIxProto | null);
  'enableNonConfidentialCreditsIx'?: (_vixen_parser_EnableNonConfidentialCreditsIxProto | null);
  'disableNonConfidentialCreditsIx'?: (_vixen_parser_DisableNonConfidentialCreditsIxProto | null);
  'transferWithSplitProofsIx'?: (_vixen_parser_TransferWithSplitProofsIxProto | null);
  'ixOneof'?: "initializeMintIx"|"updateMintIx"|"configureAccountIx"|"approveAccountIx"|"emptyAccountIx"|"depositIx"|"withdrawIx"|"transferIx"|"applyPendingBalanceIx"|"enableConfidentialCreditsIx"|"disableConfidentialCreditsIx"|"enableNonConfidentialCreditsIx"|"disableNonConfidentialCreditsIx"|"transferWithSplitProofsIx";
}

export interface ConfidentialTransferExtIxProto__Output {
  'initializeMintIx'?: (_vixen_parser_InitializeConfidentialMintIxProto__Output | null);
  'updateMintIx'?: (_vixen_parser_UpdateMintIxProto__Output | null);
  'configureAccountIx'?: (_vixen_parser_ConfigureAccountIxProto__Output | null);
  'approveAccountIx'?: (_vixen_parser_ApproveAccountIxProto__Output | null);
  'emptyAccountIx'?: (_vixen_parser_EmptyAccountIxProto__Output | null);
  'depositIx'?: (_vixen_parser_DepositIxProto__Output | null);
  'withdrawIx'?: (_vixen_parser_WithdrawIxProto__Output | null);
  'transferIx'?: (_vixen_parser_ConfidentialTransferIxProto__Output | null);
  'applyPendingBalanceIx'?: (_vixen_parser_ApplyPendingBalanceIxProto__Output | null);
  'enableConfidentialCreditsIx'?: (_vixen_parser_EnableConfidentialCreditsIxProto__Output | null);
  'disableConfidentialCreditsIx'?: (_vixen_parser_DisableConfidentialCreditsIxProto__Output | null);
  'enableNonConfidentialCreditsIx'?: (_vixen_parser_EnableNonConfidentialCreditsIxProto__Output | null);
  'disableNonConfidentialCreditsIx'?: (_vixen_parser_DisableNonConfidentialCreditsIxProto__Output | null);
  'transferWithSplitProofsIx'?: (_vixen_parser_TransferWithSplitProofsIxProto__Output | null);
  'ixOneof': "initializeMintIx"|"updateMintIx"|"configureAccountIx"|"approveAccountIx"|"emptyAccountIx"|"depositIx"|"withdrawIx"|"transferIx"|"applyPendingBalanceIx"|"enableConfidentialCreditsIx"|"disableConfidentialCreditsIx"|"enableNonConfidentialCreditsIx"|"disableNonConfidentialCreditsIx"|"transferWithSplitProofsIx";
}
