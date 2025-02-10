// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { ObservationProto as _vixen_parser_ObservationProto, ObservationProto__Output as _vixen_parser_ObservationProto__Output } from '../../vixen/parser/ObservationProto';
import type { Long } from '@grpc/proto-loader';

export interface ObservationStateProto {
  'initialized'?: (boolean);
  'recentEpoch'?: (number | string | Long);
  'observationIndex'?: (number);
  'poolId'?: (string);
  'observations'?: (_vixen_parser_ObservationProto)[];
  'padding'?: (number | string | Long)[];
}

export interface ObservationStateProto__Output {
  'initialized': (boolean);
  'recentEpoch': (string);
  'observationIndex': (number);
  'poolId': (string);
  'observations': (_vixen_parser_ObservationProto__Output)[];
  'padding': (string)[];
}
