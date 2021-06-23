use super::speed::Speed;
use bevy::prelude::{Bundle, SpriteSheetBundle};

pub struct PlayerName(pub String);

pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PlayerName,
    pub speed: Speed,
    pub player: Player,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
