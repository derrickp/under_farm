use bevy::prelude::{Bundle, SpriteSheetBundle};

use super::{body::Body, health::Health};

#[derive(Default)]
pub struct Structure {
    pub can_be_broken: bool,
    pub can_be_walked_on: bool,
    pub health: Health,
    pub structure_type: StructureType,
}

pub enum StructureType {
    Table,
    Wall,
    Unknown,
}

impl Default for StructureType {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Structure {
    pub fn damage(&mut self, damage: i32) {
        self.health.current_health -= damage;
    }

    pub fn is_destroyed(&self) -> bool {
        self.health.has_no_health()
    }
}

#[derive(Bundle)]
pub struct StructureBundle {
    pub structure: Structure,
    pub body: Body,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
