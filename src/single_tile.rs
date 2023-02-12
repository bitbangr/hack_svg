use euclid::{Box2D, UnknownUnit};
use ndarray::{Array, Array2};

use crate::dfs_tiles::get_contiguous_tiles_mod;
use crate::{pane_vec_to_ndarray, get_bool_arr, NORTH,EAST,SOUTH,WEST};
use crate::{modtile::{RGB, self}, create_data};

// For a single tile mosiac the dimension are 1 row by 1 col
const TILES_PER_PANE_WIDTH: usize = 1;
const TILES_PER_PANE_HEIGHT: usize = 1;

/*
    This function creates a 1x1 mosaic of a single tile and then creates an SVG file with this info
*/
pub(crate) fn create_svg(){

    // Create a simple 1x1 mosaic
    let mosaic_vec: Vec<Vec<(Box2D<i32, UnknownUnit>, RGB)>> = create_single_tile_data();
    println!("test of module call create_single_tile_data {:?}", &mosaic_vec);

    // pane_vec_to_ndarray(&pane_3x3_vec[0], TILES_PER_PANE_HEIGHT, TILES_PER_PANE_WIDTH );

    // grab the ND Array for this mosiac
    let mosaic_nd_arr = get_single_tile_ndarray(&mosaic_vec[0]);
    println!("Tile NDArray {:?} ", &mosaic_nd_arr);

    // get the test boolean array to build our svg path with
    let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&mosaic_nd_arr);

    println!("edge_booleans[0,0][0] = {:?}" , edge_booleans[[0,0]][0]);
    println!("edge_booleans = {:?}" , &edge_booleans);

    // call get_contiguous_tiles() to get contiguous tiles 
    // need to decide if using 
    //    get_contiguous_tiles (&mosaic_vec) This seems to work  
    //       created get_contiguous_tiles_mod()
    // or get_contiguous_tiles (&mosaic_nd_arr)

    let contiguous_tiles = get_contiguous_tiles_mod(&mosaic_vec);
    println!("dfs_mod search results - with mosaic_vec -> {:?}", &contiguous_tiles);

    // lets create an svg file
    let _ = write_svg(mosaic_nd_arr, edge_booleans, contiguous_tiles);
    // simple_draw_svg_grid (&line_bucket);

    // draw_svg_grid (line_bucket, pane_nd_arr);

    // draw_svg_grid_one(edge_booleans, pane_nd_arr);


}


fn write_svg(mosaic_nd_arr: ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32, UnknownUnit>, RGB)>, ndarray::Dim<[usize; 2]>>, edge_booleans: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>, contiguous_tiles: Vec<Vec<(isize, isize)>>) 
{
    todo!()
}

///
/// Get the Array2 ND array for the tile 
fn get_single_tile_ndarray (vec: &Vec<(Box2D<i32, UnknownUnit>, modtile::RGB)>) -> Array2<(Box2D<i32, UnknownUnit>, modtile::RGB)> {

   let pane_nd_array =  pane_vec_to_ndarray(&vec, TILES_PER_PANE_HEIGHT, TILES_PER_PANE_WIDTH );
   
   pane_nd_array
}


///  This function creates the simplest possible mosaic which consists of one window of one pane with a single tile
/// 100 by 100 UnknownUnits size
pub fn create_single_tile_data() -> Vec<Vec<( Box2D<i32,UnknownUnit>, modtile::RGB)>> {

    let mut result_window: Vec<Vec<(Box2D<i32,UnknownUnit>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32,UnknownUnit>, modtile::RGB)> = Vec::new();

    // [(Box2D((0, 0), (100, 100)), RGB(45, 54, 147)),]
    let (tile_box, rgb): (Box2D<i32,UnknownUnit>, modtile::RGB) = create_data((0, 0), (100, 100), 
                                                                                                    (45, 54, 147));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window
}


/// Create an Array2 nd array of booleans
/// 
/// Each tile has a north, east, south and west edge
/// If a tile matches the colour of its neighbour then corresponding cardinal edge boolean is set to true
/// if it does not or if it is an edge then direction boolean is set to false
/// Lines are drawn for all false edges. No lines are drawn for true edges
/// 
// fn get_cardinal_edge_boolean() -> ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> 
fn get_edge_bools(mosaic_nd_arr: &ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32, UnknownUnit>, RGB)>, ndarray::Dim<[usize; 2]>>)  -> ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>
{
    let mut edges: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = 
                                    get_bool_arr(TILES_PER_PANE_HEIGHT, TILES_PER_PANE_WIDTH);

    // go through mosaic_nd_arr and set the corresponding boolean edge 
    // As we only have a single tile with no edges we shall just set all the values to false.
    // For anything more complex we need to visit each tile and compare to neighbour to set the values properly 
    // TODO need to impliment this algo.
    edges[[0,0]][NORTH] = false;
    edges[[0,0]][EAST] = false;
    edges[[0,0]][SOUTH] = false;
    edges[[0,0]][WEST] = false;

    println!("edges = {:?}" , &edges);
    // println!("edges[0,0][0] = {:?}" , edges[[0,0]][0]);

    edges

}





