// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { ExtInitializeIxProto as _vixen_parser_ExtInitializeIxProto, ExtInitializeIxProto__Output as _vixen_parser_ExtInitializeIxProto__Output } from '../../vixen/parser/ExtInitializeIxProto';
import type { UpdateIxProto as _vixen_parser_UpdateIxProto, UpdateIxProto__Output as _vixen_parser_UpdateIxProto__Output } from '../../vixen/parser/UpdateIxProto';
import type { EnableIxProto as _vixen_parser_EnableIxProto, EnableIxProto__Output as _vixen_parser_EnableIxProto__Output } from '../../vixen/parser/EnableIxProto';
import type { DisableIxProto as _vixen_parser_DisableIxProto, DisableIxProto__Output as _vixen_parser_DisableIxProto__Output } from '../../vixen/parser/DisableIxProto';

export interface CommonExtensionIxProto {
  'extInitializeIx'?: (_vixen_parser_ExtInitializeIxProto | null);
  'updateIx'?: (_vixen_parser_UpdateIxProto | null);
  'enableIx'?: (_vixen_parser_EnableIxProto | null);
  'disableIx'?: (_vixen_parser_DisableIxProto | null);
  'ixOneof'?: "extInitializeIx"|"updateIx"|"enableIx"|"disableIx";
}

export interface CommonExtensionIxProto__Output {
  'extInitializeIx'?: (_vixen_parser_ExtInitializeIxProto__Output | null);
  'updateIx'?: (_vixen_parser_UpdateIxProto__Output | null);
  'enableIx'?: (_vixen_parser_EnableIxProto__Output | null);
  'disableIx'?: (_vixen_parser_DisableIxProto__Output | null);
  'ixOneof': "extInitializeIx"|"updateIx"|"enableIx"|"disableIx";
}
