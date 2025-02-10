// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { UpdateGroupAuthorityAccountsProto as _vixen_parser_UpdateGroupAuthorityAccountsProto, UpdateGroupAuthorityAccountsProto__Output as _vixen_parser_UpdateGroupAuthorityAccountsProto__Output } from '../../vixen/parser/UpdateGroupAuthorityAccountsProto';
import type { UpdateGroupAuthorityDataProto as _vixen_parser_UpdateGroupAuthorityDataProto, UpdateGroupAuthorityDataProto__Output as _vixen_parser_UpdateGroupAuthorityDataProto__Output } from '../../vixen/parser/UpdateGroupAuthorityDataProto';

export interface UpdateGroupAuthorityIxProto {
  'accounts'?: (_vixen_parser_UpdateGroupAuthorityAccountsProto | null);
  'data'?: (_vixen_parser_UpdateGroupAuthorityDataProto | null);
}

export interface UpdateGroupAuthorityIxProto__Output {
  'accounts': (_vixen_parser_UpdateGroupAuthorityAccountsProto__Output | null);
  'data': (_vixen_parser_UpdateGroupAuthorityDataProto__Output | null);
}
