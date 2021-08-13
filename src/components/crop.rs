use bevy::prelude::{Bundle, SpriteSheetBundle};

pub struct CropName(pub String);

pub struct Crop {
    pub current_stage_index: usize,
}

pub struct CropStage {
    pub ticks_in_stage: u32,
    pub min_ticks_in_stage: u32,
    pub chance_to_advance: u32,
    pub sprite_index: u32,
}

pub struct CropStages {
    pub stages: Vec<CropStage>,
}

#[derive(Bundle)]
pub struct CropBundle {
    pub name: CropName,
    pub crop: Crop,
    pub stages: CropStages,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
