use bevy::prelude::{Bundle, TextBundle};

pub struct InventoryText;

pub struct InventoryTextStatus {
    pub index: usize,
}

#[derive(Bundle)]
pub struct InventoryTextBundle {
    pub inventory_text: InventoryText,
    pub status: InventoryTextStatus,

    #[bundle]
    pub text: TextBundle,
}
