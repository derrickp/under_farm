use bevy::{
    math::Vec3,
    prelude::{Commands, Mut, Query, Res, SpriteSheetBundle, Transform},
    sprite::TextureAtlasSprite,
};

use crate::{
    components::{
        crop::{Crop, CropBundle, CropStage, CropStages},
        name::Name,
        spawns::{CropSpawn, Spawns},
    },
    configuration::{
        crops::{CropConfiguration, CropConfigurations},
        sprites::CROP_SPRITE_SCALE,
    },
    sprites::Sprites,
};

pub fn spawn_crops(
    mut commands: Commands,
    sprites: Res<Sprites>,
    crop_configurations: Res<CropConfigurations>,
    query: Query<&Spawns>,
) {
    let spawns: &Spawns = match query.single() {
        Ok(it) => it,
        _ => return,
    };

    if spawns.crops.is_empty() {
        return;
    }

    for spawn in spawns.crops.iter() {
        let config = match crop_configurations
            .configurations
            .get(spawn.configuration_index)
        {
            Some(it) => it,
            _ => continue,
        };

        commands.spawn_bundle(crop_bundle(spawn, &sprites, config));
    }
}

fn crop_bundle(spawn: &CropSpawn, sprites: &Sprites, config: &CropConfiguration) -> CropBundle {
    let stages: Vec<CropStage> = config
        .stages
        .iter()
        .map(|stage| CropStage {
            ticks_in_stage: 0,
            min_ticks_in_stage: stage.min_ticks_in_stage,
            chance_to_advance: stage.chance_to_advance,
            sprite_index: stage.sprite_index.unwrap(),
        })
        .collect();
    CropBundle {
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

pub fn reset_crop_spawns(mut query: Query<&mut Spawns>) {
    let mut spawns: Mut<'_, Spawns> = match query.single_mut() {
        Ok(it) => it,
        _ => return,
    };

    spawns.crops.clear();
}
