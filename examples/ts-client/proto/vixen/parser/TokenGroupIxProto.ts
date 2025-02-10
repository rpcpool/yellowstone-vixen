// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { InitializeGroupIxProto as _vixen_parser_InitializeGroupIxProto, InitializeGroupIxProto__Output as _vixen_parser_InitializeGroupIxProto__Output } from '../../vixen/parser/InitializeGroupIxProto';
import type { UpdateGroupMaxSizeIxProto as _vixen_parser_UpdateGroupMaxSizeIxProto, UpdateGroupMaxSizeIxProto__Output as _vixen_parser_UpdateGroupMaxSizeIxProto__Output } from '../../vixen/parser/UpdateGroupMaxSizeIxProto';
import type { UpdateGroupAuthorityIxProto as _vixen_parser_UpdateGroupAuthorityIxProto, UpdateGroupAuthorityIxProto__Output as _vixen_parser_UpdateGroupAuthorityIxProto__Output } from '../../vixen/parser/UpdateGroupAuthorityIxProto';
import type { InitializeMemberIxProto as _vixen_parser_InitializeMemberIxProto, InitializeMemberIxProto__Output as _vixen_parser_InitializeMemberIxProto__Output } from '../../vixen/parser/InitializeMemberIxProto';

export interface TokenGroupIxProto {
  'initializeGroupIx'?: (_vixen_parser_InitializeGroupIxProto | null);
  'updateGroupMaxSizeIx'?: (_vixen_parser_UpdateGroupMaxSizeIxProto | null);
  'updateGroupAuthorityIx'?: (_vixen_parser_UpdateGroupAuthorityIxProto | null);
  'initializeMemberIx'?: (_vixen_parser_InitializeMemberIxProto | null);
  'ixOneof'?: "initializeGroupIx"|"updateGroupMaxSizeIx"|"updateGroupAuthorityIx"|"initializeMemberIx";
}

export interface TokenGroupIxProto__Output {
  'initializeGroupIx'?: (_vixen_parser_InitializeGroupIxProto__Output | null);
  'updateGroupMaxSizeIx'?: (_vixen_parser_UpdateGroupMaxSizeIxProto__Output | null);
  'updateGroupAuthorityIx'?: (_vixen_parser_UpdateGroupAuthorityIxProto__Output | null);
  'initializeMemberIx'?: (_vixen_parser_InitializeMemberIxProto__Output | null);
  'ixOneof': "initializeGroupIx"|"updateGroupMaxSizeIx"|"updateGroupAuthorityIx"|"initializeMemberIx";
}
