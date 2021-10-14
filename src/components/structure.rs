use bevy::prelude::{Bundle, SpriteSheetBundle};

use super::body::Body;

#[derive(Default)]
pub struct Structure {
    pub can_be_broken: bool,
    pub can_be_walked_on: bool,
    pub current_damage: usize,
    pub structure_damage_sprites: Vec<StructureSprite>,
}

pub struct StructureSprite {
    pub texture_index: usize,
}

#[derive(Bundle)]
pub struct StructureBundle {
    pub tile_type: Structure,
    pub body: Body,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
