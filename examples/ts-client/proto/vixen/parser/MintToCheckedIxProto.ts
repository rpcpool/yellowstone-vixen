// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { MintToCheckedAccountsProto as _vixen_parser_MintToCheckedAccountsProto, MintToCheckedAccountsProto__Output as _vixen_parser_MintToCheckedAccountsProto__Output } from '../../vixen/parser/MintToCheckedAccountsProto';
import type { MintToCheckedDataProto as _vixen_parser_MintToCheckedDataProto, MintToCheckedDataProto__Output as _vixen_parser_MintToCheckedDataProto__Output } from '../../vixen/parser/MintToCheckedDataProto';

export interface MintToCheckedIxProto {
  'accounts'?: (_vixen_parser_MintToCheckedAccountsProto | null);
  'data'?: (_vixen_parser_MintToCheckedDataProto | null);
}

export interface MintToCheckedIxProto__Output {
  'accounts': (_vixen_parser_MintToCheckedAccountsProto__Output | null);
  'data': (_vixen_parser_MintToCheckedDataProto__Output | null);
}
