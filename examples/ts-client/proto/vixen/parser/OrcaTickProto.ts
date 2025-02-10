// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto


export interface OrcaTickProto {
  'initialized'?: (boolean);
  'liquidityNet'?: (string);
  'liquidityGross'?: (string);
  'feeGrowthOutsideA'?: (string);
  'feeGrowthOutsideB'?: (string);
  'rewardGrowthsOutside'?: (string)[];
}

export interface OrcaTickProto__Output {
  'initialized': (boolean);
  'liquidityNet': (string);
  'liquidityGross': (string);
  'feeGrowthOutsideA': (string);
  'feeGrowthOutsideB': (string);
  'rewardGrowthsOutside': (string)[];
}
