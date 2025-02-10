// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { InitializeAccountsProto as _vixen_parser_InitializeAccountsProto, InitializeAccountsProto__Output as _vixen_parser_InitializeAccountsProto__Output } from '../../vixen/parser/InitializeAccountsProto';
import type { InitializeDataProto as _vixen_parser_InitializeDataProto, InitializeDataProto__Output as _vixen_parser_InitializeDataProto__Output } from '../../vixen/parser/InitializeDataProto';

export interface InitializeIxProto {
  'accounts'?: (_vixen_parser_InitializeAccountsProto | null);
  'data'?: (_vixen_parser_InitializeDataProto | null);
}

export interface InitializeIxProto__Output {
  'accounts': (_vixen_parser_InitializeAccountsProto__Output | null);
  'data': (_vixen_parser_InitializeDataProto__Output | null);
}
