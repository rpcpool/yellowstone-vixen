// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { Long } from '@grpc/proto-loader';

export interface RaydiumSwapIxDataProto {
  'amount'?: (number | string | Long);
  'otherAmountThreshold'?: (number | string | Long);
  'sqrtPriceLimitX64'?: (string);
  'isBaseInput'?: (boolean);
}

export interface RaydiumSwapIxDataProto__Output {
  'amount': (string);
  'otherAmountThreshold': (string);
  'sqrtPriceLimitX64': (string);
  'isBaseInput': (boolean);
}
