use bevy::{
    math::Vec3,
    prelude::{Bundle, Component, SpriteSheetBundle, Transform, Visibility},
    sprite::TextureAtlasSprite,
    utils::default,
};

use crate::{configuration::floors::FloorConfig, sprites::Sprites};

use super::body::Body;

use rand::Rng;

#[derive(Component)]
pub struct GroundTile;

#[derive(Bundle)]
pub struct GroundTileBundle {
    pub tile_type: GroundTile,
    pub collide: Body,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl GroundTileBundle {
    pub fn build(
        position: Vec3,
        sprites: &Sprites,
        floor_config: &FloorConfig,
        sprite_scale: f32,
        tile_size: f32,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let num_options = floor_config.sprite_options.len();
        let random_index: usize = rng.gen_range(0..num_options);
        let floor_index = floor_config
            .sprite_options
            .get(random_index)
            .unwrap()
            .sprite_index
            .unwrap();
        Self {
            tile_type: GroundTile,
            collide: Body {
                tile_size,
                cell_center: position,
                underground: false,
                visibility_before_inventory: false,
            },
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: position,
                    scale: Vec3::splat(sprite_scale),
                    ..default()
                },
                sprite: TextureAtlasSprite::new(floor_index),
                texture_atlas: sprites.atlas_handle.clone(),
                visibility: Visibility { is_visible: true },
                ..default()
            },
        }
    }
}
