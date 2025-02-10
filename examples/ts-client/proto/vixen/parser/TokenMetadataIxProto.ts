// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { InitializeIxProto as _vixen_parser_InitializeIxProto, InitializeIxProto__Output as _vixen_parser_InitializeIxProto__Output } from '../../vixen/parser/InitializeIxProto';
import type { UpdateFieldIxProto as _vixen_parser_UpdateFieldIxProto, UpdateFieldIxProto__Output as _vixen_parser_UpdateFieldIxProto__Output } from '../../vixen/parser/UpdateFieldIxProto';
import type { RemoveKeyIxProto as _vixen_parser_RemoveKeyIxProto, RemoveKeyIxProto__Output as _vixen_parser_RemoveKeyIxProto__Output } from '../../vixen/parser/RemoveKeyIxProto';
import type { UpdateAuthorityIxProto as _vixen_parser_UpdateAuthorityIxProto, UpdateAuthorityIxProto__Output as _vixen_parser_UpdateAuthorityIxProto__Output } from '../../vixen/parser/UpdateAuthorityIxProto';
import type { EmitIxProto as _vixen_parser_EmitIxProto, EmitIxProto__Output as _vixen_parser_EmitIxProto__Output } from '../../vixen/parser/EmitIxProto';

export interface TokenMetadataIxProto {
  'initializeIx'?: (_vixen_parser_InitializeIxProto | null);
  'updateFieldsIx'?: (_vixen_parser_UpdateFieldIxProto | null);
  'removeKeyIx'?: (_vixen_parser_RemoveKeyIxProto | null);
  'updateAuthorityIx'?: (_vixen_parser_UpdateAuthorityIxProto | null);
  'emitIx'?: (_vixen_parser_EmitIxProto | null);
  'ixOneof'?: "initializeIx"|"updateFieldsIx"|"removeKeyIx"|"updateAuthorityIx"|"emitIx";
}

export interface TokenMetadataIxProto__Output {
  'initializeIx'?: (_vixen_parser_InitializeIxProto__Output | null);
  'updateFieldsIx'?: (_vixen_parser_UpdateFieldIxProto__Output | null);
  'removeKeyIx'?: (_vixen_parser_RemoveKeyIxProto__Output | null);
  'updateAuthorityIx'?: (_vixen_parser_UpdateAuthorityIxProto__Output | null);
  'emitIx'?: (_vixen_parser_EmitIxProto__Output | null);
  'ixOneof': "initializeIx"|"updateFieldsIx"|"removeKeyIx"|"updateAuthorityIx"|"emitIx";
}
