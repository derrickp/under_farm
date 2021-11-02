use bevy::math::Vec2;

#[derive(Default)]
pub struct Spawns {
    pub crops: Vec<CropSpawn>,
}

pub struct CropSpawn {
    pub configuration_index: usize,
    pub location: Vec2,
}
