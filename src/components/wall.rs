use bevy::prelude::{Bundle, SpriteSheetBundle};

use super::body::Body;

#[derive(Default)]
pub struct WallTile {
    pub can_be_broken: bool,
}

#[derive(Bundle)]
pub struct WallTileBundle {
    pub cell_type: WallTile,
    pub collide: Body,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
