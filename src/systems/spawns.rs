use bevy::prelude::{Commands, Mut, Query, Res};

use crate::{
    components::{crop::CropBundle, spawns::Spawns},
    configuration::crops::CropConfigurations,
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

        commands.spawn_bundle(CropBundle::build(spawn, &sprites, config));
    }
}

pub fn reset_crop_spawns(mut query: Query<&mut Spawns>) {
    let mut spawns: Mut<'_, Spawns> = match query.single_mut() {
        Ok(it) => it,
        _ => return,
    };

    spawns.crops.clear();
}
