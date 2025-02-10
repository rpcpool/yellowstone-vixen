// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto


export interface WithdrawWithheldTokensFromAccountsAccountsProto {
  'mint'?: (string);
  'feeRecipient'?: (string);
  'withdrawWithheldAuthority'?: (string);
  'sourceAccounts'?: (string)[];
  'multisigSigners'?: (string)[];
}

export interface WithdrawWithheldTokensFromAccountsAccountsProto__Output {
  'mint': (string);
  'feeRecipient': (string);
  'withdrawWithheldAuthority': (string);
  'sourceAccounts': (string)[];
  'multisigSigners': (string)[];
}
