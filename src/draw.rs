use piston_window::{Context, G2d, ellipse, rectangle};
use piston_window::types::Color;
use std::f64::consts::{PI};


const BLOCK_SIZE: f64 = 25.0;


/// Convert game board coordinate to drawing coordinate
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

/// Draw block on screen.
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    ellipse(
        color,
        [gui_x, gui_y,BLOCK_SIZE/2.0,BLOCK_SIZE/2.0],
        con.transform,
        g,
    );
    // circle_arc(
    //     color,
    //     BLOCK_SIZE / 4 as f64,
    //     0.0,
    //      PI * 1.99999999999,
    //     [gui_x + 6.0, gui_y + 6.0, BLOCK_SIZE / 2.1 as f64, BLOCK_SIZE / 2.1 as f64],
    //     con.transform,
    //     g,
    // );
}

/// Draw rectangles -- used to make borders around play area
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [x, y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
        con.transform,
        g,
    );
}