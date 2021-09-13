use rand::Rng;
use tdlg::{grid::Grid, room::Room};

const DEFAULT_GRID_SIZE: usize = 100;

pub fn generate_world_grid(templates: &Vec<String>) -> Grid<i32> {
    let mut grid = Grid::build(DEFAULT_GRID_SIZE);

    let mut rng = rand::thread_rng();
    let mut room_count = 0;

    let room_templates: Vec<Room<i32>> = templates
        .iter()
        .map(|template| Room::<i32>::from_template_string(template.clone()))
        .collect();

    for _ in 0..50 {
        let index: usize = rng.gen_range(0..room_templates.len());
        let template = room_templates.get(index).unwrap().clone();
        let max_side_length = (&template).max_side_length;

        let x: i32 = rng.gen_range(1..=(100 - max_side_length as i32));
        let y: i32 = rng.gen_range(1..=(100 - max_side_length as i32 - 1));
        let room = template.translate(x, y);

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
