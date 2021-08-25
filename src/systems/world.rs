use bevy::{
    core::Time,
    prelude::{Commands, Res, ResMut},
};

use crate::{states::GameLoadState, world::WorldTickTimer};

pub fn tick_game_world(time: Res<Time>, mut timer: ResMut<WorldTickTimer>) {
    timer.0.tick(time.delta());
}

pub fn generate_world_grid(mut commands: Commands, mut load_state: ResMut<GameLoadState>) {
    let grid = crate::world_generation::generation::generate_world_grid();
    commands.insert_resource(grid);

    load_state.game_world_generated = true;
}
