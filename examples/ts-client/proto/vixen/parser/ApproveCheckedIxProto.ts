// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { ApproveCheckedAccountsProto as _vixen_parser_ApproveCheckedAccountsProto, ApproveCheckedAccountsProto__Output as _vixen_parser_ApproveCheckedAccountsProto__Output } from '../../vixen/parser/ApproveCheckedAccountsProto';
import type { ApproveCheckedDataProto as _vixen_parser_ApproveCheckedDataProto, ApproveCheckedDataProto__Output as _vixen_parser_ApproveCheckedDataProto__Output } from '../../vixen/parser/ApproveCheckedDataProto';

export interface ApproveCheckedIxProto {
  'accounts'?: (_vixen_parser_ApproveCheckedAccountsProto | null);
  'data'?: (_vixen_parser_ApproveCheckedDataProto | null);
}

export interface ApproveCheckedIxProto__Output {
  'accounts': (_vixen_parser_ApproveCheckedAccountsProto__Output | null);
  'data': (_vixen_parser_ApproveCheckedDataProto__Output | null);
}
