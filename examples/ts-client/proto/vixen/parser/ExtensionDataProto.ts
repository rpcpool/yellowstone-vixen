// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { ImmutableOwnerProto as _vixen_parser_ImmutableOwnerProto, ImmutableOwnerProto__Output as _vixen_parser_ImmutableOwnerProto__Output } from '../../vixen/parser/ImmutableOwnerProto';
import type { TransferFeeAmountProto as _vixen_parser_TransferFeeAmountProto, TransferFeeAmountProto__Output as _vixen_parser_TransferFeeAmountProto__Output } from '../../vixen/parser/TransferFeeAmountProto';
import type { ConfidentialTransferAccountProto as _vixen_parser_ConfidentialTransferAccountProto, ConfidentialTransferAccountProto__Output as _vixen_parser_ConfidentialTransferAccountProto__Output } from '../../vixen/parser/ConfidentialTransferAccountProto';
import type { MemoTransferProto as _vixen_parser_MemoTransferProto, MemoTransferProto__Output as _vixen_parser_MemoTransferProto__Output } from '../../vixen/parser/MemoTransferProto';
import type { NonTransferableAccountProto as _vixen_parser_NonTransferableAccountProto, NonTransferableAccountProto__Output as _vixen_parser_NonTransferableAccountProto__Output } from '../../vixen/parser/NonTransferableAccountProto';
import type { TransferHookAccountProto as _vixen_parser_TransferHookAccountProto, TransferHookAccountProto__Output as _vixen_parser_TransferHookAccountProto__Output } from '../../vixen/parser/TransferHookAccountProto';
import type { CpiGuardProto as _vixen_parser_CpiGuardProto, CpiGuardProto__Output as _vixen_parser_CpiGuardProto__Output } from '../../vixen/parser/CpiGuardProto';
import type { ConfidentialTransferFeeAmountProto as _vixen_parser_ConfidentialTransferFeeAmountProto, ConfidentialTransferFeeAmountProto__Output as _vixen_parser_ConfidentialTransferFeeAmountProto__Output } from '../../vixen/parser/ConfidentialTransferFeeAmountProto';
import type { TransferFeeConfigProto as _vixen_parser_TransferFeeConfigProto, TransferFeeConfigProto__Output as _vixen_parser_TransferFeeConfigProto__Output } from '../../vixen/parser/TransferFeeConfigProto';
import type { MintCloseAuthorityProto as _vixen_parser_MintCloseAuthorityProto, MintCloseAuthorityProto__Output as _vixen_parser_MintCloseAuthorityProto__Output } from '../../vixen/parser/MintCloseAuthorityProto';
import type { ConfidentialTransferMintProto as _vixen_parser_ConfidentialTransferMintProto, ConfidentialTransferMintProto__Output as _vixen_parser_ConfidentialTransferMintProto__Output } from '../../vixen/parser/ConfidentialTransferMintProto';
import type { DefaultAccountStateProto as _vixen_parser_DefaultAccountStateProto, DefaultAccountStateProto__Output as _vixen_parser_DefaultAccountStateProto__Output } from '../../vixen/parser/DefaultAccountStateProto';
import type { NonTransferableProto as _vixen_parser_NonTransferableProto, NonTransferableProto__Output as _vixen_parser_NonTransferableProto__Output } from '../../vixen/parser/NonTransferableProto';
import type { InterestBearingConfigProto as _vixen_parser_InterestBearingConfigProto, InterestBearingConfigProto__Output as _vixen_parser_InterestBearingConfigProto__Output } from '../../vixen/parser/InterestBearingConfigProto';
import type { PermanentDelegateProto as _vixen_parser_PermanentDelegateProto, PermanentDelegateProto__Output as _vixen_parser_PermanentDelegateProto__Output } from '../../vixen/parser/PermanentDelegateProto';
import type { TransferHookProto as _vixen_parser_TransferHookProto, TransferHookProto__Output as _vixen_parser_TransferHookProto__Output } from '../../vixen/parser/TransferHookProto';
import type { ConfidentialTransferFeeConfigProto as _vixen_parser_ConfidentialTransferFeeConfigProto, ConfidentialTransferFeeConfigProto__Output as _vixen_parser_ConfidentialTransferFeeConfigProto__Output } from '../../vixen/parser/ConfidentialTransferFeeConfigProto';
import type { MetadataPointerProto as _vixen_parser_MetadataPointerProto, MetadataPointerProto__Output as _vixen_parser_MetadataPointerProto__Output } from '../../vixen/parser/MetadataPointerProto';
import type { TokenMetadataProto as _vixen_parser_TokenMetadataProto, TokenMetadataProto__Output as _vixen_parser_TokenMetadataProto__Output } from '../../vixen/parser/TokenMetadataProto';
import type { GroupPointerProto as _vixen_parser_GroupPointerProto, GroupPointerProto__Output as _vixen_parser_GroupPointerProto__Output } from '../../vixen/parser/GroupPointerProto';
import type { TokenGroupProto as _vixen_parser_TokenGroupProto, TokenGroupProto__Output as _vixen_parser_TokenGroupProto__Output } from '../../vixen/parser/TokenGroupProto';
import type { GroupMemberPointerProto as _vixen_parser_GroupMemberPointerProto, GroupMemberPointerProto__Output as _vixen_parser_GroupMemberPointerProto__Output } from '../../vixen/parser/GroupMemberPointerProto';
import type { TokenGroupMemberProto as _vixen_parser_TokenGroupMemberProto, TokenGroupMemberProto__Output as _vixen_parser_TokenGroupMemberProto__Output } from '../../vixen/parser/TokenGroupMemberProto';

