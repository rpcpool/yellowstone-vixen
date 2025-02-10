// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto


export interface ConfidentialTransferAccountsProto {
  'sourceAccount'?: (string);
  'mint'?: (string);
  'destination'?: (string);
  'owner'?: (string);
  'contextAccount'?: (string);
  'multisigSigners'?: (string)[];
}

export interface ConfidentialTransferAccountsProto__Output {
  'sourceAccount': (string);
  'mint': (string);
  'destination': (string);
  'owner': (string);
  'contextAccount': (string);
  'multisigSigners': (string)[];
}
