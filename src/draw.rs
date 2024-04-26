use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

/// convert game block size to pixels size
pub fn make_coords_f64(game_coords: u32) -> f64 {
    (game_coords as f64) * BLOCK_SIZE
}

/// draw one block at x, y using provide color (for snake and apple)
pub fn draw_block(color: Color, x_pos: u32, y_pos: u32, con: Context, g: &mut G2d) {
    rectangle(
        color,
        [
            make_coords_f64(x_pos),
            make_coords_f64(y_pos),
            BLOCK_SIZE,
            BLOCK_SIZE,
        ],
        con.transform,
        g,
    );
}

/// draw rectangle at x, y with a height and width using provide color (for borders)
pub fn draw_rectangle(
    color: Color,
    x_pos: u32,
    y_pos: u32,
    width: u32,
    height: u32,
    con: Context,
    g: &mut G2d,
) {
    rectangle(
        color,
        [
            make_coords_f64(x_pos),
            make_coords_f64(y_pos),
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
