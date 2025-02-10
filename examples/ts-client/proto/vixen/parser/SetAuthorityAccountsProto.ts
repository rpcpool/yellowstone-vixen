// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto


export interface SetAuthorityAccountsProto {
  'currentAuthority'?: (string);
  'account'?: (string);
  'multisigSigners'?: (string)[];
}

export interface SetAuthorityAccountsProto__Output {
  'currentAuthority': (string);
  'account': (string);
  'multisigSigners': (string)[];
}
