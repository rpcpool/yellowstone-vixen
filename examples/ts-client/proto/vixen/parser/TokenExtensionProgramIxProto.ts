// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/parser.proto

import type { TokenProgramIxProto as _vixen_parser_TokenProgramIxProto, TokenProgramIxProto__Output as _vixen_parser_TokenProgramIxProto__Output } from '../../vixen/parser/TokenProgramIxProto';
import type { TransferFeeIxProto as _vixen_parser_TransferFeeIxProto, TransferFeeIxProto__Output as _vixen_parser_TransferFeeIxProto__Output } from '../../vixen/parser/TransferFeeIxProto';
import type { TokenMetadataIxProto as _vixen_parser_TokenMetadataIxProto, TokenMetadataIxProto__Output as _vixen_parser_TokenMetadataIxProto__Output } from '../../vixen/parser/TokenMetadataIxProto';
import type { TokenGroupIxProto as _vixen_parser_TokenGroupIxProto, TokenGroupIxProto__Output as _vixen_parser_TokenGroupIxProto__Output } from '../../vixen/parser/TokenGroupIxProto';
import type { ConfidentialTransferExtIxProto as _vixen_parser_ConfidentialTransferExtIxProto, ConfidentialTransferExtIxProto__Output as _vixen_parser_ConfidentialTransferExtIxProto__Output } from '../../vixen/parser/ConfidentialTransferExtIxProto';
import type { ConfidentialTransferFeeIxProto as _vixen_parser_ConfidentialTransferFeeIxProto, ConfidentialTransferFeeIxProto__Output as _vixen_parser_ConfidentialTransferFeeIxProto__Output } from '../../vixen/parser/ConfidentialTransferFeeIxProto';
import type { CpiGuardIxProto as _vixen_parser_CpiGuardIxProto, CpiGuardIxProto__Output as _vixen_parser_CpiGuardIxProto__Output } from '../../vixen/parser/CpiGuardIxProto';
import type { TransferHookIxProto as _vixen_parser_TransferHookIxProto, TransferHookIxProto__Output as _vixen_parser_TransferHookIxProto__Output } from '../../vixen/parser/TransferHookIxProto';
import type { MetadataPointerIxProto as _vixen_parser_MetadataPointerIxProto, MetadataPointerIxProto__Output as _vixen_parser_MetadataPointerIxProto__Output } from '../../vixen/parser/MetadataPointerIxProto';
import type { MemoTransferIxProto as _vixen_parser_MemoTransferIxProto, MemoTransferIxProto__Output as _vixen_parser_MemoTransferIxProto__Output } from '../../vixen/parser/MemoTransferIxProto';
import type { InterestBearingMintIxProto as _vixen_parser_InterestBearingMintIxProto, InterestBearingMintIxProto__Output as _vixen_parser_InterestBearingMintIxProto__Output } from '../../vixen/parser/InterestBearingMintIxProto';
import type { DefaultAccountStateIxProto as _vixen_parser_DefaultAccountStateIxProto, DefaultAccountStateIxProto__Output as _vixen_parser_DefaultAccountStateIxProto__Output } from '../../vixen/parser/DefaultAccountStateIxProto';
import type { GroupMemberPointerIxProto as _vixen_parser_GroupMemberPointerIxProto, GroupMemberPointerIxProto__Output as _vixen_parser_GroupMemberPointerIxProto__Output } from '../../vixen/parser/GroupMemberPointerIxProto';
import type { GroupPointerIxProto as _vixen_parser_GroupPointerIxProto, GroupPointerIxProto__Output as _vixen_parser_GroupPointerIxProto__Output } from '../../vixen/parser/GroupPointerIxProto';
import type { WithdrawExcessLamportsIxProto as _vixen_parser_WithdrawExcessLamportsIxProto, WithdrawExcessLamportsIxProto__Output as _vixen_parser_WithdrawExcessLamportsIxProto__Output } from '../../vixen/parser/WithdrawExcessLamportsIxProto';
import type { InitializePermanentDelegateIxProto as _vixen_parser_InitializePermanentDelegateIxProto, InitializePermanentDelegateIxProto__Output as _vixen_parser_InitializePermanentDelegateIxProto__Output } from '../../vixen/parser/InitializePermanentDelegateIxProto';
import type { ReallocateIxProto as _vixen_parser_ReallocateIxProto, ReallocateIxProto__Output as _vixen_parser_ReallocateIxProto__Output } from '../../vixen/parser/ReallocateIxProto';
import type { InitializeNonTransferableMintIxProto as _vixen_parser_InitializeNonTransferableMintIxProto, InitializeNonTransferableMintIxProto__Output as _vixen_parser_InitializeNonTransferableMintIxProto__Output } from '../../vixen/parser/InitializeNonTransferableMintIxProto';
import type { InitializeMintCloseAuthorityIxProto as _vixen_parser_InitializeMintCloseAuthorityIxProto, InitializeMintCloseAuthorityIxProto__Output as _vixen_parser_InitializeMintCloseAuthorityIxProto__Output } from '../../vixen/parser/InitializeMintCloseAuthorityIxProto';
import type { CreateNativeMintIxProto as _vixen_parser_CreateNativeMintIxProto, CreateNativeMintIxProto__Output as _vixen_parser_CreateNativeMintIxProto__Output } from '../../vixen/parser/CreateNativeMintIxProto';
import type { SetAuthorityIxProto as _vixen_parser_SetAuthorityIxProto, SetAuthorityIxProto__Output as _vixen_parser_SetAuthorityIxProto__Output } from '../../vixen/parser/SetAuthorityIxProto';

