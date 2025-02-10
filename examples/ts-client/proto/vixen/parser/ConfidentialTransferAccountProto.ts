// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

import type { Long } from '@grpc/proto-loader';

export interface ConfidentialTransferAccountProto {
  'approved'?: (boolean);
  'elgamalPubkey'?: (string);
  'pendingBalanceLo'?: (string);
  'pendingBalanceHi'?: (string);
  'pendingBalance'?: (string);
  'availableBalance'?: (string);
  'decryptableAvailableBalance'?: (string);
  'allowConfidentialCredits'?: (boolean);
  'pendingBalanceCreditCounter'?: (number | string | Long);
  'maximumPendingBalanceCreditCounter'?: (number | string | Long);
  'expectedPendingBalanceCreditCounter'?: (number | string | Long);
  'actualPendingBalanceCreditCounter'?: (number | string | Long);
}

export interface ConfidentialTransferAccountProto__Output {
  'approved': (boolean);
  'elgamalPubkey': (string);
  'pendingBalanceLo': (string);
  'pendingBalanceHi': (string);
  'pendingBalance': (string);
  'availableBalance': (string);
  'decryptableAvailableBalance': (string);
  'allowConfidentialCredits': (boolean);
  'pendingBalanceCreditCounter': (string);
  'maximumPendingBalanceCreditCounter': (string);
  'expectedPendingBalanceCreditCounter': (string);
  'actualPendingBalanceCreditCounter': (string);
}
