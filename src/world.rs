use bevy::core::Timer;

pub struct WorldTickTimer(pub Timer);

impl Default for WorldTickTimer {
    fn default() -> Self {
        return Self(Timer::from_seconds(0.2, true));
    }
}
