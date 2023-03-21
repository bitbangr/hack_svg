#![allow(unused)]

use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE,RGB_RED,RGB_GREEN,RGB_BLUE, RGB_YELLOW};
use crate::create_tile;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::create_svg;

// This file holds functions to create various configurations for
// a 5x5 square mosaic made of of tiles in various colour configurations.
// tile  For a twenty five tile square mosiac of one single tile the dimension are 5 row by 5 col

// 4x4 Blue mosaic
pub fn svg_1() {
    let op_svg_file_name = "./svg_output/fourXfour/output_1.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    //  4x4 array     
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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

// 4x4 Blue mosaic with a 4 black tiles in the center
pub fn svg_2() {
    let op_svg_file_name = "./svg_output/fourXfour/output_2.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    //  4x4 array     
    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],   
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE,  RGB_BLUE  ],
        //   [1,0],     [1,1],     [1,2],     [1,3],  
        vec![RGB_BLUE, RGB_BLACK, RGB_BLACK,  RGB_BLUE ],
        //   [2,0],     [2,1],     [2,2],     [2,3],  
        vec![RGB_BLUE, RGB_BLACK, RGB_BLACK, RGB_BLUE],
        //   [3,0],     [3,1],     [3,2],     [3,3],  
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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


// 4x4 Blue mosaic with a 2 opposing red tiles in the center
pub fn svg_3() {
    let op_svg_file_name = "./svg_output/fourXfour/output_3.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],   
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE,  RGB_BLUE  ],
        //   [1,0],     [1,1],     [1,2],     [1,3],  
        vec![RGB_BLUE, RGB_BLUE, RGB_RED,  RGB_BLUE ],
        //   [2,0],     [2,1],     [2,2],     [2,3],  
        vec![RGB_BLUE, RGB_RED, RGB_BLUE, RGB_BLUE],
        //   [3,0],     [3,1],     [3,2],     [3,3],  
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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


// 4x4 Blue mosaic with a 2 opposing red tiles in the center opposite of svg3
pub fn svg_4() {
    let op_svg_file_name = "./svg_output/fourXfour/output_4.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],   
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE,  RGB_BLUE  ],
        //   [1,0],     [1,1],     [1,2],     [1,3],  
        vec![RGB_BLUE, RGB_RED, RGB_BLUE,  RGB_BLUE ],
        //   [2,0],     [2,1],     [2,2],     [2,3],  
        vec![RGB_BLUE, RGB_BLUE, RGB_RED, RGB_BLUE],
        //   [3,0],     [3,1],     [3,2],     [3,3],  
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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

// 4x4 Blue mosaic with a red 'L' in center
pub fn svg_5() {
    let op_svg_file_name = "./svg_output/fourXfour/output_5.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],   
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE,  RGB_BLUE  ],
        //   [1,0],     [1,1],     [1,2],     [1,3],  
        vec![RGB_BLUE, RGB_RED, RGB_BLUE,  RGB_BLUE ],
        //   [2,0],     [2,1],     [2,2],     [2,3],  
        vec![RGB_BLUE, RGB_RED, RGB_RED, RGB_BLUE],
        //   [3,0],     [3,1],     [3,2],     [3,3],  
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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

// 4x4 Blue mosaic with a red 'L' in center
pub fn svg_6() {
    let op_svg_file_name = "./svg_output/fourXfour/output_6.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],   
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE,  RGB_BLUE  ],
        //   [1,0],     [1,1],     [1,2],     [1,3],  
        vec![RGB_BLUE, RGB_RED,  RGB_RED,  RGB_BLUE ],
        //   [2,0],     [2,1],     [2,2],     [2,3],  
        vec![RGB_BLUE, RGB_RED,  RGB_BLUE, RGB_BLUE],
        //   [3,0],     [3,1],     [3,2],     [3,3],  
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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
} // svg6

