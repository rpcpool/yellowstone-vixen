// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/parser.proto

import type { ExtendedTokenAccountProto as _vixen_parser_ExtendedTokenAccountProto, ExtendedTokenAccountProto__Output as _vixen_parser_ExtendedTokenAccountProto__Output } from '../../vixen/parser/ExtendedTokenAccountProto';
import type { ExtendedMintProto as _vixen_parser_ExtendedMintProto, ExtendedMintProto__Output as _vixen_parser_ExtendedMintProto__Output } from '../../vixen/parser/ExtendedMintProto';
import type { MultisigProto as _vixen_parser_MultisigProto, MultisigProto__Output as _vixen_parser_MultisigProto__Output } from '../../vixen/parser/MultisigProto';

export interface TokenExtensionStateProto {
  'extendedTokenAccount'?: (_vixen_parser_ExtendedTokenAccountProto | null);
  'extendedMintAccount'?: (_vixen_parser_ExtendedMintProto | null);
  'multisig'?: (_vixen_parser_MultisigProto | null);
  'stateOneof'?: "extendedTokenAccount"|"extendedMintAccount"|"multisig";
}

export interface TokenExtensionStateProto__Output {
  'extendedTokenAccount'?: (_vixen_parser_ExtendedTokenAccountProto__Output | null);
  'extendedMintAccount'?: (_vixen_parser_ExtendedMintProto__Output | null);
  'multisig'?: (_vixen_parser_MultisigProto__Output | null);
  'stateOneof': "extendedTokenAccount"|"extendedMintAccount"|"multisig";
}
