// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { RmoveKeyAccountsProto as _vixen_parser_RmoveKeyAccountsProto, RmoveKeyAccountsProto__Output as _vixen_parser_RmoveKeyAccountsProto__Output } from '../../vixen/parser/RmoveKeyAccountsProto';
import type { RemoveKeyDataProto as _vixen_parser_RemoveKeyDataProto, RemoveKeyDataProto__Output as _vixen_parser_RemoveKeyDataProto__Output } from '../../vixen/parser/RemoveKeyDataProto';

export interface RemoveKeyIxProto {
  'accounts'?: (_vixen_parser_RmoveKeyAccountsProto | null);
  'data'?: (_vixen_parser_RemoveKeyDataProto | null);
}

export interface RemoveKeyIxProto__Output {
  'accounts': (_vixen_parser_RmoveKeyAccountsProto__Output | null);
  'data': (_vixen_parser_RemoveKeyDataProto__Output | null);
}
