// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { KeyValue as _vixen_parser_KeyValue, KeyValue__Output as _vixen_parser_KeyValue__Output } from '../../vixen/parser/KeyValue';

export interface TokenMetadataProto {
  'updateAuthority'?: (string);
  'mint'?: (string);
  'name'?: (string);
  'symbol'?: (string);
  'uri'?: (string);
  'additionalMetadata'?: (_vixen_parser_KeyValue)[];
}

export interface TokenMetadataProto__Output {
  'updateAuthority': (string);
  'mint': (string);
  'name': (string);
  'symbol': (string);
  'uri': (string);
  'additionalMetadata': (_vixen_parser_KeyValue__Output)[];
}
