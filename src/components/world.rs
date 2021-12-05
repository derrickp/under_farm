use bevy::{core::Timer, prelude::Component};

use crate::configuration::timers::world_tick_timer;

#[derive(Component)]
pub struct WorldTickTimer(pub Timer);

impl Default for WorldTickTimer {
    fn default() -> Self {
        Self(world_tick_timer())
    }
}
