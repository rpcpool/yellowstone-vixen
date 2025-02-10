// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { TickArrayBitmapProto as _vixen_parser_TickArrayBitmapProto, TickArrayBitmapProto__Output as _vixen_parser_TickArrayBitmapProto__Output } from '../../vixen/parser/TickArrayBitmapProto';

export interface TickArrayBitmapExtensionProto {
  'poolId'?: (string);
  'positiveTickArrayBitmap'?: (_vixen_parser_TickArrayBitmapProto)[];
  'negativeTickArrayBitmap'?: (_vixen_parser_TickArrayBitmapProto)[];
}

export interface TickArrayBitmapExtensionProto__Output {
  'poolId': (string);
  'positiveTickArrayBitmap': (_vixen_parser_TickArrayBitmapProto__Output)[];
  'negativeTickArrayBitmap': (_vixen_parser_TickArrayBitmapProto__Output)[];
}
