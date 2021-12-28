use bevy::{
    core::Time,
    prelude::{Mut, Query, Res},
};

use crate::plugins::world::components::{timer::WorldTickTimer, world::World};

pub fn tick_game_world(time: Res<Time>, mut query: Query<&mut WorldTickTimer>) {
    if query.is_empty() {
        return;
    }

    let mut timer = query.single_mut();

    timer.0.tick(time.delta());
}

pub fn check_world_actions(
    timer_query: Query<&WorldTickTimer>,
    mut world_query: Query<&mut World>,
) {
    if timer_query.is_empty() || world_query.is_empty() {
        return;
    }

    let timer: &WorldTickTimer = timer_query.single();

    let mut world: Mut<World> = world_query.single_mut();

    world.tick_just_finished = timer.0.just_finished();
}
