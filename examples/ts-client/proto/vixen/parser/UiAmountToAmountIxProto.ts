// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { UiAmountToAmountAccountsProto as _vixen_parser_UiAmountToAmountAccountsProto, UiAmountToAmountAccountsProto__Output as _vixen_parser_UiAmountToAmountAccountsProto__Output } from '../../vixen/parser/UiAmountToAmountAccountsProto';
import type { UiAmountToAmountDataProto as _vixen_parser_UiAmountToAmountDataProto, UiAmountToAmountDataProto__Output as _vixen_parser_UiAmountToAmountDataProto__Output } from '../../vixen/parser/UiAmountToAmountDataProto';

export interface UiAmountToAmountIxProto {
  'accounts'?: (_vixen_parser_UiAmountToAmountAccountsProto | null);
  'data'?: (_vixen_parser_UiAmountToAmountDataProto | null);
}

export interface UiAmountToAmountIxProto__Output {
  'accounts': (_vixen_parser_UiAmountToAmountAccountsProto__Output | null);
  'data': (_vixen_parser_UiAmountToAmountDataProto__Output | null);
}
