mod grid;
mod piece;
mod tile;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use grid::*;
use macroquad::prelude::*;
use piece::{
    get_random_shape_template_getter, get_shape, move_piece, move_piece_down, spawn, Direction,
};
use tile::Tile;

pub const BASE: u16 = 250;
const MOVE_DOWN_DELAY: f64 = 1.0;
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
    let get_random_shape = get_random_shape_template_getter(); // also closure, same reason
    let handle_input = handle_input_setup();
    let (mut x, mut y): (i8, i8) = (4, 1);
    let mut shape_template: Vec<u8> = get_random_shape();
    let mut shape: Vec<Option<Tile>> = get_shape(x, y, &shape_template);
    spawn(&mut grid, &shape);
    let mut last_update = get_time();
    loop {
        clear_background(WHITE);
        draw_grid(&grid);
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
        handle_input();
        next_frame().await;
    }
}

fn handle_input_setup() -> impl Fn() {
    let mut control_keys: HashMap<KeyCode, &str> = HashMap::new();
    control_keys.insert(KeyCode::Left, "");
    control_keys.insert(KeyCode::Right, "");
    control_keys.insert(KeyCode::Down, "");
    control_keys.insert(KeyCode::Up, "");
    control_keys.insert(KeyCode::Space, "");
    let pressed_keys_mutex: Arc<Mutex<HashMap<KeyCode, u32>>> =
        Arc::new(Mutex::new(HashMap::new()));
    control_keys.iter().for_each(|(key, _)| {
        pressed_keys_mutex.lock().unwrap().insert(*key, 0);
    });
    return move || {
        let mut pressed_keys = pressed_keys_mutex.lock().unwrap();
        control_keys.iter().for_each(|(key, _)| {
            if is_key_down(*key) {
                if let Some(k) = pressed_keys.get_mut(key) {
                    if *k == 0 || *k > 12 {
                        // invoke appropriate function here
                        println!("move ")
                    } else {
                        println!("delay")
                    }
                    *k += 1;
                }
            }
            if is_key_released(*key) {
                if let Some(k) = pressed_keys.get_mut(key) {
                    *k = 0;
                }
            }
        });
    };
}
