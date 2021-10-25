use crate::components::{damage::Damage, name::Name, tool::{Tool, ToolType}};

pub struct ToolConfiguration {
    pub name: &'static str,
    pub tool_type: ToolType,
    pub max_damage: i32,
    pub min_damage: i32,
}

impl ToolConfiguration {
    fn build(name: &'static str, tool_type: ToolType, max_damage: i32, min_damage: i32) -> Self {
        Self {
            name,
            tool_type,
            max_damage,
            min_damage,
        }
    }

    fn pick_axe(name: &'static str) -> Self {
        Self::build(name, ToolType::PickAxe, 1, 1)
    }

    fn shovel(name: &'static str) -> Self {
        Self::build(name, ToolType::Shovel, 1, 1)
    }

    fn hammer(name: &'static str) -> Self {
        Self::build(name, ToolType::Hammer, 1, 1)
    }

    fn hoe(name: &'static str) -> Self {
        Self::build(name, ToolType::Hoe, 1, 1)
    }

    pub fn to_tool(&self) -> Tool {
        Tool {
            name: Name(self.name.to_string()),
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
