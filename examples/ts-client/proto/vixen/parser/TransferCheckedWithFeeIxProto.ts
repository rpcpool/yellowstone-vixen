// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { TransferCheckedWithFeeAccountsProto as _vixen_parser_TransferCheckedWithFeeAccountsProto, TransferCheckedWithFeeAccountsProto__Output as _vixen_parser_TransferCheckedWithFeeAccountsProto__Output } from '../../vixen/parser/TransferCheckedWithFeeAccountsProto';
import type { TransferCheckedWithFeeDataProto as _vixen_parser_TransferCheckedWithFeeDataProto, TransferCheckedWithFeeDataProto__Output as _vixen_parser_TransferCheckedWithFeeDataProto__Output } from '../../vixen/parser/TransferCheckedWithFeeDataProto';

export interface TransferCheckedWithFeeIxProto {
  'accounts'?: (_vixen_parser_TransferCheckedWithFeeAccountsProto | null);
  'data'?: (_vixen_parser_TransferCheckedWithFeeDataProto | null);
}

export interface TransferCheckedWithFeeIxProto__Output {
  'accounts': (_vixen_parser_TransferCheckedWithFeeAccountsProto__Output | null);
  'data': (_vixen_parser_TransferCheckedWithFeeDataProto__Output | null);
}
