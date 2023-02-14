use euclid::default::Box2D;
use euclid::default::Point2D;
use ndarray::{Array, Array2};
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

use crate::box_corners;
use crate::create_tile;
// use crate::get_edge_bools;
use crate::constants::{NORTH,EAST,SOUTH,WEST};
use crate::constants::{NE_CORNER,NW_CORNER, SW_CORNER, SE_CORNER};
use crate::constants::{TOP_LEFT,TOP_RIGHT,BOT_RIGHT, BOT_LEFT};
use crate::dfs_tiles::get_contiguous_tiles_mod;
use crate::get_edge_bools;
use crate::svg_utils;
use crate::svg_utils::write_svg;
use crate::{pane_vec_to_ndarray, get_bool_arr, box2d_to_points};
use crate::{modtile::{RGB, self}};



// For a four tile square mosiac of one single tile the dimension are 2 row by 2 col
const TILES_PER_PANE_WIDTH: usize = 2;
const TILES_PER_PANE_HEIGHT: usize = 2;

/*
    This function creates a 2x2 mosaic of four white tiles and then create an SVG file with this info
*/
pub(crate) fn create_2x2_white_svg(){

    // Create a simple 1x2 mosaic
    let mosaic_vec: Vec<Vec<(Box2D<i32>, RGB)>> = create_2x2_white_tile_data(); 
    println!("test of module call create_2x2_white_tile_data {:?}", &mosaic_vec);

    // grab the ND Array for this mosiac
    let mosaic_nd_arr = get_tile_ndarray(&mosaic_vec[0]);
    println!("Tile NDArray {:?} ", &mosaic_nd_arr);

    // get the test boolean array to build our svg path with
    let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&mosaic_nd_arr);

    println!("edge_booleans[0,0][0] = {:?}" , edge_booleans[[0,0]][0]);
    println!("edge_booleans = {:?}" , &edge_booleans);

    // call get the contiguous tiles
    let contiguous_tiles = get_contiguous_tiles_mod(&mosaic_vec);
    println!("fn dfs_mod search results -> {:?}", &contiguous_tiles);

    let svg_file_name_str = "create_2x2_white_square.svg";

    // lets create an svg file
    let _ = svg_utils::write_svg(mosaic_nd_arr, edge_booleans, contiguous_tiles, svg_file_name_str,200 as usize,100 as usize);


}

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
 
 