// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { BurnCheckedAccountsProto as _vixen_parser_BurnCheckedAccountsProto, BurnCheckedAccountsProto__Output as _vixen_parser_BurnCheckedAccountsProto__Output } from '../../vixen/parser/BurnCheckedAccountsProto';
import type { BurnCheckedDataProto as _vixen_parser_BurnCheckedDataProto, BurnCheckedDataProto__Output as _vixen_parser_BurnCheckedDataProto__Output } from '../../vixen/parser/BurnCheckedDataProto';

export interface BurnCheckedIxProto {
  'accounts'?: (_vixen_parser_BurnCheckedAccountsProto | null);
  'data'?: (_vixen_parser_BurnCheckedDataProto | null);
}

export interface BurnCheckedIxProto__Output {
  'accounts': (_vixen_parser_BurnCheckedAccountsProto__Output | null);
  'data': (_vixen_parser_BurnCheckedDataProto__Output | null);
}
