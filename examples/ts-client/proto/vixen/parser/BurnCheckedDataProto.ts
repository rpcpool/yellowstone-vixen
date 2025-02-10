// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { Long } from '@grpc/proto-loader';

export interface BurnCheckedDataProto {
  'amount'?: (number | string | Long);
  'decimals'?: (number | string | Long);
}

export interface BurnCheckedDataProto__Output {
  'amount': (string);
  'decimals': (string);
}
