// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { Long } from '@grpc/proto-loader';

export interface SetTransferFeeDataProto {
  'transferFeeBasisPoints'?: (number | string | Long);
  'maximumFee'?: (number | string | Long);
}

export interface SetTransferFeeDataProto__Output {
  'transferFeeBasisPoints': (string);
  'maximumFee': (string);
}
