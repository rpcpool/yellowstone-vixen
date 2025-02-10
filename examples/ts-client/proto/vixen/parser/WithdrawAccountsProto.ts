// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto


export interface WithdrawAccountsProto {
  'sourceAccount'?: (string);
  'mint'?: (string);
  'destination'?: (string);
  'owner'?: (string);
  'multisigSigners'?: (string)[];
}

export interface WithdrawAccountsProto__Output {
  'sourceAccount': (string);
  'mint': (string);
  'destination': (string);
  'owner': (string);
  'multisigSigners': (string)[];
}
