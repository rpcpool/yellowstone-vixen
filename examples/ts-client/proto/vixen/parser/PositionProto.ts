// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto

import type { OrcaPositionRewardInfoProto as _vixen_parser_OrcaPositionRewardInfoProto, OrcaPositionRewardInfoProto__Output as _vixen_parser_OrcaPositionRewardInfoProto__Output } from '../../vixen/parser/OrcaPositionRewardInfoProto';
import type { Long } from '@grpc/proto-loader';

export interface PositionProto {
  'discriminator'?: (Buffer | Uint8Array | string);
  'whirlpool'?: (string);
  'positionMint'?: (string);
  'liquidity'?: (string);
  'tickLowerIndex'?: (number);
  'tickUpperIndex'?: (number);
  'feeGrowthCheckpointA'?: (string);
  'feeOwedA'?: (number | string | Long);
  'feeGrowthCheckpointB'?: (string);
  'feeOwedB'?: (number | string | Long);
  'rewardInfos'?: (_vixen_parser_OrcaPositionRewardInfoProto)[];
}

export interface PositionProto__Output {
  'discriminator': (Buffer);
  'whirlpool': (string);
  'positionMint': (string);
  'liquidity': (string);
  'tickLowerIndex': (number);
  'tickUpperIndex': (number);
  'feeGrowthCheckpointA': (string);
  'feeOwedA': (string);
  'feeGrowthCheckpointB': (string);
  'feeOwedB': (string);
  'rewardInfos': (_vixen_parser_OrcaPositionRewardInfoProto__Output)[];
}
