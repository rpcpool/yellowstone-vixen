#[derive(Debug, Clone)]
pub enum RaydiumProgramState {
    AmmConfig,
    OperationState,
    ObservationState,
    PersonalPositionState,
    PoolState,
    ProtocolPositionState,
    TickArrayState,
    TickArrayBitmapExtension,
}
