// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto


export interface WithdrawWithheldTokensFromMintAccountsProto {
  'mint'?: (string);
  'feeRecipient'?: (string);
  'withdrawWithheldAuthority'?: (string);
  'multisigSigners'?: (string)[];
}

export interface WithdrawWithheldTokensFromMintAccountsProto__Output {
  'mint': (string);
  'feeRecipient': (string);
  'withdrawWithheldAuthority': (string);
  'multisigSigners': (string)[];
}
