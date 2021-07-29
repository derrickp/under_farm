use bevy::{
    math::{Vec2, Vec3},
    prelude::{Bundle, Entity},
};

pub struct BoundingBox {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl BoundingBox {
    pub fn intersects(&self, bounding_box: &BoundingBox) -> bool {
        return self.min_x < bounding_box.max_x
            && self.max_x > bounding_box.min_x
            && self.max_y > bounding_box.min_y
            && self.min_y < bounding_box.max_y;
    }

    pub fn square(x: f32, y: f32, width: f32) -> BoundingBox {
        let half_width = (width / 2.0).floor();
        return BoundingBox {
            min_x: x - half_width,
            max_x: x + half_width,
            min_y: y - half_width,
            max_y: y + half_width,
        };
    }
}

pub struct GridName(pub String);

pub struct Grid {
    pub name: GridName,
    pub cells: Vec<GridCell>,
}

#[derive(Bundle)]
pub struct GridBundle {
    pub grid: Grid,
}

pub struct GridCell {
    pub cell_size: Vec2,
    pub cell_center: Vec3,
    pub contains_tile: bool,
    pub sprite: Option<Entity>,
    pub outline: Option<Entity>,
}

impl GridCell {
    pub fn intersects_box(&self, bounding_box: &BoundingBox) -> bool {
        return self.bounds().intersects(bounding_box);
    }

    pub fn bounds(&self) -> BoundingBox {
        return BoundingBox {
            min_x: self.cell_center.x - (self.cell_size.x / 2.0).floor(),
            max_x: self.cell_center.x + (self.cell_size.x / 2.0).floor(),
            min_y: self.cell_center.y - (self.cell_size.y / 2.0).floor(),
            max_y: self.cell_center.y + (self.cell_size.y / 2.0).floor(),
        };
    }
}
