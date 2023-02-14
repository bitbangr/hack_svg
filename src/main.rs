mod modtile;
mod dfs_tiles;
mod svg_utils;
mod constants;

mod single_tile;
mod two_tile_horiz;

use euclid::default::Box2D;
use euclid::default::Point2D;
use ndarray::{Array, Array2};

use svg::node::element::path::Data;
use svg::node::element::Path;

use svg::Document;
use std::fmt::Write;

use std::collections::HashSet;

use crate::dfs_tiles::_get_contiguous_tiles_orig;
use crate::two_tile_horiz::create_white_black_svg;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Line {
    start: Point2D<i32>,
    end: Point2D<i32>,
}

impl Line {
    pub fn new(start: Point2D<i32>, end: Point2D<i32>) -> Line {
        Line { start, end }
    }
}

/// This application will create an SVG files from a various window pane/tile configurations
/// 
/// Tile configs should be done in their own file and call the svg_utils::write_svg() with the required params
fn main() {

    println!("Hack SVG");
    
    let _ = test_corner();

    // create a single tile mosaic and draw the corresponding SVG doc    
    let _ = single_tile::create_svg();

    // create a double tile horizontal mosaic of two white tiles and draw the corresponding SVG doc    
    let _ = two_tile_horiz::create_white_white_svg();
    
    // create a double tile horizontal mosaic of one white and one black tile and create svg file
    let _ = two_tile_horiz::create_white_black_svg();
    
}

/// Function test the box_corners() call.
/// Create a box2d and check that the corner coordinates from box_corners match
fn test_corner() {
 let p_start: Point2D<i32> = Point2D::new(0, 0);
 let p_end: Point2D<i32> = Point2D::new(10,10);
 let box2d:Box2D<i32> = Box2D { min: p_start, max: p_end,};

 // convert Box2d to an array of Point2D one for each corner points
 let corner_coords:[(usize,usize);4] = box_corners(box2d);
 println!("Box2D corner coordinates {:?}", corner_coords);

 let eq_ans = [(0 as usize,0 as usize),
                                    (10 as usize,0 as usize), 
                                    (10 as usize,10 as usize), 
                                    (0 as usize,10 as usize), ];
 assert_eq!(corner_coords, eq_ans);
}


/// Initialize an Array2D size row_dim by col_dim of a vector of [false,false,false,false]
/// each boolean represents the [North, East, South, West] edges of a tile.  If the tile
/// matches the colour of an adjacent tile then the corresponding boolean is set to true.
/// otherwise the value is false.
/// This value will be used to construct the lines around contiguous colours
fn get_bool_arr(row_dim:usize, col_dim:usize) -> Array2<Vec<bool>> {
    let mut init_bool= vec![vec![false ; 4] ; row_dim * col_dim] ;
    let bool_bucket = Array::from_shape_vec((row_dim,col_dim), init_bool.to_vec()).unwrap();    
    bool_bucket
}


/// .
///
/// # Panics
///
/// Panics if .
fn pane_vec_to_ndarray(vec: &Vec<(Box2D<i32>, modtile::RGB)>, row_dim:usize, col_dim:usize) -> Array2<(Box2D<i32>, modtile::RGB)> {
    let data = vec.as_slice();
       Array::from_shape_vec((row_dim, col_dim), data.to_vec()).unwrap()
}



/// This function takes a Box2D and returns a vector of Point2D containing
/// the coordinates of each corner in the following order
///  [top_left, top_right, bottom_right, bottom_left]
///
/// # Arguments
///
/// `box2d` - This is the Box2D to get the corner coordinates for
///
/// # Return
///
/// returns a vector of Point2D containing the coordinates of each corner in the following order
///  [top_left, top_right, bottom_right, bottom_left]
///
/// # Examples
///
/// ```
/// let p_start: Point2D<i32> = Point2D::new(0, 0);
/// let p_end: Point2D<i32> = Point2D::new(10,10);
/// let box2d = Box2D { min: p_start, max: p_end,};
/// // convert Box2d to a vector Point2D one for each corner points
/// let corner_coords:Vec<Point2D<i32>> = box2d_to_points(box2d);
/// println!("Box2D corner coordinates {:?}", corner_coords);
///
/// let eq_ans: Vec<Point2D<i32>> = vec [Point2D::new(0,0),Point2D::new(0,10), Point2D::new(10,0), Point2D::new(10,10),  ]
/// assert_eq!(corner_coords, eq_ans);
///
/// ```
pub fn box2d_to_points(box2d: Box2D<i32>) -> Vec<Point2D<i32>> {
    let top_left: Point2D<i32> = box2d.min;
    let top_right: Point2D<i32> = Point2D::new(box2d.max.x, box2d.min.y);
    let bottom_right: Point2D<i32> = box2d.max;
    let bottom_left: Point2D<i32> = Point2D::new(box2d.min.x, box2d.max.y);

    vec![top_left, top_right, bottom_right, bottom_left]
}

/// This function takes a Box2D and returns a four element array of (usize,usize) 
/// with each pair containing the coordinates of each corner in the following order
///  [top_left, top_right, bottom_right, bottom_left]
///
/// # Arguments
///
/// `box2d` - This is the Box2D to get the corner coordinates for
///
/// # Return
///
/// returns a array of (usize,usize) containing the coordinates of each corner in the following order
///  [top_left, top_right, bottom_right, bottom_left]
///
/// # Examples
///
/// ```
/// let p_start: Point2D<i32> = Point2D::new(0, 0);
/// let p_end: Point2D<i32> = Point2D::new(10,10);
/// let box2d:Box2D<i32> = Box2D { min: p_start, max: p_end,};
/// 
/// // convert Box2d to an array of Point2D one for each corner points
/// let corner_coords:[(usize,usize);4] = box_corners(box2d);
/// println!("Box2D corner coordinates {:?}", corner_coords);
/// 
/// let eq_ans = [(0 as usize,0 as usize),
///                                    (10 as usize,0 as usize), 
///                                    (10 as usize,10 as usize), 
///                                    (0 as usize,10 as usize), ];
/// assert_eq!(corner_coords, eq_ans);
/// ```
fn box_corners(box2d: Box2D<i32>) -> [(usize, usize); 4] {

    let top_left:(usize,usize)  = (box2d.min.x as usize, box2d.min.y as usize);
    let top_right:(usize,usize)  = (box2d.max.x as usize, box2d.min.y as usize);
    let bottom_right:(usize,usize) = (box2d.max.x as usize, box2d.max.y as usize);
    let bottom_left:(usize, usize) = (box2d.min.x as usize, box2d.max.y as usize);

    [top_left, top_right, bottom_right, bottom_left]
}

/// Create a mosaic tile with the supplied info
///
/// # Return
///
/// returns tuple of
///  Box2D containing top_left and bottom_right coord of each tile  Box2D<i32> , and modtile:RGB value which is the tile colour.
pub fn create_tile(
    top_left: (i32, i32),
    bot_right: (i32, i32),
    rgb_val: (u8, u8, u8),
) -> (Box2D<i32>, modtile::RGB) {
    let p_start: Point2D<i32> = Point2D::new(top_left.0, top_left.1);
    let p_end: Point2D<i32> = Point2D::new(bot_right.0, bot_right.1);
    let tile_box: Box2D<i32> = Box2D {
        min: p_start,
        max: p_end,
    };
    let rgb: modtile::RGB = modtile::RGB(rgb_val.0, rgb_val.1, rgb_val.2);

    (tile_box, rgb)
}
