// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

import type { Long } from '@grpc/proto-loader';

export interface InitializeTransferFeeConfigDataProto {
  'transferFeeConfigAuthority'?: (string);
  'withdrawWithheldAuthority'?: (string);
  'transferFeeBasisPoints'?: (number | string | Long);
  'maximumFee'?: (number | string | Long);
  '_transferFeeConfigAuthority'?: "transferFeeConfigAuthority";
  '_withdrawWithheldAuthority'?: "withdrawWithheldAuthority";
}

export interface InitializeTransferFeeConfigDataProto__Output {
  'transferFeeConfigAuthority'?: (string);
  'withdrawWithheldAuthority'?: (string);
  'transferFeeBasisPoints': (string);
  'maximumFee': (string);
  '_transferFeeConfigAuthority': "transferFeeConfigAuthority";
  '_withdrawWithheldAuthority': "withdrawWithheldAuthority";
}
