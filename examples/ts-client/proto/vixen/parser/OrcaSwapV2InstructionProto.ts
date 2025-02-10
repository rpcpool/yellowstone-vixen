// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto

import type { OrcaSwapV2AccountsProto as _vixen_parser_OrcaSwapV2AccountsProto, OrcaSwapV2AccountsProto__Output as _vixen_parser_OrcaSwapV2AccountsProto__Output } from '../../vixen/parser/OrcaSwapV2AccountsProto';
import type { OrcaSwapV2IxDataProto as _vixen_parser_OrcaSwapV2IxDataProto, OrcaSwapV2IxDataProto__Output as _vixen_parser_OrcaSwapV2IxDataProto__Output } from '../../vixen/parser/OrcaSwapV2IxDataProto';

export interface OrcaSwapV2InstructionProto {
  'accounts'?: (_vixen_parser_OrcaSwapV2AccountsProto | null);
  'data'?: (_vixen_parser_OrcaSwapV2IxDataProto | null);
}

export interface OrcaSwapV2InstructionProto__Output {
  'accounts': (_vixen_parser_OrcaSwapV2AccountsProto__Output | null);
  'data': (_vixen_parser_OrcaSwapV2IxDataProto__Output | null);
}
