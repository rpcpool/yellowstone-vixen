// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { TokenAccountProto as _vixen_parser_TokenAccountProto, TokenAccountProto__Output as _vixen_parser_TokenAccountProto__Output } from '../../vixen/parser/TokenAccountProto';
import type { ExtensionDataProto as _vixen_parser_ExtensionDataProto, ExtensionDataProto__Output as _vixen_parser_ExtensionDataProto__Output } from '../../vixen/parser/ExtensionDataProto';

export interface ExtendedTokenAccountProto {
  'baseAccount'?: (_vixen_parser_TokenAccountProto | null);
  'extensionDataVec'?: (_vixen_parser_ExtensionDataProto)[];
}

export interface ExtendedTokenAccountProto__Output {
  'baseAccount': (_vixen_parser_TokenAccountProto__Output | null);
  'extensionDataVec': (_vixen_parser_ExtensionDataProto__Output)[];
}
