// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { InitializeGroupAccountsProto as _vixen_parser_InitializeGroupAccountsProto, InitializeGroupAccountsProto__Output as _vixen_parser_InitializeGroupAccountsProto__Output } from '../../vixen/parser/InitializeGroupAccountsProto';
import type { InitializeGroupDataProto as _vixen_parser_InitializeGroupDataProto, InitializeGroupDataProto__Output as _vixen_parser_InitializeGroupDataProto__Output } from '../../vixen/parser/InitializeGroupDataProto';

export interface InitializeGroupIxProto {
  'accounts'?: (_vixen_parser_InitializeGroupAccountsProto | null);
  'data'?: (_vixen_parser_InitializeGroupDataProto | null);
}

export interface InitializeGroupIxProto__Output {
  'accounts': (_vixen_parser_InitializeGroupAccountsProto__Output | null);
  'data': (_vixen_parser_InitializeGroupDataProto__Output | null);
}
