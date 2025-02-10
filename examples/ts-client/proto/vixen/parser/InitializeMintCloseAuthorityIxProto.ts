// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { InitializeMintCloseAuthorityAccountsProto as _vixen_parser_InitializeMintCloseAuthorityAccountsProto, InitializeMintCloseAuthorityAccountsProto__Output as _vixen_parser_InitializeMintCloseAuthorityAccountsProto__Output } from '../../vixen/parser/InitializeMintCloseAuthorityAccountsProto';
import type { InitializeMintCloseAuthorityDataProto as _vixen_parser_InitializeMintCloseAuthorityDataProto, InitializeMintCloseAuthorityDataProto__Output as _vixen_parser_InitializeMintCloseAuthorityDataProto__Output } from '../../vixen/parser/InitializeMintCloseAuthorityDataProto';

export interface InitializeMintCloseAuthorityIxProto {
  'accounts'?: (_vixen_parser_InitializeMintCloseAuthorityAccountsProto | null);
  'data'?: (_vixen_parser_InitializeMintCloseAuthorityDataProto | null);
}

export interface InitializeMintCloseAuthorityIxProto__Output {
  'accounts': (_vixen_parser_InitializeMintCloseAuthorityAccountsProto__Output | null);
  'data': (_vixen_parser_InitializeMintCloseAuthorityDataProto__Output | null);
}
