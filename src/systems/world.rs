use bevy::{
    core::Time,
    prelude::{Commands, Res, ResMut},
};

use crate::{
    states::GameLoadState, world::WorldTickTimer, world_generation::templates::RoomTemplates,
};

pub fn tick_game_world(time: Res<Time>, mut timer: ResMut<WorldTickTimer>) {
    timer.0.tick(time.delta());
}

pub fn generate_world_grid(
    mut commands: Commands,
    mut load_state: ResMut<GameLoadState>,
    room_templates: Res<RoomTemplates>,
) {
    let grid =
        crate::world_generation::generation::generate_world_grid(&room_templates.all_templates);
    commands.insert_resource(grid);

    load_state.game_world_generated = true;
}
