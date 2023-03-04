#![allow(unused)]

use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE,RGB_RED,RGB_GREEN,RGB_BLUE};
use crate::create_tile;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::{test_create_svg, create_svg};

// This file holds functions to create various configurations for
// a 3x3 square mosaic made of of tiles in various colour configurations.
// tile  For a nine tile square mosiac of one single tile the dimension are 3 row by 3 col

// rgb tiles shifted left once per row
pub fn svg_1() {
    let op_svg_file_name = "./svg_output/threeXthree/output_1.svg";
    let rows: usize = 3;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 3;
    let tiles_per_pane_width: usize = 3;
    let svg_width = 300;
    let svg_height = 300;

    //  3x3 array    
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_RED, RGB_GREEN, RGB_BLUE],
        vec![RGB_GREEN, RGB_BLUE, RGB_RED],
        vec![RGB_BLUE, RGB_RED, RGB_GREEN],
    ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    // println!("rgb_arr {:?}", &rgb_arr);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    // println!("svg1 data_results = {:?}", data_results);

    let _ = create_svg(
        op_svg_file_name,
        svg_width,
        svg_height,
        rows,
        cols,
        tiles_per_pane_height,
        tiles_per_pane_width,
        data_results,
    );
} // svg1

// a black cross or plus sign on white backgroud
pub fn svg_2() {
    let op_svg_file_name = "./svg_output/threeXthree/output_2.svg";
    let rows: usize = 3;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 3;
    let tiles_per_pane_width: usize = 3;
    let svg_width = 300;
    let svg_height = 300;

    //  3x3 array    
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_BLACK, RGB_WHITE],
        vec![RGB_BLACK, RGB_BLACK, RGB_BLACK],
        vec![RGB_WHITE, RGB_BLACK, RGB_WHITE],
    ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    // println!("rgb_arr {:?}", &rgb_arr);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    // println!("svg1 data_results = {:?}", data_results);

    let _ = create_svg(
        op_svg_file_name,
        svg_width,
        svg_height,
        rows,
        cols,
        tiles_per_pane_height,
        tiles_per_pane_width,
        data_results,
    );
} // svg2

pub fn svg_3() {
    let op_svg_file_name = "./svg_output/threeXthree/output_3.svg";
    let rows: usize = 3;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 3;
    let tiles_per_pane_width: usize = 3;
    let svg_width = 300;
    let svg_height = 300;

    //  3x3 array    
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_RED, RGB_RED, RGB_BLACK],
        vec![RGB_BLUE, RGB_RED, RGB_RED],
        vec![RGB_BLUE, RGB_BLUE, RGB_RED],
    ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    // println!("rgb_arr {:?}", &rgb_arr);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    // println!("svg1 data_results = {:?}", data_results);

    let _ = create_svg(
        op_svg_file_name,
        svg_width,
        svg_height,
        rows,
        cols,
        tiles_per_pane_height,
        tiles_per_pane_width,
        data_results,
    );
} // svg3

// Center Black Tile surounded by white tiles
// This is going to be a long case so leave for now
pub fn svg_4() {
    let op_svg_file_name = "./svg_output/threeXthree/output_4.svg";
    let rows: usize = 3;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 3;
    let tiles_per_pane_width: usize = 3;
    let svg_width = 300;
    let svg_height = 300;

    //  3x3 array    
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_WHITE, RGB_WHITE],
        vec![RGB_WHITE, RGB_BLACK, RGB_WHITE],
        vec![RGB_WHITE, RGB_WHITE, RGB_WHITE],
    ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    // println!("rgb_arr {:?}", &rgb_arr);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    // println!("svg1 data_results = {:?}", data_results);

    let _ = create_svg(
        op_svg_file_name,
        svg_width,
        svg_height,
        rows,
        cols,
        tiles_per_pane_height,
        tiles_per_pane_width,
        data_results,
    );
} // svg4

