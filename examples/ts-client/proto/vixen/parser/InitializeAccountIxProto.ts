// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { InitializeAccountAccountsProto as _vixen_parser_InitializeAccountAccountsProto, InitializeAccountAccountsProto__Output as _vixen_parser_InitializeAccountAccountsProto__Output } from '../../vixen/parser/InitializeAccountAccountsProto';
import type { InitializeAccountDataProto as _vixen_parser_InitializeAccountDataProto, InitializeAccountDataProto__Output as _vixen_parser_InitializeAccountDataProto__Output } from '../../vixen/parser/InitializeAccountDataProto';

export interface InitializeAccountIxProto {
  'accounts'?: (_vixen_parser_InitializeAccountAccountsProto | null);
  'data'?: (_vixen_parser_InitializeAccountDataProto | null);
}

export interface InitializeAccountIxProto__Output {
  'accounts': (_vixen_parser_InitializeAccountAccountsProto__Output | null);
  'data': (_vixen_parser_InitializeAccountDataProto__Output | null);
}
