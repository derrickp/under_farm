use bevy::{
    math::{Vec2, Vec3},
    prelude::{Bundle, Component, SpriteSheetBundle, Transform},
    sprite::TextureAtlasSprite,
};

use crate::{configuration::crops::CropConfiguration, sprites::Sprites};

use super::name::Name;

#[derive(Component)]
pub struct Crop {
    pub current_stage_index: usize,
    pub config: CropConfiguration,
}

pub struct CropStage {
    pub ticks_in_stage: u32,
    pub min_ticks_in_stage: u32,
    pub chance_to_advance: u32,
    pub sprite_index: usize,
}

#[derive(Component)]
pub struct CropStages {
    pub stages: Vec<CropStage>,
}

pub struct CropSpawn {
    pub location: Vec2,
    pub config: CropConfiguration,
}

#[derive(Bundle)]
pub struct CropBundle {
    pub name: Name,
    pub crop: Crop,
    pub stages: CropStages,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl CropBundle {
    pub fn build(
        spawn: &CropSpawn,
        sprites: &Sprites,
        config: &CropConfiguration,
        crop_sprite_scale: f32,
        sprite_scale: f32,
    ) -> Self {
        let stages: Vec<CropStage> = config
            .stages
            .iter()
            .map(|stage| CropStage {
                ticks_in_stage: 0,
                min_ticks_in_stage: stage.configured_ticks_in_stage(),
                chance_to_advance: stage.chance_to_advance(),
                sprite_index: stage.sprite_index.unwrap(),
            })
            .collect();
        Self {
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(spawn.location.x, spawn.location.y, crop_sprite_scale),
                    scale: Vec3::splat(sprite_scale),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(stages.get(0).unwrap().sprite_index),
                texture_atlas: sprites.atlas_handle.clone(),
                ..Default::default()
            },
            name: Name(config.name.to_string()),
            stages: CropStages { stages },
            crop: Crop {
                config: config.clone(),
                current_stage_index: 0,
            },
        }
    }
}
