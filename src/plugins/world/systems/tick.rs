use bevy::{
    core::Time,
    prelude::{Mut, Query, Res},
};

use crate::{components::action::WorldActions, plugins::world::components::timer::WorldTickTimer};

pub fn tick_game_world(time: Res<Time>, mut query: Query<&mut WorldTickTimer>) {
    if query.is_empty() {
        return;
    }

    let mut timer = query.single_mut();

    timer.0.tick(time.delta());
}

pub fn check_world_actions(
    timer_query: Query<&WorldTickTimer>,
    mut actions_query: Query<&mut WorldActions>,
) {
    if timer_query.is_empty() || actions_query.is_empty() {
        return;
    }

    let timer: &WorldTickTimer = timer_query.single();
    let mut actions: Mut<'_, WorldActions> = actions_query.single_mut();

    actions.grow_crops = timer.0.just_finished();
}
