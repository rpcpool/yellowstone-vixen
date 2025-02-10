import { vixen } from "./compiled";

export const decodeInnerMap = {
  TokenProgramStateProto: vixen.parser.TokenProgramStateProto.decode,
  TokenProgramIxProto: vixen.parser.TokenProgramIxProto.decode,
  TokenExtensionStateProto: vixen.parser.TokenExtensionStateProto.decode,
  TokenExtensionProgramIxProto:
    vixen.parser.TokenExtensionProgramIxProto.decode,
  OrcaProgramStateProto: vixen.parser.OrcaProgramStateProto.decode,
  OrcaProgramIxProto: vixen.parser.OrcaProgramIxProto.decode,
  RaydiumProgramStateProto: vixen.parser.RaydiumProgramStateProto.decode,
  RaydiumProgramIxProto: vixen.parser.RaydiumProgramIxProto.decode,
  TokenAccountProto: vixen.parser.TokenAccountProto.decode,
  MintProto: vixen.parser.MintProto.decode,
  MultisigProto: vixen.parser.MultisigProto.decode,
  ImmutableOwnerProto: vixen.parser.ImmutableOwnerProto.decode,
  TransferFeeAmountProto: vixen.parser.TransferFeeAmountProto.decode,
  ConfidentialTransferAccountProto:
    vixen.parser.ConfidentialTransferAccountProto.decode,
  MemoTransferProto: vixen.parser.MemoTransferProto.decode,
  NonTransferableAccountProto: vixen.parser.NonTransferableAccountProto.decode,
  TransferHookAccountProto: vixen.parser.TransferHookAccountProto.decode,
  CpiGuardProto: vixen.parser.CpiGuardProto.decode,
  ConfidentialTransferFeeAmountProto:
    vixen.parser.ConfidentialTransferFeeAmountProto.decode,
  TransferFeeProto: vixen.parser.TransferFeeProto.decode,
  TransferFeeConfigProto: vixen.parser.TransferFeeConfigProto.decode,
  MintCloseAuthorityProto: vixen.parser.MintCloseAuthorityProto.decode,
  ConfidentialTransferMintProto:
    vixen.parser.ConfidentialTransferMintProto.decode,
  DefaultAccountStateProto: vixen.parser.DefaultAccountStateProto.decode,
  NonTransferableProto: vixen.parser.NonTransferableProto.decode,
  InterestBearingConfigProto: vixen.parser.InterestBearingConfigProto.decode,
  PermanentDelegateProto: vixen.parser.PermanentDelegateProto.decode,
  TransferHookProto: vixen.parser.TransferHookProto.decode,
  ConfidentialTransferFeeConfigProto:
    vixen.parser.ConfidentialTransferFeeConfigProto.decode,
  MetadataPointerProto: vixen.parser.MetadataPointerProto.decode,
  KeyValue: vixen.parser.KeyValue.decode,
  TokenMetadataProto: vixen.parser.TokenMetadataProto.decode,
  GroupPointerProto: vixen.parser.GroupPointerProto.decode,
  TokenGroupProto: vixen.parser.TokenGroupProto.decode,
  GroupMemberPointerProto: vixen.parser.GroupMemberPointerProto.decode,
  TokenGroupMemberProto: vixen.parser.TokenGroupMemberProto.decode,
  ExtensionDataProto: vixen.parser.ExtensionDataProto.decode,
  ExtendedTokenAccountProto: vixen.parser.ExtendedTokenAccountProto.decode,
  ExtendedMintProto: vixen.parser.ExtendedMintProto.decode,
  TransferAccountsProto: vixen.parser.TransferAccountsProto.decode,
  TransferDataProto: vixen.parser.TransferDataProto.decode,
  TransferIxProto: vixen.parser.TransferIxProto.decode,
  InitializeMintAccountsProto: vixen.parser.InitializeMintAccountsProto.decode,
  InitializeMintDataProto: vixen.parser.InitializeMintDataProto.decode,
  InitializeMintIxProto: vixen.parser.InitializeMintIxProto.decode,
  InitializeAccountAccountsProto:
    vixen.parser.InitializeAccountAccountsProto.decode,
  InitializeAccountDataProto: vixen.parser.InitializeAccountDataProto.decode,
  InitializeAccountIxProto: vixen.parser.InitializeAccountIxProto.decode,
  InitializeAccount2AccountsProto:
    vixen.parser.InitializeAccount2AccountsProto.decode,
  InitializeAccountData2Proto: vixen.parser.InitializeAccountData2Proto.decode,
  InitializeAccount2IxProto: vixen.parser.InitializeAccount2IxProto.decode,
  InitializeAccount3IxProto: vixen.parser.InitializeAccount3IxProto.decode,
  InitializeMultisigAccountsProto:
    vixen.parser.InitializeMultisigAccountsProto.decode,
  InitializeMultisigDataProto: vixen.parser.InitializeMultisigDataProto.decode,
  InitializeMultisigIxProto: vixen.parser.InitializeMultisigIxProto.decode,
  ApproveAccountsProto: vixen.parser.ApproveAccountsProto.decode,
  ApproveDataProto: vixen.parser.ApproveDataProto.decode,
  ApproveIxProto: vixen.parser.ApproveIxProto.decode,
  RevokeAccountsProto: vixen.parser.RevokeAccountsProto.decode,
  RevokeIxProto: vixen.parser.RevokeIxProto.decode,
  SetAuthorityAccountsProto: vixen.parser.SetAuthorityAccountsProto.decode,
  SetAuthorityDataProto: vixen.parser.SetAuthorityDataProto.decode,
  SetAuthorityIxProto: vixen.parser.SetAuthorityIxProto.decode,
  MintToAccountsProto: vixen.parser.MintToAccountsProto.decode,
  MintToDataProto: vixen.parser.MintToDataProto.decode,
  MintToIxProto: vixen.parser.MintToIxProto.decode,
  BurnAccountsProto: vixen.parser.BurnAccountsProto.decode,
  BurnDataProto: vixen.parser.BurnDataProto.decode,
  BurnIxProto: vixen.parser.BurnIxProto.decode,
  CloseAccountAccountsProto: vixen.parser.CloseAccountAccountsProto.decode,
  CloseAccountIxProto: vixen.parser.CloseAccountIxProto.decode,
  FreezeAccountAccountsProto: vixen.parser.FreezeAccountAccountsProto.decode,
  FreezeAccountIxProto: vixen.parser.FreezeAccountIxProto.decode,
  ThawAccountAccountsProto: vixen.parser.ThawAccountAccountsProto.decode,
  ThawAccountIxProto: vixen.parser.ThawAccountIxProto.decode,
  TransferCheckedAccountsProto:
    vixen.parser.TransferCheckedAccountsProto.decode,
  TransferCheckedDataProto: vixen.parser.TransferCheckedDataProto.decode,
  TransferCheckedIxProto: vixen.parser.TransferCheckedIxProto.decode,
  ApproveCheckedAccountsProto: vixen.parser.ApproveCheckedAccountsProto.decode,
  ApproveCheckedDataProto: vixen.parser.ApproveCheckedDataProto.decode,
  ApproveCheckedIxProto: vixen.parser.ApproveCheckedIxProto.decode,
  MintToCheckedAccountsProto: vixen.parser.MintToCheckedAccountsProto.decode,
  MintToCheckedDataProto: vixen.parser.MintToCheckedDataProto.decode,
  MintToCheckedIxProto: vixen.parser.MintToCheckedIxProto.decode,
  BurnCheckedAccountsProto: vixen.parser.BurnCheckedAccountsProto.decode,
  BurnCheckedDataProto: vixen.parser.BurnCheckedDataProto.decode,
  BurnCheckedIxProto: vixen.parser.BurnCheckedIxProto.decode,
  SyncNativeAccountsProto: vixen.parser.SyncNativeAccountsProto.decode,
  SyncNativeIxProto: vixen.parser.SyncNativeIxProto.decode,
  GetAccountDataSizeAccountsProto:
    vixen.parser.GetAccountDataSizeAccountsProto.decode,
  GetAccountDataSizeIxProto: vixen.parser.GetAccountDataSizeIxProto.decode,
  InitializeImmutableOwnerAccountsProto:
    vixen.parser.InitializeImmutableOwnerAccountsProto.decode,
  InitializeImmutableOwnerIxProto:
    vixen.parser.InitializeImmutableOwnerIxProto.decode,
  AmountToUiAmountAccountsProto:
    vixen.parser.AmountToUiAmountAccountsProto.decode,
  AmountToUiAmountDataProto: vixen.parser.AmountToUiAmountDataProto.decode,
  AmountToUiAmountIxProto: vixen.parser.AmountToUiAmountIxProto.decode,
  UiAmountToAmountAccountsProto:
    vixen.parser.UiAmountToAmountAccountsProto.decode,
  UiAmountToAmountDataProto: vixen.parser.UiAmountToAmountDataProto.decode,
  UiAmountToAmountIxProto: vixen.parser.UiAmountToAmountIxProto.decode,
  TransferCheckedWithFeeAccountsProto:
    vixen.parser.TransferCheckedWithFeeAccountsProto.decode,
  TransferCheckedWithFeeDataProto:
    vixen.parser.TransferCheckedWithFeeDataProto.decode,
  TransferCheckedWithFeeIxProto:
    vixen.parser.TransferCheckedWithFeeIxProto.decode,
  InitializeTransferFeeConfigAccountsProto:
    vixen.parser.InitializeTransferFeeConfigAccountsProto.decode,
  InitializeTransferFeeConfigDataProto:
    vixen.parser.InitializeTransferFeeConfigDataProto.decode,
  InitializeTransferFeeConfigIxProto:
    vixen.parser.InitializeTransferFeeConfigIxProto.decode,
  WithdrawWithheldTokensFromMintAccountsProto:
    vixen.parser.WithdrawWithheldTokensFromMintAccountsProto.decode,
  WithdrawWithheldTokensFromMintIxProto:
    vixen.parser.WithdrawWithheldTokensFromMintIxProto.decode,
  WithdrawWithheldTokensFromAccountsAccountsProto:
    vixen.parser.WithdrawWithheldTokensFromAccountsAccountsProto.decode,
  WithdrawWithheldTokensFromAccountsDataProto:
    vixen.parser.WithdrawWithheldTokensFromAccountsDataProto.decode,
  WithdrawWithheldTokensFromAccountsIxProto:
    vixen.parser.WithdrawWithheldTokensFromAccountsIxProto.decode,
  HarvestWithheldTokensToMintAccountsProto:
    vixen.parser.HarvestWithheldTokensToMintAccountsProto.decode,
  HarvestWithheldTokensToMintIxProto:
    vixen.parser.HarvestWithheldTokensToMintIxProto.decode,
  SetTransferFeeAccountsProto: vixen.parser.SetTransferFeeAccountsProto.decode,
  SetTransferFeeDataProto: vixen.parser.SetTransferFeeDataProto.decode,
  SetTransferFeeIxProto: vixen.parser.SetTransferFeeIxProto.decode,
  TransferFeeIxProto: vixen.parser.TransferFeeIxProto.decode,
  InitializeAccountsProto: vixen.parser.InitializeAccountsProto.decode,
  InitializeDataProto: vixen.parser.InitializeDataProto.decode,
  InitializeIxProto: vixen.parser.InitializeIxProto.decode,
  UpdateFieldAccountsProto: vixen.parser.UpdateFieldAccountsProto.decode,
  UpdateFieldDataProto: vixen.parser.UpdateFieldDataProto.decode,
  UpdateFieldIxProto: vixen.parser.UpdateFieldIxProto.decode,
  RmoveKeyAccountsProto: vixen.parser.RmoveKeyAccountsProto.decode,
  RemoveKeyDataProto: vixen.parser.RemoveKeyDataProto.decode,
  RemoveKeyIxProto: vixen.parser.RemoveKeyIxProto.decode,
  UpdateAuthorityAccountsProto:
    vixen.parser.UpdateAuthorityAccountsProto.decode,
  UpdateAuthorityDataProto: vixen.parser.UpdateAuthorityDataProto.decode,
  UpdateAuthorityIxProto: vixen.parser.UpdateAuthorityIxProto.decode,
  EmitAccountsProto: vixen.parser.EmitAccountsProto.decode,
  EmitDataProto: vixen.parser.EmitDataProto.decode,
  EmitIxProto: vixen.parser.EmitIxProto.decode,
  TokenMetadataIxProto: vixen.parser.TokenMetadataIxProto.decode,
  InitializeGroupAccountsProto:
    vixen.parser.InitializeGroupAccountsProto.decode,
  InitializeGroupDataProto: vixen.parser.InitializeGroupDataProto.decode,
  InitializeGroupIxProto: vixen.parser.InitializeGroupIxProto.decode,
  UpdateGroupMaxSizeAccountsProto:
    vixen.parser.UpdateGroupMaxSizeAccountsProto.decode,
  UpdateGroupMaxSizeDataProto: vixen.parser.UpdateGroupMaxSizeDataProto.decode,
  UpdateGroupMaxSizeIxProto: vixen.parser.UpdateGroupMaxSizeIxProto.decode,
  UpdateGroupAuthorityAccountsProto:
    vixen.parser.UpdateGroupAuthorityAccountsProto.decode,
  UpdateGroupAuthorityDataProto:
    vixen.parser.UpdateGroupAuthorityDataProto.decode,
  UpdateGroupAuthorityIxProto: vixen.parser.UpdateGroupAuthorityIxProto.decode,
  InitializeMemberAccountsProto:
    vixen.parser.InitializeMemberAccountsProto.decode,
  InitializeMemberIxProto: vixen.parser.InitializeMemberIxProto.decode,
  TokenGroupIxProto: vixen.parser.TokenGroupIxProto.decode,
  InitializeConfidentialMintAccountsProto:
    vixen.parser.InitializeConfidentialMintAccountsProto.decode,
  InitializeConfidentialMintIxProto:
    vixen.parser.InitializeConfidentialMintIxProto.decode,
  UpdateMintAccountsProto: vixen.parser.UpdateMintAccountsProto.decode,
  UpdateMintIxProto: vixen.parser.UpdateMintIxProto.decode,
  ConfigureAccountAccountsProto:
    vixen.parser.ConfigureAccountAccountsProto.decode,
  ConfigureAccountIxProto: vixen.parser.ConfigureAccountIxProto.decode,
  ApproveAccountAccountsProto: vixen.parser.ApproveAccountAccountsProto.decode,
  ApproveAccountIxProto: vixen.parser.ApproveAccountIxProto.decode,
  EmptyAccountAccountsProto: vixen.parser.EmptyAccountAccountsProto.decode,
  EmptyAccountIxProto: vixen.parser.EmptyAccountIxProto.decode,
  DepositAccountsProto: vixen.parser.DepositAccountsProto.decode,
  DepositIxProto: vixen.parser.DepositIxProto.decode,
  WithdrawAccountsProto: vixen.parser.WithdrawAccountsProto.decode,
  WithdrawIxProto: vixen.parser.WithdrawIxProto.decode,
  ConfidentialTransferAccountsProto:
    vixen.parser.ConfidentialTransferAccountsProto.decode,
  ConfidentialTransferIxProto: vixen.parser.ConfidentialTransferIxProto.decode,
  ApplyPendingBalanceAccountsProto:
    vixen.parser.ApplyPendingBalanceAccountsProto.decode,
  ApplyPendingBalanceIxProto: vixen.parser.ApplyPendingBalanceIxProto.decode,
  CreditsAccountsProto: vixen.parser.CreditsAccountsProto.decode,
  EnableConfidentialCreditsIxProto:
    vixen.parser.EnableConfidentialCreditsIxProto.decode,
  DisableConfidentialCreditsIxProto:
    vixen.parser.DisableConfidentialCreditsIxProto.decode,
  EnableNonConfidentialCreditsIxProto:
    vixen.parser.EnableNonConfidentialCreditsIxProto.decode,
  DisableNonConfidentialCreditsIxProto:
    vixen.parser.DisableNonConfidentialCreditsIxProto.decode,
  TransferWithSplitProofsAccountsProto:
    vixen.parser.TransferWithSplitProofsAccountsProto.decode,
  TransferWithSplitProofsIxProto:
    vixen.parser.TransferWithSplitProofsIxProto.decode,
  ConfidentialTransferExtIxProto:
    vixen.parser.ConfidentialTransferExtIxProto.decode,
  InitializeConfidentialTransferFeeConfigAccountsProto:
    vixen.parser.InitializeConfidentialTransferFeeConfigAccountsProto.decode,
  InitializeConfidentialTransferFeeConfigIxProto:
    vixen.parser.InitializeConfidentialTransferFeeConfigIxProto.decode,
  ConfidentialWithdrawWithheldTokensFromMintAccountsProto:
    vixen.parser.ConfidentialWithdrawWithheldTokensFromMintAccountsProto.decode,
  ConfidentialWithdrawWithheldTokensFromMintIxProto:
    vixen.parser.ConfidentialWithdrawWithheldTokensFromMintIxProto.decode,
  ConfidentialWithdrawWithheldTokensFromAccountsAccountsProto:
    vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsAccountsProto
      .decode,
  ConfidentialWithdrawWithheldTokensFromAccountsIxProto:
    vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsIxProto.decode,
  ConfidentialHarvestWithheldTokensToMintAccountsProto:
    vixen.parser.ConfidentialHarvestWithheldTokensToMintAccountsProto.decode,
  ConfidentialHarvestWithheldTokensToMintIxProto:
    vixen.parser.ConfidentialHarvestWithheldTokensToMintIxProto.decode,
  EnableHarvestToMintAccountsProto:
    vixen.parser.EnableHarvestToMintAccountsProto.decode,
  DisableHarvestToMintAccountsProto:
    vixen.parser.DisableHarvestToMintAccountsProto.decode,
  EnableHarvestToMintIxProto: vixen.parser.EnableHarvestToMintIxProto.decode,
  DisableHarvestToMintIxProto: vixen.parser.DisableHarvestToMintIxProto.decode,
  ConfidentialTransferFeeIxProto:
    vixen.parser.ConfidentialTransferFeeIxProto.decode,
  ExtInitializeAccountsProto: vixen.parser.ExtInitializeAccountsProto.decode,
  ExtInitializeIxProto: vixen.parser.ExtInitializeIxProto.decode,
  UpdateAccountsProto: vixen.parser.UpdateAccountsProto.decode,
  UpdateIxProto: vixen.parser.UpdateIxProto.decode,
  EnableAccountsProto: vixen.parser.EnableAccountsProto.decode,
  EnableIxProto: vixen.parser.EnableIxProto.decode,
  DisableAccountsProto: vixen.parser.DisableAccountsProto.decode,
  DisableIxProto: vixen.parser.DisableIxProto.decode,
  CommonExtensionIxProto: vixen.parser.CommonExtensionIxProto.decode,
  CommonExtensionIxsProto: vixen.parser.CommonExtensionIxsProto.decode,
  CpiGuardIxProto: vixen.parser.CpiGuardIxProto.decode,
  DefaultAccountStateIxProto: vixen.parser.DefaultAccountStateIxProto.decode,
  InterestBearingMintIxProto: vixen.parser.InterestBearingMintIxProto.decode,
  MemoTransferIxProto: vixen.parser.MemoTransferIxProto.decode,
  GroupMemberPointerIxProto: vixen.parser.GroupMemberPointerIxProto.decode,
  GroupPointerIxProto: vixen.parser.GroupPointerIxProto.decode,
  MetadataPointerIxProto: vixen.parser.MetadataPointerIxProto.decode,
  TransferHookIxProto: vixen.parser.TransferHookIxProto.decode,
  WithdrawExcessLamportsAccountsProto:
    vixen.parser.WithdrawExcessLamportsAccountsProto.decode,
  WithdrawExcessLamportsIxProto:
    vixen.parser.WithdrawExcessLamportsIxProto.decode,
  InitializePermanentDelegateAccountsProto:
    vixen.parser.InitializePermanentDelegateAccountsProto.decode,
  InitializePermanentDelegateDataProto:
    vixen.parser.InitializePermanentDelegateDataProto.decode,
  InitializePermanentDelegateIxProto:
    vixen.parser.InitializePermanentDelegateIxProto.decode,
  ReallocateAccountsProto: vixen.parser.ReallocateAccountsProto.decode,
  ReallocateDataProto: vixen.parser.ReallocateDataProto.decode,
  ReallocateIxProto: vixen.parser.ReallocateIxProto.decode,
  InitializeNonTransferableMintAccountsProto:
    vixen.parser.InitializeNonTransferableMintAccountsProto.decode,
  InitializeNonTransferableMintIxProto:
    vixen.parser.InitializeNonTransferableMintIxProto.decode,
  InitializeMintCloseAuthorityAccountsProto:
    vixen.parser.InitializeMintCloseAuthorityAccountsProto.decode,
  InitializeMintCloseAuthorityDataProto:
    vixen.parser.InitializeMintCloseAuthorityDataProto.decode,
  InitializeMintCloseAuthorityIxProto:
    vixen.parser.InitializeMintCloseAuthorityIxProto.decode,
  CreateNativeMintAccountsProto:
    vixen.parser.CreateNativeMintAccountsProto.decode,
  CreateNativeMintIxProto: vixen.parser.CreateNativeMintIxProto.decode,
  WhirlpoolRewardInfoProto: vixen.parser.WhirlpoolRewardInfoProto.decode,
  WhirlpoolProto: vixen.parser.WhirlpoolProto.decode,
  WhirlpoolsConfigProto: vixen.parser.WhirlpoolsConfigProto.decode,
  FeeTierProto: vixen.parser.FeeTierProto.decode,
  PositionProto: vixen.parser.PositionProto.decode,
  OrcaPositionRewardInfoProto: vixen.parser.OrcaPositionRewardInfoProto.decode,
  OrcaTickProto: vixen.parser.OrcaTickProto.decode,
  OrcaTickArrayProto: vixen.parser.OrcaTickArrayProto.decode,
  OrcaSwapAccountsProto: vixen.parser.OrcaSwapAccountsProto.decode,
  OrcaSwapIxDataProto: vixen.parser.OrcaSwapIxDataProto.decode,
  OrcaSwapV2AccountsProto: vixen.parser.OrcaSwapV2AccountsProto.decode,
  OrcaSwapV2IxDataProto: vixen.parser.OrcaSwapV2IxDataProto.decode,
  OrcaSwapInstructionProto: vixen.parser.OrcaSwapInstructionProto.decode,
  OrcaSwapV2InstructionProto: vixen.parser.OrcaSwapV2InstructionProto.decode,
  AmmConfigProto: vixen.parser.AmmConfigProto.decode,
  OperationStateProto: vixen.parser.OperationStateProto.decode,
  ObservationProto: vixen.parser.ObservationProto.decode,
  ObservationStateProto: vixen.parser.ObservationStateProto.decode,
  RaydiumPositionRewardInfoProto:
    vixen.parser.RaydiumPositionRewardInfoProto.decode,
  PersonalPositionStateProto: vixen.parser.PersonalPositionStateProto.decode,
  RewardInfoProto: vixen.parser.RewardInfoProto.decode,
  PoolStateProto: vixen.parser.PoolStateProto.decode,
  ProtocolPositionStateProto: vixen.parser.ProtocolPositionStateProto.decode,
  RaydiumTickStateProto: vixen.parser.RaydiumTickStateProto.decode,
  RaydiumTickArrayStateProto: vixen.parser.RaydiumTickArrayStateProto.decode,
  TickArrayBitmapProto: vixen.parser.TickArrayBitmapProto.decode,
  TickArrayBitmapExtensionProto:
    vixen.parser.TickArrayBitmapExtensionProto.decode,
  RaydiumSwapAccountsProto: vixen.parser.RaydiumSwapAccountsProto.decode,
  RaydiumSwapV2AccountsProto: vixen.parser.RaydiumSwapV2AccountsProto.decode,
  RaydiumSwapIxDataProto: vixen.parser.RaydiumSwapIxDataProto.decode,
  RaydiumSwapInstructionProto: vixen.parser.RaydiumSwapInstructionProto.decode,
  RaydiumSwapV2InstructionProto:
    vixen.parser.RaydiumSwapV2InstructionProto.decode,
};

