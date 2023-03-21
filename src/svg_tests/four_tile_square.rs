#![allow(unused)]

use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE,RGB_GREEN};
use crate::create_tile;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::create_svg;
// use crate::{modtile::{RGB, self}};

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

    //  2x2 array White row black row
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_WHITE],
        vec![RGB_BLACK, RGB_BLACK],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg1


pub(crate) fn svg_2(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_2.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array White row black row
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_WHITE],
        vec![RGB_WHITE, RGB_WHITE],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg_2

pub(crate) fn svg_3(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_3.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  white black checkerboard
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_BLACK],
        vec![RGB_BLACK, RGB_WHITE],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg3



pub(crate) fn svg_4(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_4.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  white col black col
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_BLACK],
        vec![RGB_WHITE, RGB_BLACK],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg4

pub(crate) fn svg_5(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_5.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  create_blk_blk_row_wht_grn_row_tile
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_BLACK, RGB_BLACK],
        vec![RGB_WHITE, RGB_GREEN],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg5

pub(crate) fn svg_6(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_6.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  create_wht_grn_row_blk_blk_row_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_GREEN],
        vec![RGB_BLACK, RGB_BLACK],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg6

pub(crate) fn svg_7(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_7.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  create_wht_grn_col_blk_blk_col_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_BLACK],
        vec![RGB_GREEN, RGB_BLACK],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg7

pub(crate) fn svg_8(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_8.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  create_blk_blk_col_grn_wht_col_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_BLACK, RGB_GREEN],
        vec![RGB_BLACK, RGB_WHITE],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg8

pub(crate) fn svg_9(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_9.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  create_top_left_grn_rest_blk_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_GREEN, RGB_BLACK],
        vec![RGB_BLACK, RGB_BLACK],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg9

pub(crate) fn svg_10(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_10.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  create_top_right_grn_rest_blk_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_BLACK, RGB_GREEN],
        vec![RGB_BLACK, RGB_BLACK],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg10

pub(crate) fn svg_11(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_11.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  create_bot_left_grn_rest_blk_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_BLACK, RGB_BLACK],
        vec![RGB_GREEN, RGB_BLACK],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg11

pub(crate) fn svg_12(){
    let op_svg_file_name = "./svg_output/twoXtwo/output_12.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  create_bot_right_grn_rest_blk_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_BLACK, RGB_BLACK],
        vec![RGB_BLACK, RGB_GREEN],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg12

// used to call random stuff for testing
pub(crate) fn svg_98(){
    let op_svg_file_name = "./svg_output/twoXtwo/output98.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  create_wht_grn_col_blk_blk_col_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_BLACK],
        vec![RGB_GREEN, RGB_BLACK],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg98



// used to call random stuff for testing
pub(crate) fn svg_99(){
    let op_svg_file_name = "./svg_output/twoXtwo/output99.svg";
    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 200;

    //  2x2 array  create_top_right_grn_rest_blk_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_BLACK, RGB_GREEN],
        vec![RGB_BLACK, RGB_BLACK],
    ];
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    let _ = create_svg(op_svg_file_name,
                        svg_width,
                        svg_height, 
                        rows, 
                        cols, 
                        tiles_per_pane_height,
                        tiles_per_pane_width,
                        data_results);
} // svg99
