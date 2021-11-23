use bevy::{
    core::Time,
    prelude::{Commands, Query, Res, ResMut},
};

use crate::{
    components::world::WorldTickTimer, configuration::game::GameConfiguration,
    states::GameLoadState,
};

pub fn tick_game_world(time: Res<Time>, mut query: Query<&mut WorldTickTimer>) {
    let mut timer = match query.single_mut() {
        Ok(it) => it,
        _ => return,
    };

    timer.0.tick(time.delta());
}

pub fn generate_world_grid(
    mut commands: Commands,
    mut load_state: ResMut<GameLoadState>,
    game_config: Res<GameConfiguration>,
) {
    let generator = game_config.world_config.generator(game_config.seed.clone());
    let world = generator.generate_top_down_map().unwrap();
    commands.insert_resource(world);

    load_state.game_world_generated = true;
}
