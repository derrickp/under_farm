use bevy::{
    math::{Vec2, Vec3},
    prelude::{Bundle, Handle, SpriteSheetBundle, Transform, Visible},
    sprite::{TextureAtlas, TextureAtlasSprite},
};

use crate::configuration::{
    map::TILE_SIZE,
    structures::{StructureConfig, StructureHealthConfig},
};

use super::{
    body::Body,
    health::{Health, HealthTextureMap},
};

#[derive(Default)]
pub struct Structure {
    pub health: Health,
    pub health_configs: Vec<StructureHealth>,
    default_can_be_walked_on: bool,
    default_can_be_broken: bool,
}

pub struct StructureHealth {
    health_texture: HealthTextureMap,
    pub can_be_walked_on: bool,
    pub can_be_broken: bool,
}

impl StructureHealth {
    pub fn matches_health(&self, health: i32) -> bool {
        health >= self.health_texture.min_health && health <= self.health_texture.max_health
    }

    pub fn texture_index(&self) -> usize {
        self.health_texture.texture_index
    }
}

impl From<&StructureHealthConfig> for StructureHealth {
    fn from(value: &StructureHealthConfig) -> Self {
        let health_texture = HealthTextureMap {
            min_health: value.min_health(),
            max_health: value.max_health(),
            texture_index: value.sprite_index.unwrap() as usize,
        };

        Self {
            health_texture,
            can_be_broken: value.can_be_broken(),
            can_be_walked_on: value.can_be_walked_on(),
        }
    }
}

impl Structure {
    pub fn damage(&mut self, damage: i32) {
        self.health.current_health -= damage;
    }

    pub fn current_texture_index(&self) -> Option<usize> {
        match self.current_config() {
            Some(config) => Some(config.texture_index()),
            _ => None,
        }
    }

    pub fn can_be_broken(&self) -> bool {
        let config = self.current_config();
        match config {
            Some(structure_health) => structure_health.can_be_broken,
            _ => self.default_can_be_broken,
        }
    }

    pub fn can_be_walked_on(&self) -> bool {
        let config = self.current_config();
        match config {
            Some(structure_health) => structure_health.can_be_walked_on,
            _ => self.default_can_be_walked_on,
        }
    }

    fn current_config(&self) -> Option<&StructureHealth> {
        let current = self.health.current_health;
        self.health_configs
            .iter()
            .find(|config| config.matches_health(current))
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
    pub fn build(
        coordinate: &Vec2,
        atlas_handle: &Handle<TextureAtlas>,
        structure_config: &StructureConfig,
    ) -> Self {
        let cell_center = Vec3::new(coordinate.x, coordinate.y, 1.0);
        let health_configs: Vec<StructureHealth> = structure_config
            .health_configs
            .iter()
            .map(|health_config| StructureHealth::from(health_config))
            .collect();

        let structure = Structure {
            health: Health::same_health(structure_config.starting_health),
            health_configs,
            ..Default::default()
        };
        let starting_sprite = structure.current_texture_index().unwrap() as u32;

        Self {
            structure,
            body: Body {
                cell_center,
                tile_size: TILE_SIZE as f32,
            },
            sprite: Self::sprite(
                atlas_handle,
                cell_center,
                starting_sprite,
                structure_config.initial_visible,
            ),
        }
    }

    fn sprite(
        atlas: &Handle<TextureAtlas>,
        center: Vec3,
        sprite: u32,
        visible: bool,
    ) -> SpriteSheetBundle {
        SpriteSheetBundle {
            transform: Transform {
                translation: center,
                scale: crate::configuration::sprites::sprite_scale(),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(sprite),
            texture_atlas: atlas.clone(),
            visible: Visible {
                is_visible: visible,
                is_transparent: true,
            },
            ..Default::default()
        }
    }
}
