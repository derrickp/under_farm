use super::{
    action::Action,
    movement::{Direction, Speed},
    tool::Tool,
};
use bevy::prelude::{Bundle, SpriteSheetBundle};

pub struct PlayerName(pub String);

pub struct PlayerInventory {
    pub current_crop_selection: Option<usize>,
    pub current_tool: Option<Tool>,
    pub current_tool_selection: Option<usize>,
}

pub struct Player;

pub struct PlayerMovement {
    pub speed: Speed,
    pub direction: Direction,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub player_movement: PlayerMovement,
    pub player: Player,
    pub action: Action,
    pub inventory: PlayerInventory,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        return PlayerBundle {
            name: PlayerName("Goblin?!".to_string()),
            player_movement: PlayerMovement {
                direction: Direction::None,
                speed: Speed::default(),
            },
            player: Player,
            action: Action::default(),
            inventory: PlayerInventory {
                current_crop_selection: None,
                current_tool: None,
                current_tool_selection: None,
            },
            sprite: SpriteSheetBundle::default(),
        };
    }
}
