// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { TransferFeeProto as _vixen_parser_TransferFeeProto, TransferFeeProto__Output as _vixen_parser_TransferFeeProto__Output } from '../../vixen/parser/TransferFeeProto';
import type { Long } from '@grpc/proto-loader';

export interface TransferFeeConfigProto {
  'transferFeeConfigAuthority'?: (string);
  'withdrawWithheldAuthority'?: (string);
  'withheldAmount'?: (number | string | Long);
  'olderTransferFee'?: (_vixen_parser_TransferFeeProto | null);
  'newerTransferFee'?: (_vixen_parser_TransferFeeProto | null);
}

export interface TransferFeeConfigProto__Output {
  'transferFeeConfigAuthority': (string);
  'withdrawWithheldAuthority': (string);
  'withheldAmount': (string);
  'olderTransferFee': (_vixen_parser_TransferFeeProto__Output | null);
  'newerTransferFee': (_vixen_parser_TransferFeeProto__Output | null);
}
