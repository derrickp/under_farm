use crate::configuration::crops::CropConfiguration;

use super::{action::Action, speed::Speed};
use bevy::prelude::{Bundle, SpriteSheetBundle};

pub struct PlayerName(pub String);

pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub speed: Speed,
    pub player: Player,
    pub action: Action,
    pub current_crop: Option<CropConfiguration>,

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
            current_crop: None,
            sprite: SpriteSheetBundle::default(),
        };
    }
}
