// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto

import type { Long } from '@grpc/proto-loader';

export interface OrcaPositionRewardInfoProto {
  'growthInsideCheckpoint'?: (string);
  'amountOwed'?: (number | string | Long);
}

export interface OrcaPositionRewardInfoProto__Output {
  'growthInsideCheckpoint': (string);
  'amountOwed': (string);
}
