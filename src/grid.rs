use std::collections::HashMap;

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
            if grid[x][y] != 0 {
                draw_rectangle(
                    x as f32 * TILE_W,
                    y as f32 * TILE_W,
                    TILE_W,
                    TILE_W,
                    get_color(grid[x][y]),
                );
                continue;
            };
            draw_rectangle(
                x as f32 * TILE_W + 1.0,
                y as f32 * TILE_W + 1.0,
                TILE_W - 1.0,
                TILE_W - 1.0,
                WHITE,
            )
        }
    }
}
fn get_color(val: u8) -> Color {
    // pattern matching doesn't seem to be working with u8 idk
    if val == 1 {
        return Color::from_rgba(253, 243, 142, 255);
    }
    if val == 2 {
        return Color::from_rgba(0, 221, 255, 255); //light blue
    }
    if val == 3 {
        return Color::from_rgba(148, 221, 142, 255); // green
    }
    if val == 4 {
        return Color::from_rgba(237, 77, 69, 255); //red
    }
    if val == 5 {
        return Color::from_rgba(0, 88, 227, 255); //darker blue
    }
    if val == 6 {
        return Color::from_rgba(234, 114, 24, 255); //orange
    }
    if val == 7 {
        return Color::from_rgba(237, 78, 224, 255); //purple
    }
    return BLUE;
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
