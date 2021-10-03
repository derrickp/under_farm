use bevy::prelude::{Bundle, SpriteSheetBundle};

use super::structure::Structure;

pub struct GroundTile;

#[derive(Bundle)]
pub struct GroundTileBundle {
    pub cell_type: GroundTile,
    pub collide: Structure,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
