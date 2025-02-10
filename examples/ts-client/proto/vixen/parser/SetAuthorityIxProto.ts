// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { SetAuthorityAccountsProto as _vixen_parser_SetAuthorityAccountsProto, SetAuthorityAccountsProto__Output as _vixen_parser_SetAuthorityAccountsProto__Output } from '../../vixen/parser/SetAuthorityAccountsProto';
import type { SetAuthorityDataProto as _vixen_parser_SetAuthorityDataProto, SetAuthorityDataProto__Output as _vixen_parser_SetAuthorityDataProto__Output } from '../../vixen/parser/SetAuthorityDataProto';

export interface SetAuthorityIxProto {
  'accounts'?: (_vixen_parser_SetAuthorityAccountsProto | null);
  'data'?: (_vixen_parser_SetAuthorityDataProto | null);
}

export interface SetAuthorityIxProto__Output {
  'accounts': (_vixen_parser_SetAuthorityAccountsProto__Output | null);
  'data': (_vixen_parser_SetAuthorityDataProto__Output | null);
}
