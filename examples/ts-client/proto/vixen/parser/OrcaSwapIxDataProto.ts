// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto

import type { Long } from '@grpc/proto-loader';

export interface OrcaSwapIxDataProto {
  'amount'?: (number | string | Long);
  'otherAmountThreshold'?: (number | string | Long);
  'sqrtPriceLimit'?: (string);
  'amountSpecifiedIsInput'?: (boolean);
  'aToB'?: (boolean);
}

export interface OrcaSwapIxDataProto__Output {
  'amount': (string);
  'otherAmountThreshold': (string);
  'sqrtPriceLimit': (string);
  'amountSpecifiedIsInput': (boolean);
  'aToB': (boolean);
}
