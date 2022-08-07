mod grid;
mod piece;
mod tile;

use grid::*;
use macroquad::prelude::*;
use piece::{get_random_shape_template_getter, get_shape};
use tile::Tile;

pub const BASE: u16 = 250;

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
    let draw_grid = draw_grid_setup(&grid); // returns a closure to get some grid dependant values that i don't want to calculate on every frame
    let (x, y): (u8, u8) = (4, 5);
    let mut shape_template: Vec<u8> = vec![0, 0, 0, 0, 1, 0, 1, 1, 1];
    let mut shape: Vec<Option<Tile>> = get_shape(x, y, shape_template);
    let get_random_shape = get_random_shape_template_getter();
    shape.iter().for_each(|tile| {
        if let Some(t) = tile {
            grid[t.x as usize][t.y as usize] = t.val;
        }
    });

    loop {
        clear_background(WHITE);
        draw_grid(&grid);
        shape_template = get_random_shape();
        println!("{}", shape_template[0]);
        // current_piece.move_to(BOTTOM);
        next_frame().await;
    }
}
