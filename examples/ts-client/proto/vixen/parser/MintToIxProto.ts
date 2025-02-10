// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { MintToAccountsProto as _vixen_parser_MintToAccountsProto, MintToAccountsProto__Output as _vixen_parser_MintToAccountsProto__Output } from '../../vixen/parser/MintToAccountsProto';
import type { MintToDataProto as _vixen_parser_MintToDataProto, MintToDataProto__Output as _vixen_parser_MintToDataProto__Output } from '../../vixen/parser/MintToDataProto';

export interface MintToIxProto {
  'accounts'?: (_vixen_parser_MintToAccountsProto | null);
  'data'?: (_vixen_parser_MintToDataProto | null);
}

export interface MintToIxProto__Output {
  'accounts': (_vixen_parser_MintToAccountsProto__Output | null);
  'data': (_vixen_parser_MintToDataProto__Output | null);
}
