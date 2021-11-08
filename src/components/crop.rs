use bevy::{
    math::{Vec2, Vec3},
    prelude::{Bundle, SpriteSheetBundle, Transform},
    sprite::TextureAtlasSprite,
};

use crate::{
    configuration::{crops::CropConfiguration, sprites::CROP_SPRITE_SCALE},
    sprites::Sprites,
};

use super::name::Name;

pub struct Crop {
    pub current_stage_index: usize,
    pub config_index: usize,
}

pub struct CropStage {
    pub ticks_in_stage: u32,
    pub min_ticks_in_stage: u32,
    pub chance_to_advance: u32,
    pub sprite_index: u32,
}

pub struct CropStages {
    pub stages: Vec<CropStage>,
}

pub struct CropSpawn {
    pub configuration_index: usize,
    pub location: Vec2,
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
    pub fn build(spawn: &CropSpawn, sprites: &Sprites, config: &CropConfiguration) -> Self {
        let stages: Vec<CropStage> = config
            .stages
            .iter()
            .map(|stage| CropStage {
                ticks_in_stage: 0,
                min_ticks_in_stage: stage.configured_ticks_in_stage(),
                chance_to_advance: stage.chance_to_advance,
                sprite_index: stage.sprite_index.unwrap(),
            })
            .collect();
        Self {
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(spawn.location.x, spawn.location.y, CROP_SPRITE_SCALE),
                    scale: crate::configuration::sprites::sprite_scale(),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(stages.get(0).unwrap().sprite_index),
                texture_atlas: sprites.atlas_handle.clone(),
                ..Default::default()
            },
            name: Name(config.name),
            stages: CropStages { stages },
            crop: Crop {
                config_index: spawn.configuration_index,
                current_stage_index: 0,
            },
        }
    }
}
