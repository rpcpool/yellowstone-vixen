// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { InitializeMintAccountsProto as _vixen_parser_InitializeMintAccountsProto, InitializeMintAccountsProto__Output as _vixen_parser_InitializeMintAccountsProto__Output } from '../../vixen/parser/InitializeMintAccountsProto';
import type { InitializeMintDataProto as _vixen_parser_InitializeMintDataProto, InitializeMintDataProto__Output as _vixen_parser_InitializeMintDataProto__Output } from '../../vixen/parser/InitializeMintDataProto';

export interface InitializeMintIxProto {
  'accounts'?: (_vixen_parser_InitializeMintAccountsProto | null);
  'data'?: (_vixen_parser_InitializeMintDataProto | null);
}

export interface InitializeMintIxProto__Output {
  'accounts': (_vixen_parser_InitializeMintAccountsProto__Output | null);
  'data': (_vixen_parser_InitializeMintDataProto__Output | null);
}
