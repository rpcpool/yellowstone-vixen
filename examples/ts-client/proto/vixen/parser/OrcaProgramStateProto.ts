// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/parser.proto

import type { WhirlpoolProto as _vixen_parser_WhirlpoolProto, WhirlpoolProto__Output as _vixen_parser_WhirlpoolProto__Output } from '../../vixen/parser/WhirlpoolProto';
import type { WhirlpoolsConfigProto as _vixen_parser_WhirlpoolsConfigProto, WhirlpoolsConfigProto__Output as _vixen_parser_WhirlpoolsConfigProto__Output } from '../../vixen/parser/WhirlpoolsConfigProto';
import type { FeeTierProto as _vixen_parser_FeeTierProto, FeeTierProto__Output as _vixen_parser_FeeTierProto__Output } from '../../vixen/parser/FeeTierProto';
import type { PositionProto as _vixen_parser_PositionProto, PositionProto__Output as _vixen_parser_PositionProto__Output } from '../../vixen/parser/PositionProto';
import type { OrcaTickArrayProto as _vixen_parser_OrcaTickArrayProto, OrcaTickArrayProto__Output as _vixen_parser_OrcaTickArrayProto__Output } from '../../vixen/parser/OrcaTickArrayProto';

export interface OrcaProgramStateProto {
  'whirlpool'?: (_vixen_parser_WhirlpoolProto | null);
  'whirlpoolsConfig'?: (_vixen_parser_WhirlpoolsConfigProto | null);
  'feeTier'?: (_vixen_parser_FeeTierProto | null);
  'position'?: (_vixen_parser_PositionProto | null);
  'tickArray'?: (_vixen_parser_OrcaTickArrayProto | null);
  'stateOneof'?: "whirlpool"|"whirlpoolsConfig"|"feeTier"|"position"|"tickArray";
}

export interface OrcaProgramStateProto__Output {
  'whirlpool'?: (_vixen_parser_WhirlpoolProto__Output | null);
  'whirlpoolsConfig'?: (_vixen_parser_WhirlpoolsConfigProto__Output | null);
  'feeTier'?: (_vixen_parser_FeeTierProto__Output | null);
  'position'?: (_vixen_parser_PositionProto__Output | null);
  'tickArray'?: (_vixen_parser_OrcaTickArrayProto__Output | null);
  'stateOneof': "whirlpool"|"whirlpoolsConfig"|"feeTier"|"position"|"tickArray";
}
