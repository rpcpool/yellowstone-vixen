// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/parser.proto

import type { OrcaSwapInstructionProto as _vixen_parser_OrcaSwapInstructionProto, OrcaSwapInstructionProto__Output as _vixen_parser_OrcaSwapInstructionProto__Output } from '../../vixen/parser/OrcaSwapInstructionProto';
import type { OrcaSwapV2InstructionProto as _vixen_parser_OrcaSwapV2InstructionProto, OrcaSwapV2InstructionProto__Output as _vixen_parser_OrcaSwapV2InstructionProto__Output } from '../../vixen/parser/OrcaSwapV2InstructionProto';

export interface OrcaProgramIxProto {
  'swap'?: (_vixen_parser_OrcaSwapInstructionProto | null);
  'swapV2'?: (_vixen_parser_OrcaSwapV2InstructionProto | null);
  'ixOneof'?: "swap"|"swapV2";
}

export interface OrcaProgramIxProto__Output {
  'swap'?: (_vixen_parser_OrcaSwapInstructionProto__Output | null);
  'swapV2'?: (_vixen_parser_OrcaSwapV2InstructionProto__Output | null);
  'ixOneof': "swap"|"swapV2";
}
