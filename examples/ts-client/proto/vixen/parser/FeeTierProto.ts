// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto


export interface FeeTierProto {
  'discriminator'?: (Buffer | Uint8Array | string);
  'whirlpoolsConfig'?: (string);
  'tickSpacing'?: (number);
  'defaultFeeRate'?: (number);
}

export interface FeeTierProto__Output {
  'discriminator': (Buffer);
  'whirlpoolsConfig': (string);
  'tickSpacing': (number);
  'defaultFeeRate': (number);
}
