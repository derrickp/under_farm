use bevy::core::Timer;

use crate::configuration::timers::world_tick_timer;

pub struct WorldTickTimer(pub Timer);

impl Default for WorldTickTimer {
    fn default() -> Self {
        Self(world_tick_timer())
    }
}
