use std::fs;

use super::timers::WORLD_TICK_TIME;

use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct CropStageFileConfig {
    name: String,
    sprite_location: String,
    min_ticks_in_stage: u32,
    max_ticks_in_stage: u32,
    chance_to_advance: u32,
}

#[derive(Serialize, Deserialize)]
struct CropFileConfiguration {
    name: String,
    stages: Vec<CropStageFileConfig>,
}

#[derive(Serialize, Deserialize)]
struct CropFileConfigurations {
    configurations: Vec<CropFileConfiguration>,
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
    pub fn load() -> Self {
        let config_file_name = "./assets/config/crops.json";
        let contents = fs::read_to_string(config_file_name).unwrap();
        let config: CropFileConfigurations = serde_json::from_str(&contents).unwrap();

        let crops: Vec<CropConfiguration> = config
            .configurations
            .iter()
            .map(|file_config| CropConfiguration {
                name: file_config.name.clone(),
                stages: file_config
                    .stages
                    .iter()
                    .map(|file_stage_config| CropStage {
                        file_config: file_stage_config.clone(),
                        sprite_index: None,
                    })
                    .collect(),
            })
            .collect();

        Self {
            configurations: crops,
        }
    }
}
