use bevy::{
    prelude::{Mut, Query, Res},
    sprite::TextureAtlasSprite,
};
use rand::Rng;

use crate::{
    components::crop::{Crop, CropStages},
    world::WorldTickTimer,
};

pub fn grow_crops_system(
    timer: Res<WorldTickTimer>,
    mut query: Query<(&mut Crop, &mut CropStages, &mut TextureAtlasSprite)>,
) {
    if !timer.0.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let chance_to_grow: u32 = rng.gen_range(1..100);
    for crop_data in query.iter_mut() {
        let (mut crop, mut stages, mut sprite): (
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
                        crop.current_stage_index = crop.current_stage_index + 1;
                    }
                    _ => {
                        // TODO Despawn later
                        println!("Would despawn crop {}", chance_to_grow);
                    }
                }
            }
        }
    }
}
