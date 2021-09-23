use crate::components::tool::{Tool, ToolName, ToolType};

pub struct ToolConfiguration {
    pub name: &'static str,
    pub tool_type: ToolType,
}

impl ToolConfiguration {
    fn build(name: &'static str, tool_type: ToolType) -> Self {
        return Self { name, tool_type };
    }

    fn pick_axe(name: &'static str) -> Self {
        return Self::build(name, ToolType::PickAxe);
    }

    fn shovel(name: &'static str) -> Self {
        return Self::build(name, ToolType::Shovel);
    }

    fn hammer(name: &'static str) -> Self {
        return Self::build(name, ToolType::Hammer);
    }

    fn hoe(name: &'static str) -> Self {
        return Self::build(name, ToolType::Hoe);
    }

    pub fn to_tool(&self) -> Tool {
        return Tool {
            name: ToolName(self.name.to_string()),
            tool_type: self.tool_type,
        };
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
