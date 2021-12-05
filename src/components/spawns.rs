use bevy::prelude::Component;

use super::crop::CropSpawn;

#[derive(Default, Component)]
pub struct Spawns {
    pub crops: Vec<CropSpawn>,
}
