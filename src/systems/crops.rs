use bevy::{
    math::Vec2,
    prelude::{Commands, Entity, Mut, Query, Transform},
    sprite::TextureAtlasSprite,
};
use rand::Rng;

use crate::{
    components::{
        crop::{Crop, CropSpawn, CropStages},
        spawns::Spawns, world::World,
    },
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
    world_query: Query<&World>,
) {
    if world_query.is_empty() {
        return;
    }

    let world: &World = world_query.single();

    if !world.tick_just_finished {
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
                if !spawns_query.is_empty() {
                    let mut spawns = spawns_query.single_mut();
                    spawns.crops.push(CropSpawn {
                        config: crop.config.clone(),
                        location: Vec2::new(transform.translation.x, transform.translation.y),
                    });
                }

                commands.entity(entity).despawn();
            }
        }
    }
}
