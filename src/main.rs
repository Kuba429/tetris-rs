use macroquad::prelude::*;
const BASE: u16 = 250;
fn conf() -> Conf {
    Conf {
        fullscreen: false,
        window_resizable: false,
        window_height: BASE as i32 * 2,
        window_width: BASE as i32 + 2 * BASE as i32 / 3,
        window_title: String::from("Tetris"),
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut grid: [[u8; 20]; 10] = [[0; 20]; 10];
    let draw_grid = draw_grid_setup(&mut grid); // returns a closure to get some grid dependant values that i don't want to calculate on every frame
    loop {
        clear_background(WHITE);
        draw_grid(&mut grid);
        next_frame().await;
    }
}

fn draw_grid_setup(grid: &mut [[u8; 20]; 10]) -> impl Fn(&mut [[u8; 20]; 10]) {
    let tile_width: f32 = BASE as f32 / grid.len() as f32;
    let offset: f32 = BASE as f32 / 3.0;
    return move |grid: &mut [[u8; 20]; 10]| {
        draw_rectangle(
            offset,
            0.0,
            BASE.into(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.1),
        ); // mesh
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                let mut color = WHITE;
                if grid[x][y] != 0 {
                    color = BLUE;
                };
                draw_rectangle(
                    x as f32 * tile_width + offset + 0.5,
                    y as f32 * tile_width + 0.5,
                    tile_width - 1.0,
                    tile_width - 1.0,
                    color,
                )
            }
        }
    };
}
