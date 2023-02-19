use euclid::default::Box2D;
use crate::create_tile;
use crate::svg_utils::{create_svg, test_create_svg};
use crate::{modtile::{RGB, self}};

// This file holds functions to create various configurations for
// a 2x2 square mosaic made of of tiles in various colour configurations.
// tile  For a four tile square mosiac of one single tile the dimension are 2 row by 2 col

pub fn svg_1(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_1.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_white_row_black_row_tile_data);
} // svg1

pub(crate) fn svg_2(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_2.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_2x2_white_tile_data);
} // svg_2

pub(crate) fn svg_3(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_3.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_white_black_checkerboard_data);
} // svg3



pub(crate) fn svg_4(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_4.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_white_col_black_col_tile_data);
} // svg4

pub(crate) fn svg_5(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_5.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_blk_blk_row_wht_grn_row_tile_data);
} // svg5

pub(crate) fn svg_6(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_6.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_wht_grn_row_blk_blk_row_tile_data);
} // svg6

pub(crate) fn svg_7(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_7.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_wht_grn_col_blk_blk_col_tile_data);
} // svg7

pub(crate) fn svg_8(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_8.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_blk_blk_col_grn_wht_col_tile_data);
} // svg8

pub(crate) fn svg_9(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_9.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_top_left_grn_rest_blk_tile_data);
} // svg9

pub(crate) fn svg_10(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_10.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_top_right_grn_rest_blk_tile_data);
} // svg10

pub(crate) fn svg_11(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_11.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_bot_left_grn_rest_blk_tile_data);
} // svg11

pub(crate) fn svg_12(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_12.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    let _ = create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_bot_right_grn_rest_blk_tile_data);
} // svg12

// used to call random stuff for testing
pub(crate) fn svg_99(){
    let op_svg_file_name = "./svg_output/twoXtwo/output99.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    // calling test_create_svg to instead of create_svg() 
    let _ = test_create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_top_right_grn_rest_blk_tile_data);
} // svg99



// ******************************************************
// ******************************************************
// Create 2x2 White Black checkerboard 
fn create_white_black_checkerboard_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // white tile
    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // black tile
    // (Box2D((100, 0), (200, 100)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // black tile 
    // (Box2D((0,100), (100, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // white tile
    // (Box2D((100,100), (200, 200)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_white_black_checkerboard_data

// ******************************************************
// ******************************************************
fn create_white_col_black_col_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // white col first row
    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // black col first row
    // (Box2D((100, 0), (200, 100)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // white col 2nd row
    // (Box2D((0,100), (100, 200)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // black col second row
    // (Box2D((100,100), (200, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_white_col_black_col_tile_data


// ******************************************************
// ******************************************************
pub(crate) fn create_white_row_black_row_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // white row
    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // (Box2D((100, 0), (200, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // black row
    // (Box2D((0,100), (100, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // (Box2D((100,100), (200, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_white_row_black_row_tile_data

// ******************************************************
// ******************************************************
/// Make a 2x2 mosaic of a single pane of all white tiles
/// 
fn create_2x2_white_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // (Box2D((100, 0), (200, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // (Box2D((0,100), (100, 200)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // (Box2D((100,100), (200, 200)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_2x2_white_tile_data


// ******************************************************
// ******************************************************
/// Make a 2x2 mosaic of 4 tiles in two rows 
///  Black Black
///  White Green
pub(crate) fn create_blk_blk_row_wht_grn_row_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // black row
    // [(Box2D((0, 0), (100, 100)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // (Box2D((100, 0), (200, 100)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // white tile second row first col
    // (Box2D((0,100), (100, 200)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // green tile second row second col
    // (Box2D((100,100), (200, 200)), RGB(0, 255, 0)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 255, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_blk_blk_row_wht_grn_row_tile_data

// ******************************************************
// ******************************************************
/// Make a 2x2 mosaic of 4 tiles in two rows 
///  Black Black
///  White Green
pub(crate) fn create_wht_grn_row_blk_blk_row_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // white tile
    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // green tile
    // (Box2D((100, 0), (200, 100)), RGB(0, 255, 0)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 255, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // black row
    // (Box2D((0,100), (100, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // black row
    // (Box2D((100,100), (200, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_blk_blk_row_wht_grn_row_tile_data



