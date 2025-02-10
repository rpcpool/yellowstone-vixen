// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { TransferCheckedWithFeeIxProto as _vixen_parser_TransferCheckedWithFeeIxProto, TransferCheckedWithFeeIxProto__Output as _vixen_parser_TransferCheckedWithFeeIxProto__Output } from '../../vixen/parser/TransferCheckedWithFeeIxProto';
import type { InitializeTransferFeeConfigIxProto as _vixen_parser_InitializeTransferFeeConfigIxProto, InitializeTransferFeeConfigIxProto__Output as _vixen_parser_InitializeTransferFeeConfigIxProto__Output } from '../../vixen/parser/InitializeTransferFeeConfigIxProto';
import type { WithdrawWithheldTokensFromMintIxProto as _vixen_parser_WithdrawWithheldTokensFromMintIxProto, WithdrawWithheldTokensFromMintIxProto__Output as _vixen_parser_WithdrawWithheldTokensFromMintIxProto__Output } from '../../vixen/parser/WithdrawWithheldTokensFromMintIxProto';
import type { WithdrawWithheldTokensFromAccountsIxProto as _vixen_parser_WithdrawWithheldTokensFromAccountsIxProto, WithdrawWithheldTokensFromAccountsIxProto__Output as _vixen_parser_WithdrawWithheldTokensFromAccountsIxProto__Output } from '../../vixen/parser/WithdrawWithheldTokensFromAccountsIxProto';
import type { HarvestWithheldTokensToMintIxProto as _vixen_parser_HarvestWithheldTokensToMintIxProto, HarvestWithheldTokensToMintIxProto__Output as _vixen_parser_HarvestWithheldTokensToMintIxProto__Output } from '../../vixen/parser/HarvestWithheldTokensToMintIxProto';
import type { SetTransferFeeIxProto as _vixen_parser_SetTransferFeeIxProto, SetTransferFeeIxProto__Output as _vixen_parser_SetTransferFeeIxProto__Output } from '../../vixen/parser/SetTransferFeeIxProto';

export interface TransferFeeIxProto {
  'transferCheckedWithFeeIx'?: (_vixen_parser_TransferCheckedWithFeeIxProto | null);
  'initializeTransferFeeConfigIx'?: (_vixen_parser_InitializeTransferFeeConfigIxProto | null);
  'withdrawWithheldTokensFromMintIx'?: (_vixen_parser_WithdrawWithheldTokensFromMintIxProto | null);
  'withdrawWithheldTokensFromAccountsIx'?: (_vixen_parser_WithdrawWithheldTokensFromAccountsIxProto | null);
  'harvestWithheldTokensToMintIx'?: (_vixen_parser_HarvestWithheldTokensToMintIxProto | null);
  'setTransferFeeIx'?: (_vixen_parser_SetTransferFeeIxProto | null);
  'ixOneof'?: "transferCheckedWithFeeIx"|"initializeTransferFeeConfigIx"|"withdrawWithheldTokensFromMintIx"|"withdrawWithheldTokensFromAccountsIx"|"harvestWithheldTokensToMintIx"|"setTransferFeeIx";
}

export interface TransferFeeIxProto__Output {
  'transferCheckedWithFeeIx'?: (_vixen_parser_TransferCheckedWithFeeIxProto__Output | null);
  'initializeTransferFeeConfigIx'?: (_vixen_parser_InitializeTransferFeeConfigIxProto__Output | null);
  'withdrawWithheldTokensFromMintIx'?: (_vixen_parser_WithdrawWithheldTokensFromMintIxProto__Output | null);
  'withdrawWithheldTokensFromAccountsIx'?: (_vixen_parser_WithdrawWithheldTokensFromAccountsIxProto__Output | null);
  'harvestWithheldTokensToMintIx'?: (_vixen_parser_HarvestWithheldTokensToMintIxProto__Output | null);
  'setTransferFeeIx'?: (_vixen_parser_SetTransferFeeIxProto__Output | null);
  'ixOneof': "transferCheckedWithFeeIx"|"initializeTransferFeeConfigIx"|"withdrawWithheldTokensFromMintIx"|"withdrawWithheldTokensFromAccountsIx"|"harvestWithheldTokensToMintIx"|"setTransferFeeIx";
}
