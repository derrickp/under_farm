use bevy::math::Vec3;

use super::bounding_box::BoundingBox;

pub struct Body {
    pub tile_size: f32,
    pub cell_center: Vec3,
}

impl Body {
    pub fn intersects_box(&self, bounding_box: &BoundingBox) -> bool {
        self.bounds().intersects(bounding_box)
    }

    pub fn bounds(&self) -> BoundingBox {
        BoundingBox::square(self.cell_center.x, self.cell_center.y, self.tile_size)
    }
}
