use bevy::prelude::{Bundle, SpriteSheetBundle};

use super::structure::Structure;

#[derive(Default)]
pub struct WallTile {
    pub can_be_broken: bool,
}

#[derive(Bundle)]
pub struct WallTileBundle {
    pub cell_type: WallTile,
    pub collide: Structure,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
