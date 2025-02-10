// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { AccountStateProto as _vixen_parser_AccountStateProto, AccountStateProto__Output as _vixen_parser_AccountStateProto__Output } from '../../vixen/parser/AccountStateProto';
import type { Long } from '@grpc/proto-loader';

export interface TokenAccountProto {
  'mint'?: (string);
  'owner'?: (string);
  'amount'?: (number | string | Long);
  'delegate'?: (string);
  'state'?: (_vixen_parser_AccountStateProto);
  'isNative'?: (number | string | Long);
  'delegatedAmount'?: (number | string | Long);
  'closeAuthority'?: (string);
  '_delegate'?: "delegate";
  '_isNative'?: "isNative";
  '_closeAuthority'?: "closeAuthority";
}

export interface TokenAccountProto__Output {
  'mint': (string);
  'owner': (string);
  'amount': (string);
  'delegate'?: (string);
  'state': (_vixen_parser_AccountStateProto__Output);
  'isNative'?: (string);
  'delegatedAmount': (string);
  'closeAuthority'?: (string);
  '_delegate': "delegate";
  '_isNative': "isNative";
  '_closeAuthority': "closeAuthority";
}
