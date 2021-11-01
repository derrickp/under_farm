use bevy::math::Vec3;

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
    pub game_world_generated: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub initial_spawn_complete: bool,
    pub game_camera_scale: Vec3,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            initial_spawn_complete: false,
            game_camera_scale: Vec3::splat(1.0),
        }
    }
}
