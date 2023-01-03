use bevy::{prelude::Resource, time::Timer};
use kdl::{KdlNode, KdlValue};
use tdlg::generator::Generator;

use crate::configuration::{kdl_utils::parse, world::WorldGenerationConfig};

use super::{
    crops::CropsConfig, floors::FloorsConfig, player::PlayerConfig, structures::StructuresConfig,
    tools::ToolConfigurations,
};

#[derive(Resource)]
pub struct GameConfiguration {
    pub crops_config: CropsConfig,
    pub floors_config: FloorsConfig,
    pub structures_config: StructuresConfig,
    pub player_config: PlayerConfig,
    pub world_config: WorldGenerationConfig,
    pub sprite_config: SpriteConfig,
    pub tool_configs: ToolConfigurations,
    pub seed: String,
    pub world_tick_time: f32,
    level: usize,
}

impl GameConfiguration {
    pub fn map_size(&self) -> usize {
        self.world_config.world_stats.map_size
    }

    pub fn tile_size(&self) -> f32 {
        self.sprite_config.size * self.sprite_config.scale
    }

    pub fn world_tick_timer(&self) -> Timer {
        Timer::from_seconds(self.world_tick_time, bevy::time::TimerMode::Repeating)
    }

    pub fn generator(&mut self, increment: bool) -> Generator {
        let seed = if increment {
            self.level += 1;
            format!("{}|{}", self.seed, self.level)
        } else {
            self.seed.clone()
        };

        self.world_config.generator(seed)
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
        let size = match node.get("size") {
            Some(entry) => match entry.value() {
                KdlValue::Base10Float(it) => *it as f32,
                _ => SpriteConfig::default().size,
            },
            _ => SpriteConfig::default().size,
        };

        let crop_scale = match node.get("crop_scale") {
            Some(entry) => match entry.value() {
                KdlValue::Base10Float(it) => *it as f32,
                _ => SpriteConfig::default().crop_scale,
            },
            _ => SpriteConfig::default().crop_scale,
        };

        let scale = match node.get("scale") {
            Some(entry) => match entry.value() {
                KdlValue::Base10Float(it) => *it as f32,
                _ => SpriteConfig::default().scale,
            },
            _ => SpriteConfig::default().scale,
        };

        let player_scale = match node.get("player_scale") {
            Some(entry) => match entry.value() {
                KdlValue::Base10Float(it) => *it as f32,
                _ => SpriteConfig::default().player_scale,
            },
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
        let seed = match node.get("seed") {
            Some(entry) => match entry.value() {
                KdlValue::RawString(it) | KdlValue::String(it) => {
                    super::kdl_utils::trim(it.clone())
                }
                _ => "".to_string(),
            },
            _ => "".to_string(),
        };

        Self { seed }
    }
}

pub const WORLD_TICK_TIME: f32 = 0.2;

impl GameConfiguration {
    pub fn load(path: &str) -> Self {
        let crops_config_path = format!("{path}/crops.kdl");
        let floors_config_path = format!("{path}/floors.kdl");
        let structures_config_path = format!("{path}/structures.kdl");
        let player_config_path = format!("{path}/player.kdl");
        let game_config_path = format!("{path}/game.kdl");
        let world_config_path = format!("{path}/world.kdl");
        let tool_config_path = format!("{path}/tools.kdl");
        let game_config_nodes = parse(&game_config_path).unwrap();

        let basic_node = game_config_nodes
            .iter()
            .find(|node| node.name().value().eq_ignore_ascii_case("basic"))
            .unwrap();
        let basic_config = BasicConfig::from(basic_node);

        let sprite_config = game_config_nodes
            .iter()
            .find(|node| node.name().value().eq_ignore_ascii_case("sprite_stats"))
            .map_or_else(SpriteConfig::default, SpriteConfig::from);

        let crops_config = CropsConfig::load(&crops_config_path, WORLD_TICK_TIME);
        let floors_config = FloorsConfig::load(&floors_config_path);
        let structures_config = StructuresConfig::load(&structures_config_path);
        let player_config = PlayerConfig::load(&player_config_path);
        let world_config = WorldGenerationConfig::load(&world_config_path);
        let tool_configs = ToolConfigurations::load(&tool_config_path);

        Self {
            crops_config,
            floors_config,
            structures_config,
            player_config,
            world_config,
            sprite_config,
            tool_configs,
            seed: basic_config.seed,
            world_tick_time: WORLD_TICK_TIME,
            level: 0,
        }
    }
}
