use bevy::prelude::Component;

mod timer;

pub use timer::WorldTickTimer;

#[derive(Component, Default)]
pub struct World {
    pub tick_just_finished: bool,
}
