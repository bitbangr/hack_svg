#![allow(unused)]
use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE};
use crate::create_tile;
use crate::mosaic_tile::RGB;
use crate::svg_utils::test_create_svg;
// use crate::{modtile::{RGB, self}};

// For a two tile mosiac the dimension are 1 row by 2 col or 2 rows by 1 col
// For each orientation there are two options - both tiles same colour, or each tile a different colour
// There is one test for each option

pub(crate) fn svg_1(){
    let op_svg_file_name = "../svgoutput/one_two/horiz_1.svg";
    let rows: usize = 1;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 1; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 100;

    let _ = test_create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_double_white_horiz_tile_data);
} // svg1

pub(crate) fn svg_2(){
    let op_svg_file_name = "../svgoutput/one_two/horiz_2.svg";
    let rows: usize = 1;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 1; 
    let tiles_per_pane_width: usize = 2; 
    let svg_width = 200;
    let svg_height = 100;

    let _ = test_create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_white_black_horiz_tile_data);
} // svg2

pub(crate) fn svg_3(){
    let op_svg_file_name = "../svgoutput/one_two/vert_1.svg";
    let rows: usize = 2;
    let cols: usize = 1;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 1; 
    let svg_width = 100;
    let svg_height = 200;

    let _ = test_create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_white_vert_tile_data);
} // svg3

pub(crate) fn svg_4(){
    let op_svg_file_name = "../svgoutput/one_two/vert_2.svg";
    let rows: usize = 2;
    let cols: usize = 1;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 1; 
    let svg_width = 100;
    let svg_height = 200;

    let _ = test_create_svg(op_svg_file_name,
        svg_width,
        svg_height, 
        rows, 
        cols, 
        tiles_per_pane_height,
        tiles_per_pane_width,
        create_white_black_vert_tile_data);
} // svg4

// ******************************************************************* 
// ******************************************************************* 
/// Creates data for a single row of two white tiles
/// of 100 by 200 UnknownUnits size
pub fn create_double_white_horiz_tile_data() -> Vec<Vec<(Box2D<i32>, RGB)>> {

    let mut result_window: Vec<Vec<(Box2D<i32>, RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, RGB)> = Vec::new();

    // [(Box2D((0, 0), (100, 100)), rgb_val),
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile((0, 0), (100, 100), RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // (Box2D((100, 0), (200, 100)), rgb_val),
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile((100, 0), (200, 100), RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window
}

// ******************************************************************* 
// ******************************************************************* 
/// Creates data for a single row of two tiles
/// left tile is white, right tile is black
/// 100 by 200 UnknownUnits size
pub fn create_white_black_horiz_tile_data() -> Vec<Vec<(Box2D<i32>, RGB)>> {

    let mut result_window: Vec<Vec<(Box2D<i32>, RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, RGB)> = Vec::new();

    // [(Box2D((0, 0), (100, 100)), rgb_val),
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile((0, 0), (100, 100), RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // (Box2D((100, 0), (200, 100)), rgb_val),
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile((100, 0), (200, 100), RGB_BLACK);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window
}

// ******************************************************************* 
// ******************************************************************* 
/// Creates data for a single column of two tiles
/// both tiles are white
/// 200 by 100 UnknownUnits size
pub fn create_white_vert_tile_data() -> Vec<Vec<(Box2D<i32>, RGB)>> {

    let mut result_window: Vec<Vec<(Box2D<i32>, RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, RGB)> = Vec::new();

    // top tile white
    // [(Box2D((0, 0), (100, 100)), rgb_val),
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile((0, 0), (100, 100), RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // bottom tile white
    // (Box2D((0, 100), (100, 200)), rgb_val),
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile((0, 100), (100, 200), RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window
}
// ******************************************************************* 
// ******************************************************************* 
/// Creates data for a single column of two tiles
/// top tile is white, bottom tile is black
/// 200 by 100 UnknownUnits size
pub fn create_white_black_vert_tile_data() -> Vec<Vec<(Box2D<i32>, RGB)>> {

    let mut result_window: Vec<Vec<(Box2D<i32>, RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, RGB)> = Vec::new();

    // top tile white
    // [(Box2D((0, 0), (100, 100)), rgb_val),
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile((0, 0), (100, 100), RGB_WHITE);
    let _ = &pane_grid.push((tile_box, rgb));
    
    // bottom tile black
    // (Box2D((0, 100), (100, 200)), RGBRGB_BLACK),
    let (tile_box, rgb): (Box2D<i32>, RGB) = create_tile((0, 100), (100, 200), RGB_BLACK);
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window
}

