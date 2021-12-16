use kdl::{KdlNode, KdlValue};

use crate::components::structure::StructureType;

use super::kdl_utils::parse;

#[derive(Clone)]
pub struct StructureConfig {
    pub name: String,
    pub key: String,
    pub structure_type: StructureType,
    pub starting_health: i32,
    pub initial_visible: bool,
    pub health_configs: Vec<StructureHealthConfig>,
}

#[derive(Clone)]
pub struct StructureHealthConfig {
    file_config: StructureHealthFileConfig,
    pub sprite_index: Option<u32>,
}

impl StructureHealthConfig {
    pub fn sprite_location(&self) -> &str {
        &self.file_config.sprite_location
    }

    pub fn min_health(&self) -> i32 {
        self.file_config.min_health
    }

    pub fn max_health(&self) -> i32 {
        self.file_config.max_health
    }

    pub fn can_be_broken(&self) -> bool {
        self.file_config.can_be_broken
    }

    pub fn can_be_walked_on(&self) -> bool {
        self.file_config.can_be_walked_on
    }
}

impl From<&KdlNode> for StructureHealthConfig {
    fn from(node: &KdlNode) -> Self {
        let file_config = StructureHealthFileConfig::from(node);
        Self {
            file_config,
            sprite_index: None,
        }
    }
}

const DEFAULT_MAX_HEALTH: i32 = 10;
const DEFAULT_MIN_HEALTH: i32 = 1;

impl From<&KdlNode> for StructureConfig {
    fn from(node: &KdlNode) -> Self {
        let name = match node.values.get(0) {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let key = match node.properties.get("key") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let structure_type = match node.properties.get("type") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let starting_health = match node.properties.get("health") {
            Some(KdlValue::Int(it)) => *it as i32,
            _ => DEFAULT_MAX_HEALTH,
        };

        let initial_visible: bool = match node.properties.get("visible") {
            Some(KdlValue::Boolean(it)) => *it,
            _ => true,
        };

        let health_configs = node
            .children
            .iter()
            .map(StructureHealthConfig::from)
            .collect();

        Self {
            health_configs,
            initial_visible,
            key,
            name,
            starting_health,
            structure_type: structure_type.parse().unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct StructureHealthFileConfig {
    pub min_health: i32,
    pub max_health: i32,
    pub sprite_location: String,
    pub can_be_walked_on: bool,
    pub can_be_broken: bool,
}

impl From<&KdlNode> for StructureHealthFileConfig {
    fn from(node: &KdlNode) -> Self {
        let sprite = match node.properties.get("sprite") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let max_health = match node.properties.get("max_health") {
            Some(KdlValue::Int(it)) => *it as i32,
            _ => DEFAULT_MAX_HEALTH,
        };

        let min_health = match node.properties.get("min_health") {
            Some(KdlValue::Int(it)) => *it as i32,
            _ => DEFAULT_MIN_HEALTH,
        };

        let can_be_broken = match node.properties.get("can_be_broken") {
            Some(KdlValue::Boolean(it)) => *it,
            _ => false,
        };

        let can_be_walked_on = match node.properties.get("can_be_walked_on") {
            Some(KdlValue::Boolean(it)) => *it,
            _ => false,
        };

        Self {
            can_be_broken,
            can_be_walked_on,
            max_health,
            min_health,
            sprite_location: sprite,
        }
    }
}

pub struct StructuresConfig {
    pub configurations: Vec<StructureConfig>,
}

impl StructuresConfig {
    pub fn config_by_key(&self, key: &str) -> Option<&StructureConfig> {
        self.configurations.iter().find(|c| c.key == key)
    }

    pub fn load(path: &str) -> Self {
        let structure_nodes = parse(path).unwrap();
        let configurations: Vec<StructureConfig> =
            structure_nodes.iter().map(StructureConfig::from).collect();
        Self { configurations }
    }
}
