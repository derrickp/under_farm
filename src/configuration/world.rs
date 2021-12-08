use kdl::{KdlNode, KdlValue};
use tdlg::{
    cells::layer::LayerType,
    generator::{Generator, ItemChance, ItemGeneration},
    loading::RoomPaths,
};

use super::kdl_utils::parse;

pub struct WorldGenerationConfig {
    pub room_paths: Vec<RoomPaths>,
    pub world_stats: WorldStatsConfig,
}

impl WorldGenerationConfig {
    pub fn load(path: &str) -> Self {
        let nodes = parse(path).unwrap();

        let world_stats = nodes
            .iter()
            .find(|node| node.name.eq_ignore_ascii_case("world"))
            .map_or_else(WorldStatsConfig::default, WorldStatsConfig::from);

        let room_paths: Vec<RoomPaths> = nodes
            .iter()
            .filter(|node| node.name.eq_ignore_ascii_case("room_template"))
            .map(RoomConfig::from)
            .map(|config| config.room_paths())
            .collect();

        Self {
            room_paths,
            world_stats,
        }
    }
}

impl WorldGenerationConfig {
    pub fn generator(&self, seed: String) -> Generator {
        Generator {
            seed,
            grid_size: self.world_stats.map_size,
            target_number_rooms: self.world_stats.num_rooms,
            all_room_paths: self.room_paths.to_vec(),
            target_hidden_items: Some(ItemGeneration {
                target_num_items: 15,
                item_ranges: vec![ItemChance {
                    layer_type: LayerType::Note,
                    chance: 1..100,
                }],
            }),
            target_items: Some(ItemGeneration {
                target_num_items: 10,
                item_ranges: vec![ItemChance {
                    layer_type: LayerType::CommonItem,
                    chance: 1..100,
                }],
            }),
        }
    }
}

const DEFAULT_NUM_ROOMS: usize = 100;
const DEFAULT_MAP_SIZE: usize = 150;

pub struct WorldStatsConfig {
    pub num_rooms: usize,
    pub map_size: usize,
}

impl Default for WorldStatsConfig {
    fn default() -> Self {
        Self {
            num_rooms: DEFAULT_NUM_ROOMS,
            map_size: DEFAULT_MAP_SIZE,
        }
    }
}

impl From<&KdlNode> for WorldStatsConfig {
    fn from(node: &KdlNode) -> Self {
        let num_rooms = match node.properties.get("num_rooms") {
            Some(KdlValue::Int(it)) => *it as usize,
            _ => WorldStatsConfig::default().num_rooms,
        };

        let map_size = match node.properties.get("map_size") {
            Some(KdlValue::Int(it)) => *it as usize,
            _ => WorldStatsConfig::default().map_size,
        };

        Self {
            num_rooms,
            map_size,
        }
    }
}

struct RoomConfig {
    key: String,
    base_template_path: String,
    fill_template_paths: Vec<String>,
}

impl RoomConfig {
    pub fn room_paths(&self) -> RoomPaths {
        RoomPaths {
            name: self.key.clone(),
            base_template_path: self.base_template_path.clone(),
            fill_template_paths: self.fill_template_paths.to_vec(),
        }
    }
}

impl From<&KdlNode> for RoomConfig {
    fn from(node: &KdlNode) -> Self {
        let key = match node.properties.get("key") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let base_template_path = match node.properties.get("base") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let fill_template_paths: Vec<String> = node
            .children
            .iter()
            .map(|fill_node| match fill_node.values.get(0) {
                Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
                _ => "".to_string(),
            })
            .collect();

        Self {
            key,
            base_template_path,
            fill_template_paths,
        }
    }
}
