// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { RaydiumTickStateProto as _vixen_parser_RaydiumTickStateProto, RaydiumTickStateProto__Output as _vixen_parser_RaydiumTickStateProto__Output } from '../../vixen/parser/RaydiumTickStateProto';
import type { Long } from '@grpc/proto-loader';

export interface RaydiumTickArrayStateProto {
  'poolId'?: (string);
  'startTickIndex'?: (number);
  'ticks'?: (_vixen_parser_RaydiumTickStateProto)[];
  'initializedTickCount'?: (number);
  'recentEpoch'?: (number | string | Long);
  'padding'?: (Buffer | Uint8Array | string);
}

export interface RaydiumTickArrayStateProto__Output {
  'poolId': (string);
  'startTickIndex': (number);
  'ticks': (_vixen_parser_RaydiumTickStateProto__Output)[];
  'initializedTickCount': (number);
  'recentEpoch': (string);
  'padding': (Buffer);
}
