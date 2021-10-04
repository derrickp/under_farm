use bevy::prelude::{Bundle, SpriteSheetBundle};

use super::body::Body;

#[derive(Default)]
pub struct Structure {
    pub can_be_broken: bool,
}

#[derive(Bundle)]
pub struct StructureBundle {
    pub tile_type: Structure,
    pub collide: Body,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
