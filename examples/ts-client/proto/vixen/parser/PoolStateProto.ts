// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { RewardInfoProto as _vixen_parser_RewardInfoProto, RewardInfoProto__Output as _vixen_parser_RewardInfoProto__Output } from '../../vixen/parser/RewardInfoProto';
import type { Long } from '@grpc/proto-loader';

export interface PoolStateProto {
  'bump'?: (number);
  'ammConfig'?: (string);
  'owner'?: (string);
  'tokenMint_0'?: (string);
  'tokenMint_1'?: (string);
  'tokenVault_0'?: (string);
  'tokenVault_1'?: (string);
  'observationKey'?: (string);
  'mintDecimals_0'?: (number);
  'mintDecimals_1'?: (number);
  'tickSpacing'?: (number);
  'liquidity'?: (string);
  'sqrtPriceX64'?: (string);
  'tickCurrent'?: (number);
  'padding3'?: (number);
  'padding4'?: (number);
  'feeGrowthGlobal_0X64'?: (string);
  'feeGrowthGlobal_1X64'?: (string);
  'protocolFeesToken_0'?: (number | string | Long);
  'protocolFeesToken_1'?: (number | string | Long);
  'swapInAmountToken_0'?: (string);
  'swapOutAmountToken_1'?: (string);
  'swapInAmountToken_1'?: (string);
  'swapOutAmountToken_0'?: (string);
  'status'?: (number);
  'padding'?: (Buffer | Uint8Array | string);
  'rewardInfos'?: (_vixen_parser_RewardInfoProto)[];
  'tickArrayBitmap'?: (number | string | Long)[];
  'totalFeesToken_0'?: (number | string | Long);
  'totalFeesClaimedToken_0'?: (number | string | Long);
  'totalFeesToken_1'?: (number | string | Long);
  'totalFeesClaimedToken_1'?: (number | string | Long);
  'fundFeesToken_0'?: (number | string | Long);
  'fundFeesToken_1'?: (number | string | Long);
  'openTime'?: (number | string | Long);
  'recentEpoch'?: (number | string | Long);
  'padding1'?: (number | string | Long)[];
  'padding2'?: (number | string | Long)[];
}

export interface PoolStateProto__Output {
  'bump': (number);
  'ammConfig': (string);
  'owner': (string);
  'tokenMint_0': (string);
  'tokenMint_1': (string);
  'tokenVault_0': (string);
  'tokenVault_1': (string);
  'observationKey': (string);
  'mintDecimals_0': (number);
  'mintDecimals_1': (number);
  'tickSpacing': (number);
  'liquidity': (string);
  'sqrtPriceX64': (string);
  'tickCurrent': (number);
  'padding3': (number);
  'padding4': (number);
  'feeGrowthGlobal_0X64': (string);
  'feeGrowthGlobal_1X64': (string);
  'protocolFeesToken_0': (string);
  'protocolFeesToken_1': (string);
  'swapInAmountToken_0': (string);
  'swapOutAmountToken_1': (string);
  'swapInAmountToken_1': (string);
  'swapOutAmountToken_0': (string);
  'status': (number);
  'padding': (Buffer);
  'rewardInfos': (_vixen_parser_RewardInfoProto__Output)[];
  'tickArrayBitmap': (string)[];
  'totalFeesToken_0': (string);
  'totalFeesClaimedToken_0': (string);
  'totalFeesToken_1': (string);
  'totalFeesClaimedToken_1': (string);
  'fundFeesToken_0': (string);
  'fundFeesToken_1': (string);
  'openTime': (string);
  'recentEpoch': (string);
  'padding1': (string)[];
  'padding2': (string)[];
}
