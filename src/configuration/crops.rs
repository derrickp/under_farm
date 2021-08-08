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

impl Default for CropConfigurations {
    fn default() -> Self {
        let mut crops: Vec<CropConfiguration> = Vec::new();

        crops.push(CropConfiguration::build_basic(
            "Mushroom",
            vec![CropStage {
                name: "Mushroom Plant",
                sprite_location: "sprites/giant_mushroom.png",
                sprite_index: None,
            }],
        ));

        crops.push(CropConfiguration::build_basic(
            "Kane",
            vec![CropStage {
                name: "Kane Stalk",
                sprite_location: "sprites/kane_stalks.png",
                sprite_index: None,
            }],
        ));

        crops.push(CropConfiguration::build_basic(
            "Potatoes",
            vec![CropStage {
                name: "Potatoes",
                sprite_location: "sprites/potatoes.png",
                sprite_index: None,
            }],
        ));

        crops.push(CropConfiguration::build_basic(
            "Turnips",
            vec![CropStage {
                name: "Turnip",
                sprite_location: "sprites/turnip_top.png",
                sprite_index: None,
            }],
        ));

        return CropConfigurations {
            configurations: crops,
        };
    }
}
