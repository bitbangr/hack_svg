use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE,RGB_GREEN};
use crate::create_tile;
use crate::mosaic_tile::RGB;
use crate::svg_utils::test_create_svg;
// use crate::{modtile::{RGB, self}};

// This file holds functions to create various configurations for
// a 2x2 square mosaic made of of tiles in various colour configurations.
// tile  For a four tile square mosiac of one single tile the dimension are 2 row by 2 col

pub fn svg_1(){
    let op_svg_file_name = "./svg_output/two_op_lines/output_1.svg";
    let rows: usize = 1;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 1; 
    let tiles_per_pane_width: usize = 3; 
    let svg_width = 300;
    let svg_height = 100;

    let _ = test_create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_white_row_data);
} // svg1


// ******************************************************
// ******************************************************
// Create 3x1 White row 
fn create_white_row_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, RGB)> = Vec::new();

    // white tile
    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile(top_left, bot_right ,RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // white tile
    // (Box2D((100, 0), (200, 100)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile(top_left, bot_right ,RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));

    // white tile 
    // (Box2D((0,100), (100, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (200,0);
    let bot_right:(i32,i32) = (300,100);
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile(top_left, bot_right ,RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_white_row_data
