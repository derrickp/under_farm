use bevy::prelude::Component;
use tdlg::map::TopDownMap;

use super::{crop::CropSpawn, structure::StructureSpawn};

#[derive(Default, Component)]
pub struct Spawns {
    pub crops: Vec<CropSpawn>,
    pub structures: Vec<StructureSpawn>,
    pub map_spawn: Option<MapSpawn>,
}

pub struct MapSpawn {
    pub map: TopDownMap,
}
