use bevy::{core::Timer, prelude::Component};

#[derive(Component)]
pub struct WorldTickTimer(pub Timer);
