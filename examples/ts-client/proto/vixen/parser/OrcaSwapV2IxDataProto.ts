// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto

import type { Long } from '@grpc/proto-loader';

export interface OrcaSwapV2IxDataProto {
  'amount'?: (number | string | Long);
  'otherAmountThreshold'?: (number | string | Long);
  'sqrtPriceLimit'?: (string);
  'amountSpecifiedIsInput'?: (boolean);
  'aToB'?: (boolean);
}

export interface OrcaSwapV2IxDataProto__Output {
  'amount': (string);
  'otherAmountThreshold': (string);
  'sqrtPriceLimit': (string);
  'amountSpecifiedIsInput': (boolean);
  'aToB': (boolean);
}
