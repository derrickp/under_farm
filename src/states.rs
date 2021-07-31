use bevy::prelude::Entity;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Startup,
    FinishedLoading,
    InGame,
    InventoryScreen,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    pub initial_spawn_complete: bool,
    pub game_camera: Option<Entity>,
}

impl Default for GameState {
    fn default() -> Self {
        return GameState {
            initial_spawn_complete: false,
            game_camera: None,
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
