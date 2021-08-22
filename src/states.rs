use bevy::{math::Vec3, prelude::Entity};

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

#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub initial_spawn_complete: bool,
    pub game_camera: Option<Entity>,
    pub game_camera_scale: Vec3,
}

impl Default for GameState {
    fn default() -> Self {
        return GameState {
            initial_spawn_complete: false,
            game_camera: None,
            game_camera_scale: Vec3::splat(1.0),
        };
    }
}

pub struct InventoryState {
    pub inventory_text: Option<Vec<Entity>>,
}

impl Default for InventoryState {
    fn default() -> Self {
        return InventoryState {
            inventory_text: None,
        };
    }
}
