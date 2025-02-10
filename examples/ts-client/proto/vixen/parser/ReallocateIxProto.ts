// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { ReallocateAccountsProto as _vixen_parser_ReallocateAccountsProto, ReallocateAccountsProto__Output as _vixen_parser_ReallocateAccountsProto__Output } from '../../vixen/parser/ReallocateAccountsProto';
import type { ReallocateDataProto as _vixen_parser_ReallocateDataProto, ReallocateDataProto__Output as _vixen_parser_ReallocateDataProto__Output } from '../../vixen/parser/ReallocateDataProto';

export interface ReallocateIxProto {
  'accounts'?: (_vixen_parser_ReallocateAccountsProto | null);
  'data'?: (_vixen_parser_ReallocateDataProto | null);
}

export interface ReallocateIxProto__Output {
  'accounts': (_vixen_parser_ReallocateAccountsProto__Output | null);
  'data': (_vixen_parser_ReallocateDataProto__Output | null);
}
