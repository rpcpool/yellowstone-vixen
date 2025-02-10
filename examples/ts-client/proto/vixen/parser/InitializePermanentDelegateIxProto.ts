// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { InitializePermanentDelegateAccountsProto as _vixen_parser_InitializePermanentDelegateAccountsProto, InitializePermanentDelegateAccountsProto__Output as _vixen_parser_InitializePermanentDelegateAccountsProto__Output } from '../../vixen/parser/InitializePermanentDelegateAccountsProto';
import type { InitializePermanentDelegateDataProto as _vixen_parser_InitializePermanentDelegateDataProto, InitializePermanentDelegateDataProto__Output as _vixen_parser_InitializePermanentDelegateDataProto__Output } from '../../vixen/parser/InitializePermanentDelegateDataProto';

export interface InitializePermanentDelegateIxProto {
  'accounts'?: (_vixen_parser_InitializePermanentDelegateAccountsProto | null);
  'data'?: (_vixen_parser_InitializePermanentDelegateDataProto | null);
}

export interface InitializePermanentDelegateIxProto__Output {
  'accounts': (_vixen_parser_InitializePermanentDelegateAccountsProto__Output | null);
  'data': (_vixen_parser_InitializePermanentDelegateDataProto__Output | null);
}
