use bevy::{
    core::Time,
    prelude::{Commands, Query, Res, ResMut},
};

use crate::{
    components::world::WorldTickTimer, configuration::world::generator, states::GameLoadState,
};

pub fn tick_game_world(time: Res<Time>, mut query: Query<&mut WorldTickTimer>) {
    let mut timer = match query.single_mut() {
        Ok(it) => it,
        _ => return,
    };

    timer.0.tick(time.delta());
}

pub fn generate_world_grid(mut commands: Commands, mut load_state: ResMut<GameLoadState>) {
    let generator = generator();
    let world = generator.generate_top_down_map().unwrap();
    println!("{}", world.room_count);
    commands.insert_resource(world.grid);

    load_state.game_world_generated = true;
}
