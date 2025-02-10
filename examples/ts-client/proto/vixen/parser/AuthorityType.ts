// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/ixs.proto

export const AuthorityType = {
  MINT_TOKEN: 'MINT_TOKEN',
  FREEZE_ACCOUNT: 'FREEZE_ACCOUNT',
  ACCOUNT_OWNER: 'ACCOUNT_OWNER',
  CLOSE_ACCOUNT: 'CLOSE_ACCOUNT',
} as const;

export type AuthorityType =
  | 'MINT_TOKEN'
  | 0
  | 'FREEZE_ACCOUNT'
  | 1
  | 'ACCOUNT_OWNER'
  | 2
  | 'CLOSE_ACCOUNT'
  | 3

export type AuthorityType__Output = typeof AuthorityType[keyof typeof AuthorityType]
