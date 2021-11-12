use std::fs;

use super::timers::WORLD_TICK_TIME;

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
        let sprite = match node.properties.get("sprite") {
            Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
            _ => "".to_string(),
        };

        let min_ticks = match node.properties.get("min_ticks") {
            Some(KdlValue::Int(it)) => *it as u32,
            _ => DEFAULT_MIN_TICK,
        };

        let max_ticks = match node.properties.get("max_ticks") {
            Some(KdlValue::Int(it)) => *it as u32,
            _ => DEFAULT_MAX_TICK,
        };

        let advance_chance = match node.properties.get("advance_chance") {
            Some(KdlValue::Int(it)) => *it as u32,
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

pub struct CropConfiguration {
    pub name: String,
    pub stages: Vec<CropStage>,
}

pub struct CropStage {
    file_config: CropStageFileConfig,
    pub sprite_index: Option<u32>,
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

        TICKS_PER_SECOND * ticks
    }

    pub fn sprite_location(&self) -> &str {
        &self.file_config.sprite_location[..]
    }

    pub fn chance_to_advance(&self) -> u32 {
        self.file_config.chance_to_advance
    }
}

pub struct CropConfigurations {
    pub configurations: Vec<CropConfiguration>,
}

const TICKS_PER_SECOND: u32 = (1.0 / WORLD_TICK_TIME) as u32;

impl CropConfigurations {
    pub fn load(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap();
        let crop_nodes = kdl::parse_document(content).unwrap();
        let configurations: Vec<CropConfiguration> = crop_nodes
            .iter()
            .map(|crop_node| {
                let name = match crop_node.values.get(0) {
                    Some(KdlValue::String(it)) => super::kdl_utils::trim(it.clone()),
                    _ => "".to_string(),
                };
                let stages: Vec<CropStage> = crop_node
                    .children
                    .iter()
                    .map(|stage_node| CropStage {
                        sprite_index: None,
                        file_config: CropStageFileConfig::from(stage_node),
                    })
                    .collect();

                CropConfiguration { name, stages }
            })
            .collect();

        Self { configurations }
    }
}
