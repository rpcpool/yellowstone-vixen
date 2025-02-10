// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { Long } from '@grpc/proto-loader';

export interface TransferCheckedWithFeeDataProto {
  'amount'?: (number | string | Long);
  'feeAmount'?: (number | string | Long);
  'decimals'?: (number | string | Long);
}

export interface TransferCheckedWithFeeDataProto__Output {
  'amount': (string);
  'feeAmount': (string);
  'decimals': (string);
}
