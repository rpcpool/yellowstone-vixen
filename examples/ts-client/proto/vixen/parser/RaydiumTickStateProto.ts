// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto


export interface RaydiumTickStateProto {
  'tick'?: (number);
  'liquidityNet'?: (string);
  'liquidityGross'?: (string);
  'feeGrowthOutside_0X64'?: (string);
  'feeGrowthOutside_1X64'?: (string);
  'rewardGrowthsOutsideX64'?: (string)[];
  'padding'?: (number)[];
}

export interface RaydiumTickStateProto__Output {
  'tick': (number);
  'liquidityNet': (string);
  'liquidityGross': (string);
  'feeGrowthOutside_0X64': (string);
  'feeGrowthOutside_1X64': (string);
  'rewardGrowthsOutsideX64': (string)[];
  'padding': (number)[];
}
