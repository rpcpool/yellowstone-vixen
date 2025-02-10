// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { BurnAccountsProto as _vixen_parser_BurnAccountsProto, BurnAccountsProto__Output as _vixen_parser_BurnAccountsProto__Output } from '../../vixen/parser/BurnAccountsProto';
import type { BurnDataProto as _vixen_parser_BurnDataProto, BurnDataProto__Output as _vixen_parser_BurnDataProto__Output } from '../../vixen/parser/BurnDataProto';

export interface BurnIxProto {
  'accounts'?: (_vixen_parser_BurnAccountsProto | null);
  'data'?: (_vixen_parser_BurnDataProto | null);
}

export interface BurnIxProto__Output {
  'accounts': (_vixen_parser_BurnAccountsProto__Output | null);
  'data': (_vixen_parser_BurnDataProto__Output | null);
}
