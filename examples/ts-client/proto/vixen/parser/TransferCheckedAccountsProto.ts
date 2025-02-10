// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto


export interface TransferCheckedAccountsProto {
  'source'?: (string);
  'mint'?: (string);
  'destination'?: (string);
  'owner'?: (string);
  'multisigSigners'?: (string)[];
}

export interface TransferCheckedAccountsProto__Output {
  'source': (string);
  'mint': (string);
  'destination': (string);
  'owner': (string);
  'multisigSigners': (string)[];
}
