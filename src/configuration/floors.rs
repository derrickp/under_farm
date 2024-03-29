use kdl::{KdlNode, KdlValue};

use super::kdl_utils::parse;

pub struct FloorsConfig {
    pub configurations: Vec<FloorConfig>,
}

impl FloorsConfig {
    pub fn config_by_key(&self, key: &str) -> Option<&FloorConfig> {
        self.configurations.iter().find(|c| c.key == key)
    }
}

impl FloorsConfig {
    pub fn load(path: &str) -> Self {
        let floor_nodes = parse(path).unwrap();

        let configurations = floor_nodes.iter().map(FloorConfig::from).collect();
        Self { configurations }
    }
}

pub struct FloorConfig {
    pub key: String,
    pub sprite_options: Vec<FloorSpriteConfig>,
}

impl From<&KdlNode> for FloorConfig {
    fn from(node: &KdlNode) -> Self {
        let key = match node.get("key") {
            Some(entry) => match entry.value() {
                KdlValue::RawString(it) | KdlValue::String(it) => {
                    super::kdl_utils::trim(it.clone())
                }
                _ => "".to_string(),
            },
            _ => "".to_string(),
        };

        let sprite_options = node
            .children()
            .iter()
            .flat_map(|doc| doc.nodes())
            .map(|it| FloorSpriteConfig {
                file_option: FloorFileOption::from(it),
                sprite_index: None,
            })
            .collect();

        Self {
            sprite_options,
            key,
        }
    }
}

pub struct FloorSpriteConfig {
    file_option: FloorFileOption,
    pub sprite_index: Option<usize>,
}

impl FloorSpriteConfig {
    pub fn sprite_location(&self) -> &str {
        &self.file_option.sprite
    }
}

pub struct FloorFileOption {
    pub sprite: String,
}

impl From<&KdlNode> for FloorFileOption {
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

        Self { sprite }
    }
}
