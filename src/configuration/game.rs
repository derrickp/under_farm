use kdl::{KdlNode, KdlValue};

use crate::configuration::{kdl_utils::parse, world::WorldGenerationConfig};

use super::{
    crops::CropsConfig, floors::FloorsConfig, load::Load, player::PlayerConfig,
    structures::StructuresConfig, tools::ToolConfigurations,
};

pub struct GameConfiguration {
    pub crops_config: CropsConfig,
    pub floors_config: FloorsConfig,
    pub structures_config: StructuresConfig,
    pub player_config: PlayerConfig,
    pub world_config: WorldGenerationConfig,
    pub sprite_config: SpriteConfig,
    pub tool_configs: ToolConfigurations,
    pub seed: String,
}

impl GameConfiguration {
    pub fn map_size(&self) -> usize {
        self.world_config.world_stats.map_size
    }

    pub fn tile_size(&self) -> f32 {
        self.sprite_config.size * self.sprite_config.scale
    }
}

pub struct BasicConfig {
    seed: String,
}

pub struct SpriteConfig {
    pub crop_scale: f32,
    pub scale: f32,
    pub player_scale: f32,
    pub size: f32,
}

impl Default for SpriteConfig {
    fn default() -> Self {
        Self {
            crop_scale: 3.0,
            scale: 4.0,
            player_scale: 2.0,
            size: 32.0,
        }
    }
}

impl From<&KdlNode> for SpriteConfig {
    fn from(node: &KdlNode) -> Self {
        let size = match node.properties.get("size") {
            Some(KdlValue::Float(it)) => *it as f32,
            _ => SpriteConfig::default().size,
        };

        let crop_scale = match node.properties.get("crop_scale") {
            Some(KdlValue::Float(it)) => *it as f32,
            _ => SpriteConfig::default().crop_scale,
        };

        let scale = match node.properties.get("scale") {
            Some(KdlValue::Float(it)) => *it as f32,
            _ => SpriteConfig::default().scale,
        };

        let player_scale = match node.properties.get("player_scale") {
            Some(KdlValue::Float(it)) => *it as f32,
            _ => SpriteConfig::default().player_scale,
        };

        Self {
            size,
            crop_scale,
            scale,
            player_scale,
        }
    }
}

impl From<&KdlNode> for BasicConfig {
    fn from(node: &KdlNode) -> Self {
        let seed = match node.properties.get("seed") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        Self { seed }
    }
}

impl Load for GameConfiguration {
    fn load(path: &str) -> Self {
        let crops_config_path = format!("{}/crops.kdl", path);
        let floors_config_path = format!("{}/floors.kdl", path);
        let structures_config_path = format!("{}/structures.kdl", path);
        let player_config_path = format!("{}/player.kdl", path);
        let game_config_path = format!("{}/game.kdl", path);
        let world_config_path = format!("{}/world.kdl", path);
        let game_config_nodes = parse(&game_config_path).unwrap();

        let basic_node = game_config_nodes
            .iter()
            .find(|node| node.name.eq_ignore_ascii_case("basic"))
            .unwrap();
        let basic_config = BasicConfig::from(basic_node);

        let sprite_config = game_config_nodes
            .iter()
            .find(|node| node.name.eq_ignore_ascii_case("sprite_stats"))
            .map_or_else(SpriteConfig::default, SpriteConfig::from);

        let crops_config = CropsConfig::load(&crops_config_path);
        let floors_config = FloorsConfig::load(&floors_config_path);
        let structures_config = StructuresConfig::load(&structures_config_path);
        let player_config = PlayerConfig::load(&player_config_path);
        let world_config = WorldGenerationConfig::load(&world_config_path);

        Self {
            crops_config,
            floors_config,
            structures_config,
            player_config,
            world_config,
            sprite_config,
            tool_configs: ToolConfigurations::default(),
            seed: basic_config.seed,
        }
    }
}
