use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use crate::{input::Direction, tile::Tile};
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
        (*x, *y) = (4, 1);
        *shape = get_shape(*x, *y, shape_template);
    }
}
// TODO; rotation collision check not working how it's supposed to; pieces can go into other tiles
// and delete them from the grid
pub fn check_collision(
    grid: &mut [[u8; 20]; 10],
    shape: &Vec<Option<Tile>>,
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
            if val > 0
                && val < 100
                && !shape.iter().any(|to_check| {
                    let mut is_own = false;
                    if let Some(t2) = to_check {
                        if t2.x == t.x + diff_x && t.y + diff_y == t2.y {
                            is_own = true;
                        }
                    }
                    is_own
                })
            {
                res = true
            }
        }
    });
    return res;
}
// TODO check_collision function doesn't work with this function for some reason; fix it and
// refactor it to use check_collision and make it less messy
pub fn rotate(
    (x, y): (&mut i8, &mut i8),
    grid: &mut [[u8; 20]; 10],
    shape: &mut Vec<Option<Tile>>,
    shape_template: &mut Vec<u8>,
) {
    let offsets = [
        // try to move rotated piece in these directions. If any of them passes, use that one
        (0, 0), // x, y
        (-1, 0),
        (1, 0),
        (0, 1),
        (-1, 1),
        (1, 1),
        (0, -1),
        (1, -1),
        (-1, -1),
    ];
    let mut image = vec_to_image(shape_template);
    rotate_image(&mut image);
    let shape_template_temp: Vec<u8> = image.into_iter().flatten().collect();
    let mut shape_temp: Vec<Option<Tile>>;
    for m in offsets {
        shape_temp = get_shape(*x + m.0, *y + m.1, &shape_template_temp);
        let mut is_ok = true;
        for t in &shape_temp {
            if let Some(tile) = t {
                if tile.x < 0
                    || tile.y < 0
                    || tile.x >= grid.len() as i8
                    || tile.y >= grid[0].len() as i8
                    || (grid[tile.x as usize][tile.y as usize] != 0
                        && !shape.iter().any(|t2| {
                            let mut is_own = false;
                            if let Some(tile2) = t2 {
                                if tile2.x == tile.x && tile2.y == tile.y {
                                    is_own = true;
                                }
                            }
                            return is_own;
                        }))
                {
                    is_ok = false;
                    break; // this offset is wrong, skip all other tiles of this piece and try
                           // another tuple
                }
            }
        }
        if is_ok {
            for t in &mut *shape {
                if let Some(tile) = t {
                    grid[tile.x as usize][tile.y as usize] = 0;
                }
            }
            *shape_template = shape_template_temp;
            *shape = shape_temp;
            *x += m.0;
            *y += m.1;
            spawn(grid, shape);
            return;
        }
    }
}
pub fn vec_to_image<T>(vector: &mut Vec<T>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut image: Vec<Vec<T>> = Vec::new();
    let dimension = (vector.len() as f32).sqrt().abs();
    assert_eq!(dimension, dimension.abs());
    for i in 0..dimension as usize {
        image.push(Vec::new());
        for j in 0..dimension as usize {
            image[i].push(vector[i * dimension as usize + j]);
        }
    }
    return image;
}
pub fn rotate_image<T>(image: &mut Vec<Vec<T>>)
where
    T: Copy,
    T: Display,
{
    assert_eq!(image.len(), image[0].len());

    let image_len = image.len();
    for i in 0..image_len {
        for j in i..image_len {
            (image[i][j], image[j][i]) = (image[j][i], image[i][j]);
        }
    }

    for i in 0..image_len {
        for j in 0..image_len / 2 {
            (image[i][j], image[i][image_len - 1 - j]) = (image[i][image_len - 1 - j], image[i][j])
        }
    }
}
