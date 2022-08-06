use macroquad::prelude::*;

use crate::BASE;

pub fn draw_grid_setup(grid: &[[u8; 20]; 10]) -> impl Fn(&[[u8; 20]; 10]) {
    let tile_width: f32 = BASE as f32 / grid.len() as f32;
    let offset: f32 = BASE as f32 / 3.0;
    return move |grid: &[[u8; 20]; 10]| {
        draw_rectangle(
            offset,
            0.0,
            BASE.into(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.1),
        ); // mesh
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                let mut color = WHITE;
                if grid[x][y] != 0 {
                    color = BLUE;
                };
                draw_rectangle(
                    x as f32 * tile_width + offset + 0.5,
                    y as f32 * tile_width + 0.5,
                    tile_width - 1.0,
                    tile_width - 1.0,
                    color,
                )
            }
        }
    };
}
