use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use macroquad::prelude::{is_key_down, is_key_released, KeyCode};

use crate::{
    piece::move_piece,
    piece::{move_piece_down, rotate},
    tile::Tile,
    GRID_H, GRID_W,
};

pub fn handle_input_setup() -> impl Fn(
    (&mut i8, &mut i8),
    &mut [[u8; GRID_H]; GRID_W],
    &mut Vec<Option<Tile>>,
    &mut Vec<u8>,
    &dyn Fn() -> Vec<u8>,
) {
    let pressed_keys_mutex: Arc<Mutex<HashMap<KeyCode, u32>>> =
        Arc::new(Mutex::new(HashMap::from([
            (KeyCode::Left, 0),
            (KeyCode::Right, 0),
            (KeyCode::Down, 0),
            (KeyCode::Up, 0),
            (KeyCode::Space, 0),
        ])));
    return move |(x, y): (&mut i8, &mut i8),
                 grid: &mut [[u8; GRID_H]; GRID_W],
                 shape: &mut Vec<Option<Tile>>,
                 shape_template: &mut Vec<u8>,
                 get_random_shape_template: &dyn Fn() -> Vec<u8>| {
        let mut pressed_keys = pressed_keys_mutex.lock().unwrap();
        pressed_keys.iter_mut().for_each(|(key, count)| {
            if is_key_down(*key) {
                // invoke appropriate function here
                match *key {
                    KeyCode::Left => {
                        if *count == 0 || *count > 12 {
                            move_piece((x, y), grid, shape, Direction::LEFT);
                        }
                    }
                    KeyCode::Right => {
                        if *count == 0 || *count > 12 {
                            move_piece((x, y), grid, shape, Direction::RIGHT);
                        }
                    }
                    KeyCode::Down => {
                        if *count % 2 == 0 {
                            move_piece_down(
                                (x, y),
                                grid,
                                shape_template,
                                shape,
                                get_random_shape_template,
                            );
                        }
                    }

                    KeyCode::Up => {
                        if *count == 0 {
                            rotate((x, y), grid, shape, shape_template)
                        }
                    }
                    KeyCode::Space => {
                        todo!("drop");
                    }
                    _ => {
                        panic!("Unhandled key")
                    }
                }

                *count += 1;
            }
            if is_key_released(*key) {
                *count = 0
            }
        });
    };
}
pub enum Direction {
    RIGHT,
    LEFT,
    BOTTOM,
}
