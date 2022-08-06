use bevy::{prelude::Component, time::Timer};

#[derive(Component)]
pub struct WorldTickTimer(pub Timer);
