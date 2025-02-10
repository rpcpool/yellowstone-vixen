// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto


export interface WhirlpoolsConfigProto {
  'discriminator'?: (Buffer | Uint8Array | string);
  'feeAuthority'?: (string);
  'collectProtocolFeesAuthority'?: (string);
  'rewardEmissionsSuperAuthority'?: (string);
  'defaultProtocolFeeRate'?: (number);
}

export interface WhirlpoolsConfigProto__Output {
  'discriminator': (Buffer);
  'feeAuthority': (string);
  'collectProtocolFeesAuthority': (string);
  'rewardEmissionsSuperAuthority': (string);
  'defaultProtocolFeeRate': (number);
}
