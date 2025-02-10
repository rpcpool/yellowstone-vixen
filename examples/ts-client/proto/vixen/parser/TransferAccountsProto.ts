// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto


export interface TransferAccountsProto {
  'source'?: (string);
  'destination'?: (string);
  'owner'?: (string);
  'multisigSigners'?: (string)[];
}

export interface TransferAccountsProto__Output {
  'source': (string);
  'destination': (string);
  'owner': (string);
  'multisigSigners': (string)[];
}
