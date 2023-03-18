#![allow(unused)]

use euclid::default::Box2D;
use crate::constants::RGB_GREEN;
use crate::mosaic_tile::RGB;
use crate::svg_utils::test_create_svg;
use crate::create_tile;

// For a single tile mosiac the dimension are 1 row by 1 col
const TILES_PER_PANE_WIDTH: usize = 1;
const TILES_PER_PANE_HEIGHT: usize = 1;

pub(crate) fn svg_1(){
    let op_svg_file_name = "../svgoutput/one_two/output_single_tile.svg";
    let rows: usize = 1;
    let cols: usize = 1;
    let tiles_per_pane_height: usize = 1; 
    let tiles_per_pane_width: usize = 1; 
    let svg_width = 100;
    let svg_height = 100;

    let _ = test_create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_single_green_tile_data);
} // svg1


// ******************************************************
// ******************************************************
/// Make a 1x1 mosaic of 1 tiles 1 green
///  Green
pub(crate) fn create_single_green_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, RGB)> = Vec::new();

    let _black = (0,0,0);
    let _white = (255,255,255);
    let green = (0,255,0);

    // green tile
    // [(Box2D((0, 0), (100, 100)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile(top_left, bot_right , RGB_GREEN);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_bot_left_grn_rest_blk_tile_data

