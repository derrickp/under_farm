use std::num::NonZeroU16;

use kdl::{KdlNode, KdlValue};
use tdlg::{
    generation::{builder, Generator, ItemChance, ItemGeneration},
    map::cells::LayerType,
};

use super::kdl_utils::parse;

pub struct WorldGenerationConfig {
    pub world_stats: WorldStatsConfig,
}

impl WorldGenerationConfig {
    pub fn load(path: &str) -> Self {
        let nodes = parse(path).unwrap();

        let world_stats = nodes
            .iter()
            .find(|node| node.name().value().eq_ignore_ascii_case("world"))
            .map_or_else(WorldStatsConfig::default, WorldStatsConfig::from);

        Self { world_stats }
    }
}

impl WorldGenerationConfig {
    pub fn generator(&self, seed: String) -> Generator {
        builder()
            .seed(&seed)
            .grid_size(NonZeroU16::new(self.world_stats.map_size).unwrap())
            .target_number_rooms(NonZeroU16::new(self.world_stats.num_rooms).unwrap())
            .target_hidden_items(ItemGeneration {
                target_num_items: 15,
                item_ranges: vec![ItemChance {
                    layer_type: LayerType::Note,
                    chance: 1..100,
                }],
            })
            .target_items(ItemGeneration {
                target_num_items: 10,
                item_ranges: vec![ItemChance {
                    layer_type: LayerType::CommonItem,
                    chance: 1..100,
                }],
            })
            .build()
    }
}

const DEFAULT_NUM_ROOMS: u16 = 100;
const DEFAULT_MAP_SIZE: u16 = 150;

pub struct WorldStatsConfig {
    pub num_rooms: u16,
    pub map_size: u16,
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
        let num_rooms = match node.get("num_rooms") {
            Some(entry) => match entry.value() {
                KdlValue::Base10(it) => *it as u16,
                _ => WorldStatsConfig::default().num_rooms,
            },
            _ => WorldStatsConfig::default().num_rooms,
        };

        let map_size = match node.get("map_size") {
            Some(entry) => match entry.value() {
                KdlValue::Base10(it) => *it as u16,
                _ => WorldStatsConfig::default().map_size,
            },
            _ => WorldStatsConfig::default().map_size,
        };

        Self {
            num_rooms,
            map_size,
        }
    }
}
