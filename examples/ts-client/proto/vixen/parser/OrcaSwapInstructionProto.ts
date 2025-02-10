// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/orca.proto

import type { OrcaSwapAccountsProto as _vixen_parser_OrcaSwapAccountsProto, OrcaSwapAccountsProto__Output as _vixen_parser_OrcaSwapAccountsProto__Output } from '../../vixen/parser/OrcaSwapAccountsProto';
import type { OrcaSwapIxDataProto as _vixen_parser_OrcaSwapIxDataProto, OrcaSwapIxDataProto__Output as _vixen_parser_OrcaSwapIxDataProto__Output } from '../../vixen/parser/OrcaSwapIxDataProto';

export interface OrcaSwapInstructionProto {
  'accounts'?: (_vixen_parser_OrcaSwapAccountsProto | null);
  'data'?: (_vixen_parser_OrcaSwapIxDataProto | null);
}

export interface OrcaSwapInstructionProto__Output {
  'accounts': (_vixen_parser_OrcaSwapAccountsProto__Output | null);
  'data': (_vixen_parser_OrcaSwapIxDataProto__Output | null);
}
