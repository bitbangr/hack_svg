#![allow(unused)]

use euclid::default::Box2D;
use crate::constants::RGB_GREEN;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::create_svg;
use crate::create_tile;

// For a single tile mosiac the dimension are 1 row by 1 col
const TILES_PER_PANE_WIDTH: usize = 1;
const TILES_PER_PANE_HEIGHT: usize = 1;

pub(crate) fn svg_1(){
    let op_svg_file_name = "./svg_output/one_two/output_single_tile.svg";
    let rows: usize = 1;
    let cols: usize = 1;
    let tiles_per_pane_height: usize = 1; 
    let tiles_per_pane_width: usize = 1; 
    let svg_width = 100;
    let svg_height = 100;

    //  1x1 array  create_single_green_tile_data
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_GREEN],
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
