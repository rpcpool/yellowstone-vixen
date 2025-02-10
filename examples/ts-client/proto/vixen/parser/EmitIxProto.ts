// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { EmitAccountsProto as _vixen_parser_EmitAccountsProto, EmitAccountsProto__Output as _vixen_parser_EmitAccountsProto__Output } from '../../vixen/parser/EmitAccountsProto';
import type { EmitDataProto as _vixen_parser_EmitDataProto, EmitDataProto__Output as _vixen_parser_EmitDataProto__Output } from '../../vixen/parser/EmitDataProto';

export interface EmitIxProto {
  'accounts'?: (_vixen_parser_EmitAccountsProto | null);
  'data'?: (_vixen_parser_EmitDataProto | null);
}

export interface EmitIxProto__Output {
  'accounts': (_vixen_parser_EmitAccountsProto__Output | null);
  'data': (_vixen_parser_EmitDataProto__Output | null);
}
