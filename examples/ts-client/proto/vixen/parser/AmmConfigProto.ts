// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { Long } from '@grpc/proto-loader';

export interface AmmConfigProto {
  'bump'?: (number);
  'index'?: (number);
  'owner'?: (string);
  'protocolFeeRate'?: (number);
  'tradeFeeRate'?: (number);
  'tickSpacing'?: (number);
  'fundFeeRate'?: (number);
  'paddingU32'?: (number);
  'fundOwner'?: (string);
  'padding'?: (number | string | Long)[];
}

export interface AmmConfigProto__Output {
  'bump': (number);
  'index': (number);
  'owner': (string);
  'protocolFeeRate': (number);
  'tradeFeeRate': (number);
  'tickSpacing': (number);
  'fundFeeRate': (number);
  'paddingU32': (number);
  'fundOwner': (string);
  'padding': (string)[];
}
