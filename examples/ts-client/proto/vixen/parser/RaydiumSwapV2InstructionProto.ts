// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { RaydiumSwapV2AccountsProto as _vixen_parser_RaydiumSwapV2AccountsProto, RaydiumSwapV2AccountsProto__Output as _vixen_parser_RaydiumSwapV2AccountsProto__Output } from '../../vixen/parser/RaydiumSwapV2AccountsProto';
import type { RaydiumSwapIxDataProto as _vixen_parser_RaydiumSwapIxDataProto, RaydiumSwapIxDataProto__Output as _vixen_parser_RaydiumSwapIxDataProto__Output } from '../../vixen/parser/RaydiumSwapIxDataProto';

export interface RaydiumSwapV2InstructionProto {
  'accounts'?: (_vixen_parser_RaydiumSwapV2AccountsProto | null);
  'data'?: (_vixen_parser_RaydiumSwapIxDataProto | null);
}

export interface RaydiumSwapV2InstructionProto__Output {
  'accounts': (_vixen_parser_RaydiumSwapV2AccountsProto__Output | null);
  'data': (_vixen_parser_RaydiumSwapIxDataProto__Output | null);
}
