pub struct CropConfiguration {
    pub sprite_location: String,
    pub name: String,
}

pub struct CropConfigurations {
    pub configurations: Vec<CropConfiguration>,
}

impl Default for CropConfigurations {
    fn default() -> Self {
        let mut crops: Vec<CropConfiguration> = Vec::new();

        crops.push(CropConfiguration {
            sprite_location: "sprites/giant_mushroom.png".to_string(),
            name: "Mushroom".to_string(),
        });

        return CropConfigurations {
            configurations: crops,
        };
    }
}
