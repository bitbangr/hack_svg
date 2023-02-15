
use ndarray::Array2;
use ndarray::Array1;
use ndarray::Axis;
use euclid::default::Box2D;
use crate::modtile;
// use crate::modtile::RGB;

use crate::create_tile;

use crate::constants::{NORTH,EAST,SOUTH,WEST};
use crate::constants::{NE_CORNER,NW_CORNER, SW_CORNER, SE_CORNER};
use crate::constants::{TOP_LEFT,TOP_RIGHT,BOT_RIGHT, BOT_LEFT};

use crate::dfs_tiles::get_contiguous_tiles_mod;


use crate::get_edge_bools;

use crate::svg_utils;
use crate::{pane_vec_to_ndarray, get_bool_arr, box2d_to_points};

/// This function is a bit of a hack.  The Pane Tiles are stored in a 1 dimensional Vec that contains 
/// as many tiles there are in a single pane.  So dimension is 1 by TotalNumberOfTiles
/// 
/// The Depth first search algorithm only works if the Vec is in a shape that corresponds to the number of 
/// rows and columns. So Dimension is tiles_per_pane_height x tiles_per_pane_width (rows x cols) 
/// 
/// Hence we call the pane_to_2d_vec() with the tiles_per_pane_height and tiles_per_pane_width
/// 
/// Now we should be able to move on to create the SVG file
/// 
pub fn reshape_min() {

    let tiles_per_pane_width: usize = 2;
    let tiles_per_pane_height: usize = 2;

    // Create a simple 2x2 mosaic
    // Note there is only one pane of 4 tiles in 2 rows and 2 cols
    let mosaic_vec: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = create_2x2_white_tile_data(); 
    println!("2x2 4 white tiles mosaic vec {:?}", &mosaic_vec);

    // grab the ND Array for this mosiac pane
    // again only 1 pane so just the first element of the mosaic vec
    let pane_nd_arr = pane_vec_to_ndarray(&mosaic_vec[0],tiles_per_pane_height, tiles_per_pane_width );
    println!("\n\npane nd array {:?} ", &pane_nd_arr);

    // convert the pane_ds_arr back to a 2D vector so we can use it for the Depth First Search Algorithm
    let pane_2d_vec: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = pane_to_2d_vec(&pane_nd_arr, tiles_per_pane_height, tiles_per_pane_width);
    
    println!("\n\n2D Pane Vec -> {:?}", pane_2d_vec);

   // get the test boolean array to build our svg path with
   let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&pane_nd_arr);

println!("hey edge booleans {:?} ", &edge_booleans);
     // call get the contiguous tiles
    // TODO change this to dfs_arr::get_cont_tiles()
    let contiguous_tiles = get_contiguous_tiles_mod(&pane_2d_vec);
    println!("fn dfs_mod search results -> {:?}", &contiguous_tiles);


}

fn pane_to_2d_vec(pane_nd_arr: &ndarray::ArrayBase<ndarray::OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, modtile::RGB)>, ndarray::Dim<[usize; 2]>>, tiles_per_pane_height: usize, tiles_per_pane_width: usize) -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, modtile::RGB)>> {
    
       // Convert the ndarray into a Vec<Vec>
       let v: Vec<Vec<_>> = pane_nd_arr
       .axis_iter(Axis(0))
       .map(|row| row.to_vec())
       .collect();

//    println!("{:?}", &v);
    v

}

pub fn reshape_vec_1() {
    let a = Array2::from_shape_vec((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();

    println!("a {}", a);

    // Convert the ndarray into a Vec
    let v = a.into_raw_vec();

    println!("{:?}", v);
}

pub fn reshape_vec_2() {
    let a = Array2::from_shape_vec((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    println!("a {}", &a);

        // Reshape the 3x3 ndarray into a 1D vector
        let v = a.into_raw_vec();

        // Convert the 1D vector into a 3x3 vector
        let v3x3 = Array1::from_vec(v);

        println!("{:?}", v3x3);
}


fn reshape_vec_axis_iter() {
    let a = Array2::from_shape_vec((3, 3), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    println!("a {}", &a);
    
    // Convert the ndarray into a Vec<Vec>
    let v: Vec<Vec<_>> = a
        .axis_iter(Axis(0))
        .map(|row| row.to_vec())
        .collect();

    println!("{:?}", v);
}




/// Make a 2x2 mosaic of a single pane of all white tiles
/// 
fn create_2x2_white_tile_data() -> Vec<Vec<(Box2D<i32>, modtile::RGB)>> {
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


