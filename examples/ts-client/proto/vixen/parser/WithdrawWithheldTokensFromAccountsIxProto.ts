// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { WithdrawWithheldTokensFromAccountsAccountsProto as _vixen_parser_WithdrawWithheldTokensFromAccountsAccountsProto, WithdrawWithheldTokensFromAccountsAccountsProto__Output as _vixen_parser_WithdrawWithheldTokensFromAccountsAccountsProto__Output } from '../../vixen/parser/WithdrawWithheldTokensFromAccountsAccountsProto';
import type { WithdrawWithheldTokensFromAccountsDataProto as _vixen_parser_WithdrawWithheldTokensFromAccountsDataProto, WithdrawWithheldTokensFromAccountsDataProto__Output as _vixen_parser_WithdrawWithheldTokensFromAccountsDataProto__Output } from '../../vixen/parser/WithdrawWithheldTokensFromAccountsDataProto';

export interface WithdrawWithheldTokensFromAccountsIxProto {
  'accounts'?: (_vixen_parser_WithdrawWithheldTokensFromAccountsAccountsProto | null);
  'data'?: (_vixen_parser_WithdrawWithheldTokensFromAccountsDataProto | null);
}

export interface WithdrawWithheldTokensFromAccountsIxProto__Output {
  'accounts': (_vixen_parser_WithdrawWithheldTokensFromAccountsAccountsProto__Output | null);
  'data': (_vixen_parser_WithdrawWithheldTokensFromAccountsDataProto__Output | null);
}
