use super::{
    kdl_utils::{parse, parse_key_code},
    key_selector::KeySelector,
};

use kdl::{KdlNode, KdlValue};
use rand::Rng;

#[derive(Clone)]
struct CropStageFileConfig {
    sprite_location: String,
    min_ticks_in_stage: u32,
    max_ticks_in_stage: u32,
    chance_to_advance: u32,
}

const DEFAULT_MIN_TICK: u32 = 10;
const DEFAULT_MAX_TICK: u32 = 15;
const DEFAULT_CHANCE_TO_ADVANCE: u32 = 10;

impl From<&KdlNode> for CropStageFileConfig {
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

        let min_ticks = match node.get("min_ticks") {
            Some(entry) => match entry.value() {
                KdlValue::Base10(it) => *it as u32,
                _ => DEFAULT_MIN_TICK,
            },
            _ => DEFAULT_MIN_TICK,
        };

        let max_ticks = match node.get("max_ticks") {
            Some(entry) => match entry.value() {
                KdlValue::Base10(it) => *it as u32,
                _ => DEFAULT_MAX_TICK,
            },
            _ => DEFAULT_MAX_TICK,
        };

        let advance_chance = match node.get("advance_chance") {
            Some(entry) => match entry.value() {
                KdlValue::Base10(it) => *it as u32,
                _ => DEFAULT_CHANCE_TO_ADVANCE,
            },
            _ => DEFAULT_CHANCE_TO_ADVANCE,
        };

        Self {
            sprite_location: sprite,
            min_ticks_in_stage: min_ticks,
            max_ticks_in_stage: max_ticks,
            chance_to_advance: advance_chance,
        }
    }
}

#[derive(Clone)]
pub struct CropConfiguration {
    pub key: String,
    pub name: String,
    pub stages: Vec<CropStage>,
    pub inventory_selector: KeySelector,
    pub starter: bool,
}

#[derive(Clone)]
pub struct CropStage {
    file_config: CropStageFileConfig,
    pub sprite_index: Option<usize>,
    pub ticks_per_second: u32,
}

impl CropStage {
    pub fn configured_ticks_in_stage(&self) -> u32 {
        let mut ticks = self.file_config.min_ticks_in_stage;

        if self.file_config.min_ticks_in_stage != self.file_config.max_ticks_in_stage {
            let mut rng = rand::thread_rng();
            ticks = rng.gen_range(
                self.file_config.min_ticks_in_stage..self.file_config.max_ticks_in_stage,
            );
        }

        self.ticks_per_second * ticks
    }

    pub fn sprite_location(&self) -> &str {
        &self.file_config.sprite_location[..]
    }

    pub fn chance_to_advance(&self) -> u32 {
        self.file_config.chance_to_advance
    }
}

pub struct CropsConfig {
    pub configurations: Vec<CropConfiguration>,
}

impl CropsConfig {
    pub fn load(path: &str, world_tick_time: f32) -> Self {
        let crop_nodes = parse(path).unwrap();
        let configurations: Vec<CropConfiguration> = crop_nodes
            .iter()
            .map(|crop_node| {
                let name = match crop_node.entries().get(0) {
                    Some(entry) => match entry.value() {
                        KdlValue::RawString(it) | KdlValue::String(it) => {
                            super::kdl_utils::trim(it.clone())
                        }
                        _ => "".to_string(),
                    },
                    _ => "".to_string(),
                };
                let key = match crop_node.get("key") {
                    Some(entry) => match entry.value() {
                        KdlValue::RawString(it) | KdlValue::String(it) => {
                            super::kdl_utils::trim(it.clone())
                        }
                        _ => "".to_string(),
                    },
                    _ => "".to_string(),
                };
                let key_code = match crop_node.get("key_code") {
                    Some(entry) => match entry.value() {
                        KdlValue::RawString(it) | KdlValue::String(it) => {
                            super::kdl_utils::trim(it.clone())
                        }
                        _ => "".to_string(),
                    },
                    _ => "".to_string(),
                };
                let starter = match crop_node.get("starter") {
                    Some(entry) => match entry.value() {
                        KdlValue::Bool(it) => *it,
                        _ => false,
                    },
                    _ => false,
                };
                let stages: Vec<CropStage> = crop_node
                    .children()
                    .iter()
                    .flat_map(|doc| doc.nodes())
                    .map(|stage_node| CropStage {
                        ticks_per_second: (1.0 / world_tick_time) as u32,
                        sprite_index: None,
                        file_config: CropStageFileConfig::from(stage_node),
                    })
                    .collect();

                CropConfiguration {
                    name,
                    stages,
                    key,
                    starter,
                    inventory_selector: KeySelector {
                        key_code: parse_key_code(&key_code).unwrap(),
                        display_code: key_code.clone(),
                    },
                }
            })
            .collect();

        Self { configurations }
    }
}
