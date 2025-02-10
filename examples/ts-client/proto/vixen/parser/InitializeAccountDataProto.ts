// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { Long } from '@grpc/proto-loader';

export interface InitializeAccountDataProto {
  'amount'?: (number | string | Long);
}

export interface InitializeAccountDataProto__Output {
  'amount': (string);
}
