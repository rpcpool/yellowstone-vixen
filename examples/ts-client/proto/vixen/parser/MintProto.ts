// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { Long } from '@grpc/proto-loader';

export interface MintProto {
  'mintAuthority'?: (string);
  'supply'?: (number | string | Long);
  'decimals'?: (number | string | Long);
  'isInitialized'?: (boolean);
  'freezeAuthority'?: (string);
  '_mintAuthority'?: "mintAuthority";
  '_freezeAuthority'?: "freezeAuthority";
}

export interface MintProto__Output {
  'mintAuthority'?: (string);
  'supply': (string);
  'decimals': (string);
  'isInitialized': (boolean);
  'freezeAuthority'?: (string);
  '_mintAuthority': "mintAuthority";
  '_freezeAuthority': "freezeAuthority";
}
