use std::sync::{Arc, Mutex};

use crate::tile::Tile;
use rand::prelude::*;
// #[rustfmt::skip] // easier to see the shapes
pub fn get_random_shape_template_getter() -> impl Fn() -> Vec<u8> {
    let shape_templates: Arc<Mutex<[Vec<u8>]>> = Arc::new(Mutex::new([
        vec![0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0],
        vec![0, 3, 3, 3, 3, 0, 0, 0, 0],
        vec![4, 4, 0, 0, 4, 4, 0, 0, 0],
        vec![5, 0, 0, 5, 5, 5, 0, 0, 0],
        vec![0, 0, 6, 6, 6, 6, 0, 0, 0],
        vec![0, 7, 0, 7, 7, 7, 0, 0, 0],
    ]));
    let rng = Arc::new(Mutex::new(rand::thread_rng()));
    return move || -> Vec<u8> {
        let shapes = shape_templates.lock().unwrap();
        let random_idx = rng.lock().unwrap().gen_range(0..shapes.len());
        return (*shapes[random_idx]).try_into().unwrap();
    };
}
pub fn get_shape(x: u8, y: u8, template: Vec<u8>) -> Vec<Option<Tile>> {
    return match template.len() {
        9 => template
            .iter()
            .enumerate()
            .map(|(idx, val)| {
                if *val == 0 {
                    return None;
                }
                return match idx {
                    0 => Some(Tile::new(x - 1, y - 1, *val)),
                    1 => Some(Tile::new(x, y - 1, *val)),
                    2 => Some(Tile::new(x + 1, y - 1, *val)),
                    3 => Some(Tile::new(x - 1, y, *val)),
                    4 => Some(Tile::new(x, y, *val)),
                    5 => Some(Tile::new(x + 1, y, *val)),
                    6 => Some(Tile::new(x - 1, y + 1, *val)),
                    7 => Some(Tile::new(x, y + 1, *val)),
                    8 => Some(Tile::new(x + 1, y + 1, *val)),
                    _ => None,
                };
            })
            .collect(),
        16 => template
            .iter()
            .enumerate()
            .map(|(idx, val)| {
                if *val == 0 {
                    return None;
                }
                return match idx {
                    0 => Some(Tile::new(x - 1, y - 1, *val)),
                    1 => Some(Tile::new(x, y - 1, *val)),
                    2 => Some(Tile::new(x + 1, y - 1, *val)),
                    3 => Some(Tile::new(x + 2, y - 1, *val)),
                    4 => Some(Tile::new(x - 1, y, *val)),
                    5 => Some(Tile::new(x, y, *val)),
                    6 => Some(Tile::new(x + 1, y, *val)),
                    7 => Some(Tile::new(x + 2, y, *val)),
                    8 => Some(Tile::new(x - 1, y + 1, *val)),
                    9 => Some(Tile::new(x, y + 1, *val)),
                    10 => Some(Tile::new(x + 1, y + 1, *val)),
                    11 => Some(Tile::new(x + 2, y + 1, *val)),
                    12 => Some(Tile::new(x - 1, y + 2, *val)),
                    13 => Some(Tile::new(x, y + 2, *val)),
                    14 => Some(Tile::new(x + 1, y + 2, *val)),
                    15 => Some(Tile::new(x + 2, y + 2, *val)),
                    _ => None,
                };
            })
            .collect(),
        _ => panic!("Unhandled shape template"),
    };
}

pub enum Direction {
    RIGHT,
    LEFT,
    BOTTOM,
}
