mod grid;
mod input;
mod piece;
mod tile;

use grid::draw_grid;
use input::handle_input_setup;
use macroquad::prelude::*;
use piece::{get_random_shape_template_getter, get_shape, move_piece_down, spawn};
use tile::Tile;

pub const GRID_W: usize = 10;
pub const GRID_H: usize = 20;
pub const TILE_W: f32 = 25.0;

const MOVE_DOWN_DELAY: f64 = 1.0;
fn conf() -> Conf {
    Conf {
        fullscreen: false,
        window_resizable: false,
        window_height: (GRID_H as f32 * TILE_W) as i32,
        window_width: (GRID_W as f32 * TILE_W) as i32,
        window_title: String::from("Tetris"),
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut grid: [[u8; GRID_H]; GRID_W] = [[0; GRID_H]; GRID_W];
    let get_random_shape = get_random_shape_template_getter(); // returns a closure to get some grid dependant values that i don't want to calculate on every frame
    let (mut x, mut y): (i8, i8) = (4, 1);
    let mut shape_template: Vec<u8> = get_random_shape();
    let mut shape: Vec<Option<Tile>> = get_shape(x, y, &shape_template);
    spawn(&mut grid, &shape);
    let handle_input = handle_input_setup();
    let mut last_update = get_time();
    loop {
        clear_background(WHITE);
        draw_grid(&grid);
        handle_input(
            (&mut x, &mut y),
            &mut grid,
            &mut shape,
            &mut shape_template,
            &get_random_shape,
        );
        // move piece down
        if get_time() - last_update > MOVE_DOWN_DELAY {
            move_piece_down(
                (&mut x, &mut y),
                &mut grid,
                &mut shape_template,
                &mut shape,
                &get_random_shape,
            );
            last_update = get_time();
        }
        next_frame().await;
    }
}
