#![allow(unused)]

use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE,RGB_RED,RGB_GREEN,RGB_BLUE};
use crate::create_tile;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::{test_create_svg, create_svg};

// This file holds functions to create various configurations for
// mosaics consisting of 6 tiles
// 2x3 or 3x2 or 1x6 or 6x1 tiles in various colour configurations.

// single color 2 rows by 3 cols. To check single line FTTT TFTT TTFT TTTF tiles
pub fn svg_1() {
    let op_svg_file_name = "../svgoutput/twoXthree/output_1.svg";
    let rows: usize = 2;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 2;
    let tiles_per_pane_width: usize = 3;
    let svg_width = 300;
    let svg_height = 200;

    //  3x3 array    
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_WHITE, RGB_WHITE],
        vec![RGB_WHITE, RGB_WHITE, RGB_WHITE],
    ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    println!("rgb_arr {:?}", &rgb_arr);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    println!("svg1 data_results = {:?}", data_results);

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

// white row/black row color - 2 rows by 3 cols.
pub fn svg_2() {
    let op_svg_file_name = "../svgoutput/twoXthree/output_2.svg";
    let rows: usize = 2;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 2;
    let tiles_per_pane_width: usize = 3;
    let svg_width = 300;
    let svg_height = 200;

    //  3x3 array    
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_WHITE, RGB_WHITE],
        vec![RGB_BLACK, RGB_BLACK, RGB_BLACK],
    ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    println!("rgb_arr {:?}", &rgb_arr);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    println!("svg1 data_results = {:?}", data_results);

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

// red red green row row/ black red red row - 2 rows by 3 cols.
pub fn svg_3() {
    let op_svg_file_name = "../svgoutput/twoXthree/output_3.svg";
    let rows: usize = 2;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 2;
    let tiles_per_pane_width: usize = 3;
    let svg_width = 300;
    let svg_height = 200;

    //  3x3 array    
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_RED, RGB_RED, RGB_GREEN],
        vec![RGB_BLACK, RGB_RED, RGB_RED],
    ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    println!("rgb_arr {:?}", &rgb_arr);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    println!("svg1 data_results = {:?}", data_results);

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



