#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Setup,
    TexturesLoaded,
    SpritesLoaded,
    Playing,
}
