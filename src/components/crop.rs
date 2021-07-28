use bevy::prelude::{Bundle, SpriteSheetBundle};

pub struct CropName(pub String);

pub struct Crop;

#[derive(Bundle)]
pub struct CropBundle {
    pub name: CropName,
    pub crop: Crop,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}
