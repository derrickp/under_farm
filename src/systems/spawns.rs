use bevy::prelude::{Commands, Mut, Query, Res};

use crate::{
    components::{crop::CropBundle, spawns::Spawns},
    configuration::game::GameConfiguration,
    sprites::Sprites,
};

pub fn spawn_crops(
    mut commands: Commands,
    sprites: Res<Sprites>,
    game_config: Res<GameConfiguration>,
    query: Query<&Spawns>,
) {
    if query.is_empty() {
        return;
    }

    let spawns: &Spawns = query.single();

    if spawns.crops.is_empty() {
        return;
    }

    for spawn in spawns.crops.iter() {
        commands.spawn_bundle(CropBundle::build(
            spawn,
            &sprites,
            &spawn.config,
            game_config.sprite_config.crop_scale,
            game_config.sprite_config.scale,
        ));
    }
}

pub fn reset_crop_spawns(mut query: Query<&mut Spawns>) {
    if query.is_empty() {
        return;
    }

    let mut spawns: Mut<Spawns> = query.single_mut();

    spawns.crops.clear();
}
