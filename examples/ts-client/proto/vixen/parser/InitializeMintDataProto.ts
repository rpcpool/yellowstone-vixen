// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { Long } from '@grpc/proto-loader';

export interface InitializeMintDataProto {
  'decimals'?: (number | string | Long);
  'mintAuthority'?: (string);
  'freezeAuthority'?: (string);
  '_mintAuthority'?: "mintAuthority";
  '_freezeAuthority'?: "freezeAuthority";
}

export interface InitializeMintDataProto__Output {
  'decimals': (string);
  'mintAuthority'?: (string);
  'freezeAuthority'?: (string);
  '_mintAuthority': "mintAuthority";
  '_freezeAuthority': "freezeAuthority";
}
