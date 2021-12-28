use bevy::prelude::{Commands, Query, Res};

use crate::{
    configuration::game::GameConfiguration,
    plugins::world::components::{timer::WorldTickTimer, world::World},
};

pub fn spawn(
    mut commands: Commands,
    query: Query<&WorldTickTimer>,
    game_config: Res<GameConfiguration>,
) {
    if !query.is_empty() {
        return;
    }

    let world_timer = WorldTickTimer(game_config.world_tick_timer());
    commands.spawn().insert(world_timer);

    commands.spawn().insert(World::default());
}
