// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { Long } from '@grpc/proto-loader';

export interface InterestBearingConfigProto {
  'rateAuthority'?: (string);
  'initializationTimestamp'?: (number | string | Long);
  'preUpdateAverageRate'?: (number | string | Long);
  'lastUpdateTimestamp'?: (number | string | Long);
  'currentRate'?: (number | string | Long);
}

export interface InterestBearingConfigProto__Output {
  'rateAuthority': (string);
  'initializationTimestamp': (string);
  'preUpdateAverageRate': (string);
  'lastUpdateTimestamp': (string);
  'currentRate': (string);
}
