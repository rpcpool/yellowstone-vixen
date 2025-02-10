// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto


export interface TransferWithSplitProofsAccountsProto {
  'sourceAccount'?: (string);
  'mint'?: (string);
  'destination'?: (string);
  'verifyCiphertextCommitmentEqualityProof'?: (string);
  'verifyBatchedGroupedCipherText_2HandlesValidityProof'?: (string);
  'verifyBatchedRangeProofU128'?: (string);
  'verifyBatchedRangeProofU256'?: (string);
  'verifyBatchedGroupedCipherText_2HandlesValidityProofNext'?: (string);
  'verifyFeeSigmaProof'?: (string);
  'destinationAccountForLamports'?: (string);
  'contextStateAccountOwner'?: (string);
  'zkTokenProofProgram'?: (string);
  'owner'?: (string);
  '_verifyBatchedRangeProofU128'?: "verifyBatchedRangeProofU128";
  '_verifyBatchedRangeProofU256'?: "verifyBatchedRangeProofU256";
  '_verifyBatchedGroupedCipherText_2HandlesValidityProofNext'?: "verifyBatchedGroupedCipherText_2HandlesValidityProofNext";
  '_verifyFeeSigmaProof'?: "verifyFeeSigmaProof";
  '_destinationAccountForLamports'?: "destinationAccountForLamports";
  '_contextStateAccountOwner'?: "contextStateAccountOwner";
  '_zkTokenProofProgram'?: "zkTokenProofProgram";
  '_owner'?: "owner";
}

export interface TransferWithSplitProofsAccountsProto__Output {
  'sourceAccount': (string);
  'mint': (string);
  'destination': (string);
  'verifyCiphertextCommitmentEqualityProof': (string);
  'verifyBatchedGroupedCipherText_2HandlesValidityProof': (string);
  'verifyBatchedRangeProofU128'?: (string);
  'verifyBatchedRangeProofU256'?: (string);
  'verifyBatchedGroupedCipherText_2HandlesValidityProofNext'?: (string);
  'verifyFeeSigmaProof'?: (string);
  'destinationAccountForLamports'?: (string);
  'contextStateAccountOwner'?: (string);
  'zkTokenProofProgram'?: (string);
  'owner'?: (string);
  '_verifyBatchedRangeProofU128': "verifyBatchedRangeProofU128";
  '_verifyBatchedRangeProofU256': "verifyBatchedRangeProofU256";
  '_verifyBatchedGroupedCipherText_2HandlesValidityProofNext': "verifyBatchedGroupedCipherText_2HandlesValidityProofNext";
  '_verifyFeeSigmaProof': "verifyFeeSigmaProof";
  '_destinationAccountForLamports': "destinationAccountForLamports";
  '_contextStateAccountOwner': "contextStateAccountOwner";
  '_zkTokenProofProgram': "zkTokenProofProgram";
  '_owner': "owner";
}
