use bevy::prelude::KeyCode;

use crate::components::{
    damage::Damage,
    inventory::InventorySelector,
    name::Name,
    tool::{Tool, ToolType},
};

pub struct ToolConfiguration {
    pub name: &'static str,
    pub key: String,
    pub tool_type: ToolType,
    pub max_damage: i32,
    pub min_damage: i32,
    pub inventory_selector: InventorySelector,
}

impl ToolConfiguration {
    fn build(
        name: &'static str,
        tool_type: ToolType,
        max_damage: i32,
        min_damage: i32,
        key: String,
        inventory_selector: InventorySelector,
    ) -> Self {
        Self {
            inventory_selector,
            name,
            key,
            tool_type,
            max_damage,
            min_damage,
        }
    }

    fn pick_axe(name: &'static str) -> Self {
        let selector = InventorySelector {
            key_code: KeyCode::A,
            display_code: "a".to_string(),
        };
        Self::build(
            name,
            ToolType::PickAxe,
            1,
            1,
            name.to_ascii_lowercase(),
            selector,
        )
    }

    fn shovel(name: &'static str) -> Self {
        let selector = InventorySelector {
            key_code: KeyCode::S,
            display_code: "s".to_string(),
        };
        Self::build(
            name,
            ToolType::Shovel,
            1,
            1,
            name.to_ascii_lowercase(),
            selector,
        )
    }

    fn hammer(name: &'static str) -> Self {
        let selector = InventorySelector {
            key_code: KeyCode::H,
            display_code: "h".to_string(),
        };
        Self::build(
            name,
            ToolType::Hammer,
            1,
            1,
            name.to_ascii_lowercase(),
            selector,
        )
    }

    fn hoe(name: &'static str) -> Self {
        let selector = InventorySelector {
            key_code: KeyCode::O,
            display_code: "o".to_string(),
        };
        Self::build(
            name,
            ToolType::Hoe,
            1,
            1,
            name.to_ascii_lowercase(),
            selector,
        )
    }

    pub fn to_tool(&self) -> Tool {
        Tool {
            name: Name(self.name.to_string()),
            key: self.key.clone(),
            tool_type: self.tool_type,
            damage: Some(Damage {
                max_damage: self.max_damage,
                min_damage: self.min_damage,
            }),
        }
    }
}

pub struct ToolConfigurations {
    pub configurations: Vec<ToolConfiguration>,
}

impl Default for ToolConfigurations {
    fn default() -> Self {
        Self {
            configurations: vec![
                ToolConfiguration::pick_axe("Rusty Pick Axe"),
                ToolConfiguration::hammer("Small Hammer"),
                ToolConfiguration::hoe("Stone Hoe"),
                ToolConfiguration::shovel("Bone Shovel"),
            ],
        }
    }
}
