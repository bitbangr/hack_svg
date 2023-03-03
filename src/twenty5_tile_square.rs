#![allow(unused)]

use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE,RGB_RED,RGB_GREEN,RGB_BLUE};
use crate::create_tile;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::{test_create_svg, create_svg};

// This file holds functions to create various configurations for
// a 5x5 square mosaic made of of tiles in various colour configurations.
// tile  For a twenty five tile square mosiac of one single tile the dimension are 5 row by 5 col

// White mosaic with one single black tile in the center
pub fn svg_1() {
    let op_svg_file_name = "./svg_output/fiveXfive/output_1.svg";
    let rows: usize = 5;
    let cols: usize = 5;
    let tiles_per_pane_height: usize = 5;
    let tiles_per_pane_width: usize = 5;
    let svg_width = 500;
    let svg_height = 500;

    //  5x5 array     
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_WHITE, RGB_WHITE, RGB_WHITE, RGB_WHITE],
        vec![RGB_WHITE, RGB_WHITE, RGB_WHITE, RGB_WHITE, RGB_WHITE],
        vec![RGB_WHITE, RGB_WHITE, RGB_BLACK, RGB_WHITE, RGB_WHITE],
        vec![RGB_WHITE, RGB_WHITE, RGB_WHITE, RGB_WHITE, RGB_WHITE],
        vec![RGB_WHITE, RGB_WHITE, RGB_WHITE, RGB_WHITE, RGB_WHITE],
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

