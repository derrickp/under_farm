use bevy::{
    math::Vec2,
    prelude::{Commands, Entity, Mut, Query, Res, Transform},
    sprite::TextureAtlasSprite,
};
use rand::Rng;

use crate::{
    components::{
        crop::{Crop, CropStages},
        spawns::{CropSpawn, Spawns},
    },
    world::WorldTickTimer,
};

pub fn grow_crops_system(
    mut commands: Commands,
    timer: Res<WorldTickTimer>,
    mut query: Query<(
        Entity,
        &Transform,
        &mut Crop,
        &mut CropStages,
        &mut TextureAtlasSprite,
    )>,
    mut spawns_query: Query<&mut Spawns>,
) {
    if !timer.0.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let chance_to_grow: u32 = rng.gen_range(1..100);
    for crop_data in query.iter_mut() {
        let (entity, transform, mut crop, mut stages, mut sprite): (
            Entity,
            &Transform,
            Mut<'_, Crop>,
            Mut<'_, CropStages>,
            Mut<'_, TextureAtlasSprite>,
        ) = crop_data;

        if let Some(stage) = stages.stages.get_mut(crop.current_stage_index) {
            stage.ticks_in_stage += 1;
            if stage.ticks_in_stage > stage.min_ticks_in_stage
                && chance_to_grow < stage.chance_to_advance
            {
                let next_stage_result = stages.stages.get(crop.current_stage_index + 1);
                match next_stage_result {
                    Some(next_stage) => {
                        sprite.index = next_stage.sprite_index;
                        crop.current_stage_index += 1;
                    }
                    _ => {
                        if let Ok(mut spawns) = spawns_query.single_mut() {
                            spawns.crops.push(CropSpawn {
                                configuration_index: crop.config_index,
                                location: Vec2::new(
                                    transform.translation.x,
                                    transform.translation.y,
                                ),
                            });
                        }

                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}
