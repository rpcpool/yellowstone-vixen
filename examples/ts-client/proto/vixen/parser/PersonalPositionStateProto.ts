// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { RaydiumPositionRewardInfoProto as _vixen_parser_RaydiumPositionRewardInfoProto, RaydiumPositionRewardInfoProto__Output as _vixen_parser_RaydiumPositionRewardInfoProto__Output } from '../../vixen/parser/RaydiumPositionRewardInfoProto';
import type { Long } from '@grpc/proto-loader';

export interface PersonalPositionStateProto {
  'bump'?: (number);
  'nftMint'?: (string);
  'poolId'?: (string);
  'tickLowerIndex'?: (number);
  'tickUpperIndex'?: (number);
  'liquidity'?: (string);
  'feeGrowthInside_0LastX64'?: (string);
  'feeGrowthInside_1LastX64'?: (string);
  'tokenFeesOwed_0'?: (number | string | Long);
  'tokenFeesOwed_1'?: (number | string | Long);
  'rewardInfos'?: (_vixen_parser_RaydiumPositionRewardInfoProto)[];
  'recentEpoch'?: (number | string | Long);
  'padding'?: (number | string | Long)[];
}

export interface PersonalPositionStateProto__Output {
  'bump': (number);
  'nftMint': (string);
  'poolId': (string);
  'tickLowerIndex': (number);
  'tickUpperIndex': (number);
  'liquidity': (string);
  'feeGrowthInside_0LastX64': (string);
  'feeGrowthInside_1LastX64': (string);
  'tokenFeesOwed_0': (string);
  'tokenFeesOwed_1': (string);
  'rewardInfos': (_vixen_parser_RaydiumPositionRewardInfoProto__Output)[];
  'recentEpoch': (string);
  'padding': (string)[];
}
