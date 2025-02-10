// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { Long } from '@grpc/proto-loader';

export interface WithdrawWithheldTokensFromAccountsDataProto {
  'numTokenAccounts'?: (number | string | Long);
}

export interface WithdrawWithheldTokensFromAccountsDataProto__Output {
  'numTokenAccounts': (string);
}
