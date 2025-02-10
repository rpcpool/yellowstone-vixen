// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { TransferAccountsProto as _vixen_parser_TransferAccountsProto, TransferAccountsProto__Output as _vixen_parser_TransferAccountsProto__Output } from '../../vixen/parser/TransferAccountsProto';
import type { TransferDataProto as _vixen_parser_TransferDataProto, TransferDataProto__Output as _vixen_parser_TransferDataProto__Output } from '../../vixen/parser/TransferDataProto';

export interface TransferIxProto {
  'accounts'?: (_vixen_parser_TransferAccountsProto | null);
  'data'?: (_vixen_parser_TransferDataProto | null);
}

export interface TransferIxProto__Output {
  'accounts': (_vixen_parser_TransferAccountsProto__Output | null);
  'data': (_vixen_parser_TransferDataProto__Output | null);
}
