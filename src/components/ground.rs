use bevy::{
    math::{Vec2, Vec3},
    prelude::{Bundle, SpriteSheetBundle, Transform, Visible},
    sprite::TextureAtlasSprite,
};

use crate::{configuration::map::TILE_SIZE, sprites::Sprites};

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
    pub fn build_room_floor(coordinate: &Vec2, sprites: &Sprites) -> Self {
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
                sprite: TextureAtlasSprite::new(sprites.room_floor_index as u32),
                texture_atlas: sprites.atlas_handle.clone(),
                visible: Visible {
                    is_visible: false,
                    is_transparent: false,
                },
                ..Default::default()
            },
        }
    }

    pub fn build_floor(coordinate: &Vec2, sprites: &Sprites) -> Self {
        let mut rng = rand::thread_rng();
        let random_index: usize = rng.gen_range(0..sprites.dirt_floor_indexes.len());
        let dirt_floor_index = *sprites.dirt_floor_indexes.get(random_index).unwrap();
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
                sprite: TextureAtlasSprite::new(dirt_floor_index as u32),
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
