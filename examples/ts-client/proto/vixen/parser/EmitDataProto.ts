// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { Long } from '@grpc/proto-loader';

export interface EmitDataProto {
  'start'?: (number | string | Long);
  'end'?: (number | string | Long);
  '_start'?: "start";
  '_end'?: "end";
}

export interface EmitDataProto__Output {
  'start'?: (string);
  'end'?: (string);
  '_start': "start";
  '_end': "end";
}
