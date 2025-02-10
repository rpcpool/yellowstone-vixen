// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { AmountToUiAmountAccountsProto as _vixen_parser_AmountToUiAmountAccountsProto, AmountToUiAmountAccountsProto__Output as _vixen_parser_AmountToUiAmountAccountsProto__Output } from '../../vixen/parser/AmountToUiAmountAccountsProto';
import type { AmountToUiAmountDataProto as _vixen_parser_AmountToUiAmountDataProto, AmountToUiAmountDataProto__Output as _vixen_parser_AmountToUiAmountDataProto__Output } from '../../vixen/parser/AmountToUiAmountDataProto';

export interface AmountToUiAmountIxProto {
  'accounts'?: (_vixen_parser_AmountToUiAmountAccountsProto | null);
  'data'?: (_vixen_parser_AmountToUiAmountDataProto | null);
}

export interface AmountToUiAmountIxProto__Output {
  'accounts': (_vixen_parser_AmountToUiAmountAccountsProto__Output | null);
  'data': (_vixen_parser_AmountToUiAmountDataProto__Output | null);
}
