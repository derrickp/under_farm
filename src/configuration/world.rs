use kdl::{KdlNode, KdlValue};
use tdlg::{generator::Generator, loading::RoomPaths};

use super::{kdl_utils::parse, load::Load};

pub struct WorldGenerationConfig {
    pub room_paths: Vec<RoomPaths>,
    pub world_stats: WorldStatsConfig,
}

impl Load for WorldGenerationConfig {
    fn load(path: &str) -> Self {
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
        }
    }
}

pub struct WorldStatsConfig {
    pub num_rooms: usize,
    pub map_size: usize,
}

impl Default for WorldStatsConfig {
    fn default() -> Self {
        Self {
            num_rooms: 100,
            map_size: 150,
        }
    }
}

impl From<&KdlNode> for WorldStatsConfig {
    fn from(node: &KdlNode) -> Self {
        let num_rooms = match node.properties.get("num_rooms") {
            Some(KdlValue::Int(it)) => *it as usize,
            _ => 100,
        };

        let map_size = match node.properties.get("map_size") {
            Some(KdlValue::Int(it)) => *it as usize,
            _ => 100,
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
