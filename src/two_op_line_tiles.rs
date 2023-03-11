// #![allow(unused)]

use crate::constants::{RGB_BLACK, RGB_BLUE, RGB_GREEN, RGB_RED, RGB_WHITE};
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::create_svg;

// This file holds functions to create various configurations for
// a opposing line tiles mosaic made  of tiles in various colour configurations.
// tile. So tiles with FTFT and TFTF configs

pub fn svg_1() {
    let op_svg_file_name = "./svg_output/two_op_lines/output_1.svg";
    let rows: usize = 1;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 1;
    let tiles_per_pane_width: usize = 3;
    let svg_width = 300;
    let svg_height = 100;

    let rgb_vec: Vec<Vec<RGB>> = vec![
        vec![RGB_WHITE, RGB_WHITE, RGB_WHITE],
    ];

    // sample 2x3 array    
    // let rgb_vec: Vec<Vec<(RGB)>> = vec![
    //     vec![RGB_WHITE, RGB_GREEN, RGB_BLACK],
    //     vec![RGB_WHITE, RGB_WHITE, RGB_BLACK],
    // ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    println!("svg1 rgb_arr {:?}", &rgb_arr);

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

pub fn svg_2() {
    let op_svg_file_name = "./svg_output/two_op_lines/output_2.svg";
    let rows: usize = 3;
    let cols: usize = 1;
    let tiles_per_pane_height: usize = 3;
    let tiles_per_pane_width: usize = 1;
    let svg_width = 100;
    let svg_height = 300;

    let rgb_vec: Vec<Vec<RGB>> = vec![
                                    vec![RGB_WHITE],
                                    vec![RGB_WHITE],
                                    vec![RGB_WHITE],
                                    ];

    // sample 2x3 array    
    // let rgb_vec: Vec<Vec<(RGB)>> = vec![
    //     vec![RGB_WHITE, RGB_GREEN, RGB_BLACK],
    //     vec![RGB_WHITE, RGB_WHITE, RGB_BLACK],
    // ];

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


// test one vertical TFTF tile plus one horiz FTFT tile in same file
pub fn svg_3() {
    let op_svg_file_name = "./svg_output/two_op_lines/output_3.svg";
    let rows: usize = 3;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 3;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 300;

    //  3x4 array    
    let rgb_vec: Vec<Vec<RGB>> = vec![
        vec![RGB_RED, RGB_GREEN, RGB_GREEN, RGB_GREEN],
        vec![RGB_RED, RGB_BLACK, RGB_BLACK, RGB_BLACK],
        vec![RGB_RED, RGB_BLACK, RGB_BLACK, RGB_BLACK],

    ];

    // sample 3 rows by 1 col
    // let rgb_vec: Vec<Vec<RGB>> = vec![
    //                                 vec![RGB_WHITE],
    //                                 vec![RGB_WHITE],
    //                                 vec![RGB_WHITE],
    //                                 ];

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



pub fn svg_98() {
    let op_svg_file_name = "./svg_output/two_op_lines/output_98.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    let rgb_vec: Vec<Vec<RGB>> = vec![
        vec![RGB_RED, RGB_RED, RGB_BLACK, RGB_BLACK],
        vec![RGB_RED, RGB_RED, RGB_GREEN, RGB_GREEN],
        vec![RGB_WHITE, RGB_BLUE, RGB_GREEN,RGB_GREEN],
        vec![RGB_WHITE, RGB_BLUE, RGB_GREEN,RGB_GREEN],
    ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    println!("rgb_arr {:?}", &rgb_arr);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    println!("Data Results = {:?}", data_results);

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
} // svg98



pub fn svg_99() {
    let op_svg_file_name = "./svg_output/two_op_lines/output_99.svg";
    let rows: usize = 3;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 3;
    let tiles_per_pane_width: usize = 3;
    let svg_width = 300;
    let svg_height = 300;

    let rgb_vec: Vec<Vec<RGB>> = vec![
        vec![RGB_RED, RGB_BLACK, RGB_BLUE],
        vec![RGB_BLACK, RGB_BLACK, RGB_GREEN],
        vec![RGB_WHITE, RGB_BLACK, RGB_RED],
    ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    println!("rgb_arr {:?}", &rgb_arr);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    println!("Data Results = {:?}", data_results);

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
} // svg99


