use rand::Rng;

pub enum RoomSize {
    TwoByTwo,
}

pub struct Room {
    pub wall_coordinates: Vec<(i32, i32)>,
    pub floor_coordinates: Vec<(i32, i32)>,
    pub door_coordinate: (i32, i32),
}

impl Room {
    // 2x2 floor, so 4x4 with walls
    pub fn two_by_two_square(bottom_left_x: i32, bottom_left_y: i32) -> Self {
        let right_x = bottom_left_x + 3;
        let top_y = bottom_left_y + 3;

        let mut corner_wall_coordinates: Vec<(i32, i32)> = Vec::new();
        let mut outside_wall_coordinates: Vec<(i32, i32)> = Vec::new();
        let mut floor_coordinates: Vec<(i32, i32)> = Vec::new();

        // Split our walls that can be doors from the ones that cannot
        // If we were to put a "door" on a corner, you'd be moving into the
        // room at an angle, and I don't want that.
        for x in (bottom_left_x + 1)..right_x {
            outside_wall_coordinates.push((x, bottom_left_y));
            outside_wall_coordinates.push((x, top_y));
        }

        for y in (bottom_left_y + 1)..top_y {
            outside_wall_coordinates.push((bottom_left_x, y));
            outside_wall_coordinates.push((right_x, y));
        }

        corner_wall_coordinates.push((bottom_left_x, bottom_left_y));
        corner_wall_coordinates.push((right_x, bottom_left_y));
        corner_wall_coordinates.push((bottom_left_x, top_y));
        corner_wall_coordinates.push((right_x, top_y));

        // Then we fill in the rest of the square with our floor
        for x in (bottom_left_x + 1)..(right_x) {
            for y in (bottom_left_y + 1)..(top_y) {
                floor_coordinates.push((x, y));
            }
        }

        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..outside_wall_coordinates.len());
        let door_coordinate = outside_wall_coordinates.remove(index); // Remove our door
        outside_wall_coordinates.append(&mut corner_wall_coordinates);

        return Self {
            door_coordinate,
            floor_coordinates,
            wall_coordinates: outside_wall_coordinates,
        };
    }
}

pub fn room_sizes(room_size: RoomSize) -> (i32, i32) {
    match room_size {
        RoomSize::TwoByTwo => (4, 4),
    }
}
