#![allow(unused)]

use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE,RGB_RED,RGB_GREEN,RGB_BLUE, RGB_YELLOW};
use crate::create_tile;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::{test_create_svg, create_svg};

// This file holds functions to create various configurations for
// a 5x5 square mosaic made of of tiles in various colour configurations.
// tile  For a twenty five tile square mosiac of one single tile the dimension are 5 row by 5 col

// Blue mosaic with one single black tile in the center
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
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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

// 6x6 Blue mosaic with a 4 black tiles in the center
pub fn svg_2() {
    let op_svg_file_name = "./svg_output/fiveXfive/output_2_6x6.svg";
    let rows: usize = 6;
    let cols: usize = 6;
    let tiles_per_pane_height: usize = 6;
    let tiles_per_pane_width: usize = 6;
    let svg_width = 600;
    let svg_height = 600;

    //  6x6 array     
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE,  RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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

// 8x8 Blue mosaic with a 4x4 black tiles in the center
pub fn svg_3() {
    let op_svg_file_name = "./svg_output/fiveXfive/output_3_8x8.svg";
    let rows: usize = 8;
    let cols: usize = 8;
    let tiles_per_pane_height: usize = 8;
    let tiles_per_pane_width: usize = 8;
    let svg_width = 800;
    let svg_height = 800;

    //  8x8 array     
    // let rgb_vec: Vec<Vec<(RGB)>> = vec![
    //     vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
    //     vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
    //     vec![RGB_BLUE, RGB_BLUE, RGB_RED, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
    //     vec![RGB_BLUE, RGB_BLUE, RGB_RED, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
    //     vec![RGB_BLUE, RGB_BLUE, RGB_RED, RGB_BLACK, RGB_BLACK, RGB_WHITE, RGB_BLUE, RGB_BLUE],
    //     vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
    //     vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
    //     vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
    // ];

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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

}

// 8x8 Blue mosaic with a 4x4 black tiles in the center
pub fn svg_4() {
    let op_svg_file_name = "./svg_output/fiveXfive/output_4_8x10.svg";
    let rows: usize = 8;
    let cols: usize = 10;
    let tiles_per_pane_height: usize = 8;
    let tiles_per_pane_width: usize = 10;
    let svg_width = 1000;
    let svg_height = 800;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],     [0,4],     [0,5],     [0,6],     [0,7],     [0,8],     [0,9],
        vec![RGB_RED, RGB_BLUE, RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_GREEN, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        //   [1,0],     [1,1],     [1,2],     [1,3],     [1,4],     [1,5],     [1,6],     [1,7],     [1,8],     [1,9],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        //   [2,0],     [2,1],     [2,2],     [2,3],     [2,4],     [2,5],     [2,6],     [2,7],     [2,8],     [2,9],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE,  RGB_BLUE,  RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_GREEN],
        //   [3,0],     [3,1],     [3,2],     [3,3],     [3,4],     [3,5],     [3,6],     [3,7],     [3,8],     [3,9],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE,  RGB_BLUE,  RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        //   [4,0],     [4,1],     [4,2],     [4,3],     [4,4],     [4,5],     [4,6],     [4,7],     [4,8],     [4,9],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE,  RGB_BLUE,  RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        //   [5,0],     [5,1],     [5,2],     [5,3],     [5,4],     [5,5],     [5,6],     [5,7],     [5,8],     [5,9],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE,  RGB_BLUE,  RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        //   [6,0],     [6,1],     [6,2],     [6,3],     [6,4],     [6,5],     [6,6],     [6,7],     [6,8],     [6,9],
        vec![RGB_YELLOW, RGB_YELLOW, RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        //   [7,0],     [7,1],     [7,2],     [7,3],     [7,4],     [7,5],     [7,6],     [7,7],     [7,8],     [7,9],
        vec![RGB_YELLOW, RGB_YELLOW, RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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

// 8x10 Blue mosaic with a 4x4 black tiles in the center
pub fn svg_5() {
    let op_svg_file_name = "./svg_output/fiveXfive/output_5_8x10.svg";
    let rows: usize = 8;
    let cols: usize = 10;
    let tiles_per_pane_height: usize = 8;
    let tiles_per_pane_width: usize = 10;
    let svg_width = 1000;
    let svg_height = 800;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],     [0,4],     [0,5],     [0,6],     [0,7],     [0,8],     [0,9],
        vec![RGB_RED, RGB_BLUE, RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_GREEN, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        //   [1,0],     [1,1],     [1,2],     [1,3],     [1,4],     [1,5],     [1,6],     [1,7],     [1,8],     [1,9],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        //   [2,0],     [2,1],     [2,2],     [2,3],     [2,4],     [2,5],     [2,6],     [2,7],     [2,8],     [2,9],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE,  RGB_BLUE,  RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_GREEN],
        //   [3,0],     [3,1],     [3,2],     [3,3],     [3,4],     [3,5],     [3,6],     [3,7],     [3,8],     [3,9],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE,  RGB_BLUE,  RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        //   [4,0],     [4,1],     [4,2],     [4,3],     [4,4],     [4,5],     [4,6],     [4,7],     [4,8],     [4,9],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE,  RGB_BLUE,  RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        //   [5,0],     [5,1],     [5,2],     [5,3],     [5,4],     [5,5],     [5,6],     [5,7],     [5,8],     [5,9],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE,  RGB_BLUE,  RGB_BLACK, RGB_BLACK, RGB_BLUE, RGB_BLUE],
        //   [6,0],     [6,1],     [6,2],     [6,3],     [6,4],     [6,5],     [6,6],     [6,7],     [6,8],     [6,9],
        vec![RGB_YELLOW, RGB_YELLOW, RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        //   [7,0],     [7,1],     [7,2],     [7,3],     [7,4],     [7,5],     [7,6],     [7,7],     [7,8],     [7,9],
        vec![RGB_YELLOW, RGB_YELLOW, RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE,  RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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
} // svg5
