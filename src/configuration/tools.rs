use kdl::{KdlNode, KdlValue};

use crate::components::{
    damage::Damage,
    inventory::InventorySelector,
    name::Name,
    tool::{Tool, ToolType},
};

use super::{
    kdl_utils::{parse, parse_key_code},
    load::Load,
};

#[derive(Clone)]
pub struct ToolFileConfig {
    name: String,
    key: String,
    tool_type: ToolType,
    min_damage: i32,
    max_damage: i32,
    starter: bool,
    sprite: String,
    inventory_selector: InventorySelector,
}

impl From<&KdlNode> for ToolFileConfig {
    fn from(node: &KdlNode) -> Self {
        let name = match node.values.get(0) {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let key = match node.properties.get("key") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let key_code = match node.properties.get("key_code") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let starter = match node.properties.get("starter") {
            Some(KdlValue::Boolean(it)) => *it,
            _ => false,
        };

        let min_damage = match node.properties.get("min_damage") {
            Some(KdlValue::Int(it)) => *it as i32,
            _ => 0,
        };

        let max_damage = match node.properties.get("max_damage") {
            Some(KdlValue::Int(it)) => *it as i32,
            _ => 0,
        };

        let tool_type = match node.properties.get("type") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let sprite = match node.properties.get("sprite") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        Self {
            key,
            max_damage,
            min_damage,
            name,
            sprite,
            starter,
            tool_type: tool_type.parse().unwrap(),
            inventory_selector: InventorySelector {
                key_code: parse_key_code(&key_code).unwrap(),
                display_code: key_code.clone(),
            },
        }
    }
}

#[derive(Clone)]
pub struct ToolConfiguration {
    file_config: ToolFileConfig,
    pub sprite_index: Option<u32>,
}

impl ToolConfiguration {
    pub fn key(&self) -> &String {
        &self.file_config.key
    }

    pub fn min_damage(&self) -> i32 {
        self.file_config.min_damage
    }

    pub fn max_damage(&self) -> i32 {
        self.file_config.max_damage
    }

    pub fn name(&self) -> &String {
        &self.file_config.name
    }

    pub fn tool_type(&self) -> ToolType {
        self.file_config.tool_type
    }

    pub fn inventory_selector(&self) -> &InventorySelector {
        &self.file_config.inventory_selector
    }

    pub fn starter(&self) -> bool {
        self.file_config.starter
    }

    pub fn sprite_location(&self) -> &str {
        &self.file_config.sprite[..]
    }
}

impl ToolConfiguration {
    pub fn to_tool(&self) -> Tool {
        Tool {
            name: Name(self.name().clone()),
            key: self.key().clone(),
            tool_type: self.tool_type(),
            damage: Some(Damage {
                max_damage: self.max_damage(),
                min_damage: self.min_damage(),
            }),
        }
    }
}

impl From<&KdlNode> for ToolConfiguration {
    fn from(node: &KdlNode) -> Self {
        let file_config = ToolFileConfig::from(node);

        Self {
            file_config,
            sprite_index: None,
        }
    }
}

pub struct ToolConfigurations {
    pub configurations: Vec<ToolConfiguration>,
}

impl Load for ToolConfigurations {
    fn load(path: &str) -> Self {
        let tool_nodes = parse(path).unwrap();
        let configurations = tool_nodes.iter().map(ToolConfiguration::from).collect();

        Self { configurations }
    }
}

impl ToolConfigurations {
    pub fn tool_by_type(&self, tool_type: ToolType) -> Option<ToolConfiguration> {
        self.configurations
            .iter()
            .find(|config| config.tool_type() == tool_type)
            .cloned()
    }
}
