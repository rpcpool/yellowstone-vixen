// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { InitializeConfidentialTransferFeeConfigIxProto as _vixen_parser_InitializeConfidentialTransferFeeConfigIxProto, InitializeConfidentialTransferFeeConfigIxProto__Output as _vixen_parser_InitializeConfidentialTransferFeeConfigIxProto__Output } from '../../vixen/parser/InitializeConfidentialTransferFeeConfigIxProto';
import type { ConfidentialWithdrawWithheldTokensFromMintIxProto as _vixen_parser_ConfidentialWithdrawWithheldTokensFromMintIxProto, ConfidentialWithdrawWithheldTokensFromMintIxProto__Output as _vixen_parser_ConfidentialWithdrawWithheldTokensFromMintIxProto__Output } from '../../vixen/parser/ConfidentialWithdrawWithheldTokensFromMintIxProto';
import type { ConfidentialWithdrawWithheldTokensFromAccountsIxProto as _vixen_parser_ConfidentialWithdrawWithheldTokensFromAccountsIxProto, ConfidentialWithdrawWithheldTokensFromAccountsIxProto__Output as _vixen_parser_ConfidentialWithdrawWithheldTokensFromAccountsIxProto__Output } from '../../vixen/parser/ConfidentialWithdrawWithheldTokensFromAccountsIxProto';
import type { ConfidentialHarvestWithheldTokensToMintIxProto as _vixen_parser_ConfidentialHarvestWithheldTokensToMintIxProto, ConfidentialHarvestWithheldTokensToMintIxProto__Output as _vixen_parser_ConfidentialHarvestWithheldTokensToMintIxProto__Output } from '../../vixen/parser/ConfidentialHarvestWithheldTokensToMintIxProto';
import type { EnableHarvestToMintIxProto as _vixen_parser_EnableHarvestToMintIxProto, EnableHarvestToMintIxProto__Output as _vixen_parser_EnableHarvestToMintIxProto__Output } from '../../vixen/parser/EnableHarvestToMintIxProto';
import type { DisableHarvestToMintIxProto as _vixen_parser_DisableHarvestToMintIxProto, DisableHarvestToMintIxProto__Output as _vixen_parser_DisableHarvestToMintIxProto__Output } from '../../vixen/parser/DisableHarvestToMintIxProto';

export interface ConfidentialTransferFeeIxProto {
  'initializeConfidentialTransferFeeConfigIx'?: (_vixen_parser_InitializeConfidentialTransferFeeConfigIxProto | null);
  'withdrawWithheldTokensFromMintIx'?: (_vixen_parser_ConfidentialWithdrawWithheldTokensFromMintIxProto | null);
  'withdrawWithheldTokensFromAccountsIx'?: (_vixen_parser_ConfidentialWithdrawWithheldTokensFromAccountsIxProto | null);
  'harvestWithheldTokensToMintIx'?: (_vixen_parser_ConfidentialHarvestWithheldTokensToMintIxProto | null);
  'enableHarvestToMintIx'?: (_vixen_parser_EnableHarvestToMintIxProto | null);
  'disableHarvestToMintIx'?: (_vixen_parser_DisableHarvestToMintIxProto | null);
  'ixOneof'?: "initializeConfidentialTransferFeeConfigIx"|"withdrawWithheldTokensFromMintIx"|"withdrawWithheldTokensFromAccountsIx"|"harvestWithheldTokensToMintIx"|"enableHarvestToMintIx"|"disableHarvestToMintIx";
}

export interface ConfidentialTransferFeeIxProto__Output {
  'initializeConfidentialTransferFeeConfigIx'?: (_vixen_parser_InitializeConfidentialTransferFeeConfigIxProto__Output | null);
  'withdrawWithheldTokensFromMintIx'?: (_vixen_parser_ConfidentialWithdrawWithheldTokensFromMintIxProto__Output | null);
  'withdrawWithheldTokensFromAccountsIx'?: (_vixen_parser_ConfidentialWithdrawWithheldTokensFromAccountsIxProto__Output | null);
  'harvestWithheldTokensToMintIx'?: (_vixen_parser_ConfidentialHarvestWithheldTokensToMintIxProto__Output | null);
  'enableHarvestToMintIx'?: (_vixen_parser_EnableHarvestToMintIxProto__Output | null);
  'disableHarvestToMintIx'?: (_vixen_parser_DisableHarvestToMintIxProto__Output | null);
  'ixOneof': "initializeConfidentialTransferFeeConfigIx"|"withdrawWithheldTokensFromMintIx"|"withdrawWithheldTokensFromAccountsIx"|"harvestWithheldTokensToMintIx"|"enableHarvestToMintIx"|"disableHarvestToMintIx";
}
