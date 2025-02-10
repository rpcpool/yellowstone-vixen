// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/raydium.proto

import type { RaydiumSwapAccountsProto as _vixen_parser_RaydiumSwapAccountsProto, RaydiumSwapAccountsProto__Output as _vixen_parser_RaydiumSwapAccountsProto__Output } from '../../vixen/parser/RaydiumSwapAccountsProto';
import type { RaydiumSwapIxDataProto as _vixen_parser_RaydiumSwapIxDataProto, RaydiumSwapIxDataProto__Output as _vixen_parser_RaydiumSwapIxDataProto__Output } from '../../vixen/parser/RaydiumSwapIxDataProto';

export interface RaydiumSwapInstructionProto {
  'accounts'?: (_vixen_parser_RaydiumSwapAccountsProto | null);
  'data'?: (_vixen_parser_RaydiumSwapIxDataProto | null);
}

export interface RaydiumSwapInstructionProto__Output {
  'accounts': (_vixen_parser_RaydiumSwapAccountsProto__Output | null);
  'data': (_vixen_parser_RaydiumSwapIxDataProto__Output | null);
}
