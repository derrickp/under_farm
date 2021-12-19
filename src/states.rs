#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Startup,
    FinishedLoading,
    InGame,
    InventoryScreen,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GameLoadState {
    pub texture_load_complete: bool,
    pub textures_set: bool,
}
