// Original file: /home/fernando/Documents/abklabs/yellowstone-vixen/crates/proto/proto/parser.proto

import type { AmmConfigProto as _vixen_parser_AmmConfigProto, AmmConfigProto__Output as _vixen_parser_AmmConfigProto__Output } from '../../vixen/parser/AmmConfigProto';
import type { OperationStateProto as _vixen_parser_OperationStateProto, OperationStateProto__Output as _vixen_parser_OperationStateProto__Output } from '../../vixen/parser/OperationStateProto';
import type { ObservationStateProto as _vixen_parser_ObservationStateProto, ObservationStateProto__Output as _vixen_parser_ObservationStateProto__Output } from '../../vixen/parser/ObservationStateProto';
import type { PersonalPositionStateProto as _vixen_parser_PersonalPositionStateProto, PersonalPositionStateProto__Output as _vixen_parser_PersonalPositionStateProto__Output } from '../../vixen/parser/PersonalPositionStateProto';
import type { PoolStateProto as _vixen_parser_PoolStateProto, PoolStateProto__Output as _vixen_parser_PoolStateProto__Output } from '../../vixen/parser/PoolStateProto';
import type { ProtocolPositionStateProto as _vixen_parser_ProtocolPositionStateProto, ProtocolPositionStateProto__Output as _vixen_parser_ProtocolPositionStateProto__Output } from '../../vixen/parser/ProtocolPositionStateProto';
import type { RaydiumTickArrayStateProto as _vixen_parser_RaydiumTickArrayStateProto, RaydiumTickArrayStateProto__Output as _vixen_parser_RaydiumTickArrayStateProto__Output } from '../../vixen/parser/RaydiumTickArrayStateProto';
import type { TickArrayBitmapExtensionProto as _vixen_parser_TickArrayBitmapExtensionProto, TickArrayBitmapExtensionProto__Output as _vixen_parser_TickArrayBitmapExtensionProto__Output } from '../../vixen/parser/TickArrayBitmapExtensionProto';

export interface RaydiumProgramStateProto {
  'ammConfig'?: (_vixen_parser_AmmConfigProto | null);
  'operationState'?: (_vixen_parser_OperationStateProto | null);
  'observationState'?: (_vixen_parser_ObservationStateProto | null);
  'personalPositionState'?: (_vixen_parser_PersonalPositionStateProto | null);
  'poolState'?: (_vixen_parser_PoolStateProto | null);
  'protocolPositionState'?: (_vixen_parser_ProtocolPositionStateProto | null);
  'tickArrayState'?: (_vixen_parser_RaydiumTickArrayStateProto | null);
  'tickArrayBitmapExtension'?: (_vixen_parser_TickArrayBitmapExtensionProto | null);
  'stateOneof'?: "ammConfig"|"operationState"|"observationState"|"personalPositionState"|"poolState"|"protocolPositionState"|"tickArrayState"|"tickArrayBitmapExtension";
}

export interface RaydiumProgramStateProto__Output {
  'ammConfig'?: (_vixen_parser_AmmConfigProto__Output | null);
  'operationState'?: (_vixen_parser_OperationStateProto__Output | null);
  'observationState'?: (_vixen_parser_ObservationStateProto__Output | null);
  'personalPositionState'?: (_vixen_parser_PersonalPositionStateProto__Output | null);
  'poolState'?: (_vixen_parser_PoolStateProto__Output | null);
  'protocolPositionState'?: (_vixen_parser_ProtocolPositionStateProto__Output | null);
  'tickArrayState'?: (_vixen_parser_RaydiumTickArrayStateProto__Output | null);
  'tickArrayBitmapExtension'?: (_vixen_parser_TickArrayBitmapExtensionProto__Output | null);
  'stateOneof': "ammConfig"|"operationState"|"observationState"|"personalPositionState"|"poolState"|"protocolPositionState"|"tickArrayState"|"tickArrayBitmapExtension";
}
