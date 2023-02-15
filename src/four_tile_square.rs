use std::usize;

use euclid::default::Box2D;
use euclid::default::Point2D;
use ndarray::Axis;
use ndarray::{Array, Array2};

use crate::box_corners;
use crate::create_tile;
use crate::constants::{NORTH,EAST,SOUTH,WEST};
use crate::constants::{NE_CORNER,NW_CORNER, SW_CORNER, SE_CORNER};
use crate::constants::{TOP_LEFT,TOP_RIGHT,BOT_RIGHT, BOT_LEFT};
use crate::dfs_tiles::get_contiguous_tiles_mod;
use crate::get_edge_bools;
use crate::pane_to_2d_vec;
use crate::svg_utils;
use crate::{pane_vec_to_ndarray, get_bool_arr, box2d_to_points};
use crate::{modtile::{RGB, self}};



// For a four tile square mosiac of one single tile the dimension are 2 row by 2 col
const TILES_PER_PANE_WIDTH: usize = 2;
const TILES_PER_PANE_HEIGHT: usize = 2;

/*
    This function creates a 2x2 mosaic of four white tiles and then create an SVG file with this info
*/
pub(crate) fn create_2x2_white_svg(){

    let tiles_per_pane_width: usize = 2;
    let tiles_per_pane_height: usize = 2;

    // Create a simple 2x2 mosaic
    let mosaic_vec: Vec<Vec<(Box2D<i32>, RGB)>> = create_2x2_white_tile_data(); 
    println!("test of module call create_2x2_white_tile_data {:?}", &mosaic_vec);

    // grab the ND Array for this mosiac pane
    // again only 1 pane so just the first element of the mosaic vec
    let pane_nd_arr = pane_vec_to_ndarray(&mosaic_vec[0],tiles_per_pane_height, tiles_per_pane_width );
    println!("\n\npane nd array {:?} ", &pane_nd_arr);

    // convert the pane_ds_arr back to a 2D vector so we can use it for the Depth First Search Algorithm
    let pane_2d_vec: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = pane_to_2d_vec(&pane_nd_arr, tiles_per_pane_height, tiles_per_pane_width);
    println!("\n\n2D Pane Vec -> {:?}", pane_2d_vec);

    // get the test boolean array to build our svg path with
    let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&pane_nd_arr);

    println!("edge_booleans[0,0][0] = {:?}" , edge_booleans[[0,0]][0]);
    println!("edge_booleans = {:?}" , &edge_booleans);

    // call get the contiguous tiles
    let contiguous_tiles = get_contiguous_tiles_mod(&pane_2d_vec);
    println!("fn get_contiguous_tiles_mod search results -> {:?}", &contiguous_tiles);

    let svg_file_name_str = "create_2x2_white_square.svg";

    // lets create an svg file
    let _ = svg_utils::write_svg(pane_nd_arr, edge_booleans, contiguous_tiles, svg_file_name_str,200 as usize,200 as usize);

} // create_2x2_white_svg

/*
    This function creates a 2x2 mosaic of four tiles
    1x2 white tiles first row
    1x2 black tiles second and then create an SVG file with this info
*/
pub(crate) fn create_white_row_black_row_svg(){

    let tiles_per_pane_width: usize = 2;
    let tiles_per_pane_height: usize = 2;

    // Create a white row/black row 2x2 mosaic
    let mosaic_vec: Vec<Vec<(Box2D<i32>, RGB)>> = create_white_row_black_row_tile_data(); 
    println!("test of module call create_white_row_black_row_tile_data {:?}", &mosaic_vec);

    // grab the ND Array for this mosiac pane
    // again only 1 pane so just the first element of the mosaic vec
    let pane_nd_arr = pane_vec_to_ndarray(&mosaic_vec[0],tiles_per_pane_height, tiles_per_pane_width );
    println!("\n\npane nd array {:?} ", &pane_nd_arr);

    // convert the pane_ds_arr back to a 2D vector so we can use it for the Depth First Search Algorithm
    let pane_2d_vec: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = pane_to_2d_vec(&pane_nd_arr, tiles_per_pane_height, tiles_per_pane_width);
    println!("\n\n2D Pane Vec -> {:?}", pane_2d_vec);

    // get the test boolean array to build our svg path with
    let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&pane_nd_arr);

    println!("edge_booleans[0,0][0] = {:?}" , edge_booleans[[0,0]][0]);
    println!("edge_booleans = {:?}" , &edge_booleans);

    // call get the contiguous tiles
    let contiguous_tiles = get_contiguous_tiles_mod(&pane_2d_vec);
    println!("fn get_contiguous_tiles_mod search results -> {:?}", &contiguous_tiles);

    let svg_file_name_str = "create_2x2_white_row_black_row_square.svg";

    // lets create an svg file
    let _ = svg_utils::write_svg(pane_nd_arr, edge_booleans, contiguous_tiles, svg_file_name_str,200 as usize,200 as usize);

} // create_white_row_black_row_svg



