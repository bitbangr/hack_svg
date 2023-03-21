#![allow(unused)]
use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE};
use crate::create_tile;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::create_svg;
// use crate::{modtile::{RGB, self}};

// For a two tile mosiac the dimension are 1 row by 2 col or 2 rows by 1 col
// For each orientation there are two options - both tiles same colour, or each tile a different colour
// There is one test for each option

pub(crate) fn svg_1(){
    let op_svg_file_name = "./svg_output/one_two/horiz_1.svg";
    let rows: usize = 1;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 1; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 100;

    //  2x2 array  create_double_white_horiz_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
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
} // svg1

pub(crate) fn svg_2(){
    let op_svg_file_name = "./svg_output/one_two/horiz_2.svg";
    let rows: usize = 1;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 1; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 100;

    //  2x2 array  create_white_black_horiz_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
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
} // svg2

pub(crate) fn svg_3(){
    let op_svg_file_name = "./svg_output/one_two/vert_1.svg";
    let rows: usize = 2;
    let cols: usize = 1;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 1; 
    let svg_width = 100;
    let svg_height = 200;

    //  2x2 array  create_white_vert_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE],
        vec![RGB_WHITE],
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
    let op_svg_file_name = "./svg_output/one_two/vert_2.svg";
    let rows: usize = 2;
    let cols: usize = 1;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 1; 
    let svg_width = 100;
    let svg_height = 200;


    //  2x2 array  create_white_black_vert_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE],
        vec![RGB_BLACK],
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
