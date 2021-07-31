#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Startup,
    FinishedLoading,
    InGame,
    InventoryScreen,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GameState {
}
