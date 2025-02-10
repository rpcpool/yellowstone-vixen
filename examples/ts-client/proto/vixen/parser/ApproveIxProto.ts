// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { ApproveAccountsProto as _vixen_parser_ApproveAccountsProto, ApproveAccountsProto__Output as _vixen_parser_ApproveAccountsProto__Output } from '../../vixen/parser/ApproveAccountsProto';
import type { ApproveDataProto as _vixen_parser_ApproveDataProto, ApproveDataProto__Output as _vixen_parser_ApproveDataProto__Output } from '../../vixen/parser/ApproveDataProto';

export interface ApproveIxProto {
  'accounts'?: (_vixen_parser_ApproveAccountsProto | null);
  'data'?: (_vixen_parser_ApproveDataProto | null);
}

export interface ApproveIxProto__Output {
  'accounts': (_vixen_parser_ApproveAccountsProto__Output | null);
  'data': (_vixen_parser_ApproveDataProto__Output | null);
}
