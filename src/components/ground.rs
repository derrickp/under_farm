use bevy::prelude::{Bundle, SpriteSheetBundle};

use super::body::Body;

pub struct GroundTile;

#[derive(Bundle)]
pub struct GroundTileBundle {
    pub cell_type: GroundTile,
    pub collide: Body,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
