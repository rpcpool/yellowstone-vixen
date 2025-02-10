// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/parser.proto

import type { TokenAccountProto as _vixen_parser_TokenAccountProto, TokenAccountProto__Output as _vixen_parser_TokenAccountProto__Output } from '../../vixen/parser/TokenAccountProto';
import type { MintProto as _vixen_parser_MintProto, MintProto__Output as _vixen_parser_MintProto__Output } from '../../vixen/parser/MintProto';
import type { MultisigProto as _vixen_parser_MultisigProto, MultisigProto__Output as _vixen_parser_MultisigProto__Output } from '../../vixen/parser/MultisigProto';

export interface TokenProgramStateProto {
  'tokenAccount'?: (_vixen_parser_TokenAccountProto | null);
  'mint'?: (_vixen_parser_MintProto | null);
  'multisig'?: (_vixen_parser_MultisigProto | null);
  'stateOneof'?: "tokenAccount"|"mint"|"multisig";
}

export interface TokenProgramStateProto__Output {
  'tokenAccount'?: (_vixen_parser_TokenAccountProto__Output | null);
  'mint'?: (_vixen_parser_MintProto__Output | null);
  'multisig'?: (_vixen_parser_MultisigProto__Output | null);
  'stateOneof': "tokenAccount"|"mint"|"multisig";
}
