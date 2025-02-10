// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { UpdateGroupMaxSizeAccountsProto as _vixen_parser_UpdateGroupMaxSizeAccountsProto, UpdateGroupMaxSizeAccountsProto__Output as _vixen_parser_UpdateGroupMaxSizeAccountsProto__Output } from '../../vixen/parser/UpdateGroupMaxSizeAccountsProto';
import type { UpdateGroupMaxSizeDataProto as _vixen_parser_UpdateGroupMaxSizeDataProto, UpdateGroupMaxSizeDataProto__Output as _vixen_parser_UpdateGroupMaxSizeDataProto__Output } from '../../vixen/parser/UpdateGroupMaxSizeDataProto';

export interface UpdateGroupMaxSizeIxProto {
  'accounts'?: (_vixen_parser_UpdateGroupMaxSizeAccountsProto | null);
  'data'?: (_vixen_parser_UpdateGroupMaxSizeDataProto | null);
}

export interface UpdateGroupMaxSizeIxProto__Output {
  'accounts': (_vixen_parser_UpdateGroupMaxSizeAccountsProto__Output | null);
  'data': (_vixen_parser_UpdateGroupMaxSizeDataProto__Output | null);
}
