// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { Long } from '@grpc/proto-loader';

export interface TokenGroupMemberProto {
  'mint'?: (string);
  'group'?: (string);
  'memberNumber'?: (number | string | Long);
}

export interface TokenGroupMemberProto__Output {
  'mint': (string);
  'group': (string);
  'memberNumber': (string);
}
