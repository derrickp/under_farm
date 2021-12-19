use std::str::FromStr;

use bevy::{
    math::Vec3,
    prelude::{Bundle, Component, Handle, SpriteSheetBundle, Transform, Visible},
    sprite::{TextureAtlas, TextureAtlasSprite},
};

use crate::configuration::{
    game::SpriteConfig,
    structures::{StructureConfig, StructureHealthConfig},
};

use super::{
    body::Body,
    health::{Health, HealthTextureMap},
};

#[derive(Clone, PartialEq)]
pub enum StructureType {
    Wall,
    Table,
    Hole,
    Unknown,
}

#[derive(Debug)]
pub struct ParseStructureTypeError;

impl FromStr for StructureType {
    type Err = ParseStructureTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wall" => Ok(Self::Wall),
            "table" => Ok(Self::Table),
            "hole" => Ok(Self::Hole),
            _ => Err(ParseStructureTypeError),
        }
    }
}

impl Default for StructureType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Default, Component)]
pub struct Structure {
    pub health: Health,
    pub health_configs: Vec<StructureHealth>,
    structure_type: StructureType,
    default_can_be_walked_on: bool,
    default_can_be_broken: bool,
}

#[derive(Component)]
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
        self.current_config().map(|config| config.texture_index())
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

    pub fn is_exit(&self) -> bool {
        self.structure_type == StructureType::Hole
    }
}

pub struct StructureSpawn {
    pub position: Vec3,
    pub structure_key: &'static str,
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
        position: Vec3,
        atlas_handle: &Handle<TextureAtlas>,
        structure_config: &StructureConfig,
        sprite_config: &SpriteConfig,
        tile_size: f32,
    ) -> Self {
        let health_configs: Vec<StructureHealth> = structure_config
            .health_configs
            .iter()
            .map(StructureHealth::from)
            .collect();

        let structure = Structure {
            health_configs,
            health: Health::same_health(structure_config.starting_health),
            structure_type: structure_config.structure_type.clone(),
            ..Default::default()
        };
        let starting_sprite = structure.current_texture_index().unwrap() as u32;

        Self {
            structure,
            body: Body {
                tile_size,
                underground: false,
                cell_center: position,
            },
            sprite: Self::sprite(
                atlas_handle,
                position,
                starting_sprite,
                structure_config.initial_visible,
                sprite_config.scale,
            ),
        }
    }

    fn sprite(
        atlas: &Handle<TextureAtlas>,
        position: Vec3,
        sprite: u32,
        visible: bool,
        sprite_scale: f32,
    ) -> SpriteSheetBundle {
        SpriteSheetBundle {
            transform: Transform {
                translation: position,
                scale: Vec3::splat(sprite_scale),
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
