use bevy::time::Timer;

pub const CONSTANT_MOVE_WAIT_TIME: f32 = 0.2;

pub fn movement_timer() -> Timer {
    Timer::from_seconds(CONSTANT_MOVE_WAIT_TIME, true)
}
