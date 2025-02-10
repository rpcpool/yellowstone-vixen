// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { UpdateAuthorityAccountsProto as _vixen_parser_UpdateAuthorityAccountsProto, UpdateAuthorityAccountsProto__Output as _vixen_parser_UpdateAuthorityAccountsProto__Output } from '../../vixen/parser/UpdateAuthorityAccountsProto';
import type { UpdateAuthorityDataProto as _vixen_parser_UpdateAuthorityDataProto, UpdateAuthorityDataProto__Output as _vixen_parser_UpdateAuthorityDataProto__Output } from '../../vixen/parser/UpdateAuthorityDataProto';

export interface UpdateAuthorityIxProto {
  'accounts'?: (_vixen_parser_UpdateAuthorityAccountsProto | null);
  'data'?: (_vixen_parser_UpdateAuthorityDataProto | null);
}

export interface UpdateAuthorityIxProto__Output {
  'accounts': (_vixen_parser_UpdateAuthorityAccountsProto__Output | null);
  'data': (_vixen_parser_UpdateAuthorityDataProto__Output | null);
}