// 4x4 Blue mosaic with a red 'L' in center
pub fn svg_7() {
    let op_svg_file_name = "./svg_output/fourXfour/output_7.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],   
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE,  RGB_BLUE  ],
        //   [1,0],     [1,1],     [1,2],     [1,3],  
        vec![RGB_BLUE, RGB_RED,  RGB_RED,  RGB_BLUE ],
        //   [2,0],     [2,1],     [2,2],     [2,3],  
        vec![RGB_BLUE, RGB_BLUE,  RGB_RED, RGB_BLUE],
        //   [3,0],     [3,1],     [3,2],     [3,3],  
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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
} // svg7

// 4x4 Blue mosaic with a red 'L' in center
pub fn svg_8() {
    let op_svg_file_name = "./svg_output/fourXfour/output_8.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],   
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE,  RGB_BLUE  ],
        //   [1,0],     [1,1],     [1,2],     [1,3],  
        vec![RGB_BLUE, RGB_BLUE,  RGB_RED,  RGB_BLUE ],
        //   [2,0],     [2,1],     [2,2],     [2,3],  
        vec![RGB_BLUE, RGB_RED,  RGB_RED, RGB_BLUE],
        //   [3,0],     [3,1],     [3,2],     [3,3],  
        vec![RGB_BLUE, RGB_BLUE, RGB_BLUE, RGB_BLUE],
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
} // svg8


// 4x4 4 colour Diagonal opposite white black  
pub fn svg_9() {
    let op_svg_file_name = "./svg_output/fourXfour/output_9.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],   
        vec![RGB_RED, RGB_WHITE, RGB_WHITE,  RGB_WHITE  ],
        //   [1,0],     [1,1],     [1,2],     [1,3],  
        vec![RGB_BLACK, RGB_GREEN,  RGB_WHITE,  RGB_WHITE ],
        //   [2,0],     [2,1],     [2,2],     [2,3],  
        vec![RGB_BLACK, RGB_BLACK,  RGB_YELLOW, RGB_WHITE],
        //   [3,0],     [3,1],     [3,2],     [3,3],  
        vec![RGB_BLACK, RGB_BLACK, RGB_BLACK, RGB_BLUE],
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
} // svg9

// 4x4 4 colour FTFT TFTF test
pub fn svg_10() {
    let op_svg_file_name = "./svg_output/fourXfour/output_10.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],     [0,3],   
        vec![RGB_BLACK, RGB_BLACK, RGB_BLACK,  RGB_BLACK  ],
        //   [1,0],     [1,1],     [1,2],     [1,3],  
        vec![RGB_BLACK, RGB_GREEN,  RGB_BLUE,  RGB_BLUE ],
        //   [2,0],     [2,1],     [2,2],     [2,3],  
        vec![RGB_BLACK, RGB_GREEN,  RGB_YELLOW, RGB_YELLOW],
        //   [3,0],     [3,1],     [3,2],     [3,3],  
        vec![RGB_RED, RGB_RED, RGB_RED, RGB_RED],
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
} // svg10

// 4x4 4 colour FTFT TFTF test
pub fn svg_11() {
    let op_svg_file_name = "./svg_output/fourXfour/output_11.svg";
    let rows: usize = 4;
    let cols: usize = 4;
    let tiles_per_pane_height: usize = 4;
    let tiles_per_pane_width: usize = 4;
    let svg_width = 400;
    let svg_height = 400;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],     [0,1],     [0,2],      [0,3],   
        vec![RGB_GREEN, RGB_WHITE,  RGB_BLUE, RGB_BLUE  ],
        //   [1,0],     [1,1],     [1,2],      [1,3],  
        vec![RGB_BLUE, RGB_BLUE,  RGB_BLUE,  RGB_BLUE ],
        //   [2,0],     [2,1],     [2,2],      [2,3],  
        vec![RGB_BLACK, RGB_BLUE,  RGB_GREEN, RGB_BLACK],
        //   [3,0],     [3,1],     [3,2],      [3,3],  
        vec![RGB_WHITE,   RGB_BLUE,   RGB_GREEN, RGB_BLACK],
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
} // svg11
