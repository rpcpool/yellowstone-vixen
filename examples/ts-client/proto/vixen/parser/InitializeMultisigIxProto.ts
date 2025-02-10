// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { InitializeMultisigAccountsProto as _vixen_parser_InitializeMultisigAccountsProto, InitializeMultisigAccountsProto__Output as _vixen_parser_InitializeMultisigAccountsProto__Output } from '../../vixen/parser/InitializeMultisigAccountsProto';
import type { InitializeMultisigDataProto as _vixen_parser_InitializeMultisigDataProto, InitializeMultisigDataProto__Output as _vixen_parser_InitializeMultisigDataProto__Output } from '../../vixen/parser/InitializeMultisigDataProto';

export interface InitializeMultisigIxProto {
  'accounts'?: (_vixen_parser_InitializeMultisigAccountsProto | null);
  'data'?: (_vixen_parser_InitializeMultisigDataProto | null);
}

export interface InitializeMultisigIxProto__Output {
  'accounts': (_vixen_parser_InitializeMultisigAccountsProto__Output | null);
  'data': (_vixen_parser_InitializeMultisigDataProto__Output | null);
}
