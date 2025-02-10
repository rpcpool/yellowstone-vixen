// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { InitializeAccount2AccountsProto as _vixen_parser_InitializeAccount2AccountsProto, InitializeAccount2AccountsProto__Output as _vixen_parser_InitializeAccount2AccountsProto__Output } from '../../vixen/parser/InitializeAccount2AccountsProto';
import type { InitializeAccountData2Proto as _vixen_parser_InitializeAccountData2Proto, InitializeAccountData2Proto__Output as _vixen_parser_InitializeAccountData2Proto__Output } from '../../vixen/parser/InitializeAccountData2Proto';

export interface InitializeAccount2IxProto {
  'accounts'?: (_vixen_parser_InitializeAccount2AccountsProto | null);
  'data'?: (_vixen_parser_InitializeAccountData2Proto | null);
}

export interface InitializeAccount2IxProto__Output {
  'accounts': (_vixen_parser_InitializeAccount2AccountsProto__Output | null);
  'data': (_vixen_parser_InitializeAccountData2Proto__Output | null);
}
