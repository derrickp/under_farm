use bevy::{
    math::{Vec2, Vec3},
    prelude::{Bundle, SpriteSheetBundle, Transform, Visible},
    sprite::TextureAtlasSprite,
};

use crate::{configuration::map::TILE_SIZE, sprites::Sprites};

use super::{
    body::Body,
    health::{Health, HealthTextureMap},
};

#[derive(Default)]
pub struct Structure {
    pub can_be_broken: bool,
    pub can_be_walked_on: bool,
    pub health: Health,
    pub structure_type: StructureType,
    pub health_textures: Vec<HealthTextureMap>,
}

pub enum StructureType {
    Table,
    Wall,
    Unknown,
}

impl Default for StructureType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Structure {
    pub fn damage(&mut self, damage: i32) {
        self.health.current_health -= damage;
    }

    pub fn is_destroyed(&self) -> bool {
        self.health.has_no_health()
    }

    pub fn current_texture_index(&self) -> Option<usize> {
        let current = self.health.current_health;
        if let Some(health_texture) = self
            .health_textures
            .iter()
            .find(|map| current >= map.min_health && current <= map.max_health)
        {
            return Some(health_texture.texture_index);
        }
        None
    }
}

#[derive(Bundle)]
pub struct StructureBundle {
    pub structure: Structure,
    pub body: Body,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl StructureBundle {
    pub fn build_table(coordinate: &Vec2, sprites: &Sprites) -> Self {
        let cell_center = Vec3::new(coordinate.x, coordinate.y, 2.0);
        Self {
            structure: Structure {
                health: Health::same_health(2),
                structure_type: StructureType::Table,
                can_be_broken: true,
                health_textures: vec![
                    HealthTextureMap {
                        min_health: -99,
                        max_health: 0,
                        texture_index: sprites.broken_small_table,
                    },
                    HealthTextureMap {
                        min_health: 1,
                        max_health: 3,
                        texture_index: sprites.table_index,
                    },
                ],
                ..Default::default()
            },
            body: Body {
                cell_center,
                tile_size: TILE_SIZE as f32,
                sprite: None,
            },
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(sprites.table_index as u32),
                texture_atlas: sprites.atlas_handle.clone(),
                visible: Visible {
                    is_visible: false,
                    is_transparent: true,
                },
                transform: Transform {
                    translation: cell_center,
                    scale: crate::configuration::sprites::sprite_scale(),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    pub fn build_rubble(coordinate: &Vec2, sprites: &Sprites) -> Self {
        let cell_center = Vec3::new(coordinate.x, coordinate.y, 2.0);
        Self {
            structure: Structure {
                can_be_broken: false,
                can_be_walked_on: true,
                ..Default::default()
            },
            body: Body {
                cell_center,
                tile_size: TILE_SIZE as f32,
                sprite: None,
            },
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(sprites.broken_wall_index as u32),
                texture_atlas: sprites.atlas_handle.clone(),
                transform: Transform {
                    translation: cell_center,
                    scale: crate::configuration::sprites::sprite_scale(),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    pub fn build_outer_wall(coordinate: &Vec2, sprites: &Sprites) -> Self {
        let cell_center = Vec3::new(coordinate.x, coordinate.y, 0.0);
        Self {
            structure: Structure::default(),
            body: Body {
                cell_center,
                tile_size: TILE_SIZE as f32,
                sprite: None,
            },
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: cell_center,
                    scale: crate::configuration::sprites::sprite_scale(),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(sprites.outer_wall_index as u32),
                texture_atlas: sprites.atlas_handle.clone(),
                visible: Visible {
                    is_visible: true,
                    is_transparent: true,
                },
                ..Default::default()
            },
        }
    }

    pub fn build_room_wall(coordinate: &Vec2, sprites: &Sprites) -> Self {
        let cell_center = Vec3::new(coordinate.x, coordinate.y, 1.0);
        Self {
            structure: Structure {
                can_be_broken: true,
                can_be_walked_on: false,
                health: Health::same_health(3),
                structure_type: StructureType::Wall,
                health_textures: vec![
                    HealthTextureMap {
                        min_health: -99,
                        max_health: 0,
                        texture_index: sprites.broken_wall_index,
                    },
                    HealthTextureMap {
                        min_health: 1,
                        max_health: 1,
                        texture_index: sprites.brick_wall_really_cracked_index,
                    },
                    HealthTextureMap {
                        min_health: 2,
                        max_health: 2,
                        texture_index: sprites.brick_wall_cracked_index,
                    },
                    HealthTextureMap {
                        min_health: 3,
                        max_health: 3,
                        texture_index: sprites.room_wall_index,
                    },
                ],
            },
            body: Body {
                cell_center,
                tile_size: TILE_SIZE as f32,
                sprite: None,
            },
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: cell_center,
                    scale: crate::configuration::sprites::sprite_scale(),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(sprites.room_wall_index as u32),
                texture_atlas: sprites.atlas_handle.clone(),
                visible: Visible {
                    is_visible: true,
                    is_transparent: true,
                },
                ..Default::default()
            },
        }
    }
}
