// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { UpdateFieldAccountsProto as _vixen_parser_UpdateFieldAccountsProto, UpdateFieldAccountsProto__Output as _vixen_parser_UpdateFieldAccountsProto__Output } from '../../vixen/parser/UpdateFieldAccountsProto';
import type { UpdateFieldDataProto as _vixen_parser_UpdateFieldDataProto, UpdateFieldDataProto__Output as _vixen_parser_UpdateFieldDataProto__Output } from '../../vixen/parser/UpdateFieldDataProto';

export interface UpdateFieldIxProto {
  'accounts'?: (_vixen_parser_UpdateFieldAccountsProto | null);
  'data'?: (_vixen_parser_UpdateFieldDataProto | null);
}

export interface UpdateFieldIxProto__Output {
  'accounts': (_vixen_parser_UpdateFieldAccountsProto__Output | null);
  'data': (_vixen_parser_UpdateFieldDataProto__Output | null);
}
