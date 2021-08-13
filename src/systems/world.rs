use bevy::{
    core::Time,
    prelude::{Res, ResMut},
};

use crate::world::WorldTickTimer;

pub fn tick_game_world(time: Res<Time>, mut timer: ResMut<WorldTickTimer>) {
    timer.0.tick(time.delta());
}
