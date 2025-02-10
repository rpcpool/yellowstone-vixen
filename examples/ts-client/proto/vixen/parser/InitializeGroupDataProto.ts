// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { Long } from '@grpc/proto-loader';

export interface InitializeGroupDataProto {
  'updateAuthority'?: (string);
  'maxSize'?: (number | string | Long);
  '_updateAuthority'?: "updateAuthority";
}

export interface InitializeGroupDataProto__Output {
  'updateAuthority'?: (string);
  'maxSize': (string);
  '_updateAuthority': "updateAuthority";
}
