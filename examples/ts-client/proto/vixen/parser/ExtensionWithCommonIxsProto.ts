// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/solana-token/extensions.proto

export const ExtensionWithCommonIxsProto = {
  CPI_GUARD: 'CPI_GUARD',
  DEFAULT_ACCOUNT_STATE: 'DEFAULT_ACCOUNT_STATE',
  INTEREST_BEARING_MINT: 'INTEREST_BEARING_MINT',
  MEMO_TRANSFER: 'MEMO_TRANSFER',
  GROUP_MEMBER_POINTER: 'GROUP_MEMBER_POINTER',
  GROUP_POINTER: 'GROUP_POINTER',
  METADATA_POINTER: 'METADATA_POINTER',
  TRANSFER_HOOK: 'TRANSFER_HOOK',
} as const;

export type ExtensionWithCommonIxsProto =
  | 'CPI_GUARD'
  | 0
  | 'DEFAULT_ACCOUNT_STATE'
  | 1
  | 'INTEREST_BEARING_MINT'
  | 2
  | 'MEMO_TRANSFER'
  | 3
  | 'GROUP_MEMBER_POINTER'
  | 4
  | 'GROUP_POINTER'
  | 5
  | 'METADATA_POINTER'
  | 6
  | 'TRANSFER_HOOK'
  | 7

export type ExtensionWithCommonIxsProto__Output = typeof ExtensionWithCommonIxsProto[keyof typeof ExtensionWithCommonIxsProto]
