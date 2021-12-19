use kdl::{KdlNode, KdlValue};

use super::kdl_utils::parse;

pub struct PlayerSpriteConfigs {
    pub options: Vec<PlayerSpriteConfig>,
}

impl From<&KdlNode> for PlayerSpriteConfigs {
    fn from(node: &KdlNode) -> Self {
        let options = node.children.iter().map(PlayerSpriteConfig::from).collect();
        Self { options }
    }
}

pub struct PlayerSpriteConfig {
    pub file_config: SpriteFileConfig,
    pub sprite_index: Option<usize>,
}

impl From<&KdlNode> for PlayerSpriteConfig {
    fn from(node: &KdlNode) -> Self {
        let file_config = SpriteFileConfig::from(node);

        Self {
            file_config,
            sprite_index: None,
        }
    }
}

impl PlayerSpriteConfig {
    pub fn sprite_location(&self) -> &str {
        &self.file_config.sprite
    }
}

pub struct SpriteFileConfig {
    pub sprite: String,
}

impl From<&KdlNode> for SpriteFileConfig {
    fn from(node: &KdlNode) -> Self {
        let sprite = match node.properties.get("sprite") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        Self { sprite }
    }
}

pub struct PlayerInfo {
    pub name: String,
}

impl From<&KdlNode> for PlayerInfo {
    fn from(node: &KdlNode) -> Self {
        let name = match node.properties.get("name") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        Self { name }
    }
}

pub struct PlayerConfig {
    pub info: PlayerInfo,
    pub sprite_configs: PlayerSpriteConfigs,
}

impl PlayerConfig {
    pub fn load(path: &str) -> Self {
        let player_nodes = parse(path).unwrap();

        let info_node = player_nodes
            .iter()
            .find(|node| node.name.eq_ignore_ascii_case("info"))
            .unwrap();
        let info = PlayerInfo::from(info_node);

        let sprites_node = player_nodes
            .iter()
            .find(|node| node.name.eq_ignore_ascii_case("sprites"))
            .unwrap();

        let sprite_configs = PlayerSpriteConfigs::from(sprites_node);

        Self {
            info,
            sprite_configs,
        }
    }
}

impl PlayerConfig {
    pub fn starting_sprite(&self) -> &PlayerSpriteConfig {
        self.sprite_configs.options.first().unwrap()
    }
}
