use rand::Rng;
use raylib::prelude::*;



const GRID_SIZE: (usize, usize) = (100, 100);
const SQUARE_SIZE: usize = 8;


struct GridPoint {
    collapsed: bool,
    states: Vec<Color>,
}



fn main() {
    let (mut rl, thread) = raylib::init()
        .size((GRID_SIZE.0 * SQUARE_SIZE) as i32, (GRID_SIZE.1 * SQUARE_SIZE) as i32)
        .title("Window")
        .build();

    rl.set_target_fps(600);

    let mut grid = generate_grid();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        wave_collape((&mut grid).into());

        for (y,row) in grid.iter().enumerate(){
            for (x ,grid_point) in row.iter().enumerate(){

                if grid_point.collapsed {
                    d.draw_rectangle(
                        (x * SQUARE_SIZE) as i32,
                        (y * SQUARE_SIZE) as i32,
                        SQUARE_SIZE as i32,
                        SQUARE_SIZE as i32,
                        <&Color as Into<ffi::Color>>::into(grid_point.states.get(0).unwrap()),
                    );
                }else{
                    d.draw_rectangle(
                        (x * SQUARE_SIZE) as i32,
                        (y * SQUARE_SIZE) as i32,
                        SQUARE_SIZE as i32,
                        SQUARE_SIZE as i32,
                        Color::BLACK
                    );
                }
            }
        }

    }
}

fn generate_grid() -> Vec<Vec<GridPoint>> {
    let mut rng = rand::thread_rng();

    let mut grid = Vec::with_capacity(GRID_SIZE.0);
    for _ in 0..GRID_SIZE.0 {
        let mut row = Vec::with_capacity(GRID_SIZE.1);
        for _ in 0..GRID_SIZE.1 {
            let states = vec![Color::RED,Color::GREEN,Color::BLUE,Color::BLACK,Color::WHITE];
            row.push(GridPoint {
                collapsed: false,
                states,
            });
        }
        grid.push(row);
    }
    let start_point = (rng.gen_range(0..GRID_SIZE.0 as i32),rng.gen_range(0..GRID_SIZE.1 as i32));
    let mut test:GridPoint = grid.get(start_point.1 as usize).unwrap().get(start_point.0 as usize).unwrap();
    test.collapsed = true;

    grid
}


fn wave_collape(grid: &mut Vec<Vec<GridPoint>>) {
    let mut rng = rand::thread_rng();

    let mut candidate_points: Vec<&mut GridPoint> = Vec::new();
    for row in grid.iter_mut(){
        for grid_point in row.iter_mut(){
            if !grid_point.collapsed {
                candidate_points.push(grid_point);
            }
        }
    }

    let candidates_length = candidate_points.len();
    if let Some(collapsed_point) = candidate_points.get_mut(rng.gen_range(0..candidates_length)) {
        let states_count = collapsed_point.states.len();
        if let Some(new_state) = collapsed_point.states.get(rng.gen_range(0..states_count)) {
            collapsed_point.states = vec![*new_state].clone();
            collapsed_point.collapsed = true;
        }
    }
}
