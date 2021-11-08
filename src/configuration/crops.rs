use crate::configuration::sprites::{
    GIANT_MUSHROOM, GIANT_MUSHROOM_SPORES, GIANT_MUSHROOM_SPROUTS, KANE_SEEDS, KANE_STALKS,
    POTATOES, POTATO_SEEDS, TURNIP_SEED, TURNIP_TOP,
};

use super::timers::WORLD_TICK_TIME;

use rand::Rng;

pub struct CropConfiguration {
    pub name: &'static str,
    pub stages: Vec<CropStage>,
}

impl CropConfiguration {
    pub fn build_basic(name: &'static str, stages: Vec<CropStage>) -> Self {
        CropConfiguration { name, stages }
    }
}

pub struct CropStage {
    pub name: &'static str,
    pub sprite_location: &'static str,
    pub sprite_index: Option<u32>,
    pub min_ticks_in_stage: u32,
    pub max_ticks_in_stage: u32,
    pub chance_to_advance: u32,
}

impl CropStage {
    pub fn configured_ticks_in_stage(&self) -> u32 {
        let mut ticks = self.min_ticks_in_stage;

        if self.min_ticks_in_stage != self.max_ticks_in_stage {
            let mut rng = rand::thread_rng();
            ticks = rng.gen_range(self.min_ticks_in_stage..self.max_ticks_in_stage);
        }

        TICKS_PER_SECOND * ticks
    }
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
            min_ticks_in_stage: 15,
            max_ticks_in_stage: 30,
            chance_to_advance: 95,
        },
        CropStage {
            name: "Kane Stalk",
            sprite_location: KANE_STALKS,
            sprite_index: None,
            min_ticks_in_stage: 30,
            max_ticks_in_stage: 45,
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
            min_ticks_in_stage: 10,
            max_ticks_in_stage: 15,
            chance_to_advance: 90,
        },
        CropStage {
            name: "Mushroom Sprouts",
            sprite_location: GIANT_MUSHROOM_SPROUTS,
            sprite_index: None,
            min_ticks_in_stage: 10,
            max_ticks_in_stage: 20,
            chance_to_advance: 95,
        },
        CropStage {
            name: "Mushroom Plant",
            sprite_location: GIANT_MUSHROOM,
            sprite_index: None,
            min_ticks_in_stage: 30,
            max_ticks_in_stage: 40,
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
            min_ticks_in_stage: 5,
            max_ticks_in_stage: 10,
            chance_to_advance: 95,
        },
        CropStage {
            name: "Potatoes",
            sprite_location: POTATOES,
            sprite_index: None,
            min_ticks_in_stage: 60,
            max_ticks_in_stage: 90,
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
            min_ticks_in_stage: 20,
            max_ticks_in_stage: 30,
            chance_to_advance: 75,
        },
        CropStage {
            name: "Turnip",
            sprite_location: TURNIP_TOP,
            sprite_index: None,
            min_ticks_in_stage: 45,
            max_ticks_in_stage: 60,
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
