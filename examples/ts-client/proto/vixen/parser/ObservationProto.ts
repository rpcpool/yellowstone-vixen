// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { Long } from '@grpc/proto-loader';

export interface ObservationProto {
  'blockTimestamp'?: (number);
  'tickCumulative'?: (number | string | Long);
  'padding'?: (number | string | Long)[];
}

export interface ObservationProto__Output {
  'blockTimestamp': (number);
  'tickCumulative': (string);
  'padding': (string)[];
}