export interface ExtensionDataProto {
  'immutableOwner'?: (_vixen_parser_ImmutableOwnerProto | null);
  'transferFeeAmount'?: (_vixen_parser_TransferFeeAmountProto | null);
  'confidentialTransferAccount'?: (_vixen_parser_ConfidentialTransferAccountProto | null);
  'memoTransfer'?: (_vixen_parser_MemoTransferProto | null);
  'nonTransferableAccount'?: (_vixen_parser_NonTransferableAccountProto | null);
  'transferHookAccount'?: (_vixen_parser_TransferHookAccountProto | null);
  'cpiGuard'?: (_vixen_parser_CpiGuardProto | null);
  'confidentialTransferFeeAmount'?: (_vixen_parser_ConfidentialTransferFeeAmountProto | null);
  'transferFeeConfig'?: (_vixen_parser_TransferFeeConfigProto | null);
  'mintCloseAuthority'?: (_vixen_parser_MintCloseAuthorityProto | null);
  'confidentialTransferMint'?: (_vixen_parser_ConfidentialTransferMintProto | null);
  'defaultAccountState'?: (_vixen_parser_DefaultAccountStateProto | null);
  'nonTransferable'?: (_vixen_parser_NonTransferableProto | null);
  'interestBearingConfig'?: (_vixen_parser_InterestBearingConfigProto | null);
  'permanentDelegate'?: (_vixen_parser_PermanentDelegateProto | null);
  'transferHook'?: (_vixen_parser_TransferHookProto | null);
  'confidentialTransferFeeConfig'?: (_vixen_parser_ConfidentialTransferFeeConfigProto | null);
  'metadataPointer'?: (_vixen_parser_MetadataPointerProto | null);
  'tokenMetadata'?: (_vixen_parser_TokenMetadataProto | null);
  'groupPointer'?: (_vixen_parser_GroupPointerProto | null);
  'tokenGroup'?: (_vixen_parser_TokenGroupProto | null);
  'groupMemberPointer'?: (_vixen_parser_GroupMemberPointerProto | null);
  'tokenGroupMember'?: (_vixen_parser_TokenGroupMemberProto | null);
  'data'?: "immutableOwner"|"transferFeeAmount"|"confidentialTransferAccount"|"memoTransfer"|"nonTransferableAccount"|"transferHookAccount"|"cpiGuard"|"confidentialTransferFeeAmount"|"transferFeeConfig"|"mintCloseAuthority"|"confidentialTransferMint"|"defaultAccountState"|"nonTransferable"|"interestBearingConfig"|"permanentDelegate"|"transferHook"|"confidentialTransferFeeConfig"|"metadataPointer"|"tokenMetadata"|"groupPointer"|"tokenGroup"|"groupMemberPointer"|"tokenGroupMember";
}

export interface ExtensionDataProto__Output {
  'immutableOwner'?: (_vixen_parser_ImmutableOwnerProto__Output | null);
  'transferFeeAmount'?: (_vixen_parser_TransferFeeAmountProto__Output | null);
  'confidentialTransferAccount'?: (_vixen_parser_ConfidentialTransferAccountProto__Output | null);
  'memoTransfer'?: (_vixen_parser_MemoTransferProto__Output | null);
  'nonTransferableAccount'?: (_vixen_parser_NonTransferableAccountProto__Output | null);
  'transferHookAccount'?: (_vixen_parser_TransferHookAccountProto__Output | null);
  'cpiGuard'?: (_vixen_parser_CpiGuardProto__Output | null);
  'confidentialTransferFeeAmount'?: (_vixen_parser_ConfidentialTransferFeeAmountProto__Output | null);
  'transferFeeConfig'?: (_vixen_parser_TransferFeeConfigProto__Output | null);
  'mintCloseAuthority'?: (_vixen_parser_MintCloseAuthorityProto__Output | null);
  'confidentialTransferMint'?: (_vixen_parser_ConfidentialTransferMintProto__Output | null);
  'defaultAccountState'?: (_vixen_parser_DefaultAccountStateProto__Output | null);
  'nonTransferable'?: (_vixen_parser_NonTransferableProto__Output | null);
  'interestBearingConfig'?: (_vixen_parser_InterestBearingConfigProto__Output | null);
  'permanentDelegate'?: (_vixen_parser_PermanentDelegateProto__Output | null);
  'transferHook'?: (_vixen_parser_TransferHookProto__Output | null);
  'confidentialTransferFeeConfig'?: (_vixen_parser_ConfidentialTransferFeeConfigProto__Output | null);
  'metadataPointer'?: (_vixen_parser_MetadataPointerProto__Output | null);
  'tokenMetadata'?: (_vixen_parser_TokenMetadataProto__Output | null);
  'groupPointer'?: (_vixen_parser_GroupPointerProto__Output | null);
  'tokenGroup'?: (_vixen_parser_TokenGroupProto__Output | null);
  'groupMemberPointer'?: (_vixen_parser_GroupMemberPointerProto__Output | null);
  'tokenGroupMember'?: (_vixen_parser_TokenGroupMemberProto__Output | null);
  'data': "immutableOwner"|"transferFeeAmount"|"confidentialTransferAccount"|"memoTransfer"|"nonTransferableAccount"|"transferHookAccount"|"cpiGuard"|"confidentialTransferFeeAmount"|"transferFeeConfig"|"mintCloseAuthority"|"confidentialTransferMint"|"defaultAccountState"|"nonTransferable"|"interestBearingConfig"|"permanentDelegate"|"transferHook"|"confidentialTransferFeeConfig"|"metadataPointer"|"tokenMetadata"|"groupPointer"|"tokenGroup"|"groupMemberPointer"|"tokenGroupMember";
}
