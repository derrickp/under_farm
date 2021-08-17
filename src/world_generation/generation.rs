use super::{
    grid::Grid,
    rooms::{room_sizes, Room, RoomSize},
};
use rand::Rng;

pub fn generate_world_grid() -> Grid {
    let mut grid = Grid::default();

    // We would randomly pick a room here
    // But instead for now we're just using the 2x2

    let room_sizes = room_sizes(RoomSize::TwoByTwo);
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(0..=(100 - room_sizes.0));
    let y: i32 = rng.gen_range(0..=(100 - room_sizes.1));

    println!("{} {}", x, y);

    let room = Room::two_by_two_square(x, y);
    grid.add_room(room);
    grid.fill_empty_cells();
    grid.create_outer_wall();

    return grid;
}
