use bevy::{
    math::Vec2,
    prelude::{Commands, Entity, Mut, Query, Transform},
    sprite::TextureAtlasSprite,
};
use rand::Rng;

use crate::components::{
    crop::{Crop, CropSpawn, CropStages},
    spawns::Spawns,
    world::WorldTickTimer,
};

pub fn grow_crops_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Transform,
        &mut Crop,
        &mut CropStages,
        &mut TextureAtlasSprite,
    )>,
    mut spawns_query: Query<&mut Spawns>,
    timer_query: Query<&WorldTickTimer>,
) {
    let timer = match timer_query.single() {
        Ok(it) => it,
        _ => return,
    };

    if !timer.0.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    for crop_data in query.iter_mut() {
        let (entity, transform, mut crop, mut stages, mut sprite): (
            Entity,
            &Transform,
            Mut<Crop>,
            Mut<CropStages>,
            Mut<TextureAtlasSprite>,
        ) = crop_data;

        let stage = match stages.stages.get_mut(crop.current_stage_index) {
            Some(it) => it,
            _ => {
                println!("No stage!");
                continue;
            }
        };

        stage.ticks_in_stage += 1;
        if stage.ticks_in_stage < stage.min_ticks_in_stage {
            continue;
        }

        let chance_to_grow: u32 = rng.gen_range(1..100);

        if chance_to_grow > stage.chance_to_advance {
            continue;
        }

        match stages.stages.get(crop.current_stage_index + 1) {
            Some(next_stage) => {
                sprite.index = next_stage.sprite_index;
                crop.current_stage_index += 1;
            }
            _ => {
                if let Ok(mut spawns) = spawns_query.single_mut() {
                    spawns.crops.push(CropSpawn {
                        configuration_index: crop.config_index,
                        location: Vec2::new(transform.translation.x, transform.translation.y),
                    });
                }

                commands.entity(entity).despawn();
            }
        }
    }
}
