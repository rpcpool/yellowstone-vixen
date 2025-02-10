// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/parser.proto

import type { RaydiumSwapInstructionProto as _vixen_parser_RaydiumSwapInstructionProto, RaydiumSwapInstructionProto__Output as _vixen_parser_RaydiumSwapInstructionProto__Output } from '../../vixen/parser/RaydiumSwapInstructionProto';
import type { RaydiumSwapV2InstructionProto as _vixen_parser_RaydiumSwapV2InstructionProto, RaydiumSwapV2InstructionProto__Output as _vixen_parser_RaydiumSwapV2InstructionProto__Output } from '../../vixen/parser/RaydiumSwapV2InstructionProto';

export interface RaydiumProgramIxProto {
  'swap'?: (_vixen_parser_RaydiumSwapInstructionProto | null);
  'swapV2'?: (_vixen_parser_RaydiumSwapV2InstructionProto | null);
  'ixOneof'?: "swap"|"swapV2";
}

export interface RaydiumProgramIxProto__Output {
  'swap'?: (_vixen_parser_RaydiumSwapInstructionProto__Output | null);
  'swapV2'?: (_vixen_parser_RaydiumSwapV2InstructionProto__Output | null);
  'ixOneof': "swap"|"swapV2";
}
