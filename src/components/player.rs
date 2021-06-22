use super::player_name::PlayerName;
use bevy::prelude::{Bundle, SpriteSheetBundle};

#[derive(Bundle)]
pub struct Player {
    pub name: PlayerName,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
