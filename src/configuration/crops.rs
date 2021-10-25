use crate::configuration::sprites::{
    GIANT_MUSHROOM, GIANT_MUSHROOM_SPORES, KANE_SEEDS, KANE_STALKS, POTATOES, POTATO_SEEDS,
    TURNIP_SEED, TURNIP_TOP,
};

use super::timers::WORLD_TICK_TIME;

pub struct CropConfiguration {
    pub name: &'static str,
    pub sprite_index: Option<u32>,
    pub stages: Vec<CropStage>,
}

impl CropConfiguration {
    pub fn build_basic(name: &'static str, stages: Vec<CropStage>) -> Self {
        CropConfiguration {
            name,
            stages,
            sprite_index: None,
        }
    }
}

pub struct CropStage {
    pub name: &'static str,
    pub sprite_location: &'static str,
    pub sprite_index: Option<u32>,
    pub min_ticks_in_stage: u32,
    pub chance_to_advance: u32,
}

pub struct CropConfigurations {
    pub configurations: Vec<CropConfiguration>,
}

const TICKS_PER_SECOND: u32 = (1.0 / WORLD_TICK_TIME) as u32;

fn kane_configuration() -> CropConfiguration {
    let stages = vec![
        CropStage {
            name: "Kane Seed",
            sprite_location: KANE_SEEDS,
            sprite_index: None,
            min_ticks_in_stage: TICKS_PER_SECOND * 15,
            chance_to_advance: 95,
        },
        CropStage {
            name: "Kane Stalk",
            sprite_location: KANE_STALKS,
            sprite_index: None,
            min_ticks_in_stage: TICKS_PER_SECOND * 30,
            chance_to_advance: 25,
        },
    ];

    CropConfiguration::build_basic("Kane", stages)
}

fn mushroom_configuration() -> CropConfiguration {
    let stages = vec![
        CropStage {
            name: "Mushroom Spores",
            sprite_location: GIANT_MUSHROOM_SPORES,
            sprite_index: None,
            min_ticks_in_stage: TICKS_PER_SECOND * 10,
            chance_to_advance: 95,
        },
        CropStage {
            name: "Mushroom Plant",
            sprite_location: GIANT_MUSHROOM,
            sprite_index: None,
            min_ticks_in_stage: TICKS_PER_SECOND * 30,
            chance_to_advance: 1,
        },
    ];
    CropConfiguration::build_basic("Mushroom", stages)
}

fn potato_configuration() -> CropConfiguration {
    let stages = vec![
        CropStage {
            name: "Potato Seeds",
            sprite_location: POTATO_SEEDS,
            sprite_index: None,
            min_ticks_in_stage: TICKS_PER_SECOND * 5,
            chance_to_advance: 95,
        },
        CropStage {
            name: "Potatoes",
            sprite_location: POTATOES,
            sprite_index: None,
            min_ticks_in_stage: TICKS_PER_SECOND * 60,
            chance_to_advance: 1,
        },
    ];

    CropConfiguration::build_basic("Potato", stages)
}

fn turnip_configuration() -> CropConfiguration {
    let stages = vec![
        CropStage {
            name: "Turnip Seeds",
            sprite_location: TURNIP_SEED,
            sprite_index: None,
            min_ticks_in_stage: TICKS_PER_SECOND * 20,
            chance_to_advance: 75,
        },
        CropStage {
            name: "Turnip",
            sprite_location: TURNIP_TOP,
            sprite_index: None,
            min_ticks_in_stage: TICKS_PER_SECOND * 45,
            chance_to_advance: 5,
        },
    ];

    CropConfiguration::build_basic("Turnip", stages)
}

impl Default for CropConfigurations {
    fn default() -> Self {
        let crops: Vec<CropConfiguration> = vec![
            mushroom_configuration(),
            kane_configuration(),
            potato_configuration(),
            turnip_configuration(),
        ];

        CropConfigurations {
            configurations: crops,
        }
    }
}
