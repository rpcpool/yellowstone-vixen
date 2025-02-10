// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { Long } from '@grpc/proto-loader';

export interface MultisigProto {
  'm'?: (number | string | Long);
  'n'?: (number | string | Long);
  'isInitialized'?: (boolean);
  'signers'?: (string)[];
}

export interface MultisigProto__Output {
  'm': (string);
  'n': (string);
  'isInitialized': (boolean);
  'signers': (string)[];
}
