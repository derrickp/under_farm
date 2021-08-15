use super::{action::Action, movement::Speed};
use bevy::prelude::{Bundle, SpriteSheetBundle};

pub struct PlayerName(pub String);

pub struct PlayerInventory {
    pub current_crop_selection: Option<usize>,
}

pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub speed: Speed,
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
            speed: Speed::default(),
            player: Player,
            action: Action::default(),
            inventory: PlayerInventory {
                current_crop_selection: None,
            },
            sprite: SpriteSheetBundle::default(),
        };
    }
}
