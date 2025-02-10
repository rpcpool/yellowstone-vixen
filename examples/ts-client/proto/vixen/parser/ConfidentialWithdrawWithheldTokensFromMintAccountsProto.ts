// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto


export interface ConfidentialWithdrawWithheldTokensFromMintAccountsProto {
  'mint'?: (string);
  'feeRecipient'?: (string);
  'sysvar'?: (string);
  'withdrawWithheldAuthority'?: (string);
  'multisigSigners'?: (string)[];
}

export interface ConfidentialWithdrawWithheldTokensFromMintAccountsProto__Output {
  'mint': (string);
  'feeRecipient': (string);
  'sysvar': (string);
  'withdrawWithheldAuthority': (string);
  'multisigSigners': (string)[];
}
