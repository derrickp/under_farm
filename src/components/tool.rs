use std::str::FromStr;

use bevy::prelude::Component;

use super::{damage::Damage, name::Name};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolType {
    PickAxe,
    Hammer,
    Hoe,
    Shovel,
}

#[derive(Debug)]
pub struct ParseToolTypeError;

impl FromStr for ToolType {
    type Err = ParseToolTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pickaxe" => Ok(ToolType::PickAxe),
            "hammer" => Ok(ToolType::Hammer),
            "hoe" => Ok(ToolType::Hoe),
            "shovel" => Ok(ToolType::Shovel),
            _ => Ok(ToolType::PickAxe),
        }
    }
}

#[derive(Clone, Debug, Component)]
pub struct Tool {
    pub name: Name,
    pub key: String,
    pub tool_type: ToolType,
    pub damage: Option<Damage>,
}

impl Tool {
    pub fn can_dig(&self) -> bool {
        self.tool_type == ToolType::Shovel
    }

    pub fn can_clear(&self) -> bool {
        self.tool_type == ToolType::Hoe
    }
}
