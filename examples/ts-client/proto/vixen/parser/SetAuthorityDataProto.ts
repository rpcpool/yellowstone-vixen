// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

import type { AuthorityType as _vixen_parser_AuthorityType, AuthorityType__Output as _vixen_parser_AuthorityType__Output } from '../../vixen/parser/AuthorityType';

export interface SetAuthorityDataProto {
  'authorityType'?: (_vixen_parser_AuthorityType);
  'newAuthority'?: (string);
  '_newAuthority'?: "newAuthority";
}

export interface SetAuthorityDataProto__Output {
  'authorityType': (_vixen_parser_AuthorityType__Output);
  'newAuthority'?: (string);
  '_newAuthority': "newAuthority";
}
