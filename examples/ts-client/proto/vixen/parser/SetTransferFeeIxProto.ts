// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { SetTransferFeeAccountsProto as _vixen_parser_SetTransferFeeAccountsProto, SetTransferFeeAccountsProto__Output as _vixen_parser_SetTransferFeeAccountsProto__Output } from '../../vixen/parser/SetTransferFeeAccountsProto';
import type { SetTransferFeeDataProto as _vixen_parser_SetTransferFeeDataProto, SetTransferFeeDataProto__Output as _vixen_parser_SetTransferFeeDataProto__Output } from '../../vixen/parser/SetTransferFeeDataProto';

export interface SetTransferFeeIxProto {
  'accounts'?: (_vixen_parser_SetTransferFeeAccountsProto | null);
  'data'?: (_vixen_parser_SetTransferFeeDataProto | null);
}

export interface SetTransferFeeIxProto__Output {
  'accounts': (_vixen_parser_SetTransferFeeAccountsProto__Output | null);
  'data': (_vixen_parser_SetTransferFeeDataProto__Output | null);
}
