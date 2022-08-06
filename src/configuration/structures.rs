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
    pub sprite_index: Option<usize>,
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

    pub fn can_be_cleared(&self) -> bool {
        self.file_config.can_be_cleared
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
        let name = match node.entries().get(0) {
            Some(entry) => match entry.value() {
                KdlValue::RawString(it) | KdlValue::String(it) => {
                    super::kdl_utils::trim(it.clone())
                }
                _ => "".to_string(),
            },
            _ => "".to_string(),
        };

        let key = match node.get("key") {
            Some(entry) => match entry.value() {
                KdlValue::RawString(it) | KdlValue::String(it) => {
                    super::kdl_utils::trim(it.clone())
                }
                _ => "".to_string(),
            },
            _ => "".to_string(),
        };

        let structure_type = match node.get("type") {
            Some(entry) => match entry.value() {
                KdlValue::RawString(it) | KdlValue::String(it) => {
                    super::kdl_utils::trim(it.clone())
                }
                _ => "".to_string(),
            },
            _ => "".to_string(),
        };

        let starting_health = match node.get("health") {
            Some(entry) => match entry.value() {
                KdlValue::Base10(it) => *it as i32,
                _ => DEFAULT_MAX_HEALTH,
            },
            _ => DEFAULT_MAX_HEALTH,
        };

        let initial_visible: bool = match node.get("visible") {
            Some(entry) => match entry.value() {
                KdlValue::Bool(it) => *it,
                _ => true,
            },
            _ => true,
        };

        let health_configs = node
            .children()
            .iter()
            .flat_map(|doc| doc.nodes())
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
    pub can_be_cleared: bool,
}

impl From<&KdlNode> for StructureHealthFileConfig {
    fn from(node: &KdlNode) -> Self {
        let sprite = match node.get("sprite") {
            Some(entry) => match entry.value() {
                KdlValue::RawString(it) | KdlValue::String(it) => {
                    super::kdl_utils::trim(it.clone())
                }
                _ => "".to_string(),
            },
            _ => "".to_string(),
        };

        let max_health = match node.get("max_health") {
            Some(entry) => match entry.value() {
                KdlValue::Base10(it) => *it as i32,
                _ => DEFAULT_MAX_HEALTH,
            },
            _ => DEFAULT_MAX_HEALTH,
        };

        let min_health = match node.get("min_health") {
            Some(entry) => match entry.value() {
                KdlValue::Base10(it) => *it as i32,
                _ => DEFAULT_MIN_HEALTH,
            },
            _ => DEFAULT_MIN_HEALTH,
        };

        let can_be_broken = match node.get("can_be_broken") {
            Some(entry) => match entry.value() {
                KdlValue::Bool(it) => *it,
                _ => false,
            },
            _ => false,
        };

        let can_be_walked_on = match node.get("can_be_walked_on") {
            Some(entry) => match entry.value() {
                KdlValue::Bool(it) => *it,
                _ => false,
            },
            _ => false,
        };

        let can_be_cleared = match node.get("can_be_cleared") {
            Some(entry) => match entry.value() {
                KdlValue::Bool(it) => *it,
                _ => false,
            },
            _ => false,
        };

        Self {
            can_be_broken,
            can_be_walked_on,
            can_be_cleared,
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
