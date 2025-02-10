// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { TransferCheckedAccountsProto as _vixen_parser_TransferCheckedAccountsProto, TransferCheckedAccountsProto__Output as _vixen_parser_TransferCheckedAccountsProto__Output } from '../../vixen/parser/TransferCheckedAccountsProto';
import type { TransferCheckedDataProto as _vixen_parser_TransferCheckedDataProto, TransferCheckedDataProto__Output as _vixen_parser_TransferCheckedDataProto__Output } from '../../vixen/parser/TransferCheckedDataProto';

export interface TransferCheckedIxProto {
  'accounts'?: (_vixen_parser_TransferCheckedAccountsProto | null);
  'data'?: (_vixen_parser_TransferCheckedDataProto | null);
}

export interface TransferCheckedIxProto__Output {
  'accounts': (_vixen_parser_TransferCheckedAccountsProto__Output | null);
  'data': (_vixen_parser_TransferCheckedDataProto__Output | null);
}
