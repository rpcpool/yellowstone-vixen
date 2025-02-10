// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { Long } from '@grpc/proto-loader';

export interface RewardInfoProto {
  'rewardState'?: (number);
  'openTime'?: (number | string | Long);
  'endTime'?: (number | string | Long);
  'lastUpdateTime'?: (number | string | Long);
  'emissionsPerSecondX64'?: (string);
  'rewardTotalEmissioned'?: (number | string | Long);
  'rewardClaimed'?: (number | string | Long);
  'tokenMint'?: (string);
  'tokenVault'?: (string);
  'authority'?: (string);
  'rewardGrowthGlobalX64'?: (string);
}

export interface RewardInfoProto__Output {
  'rewardState': (number);
  'openTime': (string);
  'endTime': (string);
  'lastUpdateTime': (string);
  'emissionsPerSecondX64': (string);
  'rewardTotalEmissioned': (string);
  'rewardClaimed': (string);
  'tokenMint': (string);
  'tokenVault': (string);
  'authority': (string);
  'rewardGrowthGlobalX64': (string);
}
