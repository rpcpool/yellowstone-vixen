// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { Long } from '@grpc/proto-loader';

export interface TransferFeeProto {
  'epoch'?: (number | string | Long);
  'maximumFee'?: (number | string | Long);
  'transferFeeBasisPoints'?: (number | string | Long);
}

export interface TransferFeeProto__Output {
  'epoch': (string);
  'maximumFee': (string);
  'transferFeeBasisPoints': (string);
}
