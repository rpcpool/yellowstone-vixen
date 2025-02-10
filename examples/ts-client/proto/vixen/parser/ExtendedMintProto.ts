// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { MintProto as _vixen_parser_MintProto, MintProto__Output as _vixen_parser_MintProto__Output } from '../../vixen/parser/MintProto';
import type { ExtensionDataProto as _vixen_parser_ExtensionDataProto, ExtensionDataProto__Output as _vixen_parser_ExtensionDataProto__Output } from '../../vixen/parser/ExtensionDataProto';

export interface ExtendedMintProto {
  'baseMint'?: (_vixen_parser_MintProto | null);
  'extensionDataVec'?: (_vixen_parser_ExtensionDataProto)[];
}

export interface ExtendedMintProto__Output {
  'baseMint': (_vixen_parser_MintProto__Output | null);
  'extensionDataVec': (_vixen_parser_ExtensionDataProto__Output)[];
}
