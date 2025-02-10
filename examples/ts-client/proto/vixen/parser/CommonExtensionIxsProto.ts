// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { ExtensionWithCommonIxsProto as _vixen_parser_ExtensionWithCommonIxsProto, ExtensionWithCommonIxsProto__Output as _vixen_parser_ExtensionWithCommonIxsProto__Output } from '../../vixen/parser/ExtensionWithCommonIxsProto';
import type { CommonExtensionIxProto as _vixen_parser_CommonExtensionIxProto, CommonExtensionIxProto__Output as _vixen_parser_CommonExtensionIxProto__Output } from '../../vixen/parser/CommonExtensionIxProto';

export interface CommonExtensionIxsProto {
  'extension'?: (_vixen_parser_ExtensionWithCommonIxsProto);
  'ix'?: (_vixen_parser_CommonExtensionIxProto | null);
}

export interface CommonExtensionIxsProto__Output {
  'extension': (_vixen_parser_ExtensionWithCommonIxsProto__Output);
  'ix': (_vixen_parser_CommonExtensionIxProto__Output | null);
}