export type VixenParserTypesUnion =
  | vixen.parser.TokenProgramStateProto
  | vixen.parser.TokenProgramIxProto
  | vixen.parser.TokenExtensionStateProto
  | vixen.parser.TokenExtensionProgramIxProto
  | vixen.parser.OrcaProgramStateProto
  | vixen.parser.OrcaProgramIxProto
  | vixen.parser.RaydiumProgramStateProto
  | vixen.parser.RaydiumProgramIxProto
  | vixen.parser.AccountStateProto
  | vixen.parser.TokenAccountProto
  | vixen.parser.MintProto
  | vixen.parser.MultisigProto
  | vixen.parser.ImmutableOwnerProto
  | vixen.parser.TransferFeeAmountProto
  | vixen.parser.ConfidentialTransferAccountProto
  | vixen.parser.MemoTransferProto
  | vixen.parser.NonTransferableAccountProto
  | vixen.parser.TransferHookAccountProto
  | vixen.parser.CpiGuardProto
  | vixen.parser.ConfidentialTransferFeeAmountProto
  | vixen.parser.TransferFeeProto
  | vixen.parser.TransferFeeConfigProto
  | vixen.parser.MintCloseAuthorityProto
  | vixen.parser.ConfidentialTransferMintProto
  | vixen.parser.DefaultAccountStateProto
  | vixen.parser.NonTransferableProto
  | vixen.parser.InterestBearingConfigProto
  | vixen.parser.PermanentDelegateProto
  | vixen.parser.TransferHookProto
  | vixen.parser.ConfidentialTransferFeeConfigProto
  | vixen.parser.MetadataPointerProto
  | vixen.parser.KeyValue
  | vixen.parser.TokenMetadataProto
  | vixen.parser.GroupPointerProto
  | vixen.parser.TokenGroupProto
  | vixen.parser.GroupMemberPointerProto
  | vixen.parser.TokenGroupMemberProto
  | vixen.parser.ExtensionDataProto
  | vixen.parser.ExtendedTokenAccountProto
  | vixen.parser.ExtendedMintProto
  | vixen.parser.AuthorityType
  | vixen.parser.TransferAccountsProto
  | vixen.parser.TransferDataProto
  | vixen.parser.TransferIxProto
  | vixen.parser.InitializeMintAccountsProto
  | vixen.parser.InitializeMintDataProto
  | vixen.parser.InitializeMintIxProto
  | vixen.parser.InitializeAccountAccountsProto
  | vixen.parser.InitializeAccountDataProto
  | vixen.parser.InitializeAccountIxProto
  | vixen.parser.InitializeAccount2AccountsProto
  | vixen.parser.InitializeAccountData2Proto
  | vixen.parser.InitializeAccount2IxProto
  | vixen.parser.InitializeAccount3IxProto
  | vixen.parser.InitializeMultisigAccountsProto
  | vixen.parser.InitializeMultisigDataProto
  | vixen.parser.InitializeMultisigIxProto
  | vixen.parser.ApproveAccountsProto
  | vixen.parser.ApproveDataProto
  | vixen.parser.ApproveIxProto
  | vixen.parser.RevokeAccountsProto
  | vixen.parser.RevokeIxProto
  | vixen.parser.SetAuthorityAccountsProto
  | vixen.parser.SetAuthorityDataProto
  | vixen.parser.SetAuthorityIxProto
  | vixen.parser.MintToAccountsProto
  | vixen.parser.MintToDataProto
  | vixen.parser.MintToIxProto
  | vixen.parser.BurnAccountsProto
  | vixen.parser.BurnDataProto
  | vixen.parser.BurnIxProto
  | vixen.parser.CloseAccountAccountsProto
  | vixen.parser.CloseAccountIxProto
  | vixen.parser.FreezeAccountAccountsProto
  | vixen.parser.FreezeAccountIxProto
  | vixen.parser.ThawAccountAccountsProto
  | vixen.parser.ThawAccountIxProto
  | vixen.parser.TransferCheckedAccountsProto
  | vixen.parser.TransferCheckedDataProto
  | vixen.parser.TransferCheckedIxProto
  | vixen.parser.ApproveCheckedAccountsProto
  | vixen.parser.ApproveCheckedDataProto
  | vixen.parser.ApproveCheckedIxProto
  | vixen.parser.MintToCheckedAccountsProto
  | vixen.parser.MintToCheckedDataProto
  | vixen.parser.MintToCheckedIxProto
  | vixen.parser.BurnCheckedAccountsProto
  | vixen.parser.BurnCheckedDataProto
  | vixen.parser.BurnCheckedIxProto
  | vixen.parser.SyncNativeAccountsProto
  | vixen.parser.SyncNativeIxProto
  | vixen.parser.GetAccountDataSizeAccountsProto
  | vixen.parser.GetAccountDataSizeIxProto
  | vixen.parser.InitializeImmutableOwnerAccountsProto
  | vixen.parser.InitializeImmutableOwnerIxProto
  | vixen.parser.AmountToUiAmountAccountsProto
  | vixen.parser.AmountToUiAmountDataProto
  | vixen.parser.AmountToUiAmountIxProto
  | vixen.parser.UiAmountToAmountAccountsProto
  | vixen.parser.UiAmountToAmountDataProto
  | vixen.parser.UiAmountToAmountIxProto
  | vixen.parser.TransferCheckedWithFeeAccountsProto
  | vixen.parser.TransferCheckedWithFeeDataProto
  | vixen.parser.TransferCheckedWithFeeIxProto
  | vixen.parser.InitializeTransferFeeConfigAccountsProto
  | vixen.parser.InitializeTransferFeeConfigDataProto
  | vixen.parser.InitializeTransferFeeConfigIxProto
  | vixen.parser.WithdrawWithheldTokensFromMintAccountsProto
  | vixen.parser.WithdrawWithheldTokensFromMintIxProto
  | vixen.parser.WithdrawWithheldTokensFromAccountsAccountsProto
  | vixen.parser.WithdrawWithheldTokensFromAccountsDataProto
  | vixen.parser.WithdrawWithheldTokensFromAccountsIxProto
  | vixen.parser.HarvestWithheldTokensToMintAccountsProto
  | vixen.parser.HarvestWithheldTokensToMintIxProto
  | vixen.parser.SetTransferFeeAccountsProto
  | vixen.parser.SetTransferFeeDataProto
  | vixen.parser.SetTransferFeeIxProto
  | vixen.parser.TransferFeeIxProto
  | vixen.parser.InitializeAccountsProto
  | vixen.parser.InitializeDataProto
  | vixen.parser.InitializeIxProto
  | vixen.parser.UpdateFieldAccountsProto
  | vixen.parser.UpdateFieldDataProto
  | vixen.parser.UpdateFieldIxProto
  | vixen.parser.RmoveKeyAccountsProto
  | vixen.parser.RemoveKeyDataProto
  | vixen.parser.RemoveKeyIxProto
  | vixen.parser.UpdateAuthorityAccountsProto
  | vixen.parser.UpdateAuthorityDataProto
  | vixen.parser.UpdateAuthorityIxProto
  | vixen.parser.EmitAccountsProto
  | vixen.parser.EmitDataProto
  | vixen.parser.EmitIxProto
  | vixen.parser.TokenMetadataIxProto
  | vixen.parser.InitializeGroupAccountsProto
  | vixen.parser.InitializeGroupDataProto
  | vixen.parser.InitializeGroupIxProto
  | vixen.parser.UpdateGroupMaxSizeAccountsProto
  | vixen.parser.UpdateGroupMaxSizeDataProto
  | vixen.parser.UpdateGroupMaxSizeIxProto
  | vixen.parser.UpdateGroupAuthorityAccountsProto
  | vixen.parser.UpdateGroupAuthorityDataProto
  | vixen.parser.UpdateGroupAuthorityIxProto
  | vixen.parser.InitializeMemberAccountsProto
  | vixen.parser.InitializeMemberIxProto
  | vixen.parser.TokenGroupIxProto
  | vixen.parser.InitializeConfidentialMintAccountsProto
  | vixen.parser.InitializeConfidentialMintIxProto
  | vixen.parser.UpdateMintAccountsProto
  | vixen.parser.UpdateMintIxProto
  | vixen.parser.ConfigureAccountAccountsProto
  | vixen.parser.ConfigureAccountIxProto
  | vixen.parser.ApproveAccountAccountsProto
  | vixen.parser.ApproveAccountIxProto
  | vixen.parser.EmptyAccountAccountsProto
  | vixen.parser.EmptyAccountIxProto
  | vixen.parser.DepositAccountsProto
  | vixen.parser.DepositIxProto
  | vixen.parser.WithdrawAccountsProto
  | vixen.parser.WithdrawIxProto
  | vixen.parser.ConfidentialTransferAccountsProto
  | vixen.parser.ConfidentialTransferIxProto
  | vixen.parser.ApplyPendingBalanceAccountsProto
  | vixen.parser.ApplyPendingBalanceIxProto
  | vixen.parser.CreditsAccountsProto
  | vixen.parser.EnableConfidentialCreditsIxProto
  | vixen.parser.DisableConfidentialCreditsIxProto
  | vixen.parser.EnableNonConfidentialCreditsIxProto
  | vixen.parser.DisableNonConfidentialCreditsIxProto
  | vixen.parser.TransferWithSplitProofsAccountsProto
  | vixen.parser.TransferWithSplitProofsIxProto
  | vixen.parser.ConfidentialTransferExtIxProto
  | vixen.parser.InitializeConfidentialTransferFeeConfigAccountsProto
  | vixen.parser.InitializeConfidentialTransferFeeConfigIxProto
  | vixen.parser.ConfidentialWithdrawWithheldTokensFromMintAccountsProto
  | vixen.parser.ConfidentialWithdrawWithheldTokensFromMintIxProto
  | vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsAccountsProto
  | vixen.parser.ConfidentialWithdrawWithheldTokensFromAccountsIxProto
  | vixen.parser.ConfidentialHarvestWithheldTokensToMintAccountsProto
  | vixen.parser.ConfidentialHarvestWithheldTokensToMintIxProto
  | vixen.parser.EnableHarvestToMintAccountsProto
  | vixen.parser.DisableHarvestToMintAccountsProto
  | vixen.parser.EnableHarvestToMintIxProto
  | vixen.parser.DisableHarvestToMintIxProto
  | vixen.parser.ConfidentialTransferFeeIxProto
  | vixen.parser.ExtInitializeAccountsProto
  | vixen.parser.ExtInitializeIxProto
  | vixen.parser.UpdateAccountsProto
  | vixen.parser.UpdateIxProto
  | vixen.parser.EnableAccountsProto
  | vixen.parser.EnableIxProto
  | vixen.parser.DisableAccountsProto
  | vixen.parser.DisableIxProto
  | vixen.parser.CommonExtensionIxProto
  | vixen.parser.ExtensionWithCommonIxsProto
  | vixen.parser.CommonExtensionIxsProto
  | vixen.parser.CpiGuardIxProto
  | vixen.parser.DefaultAccountStateIxProto
  | vixen.parser.InterestBearingMintIxProto
  | vixen.parser.MemoTransferIxProto
  | vixen.parser.GroupMemberPointerIxProto
  | vixen.parser.GroupPointerIxProto
  | vixen.parser.MetadataPointerIxProto
  | vixen.parser.TransferHookIxProto
  | vixen.parser.WithdrawExcessLamportsAccountsProto
  | vixen.parser.WithdrawExcessLamportsIxProto
  | vixen.parser.InitializePermanentDelegateAccountsProto
  | vixen.parser.InitializePermanentDelegateDataProto
  | vixen.parser.InitializePermanentDelegateIxProto
  | vixen.parser.ReallocateAccountsProto
  | vixen.parser.ExtensionType
  | vixen.parser.ReallocateDataProto
  | vixen.parser.ReallocateIxProto
  | vixen.parser.InitializeNonTransferableMintAccountsProto
  | vixen.parser.InitializeNonTransferableMintIxProto
  | vixen.parser.InitializeMintCloseAuthorityAccountsProto
  | vixen.parser.InitializeMintCloseAuthorityDataProto
  | vixen.parser.InitializeMintCloseAuthorityIxProto
  | vixen.parser.CreateNativeMintAccountsProto
  | vixen.parser.CreateNativeMintIxProto
  | vixen.parser.WhirlpoolRewardInfoProto
  | vixen.parser.WhirlpoolProto
  | vixen.parser.WhirlpoolsConfigProto
  | vixen.parser.FeeTierProto
  | vixen.parser.PositionProto
  | vixen.parser.OrcaPositionRewardInfoProto
  | vixen.parser.OrcaTickProto
  | vixen.parser.OrcaTickArrayProto
  | vixen.parser.OrcaSwapAccountsProto
  | vixen.parser.OrcaSwapIxDataProto
  | vixen.parser.OrcaSwapV2AccountsProto
  | vixen.parser.OrcaSwapV2IxDataProto
  | vixen.parser.OrcaSwapInstructionProto
  | vixen.parser.OrcaSwapV2InstructionProto
  | vixen.parser.AmmConfigProto
  | vixen.parser.OperationStateProto
  | vixen.parser.ObservationProto
  | vixen.parser.ObservationStateProto
  | vixen.parser.RaydiumPositionRewardInfoProto
  | vixen.parser.PersonalPositionStateProto
  | vixen.parser.RewardInfoProto
  | vixen.parser.PoolStateProto
  | vixen.parser.ProtocolPositionStateProto
  | vixen.parser.RaydiumTickStateProto
  | vixen.parser.RaydiumTickArrayStateProto
  | vixen.parser.TickArrayBitmapProto
  | vixen.parser.TickArrayBitmapExtensionProto
  | vixen.parser.RaydiumSwapAccountsProto
  | vixen.parser.RaydiumSwapV2AccountsProto
  | vixen.parser.RaydiumSwapIxDataProto
  | vixen.parser.RaydiumSwapInstructionProto
  | vixen.parser.RaydiumSwapV2InstructionProto;
