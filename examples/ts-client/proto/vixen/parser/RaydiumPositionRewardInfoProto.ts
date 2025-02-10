// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { Long } from '@grpc/proto-loader';

export interface RaydiumPositionRewardInfoProto {
  'growthInsideLastX64'?: (string);
  'rewardAmountOwed'?: (number | string | Long);
}

export interface RaydiumPositionRewardInfoProto__Output {
  'growthInsideLastX64': (string);
  'rewardAmountOwed': (string);
}
