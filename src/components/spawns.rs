use bevy::prelude::Component;

use super::{crop::CropSpawn, structure::StructureSpawn};

#[derive(Default, Component)]
pub struct Spawns {
    pub crops: Vec<CropSpawn>,
    pub structures: Vec<StructureSpawn>,
}
