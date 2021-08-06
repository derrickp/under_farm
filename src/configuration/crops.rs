pub struct CropConfiguration {
    pub sprite_location: &'static str,
    pub name: &'static str,
    pub sprite_index: Option<u32>,
}

pub struct CropConfigurations {
    pub configurations: Vec<CropConfiguration>,
}

impl Default for CropConfigurations {
    fn default() -> Self {
        let mut crops: Vec<CropConfiguration> = Vec::new();

        crops.push(CropConfiguration {
            sprite_location: "sprites/giant_mushroom.png",
            name: "Mushroom",
            sprite_index: None,
        });

        crops.push(CropConfiguration {
            sprite_location: "sprites/kane_stalks.png",
            name: "Kane",
            sprite_index: None,
        });

        crops.push(CropConfiguration {
            sprite_location: "sprites/potatoes.png",
            name: "Potatoes",
            sprite_index: None,
        });

        return CropConfigurations {
            configurations: crops,
        };
    }
}
