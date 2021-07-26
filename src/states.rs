#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Setup,
    Finished,
    Playing,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GameState {
    pub camera_zoom_initialized: bool,
}
