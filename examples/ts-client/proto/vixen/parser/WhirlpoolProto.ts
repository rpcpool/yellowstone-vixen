// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto

import type { WhirlpoolRewardInfoProto as _vixen_parser_WhirlpoolRewardInfoProto, WhirlpoolRewardInfoProto__Output as _vixen_parser_WhirlpoolRewardInfoProto__Output } from '../../vixen/parser/WhirlpoolRewardInfoProto';
import type { Long } from '@grpc/proto-loader';

export interface WhirlpoolProto {
  'discriminator'?: (Buffer | Uint8Array | string);
  'whirlpoolsConfig'?: (string);
  'whirlpoolBump'?: (number);
  'tickSpacing'?: (number);
  'tickSpacingSeed'?: (Buffer | Uint8Array | string);
  'feeRate'?: (number);
  'protocolFeeRate'?: (number);
  'liquidity'?: (string);
  'sqrtPrice'?: (string);
  'tickCurrentIndex'?: (number);
  'protocolFeeOwedA'?: (number | string | Long);
  'protocolFeeOwedB'?: (number | string | Long);
  'tokenMintA'?: (string);
  'tokenVaultA'?: (string);
  'feeGrowthGlobalA'?: (string);
  'tokenMintB'?: (string);
  'tokenVaultB'?: (string);
  'feeGrowthGlobalB'?: (string);
  'rewardLastUpdatedTimestamp'?: (number | string | Long);
  'rewardInfos'?: (_vixen_parser_WhirlpoolRewardInfoProto)[];
}

export interface WhirlpoolProto__Output {
  'discriminator': (Buffer);
  'whirlpoolsConfig': (string);
  'whirlpoolBump': (number);
  'tickSpacing': (number);
  'tickSpacingSeed': (Buffer);
  'feeRate': (number);
  'protocolFeeRate': (number);
  'liquidity': (string);
  'sqrtPrice': (string);
  'tickCurrentIndex': (number);
  'protocolFeeOwedA': (string);
  'protocolFeeOwedB': (string);
  'tokenMintA': (string);
  'tokenVaultA': (string);
  'feeGrowthGlobalA': (string);
  'tokenMintB': (string);
  'tokenVaultB': (string);
  'feeGrowthGlobalB': (string);
  'rewardLastUpdatedTimestamp': (string);
  'rewardInfos': (_vixen_parser_WhirlpoolRewardInfoProto__Output)[];
}
