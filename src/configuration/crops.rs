pub struct CropConfiguration {
    pub name: &'static str,
    pub sprite_index: Option<u32>,
    pub stages: Vec<CropStage>,
}

impl CropConfiguration {
    pub fn build_basic(name: &'static str, stages: Vec<CropStage>) -> Self {
        return CropConfiguration {
            name,
            stages,
            sprite_index: None,
        };
    }
}

pub struct CropStage {
    pub name: &'static str,
    pub sprite_location: &'static str,
    pub sprite_index: Option<u32>,
}

pub struct CropConfigurations {
    pub configurations: Vec<CropConfiguration>,
}

fn kane_configuration() -> CropConfiguration {
    let stages = vec![CropStage {
        name: "Kane Stalk",
        sprite_location: "sprites/kane_stalks.png",
        sprite_index: None,
    }];

    return CropConfiguration::build_basic("Kane", stages);
}

fn mushroom_configuration() -> CropConfiguration {
    let stages = vec![CropStage {
        name: "Mushroom Plant",
        sprite_location: "sprites/giant_mushroom.png",
        sprite_index: None,
    }];
    return CropConfiguration::build_basic("Mushroom", stages);
}

fn potato_configuration() -> CropConfiguration {
    let stages = vec![CropStage {
        name: "Potatoes",
        sprite_location: "sprites/potatoes.png",
        sprite_index: None,
    }];

    return CropConfiguration::build_basic("Potato", stages);
}

fn turnip_configuration() -> CropConfiguration {
    let stages = vec![CropStage {
        name: "Turnip",
        sprite_location: "sprites/turnip_top.png",
        sprite_index: None,
    }];

    return CropConfiguration::build_basic("Turnip", stages);
}

impl Default for CropConfigurations {
    fn default() -> Self {
        let crops: Vec<CropConfiguration> = vec![
            mushroom_configuration(),
            kane_configuration(),
            potato_configuration(),
            turnip_configuration(),
        ];

        return CropConfigurations {
            configurations: crops,
        };
    }
}