export interface TokenExtensionProgramIxProto {
  'tokenProgramIx'?: (_vixen_parser_TokenProgramIxProto | null);
  'transferFeeIx'?: (_vixen_parser_TransferFeeIxProto | null);
  'tokenMetadataIx'?: (_vixen_parser_TokenMetadataIxProto | null);
  'tokenGroupIx'?: (_vixen_parser_TokenGroupIxProto | null);
  'confidentialTransferIx'?: (_vixen_parser_ConfidentialTransferExtIxProto | null);
  'confidentialTransferFeeIx'?: (_vixen_parser_ConfidentialTransferFeeIxProto | null);
  'cpiGuardIx'?: (_vixen_parser_CpiGuardIxProto | null);
  'transferHookIx'?: (_vixen_parser_TransferHookIxProto | null);
  'metadataPointerIx'?: (_vixen_parser_MetadataPointerIxProto | null);
  'memoTransferIx'?: (_vixen_parser_MemoTransferIxProto | null);
  'interestBearingMintIx'?: (_vixen_parser_InterestBearingMintIxProto | null);
  'defaultAccountStateIx'?: (_vixen_parser_DefaultAccountStateIxProto | null);
  'groupMemberPointerIx'?: (_vixen_parser_GroupMemberPointerIxProto | null);
  'groupPointerIx'?: (_vixen_parser_GroupPointerIxProto | null);
  'withdrawExcessLamportsIx'?: (_vixen_parser_WithdrawExcessLamportsIxProto | null);
  'initializePermanentDelegateIx'?: (_vixen_parser_InitializePermanentDelegateIxProto | null);
  'reallocateIx'?: (_vixen_parser_ReallocateIxProto | null);
  'initializeNonTransferableMintIx'?: (_vixen_parser_InitializeNonTransferableMintIxProto | null);
  'initializeMintCloseAuthorityIx'?: (_vixen_parser_InitializeMintCloseAuthorityIxProto | null);
  'createNativeMintIx'?: (_vixen_parser_CreateNativeMintIxProto | null);
  'setAuthority'?: (_vixen_parser_SetAuthorityIxProto | null);
  'ixOneof'?: "tokenProgramIx"|"transferFeeIx"|"tokenMetadataIx"|"tokenGroupIx"|"confidentialTransferIx"|"confidentialTransferFeeIx"|"cpiGuardIx"|"transferHookIx"|"metadataPointerIx"|"memoTransferIx"|"interestBearingMintIx"|"defaultAccountStateIx"|"groupMemberPointerIx"|"groupPointerIx"|"withdrawExcessLamportsIx"|"initializePermanentDelegateIx"|"reallocateIx"|"initializeNonTransferableMintIx"|"initializeMintCloseAuthorityIx"|"createNativeMintIx"|"setAuthority";
}

export interface TokenExtensionProgramIxProto__Output {
  'tokenProgramIx'?: (_vixen_parser_TokenProgramIxProto__Output | null);
  'transferFeeIx'?: (_vixen_parser_TransferFeeIxProto__Output | null);
  'tokenMetadataIx'?: (_vixen_parser_TokenMetadataIxProto__Output | null);
  'tokenGroupIx'?: (_vixen_parser_TokenGroupIxProto__Output | null);
  'confidentialTransferIx'?: (_vixen_parser_ConfidentialTransferExtIxProto__Output | null);
  'confidentialTransferFeeIx'?: (_vixen_parser_ConfidentialTransferFeeIxProto__Output | null);
  'cpiGuardIx'?: (_vixen_parser_CpiGuardIxProto__Output | null);
  'transferHookIx'?: (_vixen_parser_TransferHookIxProto__Output | null);
  'metadataPointerIx'?: (_vixen_parser_MetadataPointerIxProto__Output | null);
  'memoTransferIx'?: (_vixen_parser_MemoTransferIxProto__Output | null);
  'interestBearingMintIx'?: (_vixen_parser_InterestBearingMintIxProto__Output | null);
  'defaultAccountStateIx'?: (_vixen_parser_DefaultAccountStateIxProto__Output | null);
  'groupMemberPointerIx'?: (_vixen_parser_GroupMemberPointerIxProto__Output | null);
  'groupPointerIx'?: (_vixen_parser_GroupPointerIxProto__Output | null);
  'withdrawExcessLamportsIx'?: (_vixen_parser_WithdrawExcessLamportsIxProto__Output | null);
  'initializePermanentDelegateIx'?: (_vixen_parser_InitializePermanentDelegateIxProto__Output | null);
  'reallocateIx'?: (_vixen_parser_ReallocateIxProto__Output | null);
  'initializeNonTransferableMintIx'?: (_vixen_parser_InitializeNonTransferableMintIxProto__Output | null);
  'initializeMintCloseAuthorityIx'?: (_vixen_parser_InitializeMintCloseAuthorityIxProto__Output | null);
  'createNativeMintIx'?: (_vixen_parser_CreateNativeMintIxProto__Output | null);
  'setAuthority'?: (_vixen_parser_SetAuthorityIxProto__Output | null);
  'ixOneof': "tokenProgramIx"|"transferFeeIx"|"tokenMetadataIx"|"tokenGroupIx"|"confidentialTransferIx"|"confidentialTransferFeeIx"|"cpiGuardIx"|"transferHookIx"|"metadataPointerIx"|"memoTransferIx"|"interestBearingMintIx"|"defaultAccountStateIx"|"groupMemberPointerIx"|"groupPointerIx"|"withdrawExcessLamportsIx"|"initializePermanentDelegateIx"|"reallocateIx"|"initializeNonTransferableMintIx"|"initializeMintCloseAuthorityIx"|"createNativeMintIx"|"setAuthority";
}
