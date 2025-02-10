// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/accounts.proto

export const AccountStateProto = {
  Uninitialized: 'Uninitialized',
  Initialized: 'Initialized',
  Frozen: 'Frozen',
} as const;

export type AccountStateProto =
  | 'Uninitialized'
  | 0
  | 'Initialized'
  | 1
  | 'Frozen'
  | 2

export type AccountStateProto__Output = typeof AccountStateProto[keyof typeof AccountStateProto]
