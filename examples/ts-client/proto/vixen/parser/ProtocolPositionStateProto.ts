// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { Long } from '@grpc/proto-loader';

export interface ProtocolPositionStateProto {
  'bump'?: (number);
  'poolId'?: (string);
  'tickLowerIndex'?: (number);
  'tickUpperIndex'?: (number);
  'liquidity'?: (string);
  'feeGrowthInside_0LastX64'?: (string);
  'feeGrowthInside_1LastX64'?: (string);
  'tokenFeesOwed_0'?: (number | string | Long);
  'tokenFeesOwed_1'?: (number | string | Long);
  'rewardGrowthInside'?: (string)[];
  'recentEpoch'?: (number | string | Long);
  'padding'?: (number | string | Long)[];
}

export interface ProtocolPositionStateProto__Output {
  'bump': (number);
  'poolId': (string);
  'tickLowerIndex': (number);
  'tickUpperIndex': (number);
  'liquidity': (string);
  'feeGrowthInside_0LastX64': (string);
  'feeGrowthInside_1LastX64': (string);
  'tokenFeesOwed_0': (string);
  'tokenFeesOwed_1': (string);
  'rewardGrowthInside': (string)[];
  'recentEpoch': (string);
  'padding': (string)[];
}
