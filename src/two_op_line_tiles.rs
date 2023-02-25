use euclid::default::Box2D;
use ndarray::Array2;
use crate::constants::{RGB_BLACK,RGB_WHITE,RGB_GREEN, RGB_RED, RGB_BLUE};
use crate::create_tile;
use crate::mosaic_tile::RGB;
use crate::svg_utils::test_create_svg;
// use crate::{modtile::{RGB, self}};

// This file holds functions to create various configurations for
// a 2x2 square mosaic made of of tiles in various colour configurations.
// tile  For a four tile square mosiac of one single tile the dimension are 2 row by 2 col

pub fn svg_1(){
    let op_svg_file_name = "./svg_output/two_op_lines/output_1.svg";
    let rows: usize = 1;
    let cols: usize = 3;
    let tiles_per_pane_height: usize = 1; 
    let tiles_per_pane_width: usize = 3; 
    let svg_width = 300;
    let svg_height = 100;


    // let tile_colours = Array2::new[[RGB_WHITE, RGB_WHITE],[RGB_BLACK,RGB_BLACK]];
    let mut tile_colours_array = Array2::<RGB>::zeros((2, 2));
    tile_colours_array[[0,0]] = RGB_WHITE;
    tile_colours_array[[0,1]] = RGB_RED;
    tile_colours_array[[1,0]] = RGB_GREEN;
    tile_colours_array[[1,1]] = RGB_BLACK;

    // (RGB_WHITE , RGB_WHITE, RGB_GREEN)
    // (RGB_WHITE , RGB_BLACK, RGB_RED)
    // (RGB_WHITE , RGB_WHITE, RGB_RED)

    // // let tile_colours = Array2::new[[RGB_WHITE, RGB_WHITE],[RGB_BLACK,RGB_BLACK]];
    // let mut tile_colours_array = Array2::<(u8,u8,u8)>::zeros((2, 2));
    // tile_colours_array[[0,0]] = RGB_WHITE;
    // tile_colours_array[[0,1]] = RGB::new_with_u8(RGB_RED);
    // tile_colours_array[[1,0]] = RGB::new_with_u8(RGB_GREEN);
    // tile_colours_array[[1,1]] = RGB::new_with_u8(RGB_BLACK);



    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        vec![RGB_WHITE, RGB_GREEN, RGB_BLACK],
        vec![RGB_BLACK, RGB_GREEN, RGB_RED],
        vec![RGB_WHITE, RGB_WHITE, RGB_BLUE],
    ];

    let rgb_vec: Vec<Vec<RGB>> = vec![
        vec![RGB_WHITE, RGB_GREEN, RGB_BLACK],
        vec![RGB_BLACK, RGB_GREEN, RGB_RED],
        vec![RGB_WHITE, RGB_WHITE, RGB_BLUE],
    ];



    // let arr: Array2<(u8, u8, u8)> = Array2::from_shape_vec((3, 3).into(),rgb_vec).unwrap();
    
    // use ndarray::{Array2, s};
    

    println!("TileColours {:?}", &tile_colours_array);

    let col_width:i32 = 100; 
    let row_height:i32 = 100;
    let data_results = create_svg_data_with_input( tile_colours_array,row_height,col_width );

    println!("Data Results = {:?}", data_results);

    // let _ = test_create_svg(op_svg_file_name,
    //     svg_width,
    //     svg_height, 
    //     rows, 
    //     cols, 
    //     tiles_per_pane_height,
    //     tiles_per_pane_width,
    //     create_white_row_data);

} // svg1



    fn vec_to_array(rgb_vec: Vec<Vec<RGB>>) -> Array2<RGB> {
        let nrows = rgb_vec.len();
        let ncols = rgb_vec[0].len();
        let mut arr = Array2::zeros((nrows, ncols));
    
        for (i, row) in rgb_vec.iter().enumerate() {
            for (j, &rgb) in row.iter().enumerate() {
                arr[[i, j]] = rgb;
            }
        }
    
        arr
    }


// ********************************************************
// ********************************************************

// use ndarray::Array2;
// use euclid::TypedRect;


fn create_svg_data_with_input(
    tile_colours_array: Array2<RGB>,
    row_height: i32,
    col_width: i32,
) -> Vec<Vec<(Box2D<i32>, RGB)>> {
    // let (nrows, ncols) = tile_colours_array.shape();

    let nrows = tile_colours_array.shape()[0] as usize;
    let ncols = tile_colours_array.shape()[1] as usize;

    let mut result_window = Vec::with_capacity(nrows);

    for i in 0..nrows {
        let mut pane_grid = Vec::with_capacity(ncols);

        for j in 0..ncols {
            let top_left = (j as i32 * col_width as i32, i as i32 * row_height as i32);
            let bot_right = ((j + 1) as i32 * col_width as i32, (i + 1) as i32 * row_height as i32);
            let rgb:RGB = tile_colours_array[[i, j]];
            let (tile_box, rgb) = create_tile(top_left, bot_right, rgb);
            pane_grid.push((tile_box, rgb));
        }

        result_window.push(pane_grid);
    }

    result_window
}


// ******************************************************
// ******************************************************
// Create 3x1 White row 
fn create_white_row_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, RGB)> = Vec::new();

    // white tile
    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile(top_left, bot_right ,RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // white tile
    // (Box2D((100, 0), (200, 100)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile(top_left, bot_right ,RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));

    // white tile 
    // (Box2D((0,100), (100, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (200,0);
    let bot_right:(i32,i32) = (300,100);
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile(top_left, bot_right ,RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_white_row_data
