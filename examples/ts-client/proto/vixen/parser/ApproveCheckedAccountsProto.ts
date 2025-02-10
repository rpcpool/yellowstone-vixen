// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto


export interface ApproveCheckedAccountsProto {
  'source'?: (string);
  'mint'?: (string);
  'delegate'?: (string);
  'owner'?: (string);
  'multisigSigners'?: (string)[];
}

export interface ApproveCheckedAccountsProto__Output {
  'source': (string);
  'mint': (string);
  'delegate': (string);
  'owner': (string);
  'multisigSigners': (string)[];
}
