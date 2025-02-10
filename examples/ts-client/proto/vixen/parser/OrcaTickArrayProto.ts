// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto

import type { OrcaTickProto as _vixen_parser_OrcaTickProto, OrcaTickProto__Output as _vixen_parser_OrcaTickProto__Output } from '../../vixen/parser/OrcaTickProto';

export interface OrcaTickArrayProto {
  'discriminator'?: (Buffer | Uint8Array | string);
  'startTickIndex'?: (number);
  'ticks'?: (_vixen_parser_OrcaTickProto)[];
  'whirlpool'?: (string);
}

export interface OrcaTickArrayProto__Output {
  'discriminator': (Buffer);
  'startTickIndex': (number);
  'ticks': (_vixen_parser_OrcaTickProto__Output)[];
  'whirlpool': (string);
}
