use kdl::{KdlNode, KdlValue};

use crate::configuration::{kdl_utils::parse, world::WorldGenerationConfig};

use super::{
    crops::CropsConfig, floors::FloorsConfig, load::Load, player::PlayerConfig,
    structures::StructuresConfig,
};

pub struct GameConfiguration {
    pub crops_config: CropsConfig,
    pub floors_config: FloorsConfig,
    pub structures_config: StructuresConfig,
    pub player_config: PlayerConfig,
    pub world_config: WorldGenerationConfig,
    pub seed: String,
}

pub struct BasicConfig {
    seed: String,
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
            seed: basic_config.seed,
        }
    }
}
