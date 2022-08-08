use std::sync::{Arc, Mutex};

use crate::tile::Tile;
use rand::prelude::*;
#[rustfmt::skip] // easier to see the shapes
pub fn get_random_shape_template_getter() -> impl Fn() -> Vec<u8> {
    let shape_templates: Arc<Mutex<[Vec<u8>]>> = Arc::new(Mutex::new([
        vec![
            0, 0, 0, 0, 
            0, 1, 1, 0, 
            0, 1, 1, 0, 
            0, 0, 0, 0],
        vec![
            0, 2, 0, 0, 
            0, 2, 0, 0, 
            0, 2, 0, 0, 
            0, 2, 0, 0],
        vec![
            0, 3, 3, 
            3, 3, 0, 
            0, 0, 0],
        vec![
            4, 4, 0, 
            0, 4, 4, 
            0, 0, 0],
        vec![
            5, 0, 0, 
            5, 5, 5, 
            0, 0, 0],
        vec![
            0, 0, 6, 
            6, 6, 6, 
            0, 0, 0],
        vec![
            0, 7, 0, 
            7, 7, 7, 
            0, 0, 0],
    ]));
    let rng = Arc::new(Mutex::new(rand::thread_rng()));
    return move || -> Vec<u8> {
        let shapes = shape_templates.lock().unwrap();
        let random_idx = rng.lock().unwrap().gen_range(0..shapes.len());
        return (*shapes[random_idx]).try_into().unwrap();
    };
}
pub fn get_shape(x: i8, y: i8, template: &Vec<u8>) -> Vec<Option<Tile>> {
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
pub fn spawn(grid: &mut [[u8; 20]; 10], shape: &Vec<Option<Tile>>) {
    shape.iter().for_each(|tile| {
        if let Some(t) = tile {
            grid[t.x as usize][t.y as usize] = t.val;
        };
    });
}
pub fn move_piece(
    (x, y): (&mut i8, &mut i8),
    grid: &mut [[u8; 20]; 10],
    shape: &mut Vec<Option<Tile>>,
    direction: Direction,
) -> bool {
    let (diff_x, diff_y) = match direction {
        Direction::LEFT => (-1, 0),
        Direction::RIGHT => (1, 0),
        Direction::BOTTOM => (0, 1),
    };
    let collision = check_collision(grid, shape, (diff_x, diff_y));
    if collision {
        return false;
    }
    // good to go
    *x += diff_x;
    *y += diff_y;
    shape.iter_mut().for_each(|tile| {
        if let Some(t) = tile {
            grid[t.x as usize][t.y as usize] = 0;
            t.x += diff_x;
            t.y += diff_y
        }
    });
    shape.iter_mut().for_each(|tile| {
        if let Some(t) = tile {
            grid[t.x as usize][t.y as usize] = t.val;
        }
    });
    return true;
}
pub fn move_piece_down(
    // this is just a proxy function, might delete it later idk
    (x, y): (&mut i8, &mut i8),
    grid: &mut [[u8; 20]; 10],
    shape_template: &mut Vec<u8>,
    shape: &mut Vec<Option<Tile>>,
    get_random_shape_template: &dyn Fn() -> Vec<u8>,
) {
    let get_new_piece = move_piece((x, y), grid, shape, Direction::BOTTOM);
    if !get_new_piece {
        *shape_template = get_random_shape_template();
        *shape = get_shape(*x, *y, shape_template);
        (*x, *y) = (4, 1);
    }
}
pub fn check_collision(
    grid: &mut [[u8; 20]; 10],
    shape: &mut Vec<Option<Tile>>,
    (diff_x, diff_y): (i8, i8),
) -> bool {
    let mut res = false;
    shape.iter().for_each(|tile| {
        if let Some(t) = tile {
            // first check if index is out of bounds
            if t.y + diff_y >= grid[0].len() as i8
                || t.y + diff_y < 0
                || t.x + diff_x >= grid.len() as i8
                || t.x + diff_x < 0
            {
                res = true
            }
            if res {
                // return early if there is collision at this point
                return;
            }
            let val = grid[(t.x + diff_x) as usize][(t.y + diff_y) as usize];
            // if there is a tile on the grid at this position, check if it's a part of this piece
            if val > 0 && val < 100 {
                let mut found = false;
                let mut i = 0;
                while !found && i < shape.len() {
                    if let Some(maybe_own) = &shape[i] {
                        if maybe_own.x == t.x + diff_x
                            && maybe_own.y == t.y + diff_y
                            && maybe_own.val == val
                        {
                            found = true
                        }
                    }
                    i += 1;
                }

                if !found {
                    res = true
                }
            }
        }
    });
    return res;
}
pub enum Direction {
    RIGHT,
    LEFT,
    BOTTOM,
}
