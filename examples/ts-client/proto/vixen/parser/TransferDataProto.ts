// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { Long } from '@grpc/proto-loader';

export interface TransferDataProto {
  'amount'?: (number | string | Long);
}

export interface TransferDataProto__Output {
  'amount': (string);
}
