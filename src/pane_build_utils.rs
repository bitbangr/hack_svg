use crate::constants::{RGB_BLACK, RGB_BLUE, RGB_GREEN, RGB_RED, RGB_WHITE};
use crate::create_tile;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::svg_utils::create_svg;
use euclid::default::Box2D;
use ndarray::Array2;


/// This returns panes and tiles as there is only ever one pane
/// for testing the result window will have 1 main vec
/// each pane has a single vector of size row x col tiles ordered from
/// top left to bottom right and 
///
///
pub fn create_pane_test_data(tile_colours_array: Array2<RGB>,
                         row_height: i32,
                         col_width: i32,) -> Vec<Vec<(Box2D<i32>, RGB)>> {

    let nrows = tile_colours_array.shape()[0] as usize;
    let ncols = tile_colours_array.shape()[1] as usize;

    // result to be returned only ever have one pane at this point for testing
    let mut result_window: Vec<Vec<(Box2D<i32>, RGB)>> = Vec::with_capacity(1);

    // ****************************
    // Start the first pane to be popluted with
    let mut pane_grid: Vec<(Box2D<i32>, RGB)> = Vec::with_capacity(nrows * ncols);

    for i in 0..nrows {
        for j in 0..ncols {
            let top_left = (j as i32 * col_width as i32, i as i32 * row_height as i32);
            let bot_right = (
                (j + 1) as i32 * col_width as i32,
                (i + 1) as i32 * row_height as i32,
            );
            let rgb: RGB = tile_colours_array[[i, j]];
            let (tile_box, rgb) = create_tile(top_left, bot_right, rgb);
            pane_grid.push((tile_box, rgb));
        }
    }

    result_window.push(pane_grid);
    result_window

} // create_pane_test_data


/// This function returns a rows by cols vec 
/// Leaving this here for posterity. Never being used
fn _create_svg_data_with_input(
    tile_colours_array: Array2<RGB>,
    row_height: i32,
    col_width: i32,
) -> Vec<Vec<(Box2D<i32>, RGB)>> {
    let nrows = tile_colours_array.shape()[0] as usize;
    let ncols = tile_colours_array.shape()[1] as usize;

    let mut result_window = Vec::with_capacity(nrows);

    for i in 0..nrows {
        let mut pane_grid = Vec::with_capacity(ncols);

        for j in 0..ncols {
            let top_left = (j as i32 * col_width as i32, i as i32 * row_height as i32);
            let bot_right = (
                (j + 1) as i32 * col_width as i32,
                (i + 1) as i32 * row_height as i32,
            );
            let rgb: RGB = tile_colours_array[[i, j]];
            let (tile_box, rgb) = create_tile(top_left, bot_right, rgb);
            pane_grid.push((tile_box, rgb));
        }

        result_window.push(pane_grid);
    }

    result_window
}

