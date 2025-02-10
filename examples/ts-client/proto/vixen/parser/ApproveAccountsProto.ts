// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto


export interface ApproveAccountsProto {
  'source'?: (string);
  'delegate'?: (string);
  'owner'?: (string);
  'multisigSigners'?: (string)[];
}

export interface ApproveAccountsProto__Output {
  'source': (string);
  'delegate': (string);
  'owner': (string);
  'multisigSigners': (string)[];
}
