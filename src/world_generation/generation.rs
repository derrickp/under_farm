use super::{grid::Grid, rooms::Room};
use rand::Rng;

pub fn generate_world_grid() -> Grid {
    let mut grid = Grid::default();

    let mut rng = rand::thread_rng();
    let mut room_count = 0;

    for _ in 0..20 {
        let template = Room::random_template();
        let max_side_length = (&template).max_side_length;
        let x: i32 = rng.gen_range(1..=(100 - max_side_length as i32));
        let y: i32 = rng.gen_range(1..=(100 - max_side_length as i32 - 1));
        let room = Room::two_by_two_square(x, y, template);

        if room
            .cells
            .iter()
            .all(|cell| grid.is_cell_empty(&cell.coordinate))
        {
            println!("{} {}", x, y);
            room_count += 1;
            grid.add_room(room);
        }
    }

    println!("Room Count: {}", room_count);

    grid.fill_empty_cells();
    grid.create_outer_wall();

    return grid;
}
