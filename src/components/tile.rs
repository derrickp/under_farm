use bevy::prelude::{Bundle, SpriteSheetBundle};

pub struct Tile;

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
