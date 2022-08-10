use macroquad::prelude::*;

use crate::{GRID_H, GRID_W, TILE_W};
pub fn draw_grid(grid: &[[u8; GRID_H]; GRID_W]) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.1),
    ); // mesh
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let mut color = WHITE;
            if grid[x][y] != 0 {
                color = BLUE;
                draw_rectangle(x as f32 * TILE_W, y as f32 * TILE_W, TILE_W, TILE_W, color);
                continue;
            };
            draw_rectangle(
                x as f32 * TILE_W + 1.0,
                y as f32 * TILE_W + 1.0,
                TILE_W - 1.0,
                TILE_W - 1.0,
                color,
            )
        }
    }
}
pub fn remove_clean_rows(grid: &mut [[u8; GRID_H]; GRID_W]) {
    // loop through rows and remove them. If row isn't clean, skip this iteration(which will result
    // in row not being deleted)
    'y_loop: for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            // values over 100 are shadows
            if grid[x][y] == 0 || grid[x][y] > 100 {
                continue 'y_loop;
            }
        }
        for y in (0..y + 1).rev() {
            for x in 0..grid.len() {
                if y <= 0 {
                    grid[x][y] = 0;
                } else {
                    grid[x][y] = grid[x][y - 1];
                }
            }
        }
    }
}
