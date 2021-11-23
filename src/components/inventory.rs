use bevy::prelude::{Bundle, KeyCode, TextBundle};

pub struct InventoryText;

pub struct InventoryTextStatus {
    pub key: String,
}

#[derive(Bundle)]
pub struct InventoryTextBundle {
    pub inventory_text: InventoryText,
    pub status: InventoryTextStatus,

    #[bundle]
    pub text: TextBundle,
}

#[derive(Clone)]
pub struct InventorySelector {
    pub key_code: KeyCode,
    pub display_code: String,
}
