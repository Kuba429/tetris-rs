use crate::{
    piece::{get_shape, spawn},
    tile::Tile,
    GRID_H, GRID_W,
};

pub fn update_shadow(
    (x, y): (&i8, &i8),
    grid: &mut [[u8; GRID_H]; GRID_W],
    shape: &Vec<Option<Tile>>,
    shape_template: &Vec<u8>,
    shadow: &mut Vec<Option<Tile>>,
) {
    for x in &mut *grid {
        for y in x {
            if *y >= 100 {
                *y = 0
            }
        }
    }
    *shadow = get_shape(*x, *y, &shape_template);
    shadow.iter_mut().for_each(|tile| {
        if let Some(t) = tile {
            if t.val > 0 {
                t.val += 100;
            }
        }
    });
    'a: loop {
        for tile in &*shadow {
            if let Some(t) = tile {
                if t.y + 1 >= grid[0].len() as i8// check for overflow
                        || (grid[t.x as usize][t.y as usize + 1] > 0
                            && grid[t.x as usize][t.y as usize + 1] < 100 // check if this tile is an
                                                                           // obstacle (only shadows
                                                                          // are over 100)
                            // if needed make sure it's not a part of current piece
                         && !shape.iter().any(|to_check| {
                           let mut is_own = false;
                           if let Some(t2) = to_check {
                               if t2.x == t.x && t.y + 1 == t2.y {
                                   is_own = true;
                               }
                           }
                           is_own
                       }))
                {
                    break 'a;
                }
            }
        }
        for tile in &mut *shadow {
            if let Some(t) = tile {
                t.y += 1;
            }
        }
    }

    shadow.iter().for_each(|tile| {
        if let Some(t) = tile {
            if grid[t.x as usize][t.y as usize] > 100 || grid[t.x as usize][t.y as usize] == 0 {
                grid[t.x as usize][t.y as usize] = t.val;
            }
        }
    });
}
