use kdl::{KdlNode, KdlValue};
use tdlg::{generator::Generator, loading::RoomPaths};

use super::{kdl_utils::parse, load::Load, map::MAP_SIZE};

const NUMBER_OF_ROOMS: usize = 100;

pub struct WorldGenerationConfig {
    pub room_paths: Vec<RoomPaths>,
}

impl Load for WorldGenerationConfig {
    fn load(path: &str) -> Self {
        let nodes = parse(path).unwrap();

        let room_paths: Vec<RoomPaths> = nodes
            .iter()
            .map(RoomConfig::from)
            .map(|config| config.room_paths())
            .collect();

        Self { room_paths }
    }
}

impl WorldGenerationConfig {
    pub fn generator(&self, seed: String) -> Generator {
        Generator {
            seed,
            grid_size: MAP_SIZE,
            target_number_rooms: NUMBER_OF_ROOMS,
            all_room_paths: self.room_paths.to_vec(),
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
