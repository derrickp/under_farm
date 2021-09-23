use bevy::core::Timer;

pub const CONSTANT_MOVE_WAIT_TIME: f32 = 0.2;
pub const WORLD_TICK_TIME: f32 = 0.2;

pub fn movement_timer() -> Timer {
    Timer::from_seconds(CONSTANT_MOVE_WAIT_TIME, true)
}

pub fn world_tick_timer() -> Timer {
    Timer::from_seconds(WORLD_TICK_TIME, true)
}