/// Make a 2x2 mosaic of a single pane of all white tiles
/// 
fn create_2x2_white_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // (Box2D((100, 0), (200, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // (Box2D((0,100), (100, 200)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // (Box2D((100,100), (200, 200)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_2x2_white_tile_data

/// Make a 2x2 mosaic of a single pane of two white and two black
/// 1 white row
/// 1 black row
fn create_white_row_black_row_tile_data() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // white row
    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (0,0);
    let bot_right:(i32,i32) = (100,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // (Box2D((100, 0), (200, 100)), RGB(255, 255, 255)),
    let top_left :(i32,i32) = (100,0);
    let bot_right:(i32,i32) = (200,100);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // black row
    // (Box2D((0,100), (100, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (0,100);
    let bot_right:(i32,i32) = (100,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // (Box2D((100,100), (200, 200)), RGB(0, 0, 0)),
    let top_left :(i32,i32) = (100,100);
    let bot_right:(i32,i32) = (200,200);
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_tile(top_left, bot_right , (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

} //create_2x2_white_tile_data





///
/// Get the Array2 ND array for the tile 
fn get_tile_ndarray (vec: &Vec<(Box2D<i32>, modtile::RGB)>) -> Array2<(Box2D<i32>, modtile::RGB)> {

    let pane_nd_array = pane_vec_to_ndarray(&vec, TILES_PER_PANE_HEIGHT, TILES_PER_PANE_WIDTH );
    
    println!("pane_nd_array[[0,0] {:?}", pane_nd_array[[0,0]]);
    println!("pane_nd_array[[0,1] {:?}", pane_nd_array[[0,1]]);
    println!("pane_nd_array[[1,0] {:?}", pane_nd_array[[1,0]]);
    println!("pane_nd_array[[1,1] {:?}", pane_nd_array[[1,1]]);

    pane_nd_array
 }
 
 /*
    This function creates a 2x2 mosaic of four white tiles and then 
    calls the dfs_arr function to test it
*/
pub(crate) fn test_2x2_dfs_arr(){

    // Create a simple 2x2 mosaic
    let mosaic_vec: Vec<Vec<(Box2D<i32>, RGB)>> = create_2x2_white_tile_data(); 
    println!("test of module call create_2x2_white_tile_data {:?}", &mosaic_vec);

    // grab the ND Array for this mosiac
    let mosaic_nd_arr = get_tile_ndarray(&mosaic_vec[0]);
    println!("Tile NDArray {:?} ", &mosaic_nd_arr);

    // get the test boolean array to build our svg path with
    let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&mosaic_nd_arr);

    println!("edge_booleans[0,0][0] = {:?}" , edge_booleans[[0,0]][0]);
    println!("edge_booleans = {:?}" , &edge_booleans);


    let _ = reshape_vec(mosaic_vec, TILES_PER_PANE_HEIGHT, TILES_PER_PANE_WIDTH ) ;


    // call get the contiguous tiles
    // TODO change this to dfs_arr::get_cont_tiles()
    // let contiguous_tiles = get_cont_tiles_arr(&mosaic_nd_arr);
    // println!("fn dfs_mod search results -> {:?}", &contiguous_tiles);

    // let svg_file_name_str = "create_2x2_white_square.svg";
  
    // lets create an svg file
    // let _ = svg_utils::write_svg(mosaic_nd_arr, edge_booleans, contiguous_tiles, svg_file_name_str,200 as usize,100 as usize);

}

// pub fn get_contiguous_tiles_mod(array: &Vec<Vec<String>>) -> Vec<Vec<(isize, isize)>> {
// pub fn reshape_vec(pane_vec: Vec<Vec<(Box2D<i32>, RGB)>>, rows: usize, cols:usize ) -> Vec<Vec<(Box2D<i32>, RGB)>> {
    pub fn reshape_vec(pane_vec: Vec<Vec<(Box2D<i32>, RGB)>>, rows: usize, cols:usize )  {    

        println!("pane_vec -> {:?}", &pane_vec);
        println!("(rows,cols) -> ({},{})", &rows,&cols);

        let a = Array2::from_shape_vec((rows-1, cols-1), pane_vec).unwrap();

        println!("a {:?}", &a);

        let v: Vec<Vec<_>> = a.axis_iter(Axis(0)).map(|row| row.to_vec()).collect();

    println!("output as vector {:?}", &v);

}
    