// ******************************************************
// ******************************************************
/// Make a 2x2 mosaic of 4 tiles in two cols 
///  White Black
///  Greet Black
pub(crate) fn create_wht_grn_col_blk_blk_col_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // white tile
    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // black tile
    // (Box2D((100, 0), (200, 100)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // green tile
    // (Box2D((0,100), (100, 200)), RGB(0, 255, 0)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 255, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // black row
    // (Box2D((100,100), (200, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_wht_grn_col_blk_blk_col_tile_data

// ******************************************************
// ******************************************************
/// Make a 2x2 mosaic of 4 tiles in two cols 
///  White Black
///  Greet Black
pub(crate) fn create_blk_blk_col_grn_wht_col_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    let black = (0,0,0);
    let white = (255,255,255);
    let green = (0,255,0);

    // black tile
    // [(Box2D((0, 0), (100, 100)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // green tile
    // (Box2D((100, 0), (200, 100)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , green);
    let _ = &pane_grid.push((tile_box, rgb));

    // black tile
    // (Box2D((0,100), (100, 200)), RGB(0, 255, 0)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));

    // white tile
    // (Box2D((100,100), (200, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , white);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_blk_blk_col_grn_wht_col_tile_data

// ******************************************************
// ******************************************************
/// Make a 2x2 mosaic of 4 tiles 1 green, rest black (L shape)
///  Green Black
///  Black Black
pub(crate) fn create_top_left_grn_rest_blk_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    let black = (0,0,0);
    let _white = (255,255,255);
    let green = (0,255,0);

    // green tile
    // [(Box2D((0, 0), (100, 100)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , green);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // black tile
    // (Box2D((100, 0), (200, 100)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));

    // black tile
    // (Box2D((0,100), (100, 200)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));

    // black tile
    // (Box2D((100,100), (200, 200)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_top_left_grn_rest_blk_tile_data



// ******************************************************
// ******************************************************
/// Make a 2x2 mosaic of 4 tiles 1 green, rest black (L shape)
///  Black Green
///  Black Black
pub(crate) fn create_top_right_grn_rest_blk_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    let black = (0,0,0);
    let _white = (255,255,255);
    let green = (0,255,0);

    // black tile
    // [(Box2D((0, 0), (100, 100)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // green tile
    // (Box2D((100, 0), (200, 100)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , green);
    let _ = &pane_grid.push((tile_box, rgb));

    // black tile
    // (Box2D((0,100), (100, 200)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));

    // black tile
    // (Box2D((100,100), (200, 200)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_top_right_grn_rest_blk_tile_data

// ******************************************************
// ******************************************************
/// Make a 2x2 mosaic of 4 tiles 1 green, rest black (L shape)
///  Black Black
///  Green Black
pub(crate) fn create_bot_left_grn_rest_blk_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    let black = (0,0,0);
    let _white = (255,255,255);
    let green = (0,255,0);

    // black tile
    // [(Box2D((0, 0), (100, 100)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // black tile
    // (Box2D((100, 0), (200, 100)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));

    // green tile
    // (Box2D((0,100), (100, 200)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , green);
    let _ = &pane_grid.push((tile_box, rgb));

    // black tile
    // (Box2D((100,100), (200, 200)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_bot_left_grn_rest_blk_tile_data



// ******************************************************
// ******************************************************
/// Make a 2x2 mosaic of 4 tiles 1 green, rest black (L shape)
///  Black Black
///  Black Green
pub(crate) fn create_bot_right_grn_rest_blk_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    let black = (0,0,0);
    let _white = (255,255,255);
    let green = (0,255,0);

    // black tile
    // [(Box2D((0, 0), (100, 100)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // black tile
    // (Box2D((100, 0), (200, 100)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));

    // black tile
    // (Box2D((0,100), (100, 200)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , black);
    let _ = &pane_grid.push((tile_box, rgb));

    // green tile
    // (Box2D((100,100), (200, 200)), RGB_VAL (#,#,#)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , green);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} // create_bot_right_grn_rest_blk_tile_data
