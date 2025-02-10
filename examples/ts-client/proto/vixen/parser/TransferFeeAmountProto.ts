// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { Long } from '@grpc/proto-loader';

export interface TransferFeeAmountProto {
  'withheldAmount'?: (number | string | Long);
}

export interface TransferFeeAmountProto__Output {
  'withheldAmount': (string);
}
