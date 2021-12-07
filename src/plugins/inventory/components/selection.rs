use bevy::prelude::{Component, KeyCode};

#[derive(Default, Component)]
pub struct CurrentInventorySelection {
    pub key_code: Option<KeyCode>,
    pub index: Option<usize>,
    pub max_index: usize,
}

pub struct InventorySelectionHelper {
    pub key: String,
    pub index: usize,
}
