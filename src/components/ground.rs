use bevy::{
    math::{Vec2, Vec3},
    prelude::{Bundle, SpriteSheetBundle, Transform, Visible},
    sprite::TextureAtlasSprite,
};

use crate::{
    configuration::{floors::FloorConfig, map::TILE_SIZE},
    sprites::Sprites,
};

use super::body::Body;

use rand::Rng;

pub struct GroundTile;

#[derive(Bundle)]
pub struct GroundTileBundle {
    pub tile_type: GroundTile,
    pub collide: Body,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl GroundTileBundle {
    pub fn build(coordinate: &Vec2, sprites: &Sprites, floor_config: &FloorConfig) -> Self {
        let mut rng = rand::thread_rng();
        let num_options = floor_config.sprite_options.len();
        let random_index: usize = rng.gen_range(0..num_options);
        let floor_index = floor_config
            .sprite_options
            .get(random_index)
            .unwrap()
            .sprite_index
            .unwrap();
        let cell_center = Vec3::new(coordinate.x, coordinate.y, 0.0);
        Self {
            tile_type: GroundTile,
            collide: Body {
                cell_center,
                tile_size: TILE_SIZE as f32,
            },
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: cell_center,
                    scale: crate::configuration::sprites::sprite_scale(),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(floor_index),
                texture_atlas: sprites.atlas_handle.clone(),
                visible: Visible {
                    is_visible: false,
                    is_transparent: false,
                },
                ..Default::default()
            },
        }
    }
}
