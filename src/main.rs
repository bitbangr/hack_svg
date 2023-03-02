mod modtile;
mod dfs_tiles;
mod svg_utils;
mod constants;

// mod reshape;
mod mosaic_tile;
mod mosaic_tile_svg_utils;
mod pane_build_utils;

// mod pv_eb_hack;

// mod create_contig_path; // to test out svg_contiguous path code

mod single_tile;
mod two_tile_horiz;
mod four_tile_square;
mod nine_tile_square;
mod two_op_line_tiles;

use euclid::default::Box2D;
use euclid::default::Point2D;
use ndarray::{Axis,Array, Array2};
use mosaic_tile::RGB;
use constants::{TOP,RIGHT,BOTTOM,LEFT};

/// This application will create an SVG files from a various window pane/tile configurations
/// 
/// Tile configs should be done in their own file and call the svg_utils::write_svg() with the required params
fn main() {

    println!("Hack SVG");
    
    // Below are the basic tests.
    // Run these everytime major changes are made to the code base

    // let _ = two_op_line_tiles::svg_98();
    // let _ = two_op_line_tiles::svg_99();
    // let _ = two_op_line_tiles::svg_1();
    let _ = two_op_line_tiles::svg_2();
    

    // let _ = nine_tile_square::svg_1(); // create_3x3_rgb_checker_tile_data
    // let _ = nine_tile_square::svg_2(); // create_black_center_cross_tile_data
    // let _ = nine_tile_square::svg_3(); // create_black_center_cross_tile_data
    
    // TODO COME BACK TO THIS for oposing 2 lines test
    // let _ = nine_tile_square::svg_4();//create_svg4_data_center_black
    
    // let _ = four_tile_square::svg_1();
    // let _ = four_tile_square::svg_2();
    // let _ = four_tile_square::svg_3();
    // let _ = four_tile_square::svg_4();
    // let _ = four_tile_square::svg_5();   // 2x2 1st row (black, black), 2nd row (white, green)
    // let _ = four_tile_square::svg_6();   // 2x2 1st row (white, green), 2nd row (black, black) 
    // let _ = four_tile_square::svg_7();   // 2x2 1st col (white, green), 2nd col (black, black)
    // let _ = four_tile_square::svg_8();   // 2x2 1st col (black, black), 2nd col (green, white)
    // let _ = four_tile_square::svg_9();   // 2x2 1st col top left green, rest black
    // let _ = four_tile_square::svg_10();   // 2x2 1st col top right green, rest black
    // let _ = four_tile_square::svg_11();   // 2x2 1st col bot left green, rest black
    // let _ = four_tile_square::svg_12();   // 2x2 1st col bot right green, rest black

    // let _ = two_tile_horiz::svg_1();
    // let _ = two_tile_horiz::svg_2();
    // let _ = two_tile_horiz::svg_3();
    // let _ = two_tile_horiz::svg_4();

    // let _ = single_tile::svg_1();   // 1x1 green tile

    // some functions to test refactored code - now execute same as above really
    // let _ = four_tile_square::svg_98();   // 2x2 1st col (white, green), 2nd col (black, black)
    // let _ = four_tile_square::svg_99();   // 2x2 1st col top right green, rest black
    
} // main

/// Initialize an Array2D size row_dim by col_dim of a vector of [false,false,false,false]
/// each boolean represents the [Top, Right, Bottom, Left] edges of a tile.  If the tile
/// matches the colour of an adjacent tile then the corresponding boolean is set to true.
/// otherwise the value is false.
/// This value will be used to construct the lines around contiguous colours
fn get_bool_arr(row_dim:usize, col_dim:usize) -> Array2<Vec<bool>> {
    let init_bool= vec![vec![false ; 4] ; row_dim * col_dim] ;
    let bool_bucket = Array::from_shape_vec((row_dim,col_dim), init_bool.to_vec()).unwrap();    
    bool_bucket
}

/// Given a vector of (Box2D,RGB) values representing a single pane construct a 2 dimensional NDArray
/// that matches the Tiles Per Pane Width (cols) and Tiles Per Pane Height (rows) Dimensions
///
/// # Panics
///
/// Panics if .
fn pane_vec_to_ndarray(vec: &Vec<(Box2D<i32>, RGB)>, row_dim:usize, col_dim:usize) -> Array2<(Box2D<i32>, RGB)> {
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

// /// This function takes a Box2D and returns a four element array of (usize,usize) 
// /// with each pair containing the coordinates of each corner in the following order
// ///  [top_left, top_right, bottom_right, bottom_left]
// ///
// /// # Arguments
// ///
// /// `box2d` - This is the Box2D to get the corner coordinates for
// ///
// /// # Return
// ///
// /// returns a array of (usize,usize) containing the coordinates of each corner in the following order
// ///  [top_left, top_right, bottom_right, bottom_left]
// ///
// /// # Examples
// ///
// /// ```
// /// let p_start: Point2D<i32> = Point2D::new(0, 0);
// /// let p_end: Point2D<i32> = Point2D::new(10,10);
// /// let box2d:Box2D<i32> = Box2D { min: p_start, max: p_end,};
// /// 
// /// // convert Box2d to an array of Point2D one for each corner points
// /// let corner_coords:[(usize,usize);4] = box_corners(box2d);
// /// println!("Box2D corner coordinates {:?}", corner_coords);
// /// 
// /// let eq_ans = [(0 as usize,0 as usize),
// ///                                    (10 as usize,0 as usize), 
// ///                                    (10 as usize,10 as usize), 
// ///                                    (0 as usize,10 as usize), ];
// /// assert_eq!(corner_coords, eq_ans);
// /// ```
fn box_corners(box2d: Box2D<i32>) -> [(usize, usize); 4] {

    let top_left:(usize,usize)  = (box2d.min.x as usize, box2d.min.y as usize);
    let top_right:(usize,usize)  = (box2d.max.x as usize, box2d.min.y as usize);
    let bottom_right:(usize,usize) = (box2d.max.x as usize, box2d.max.y as usize);
    let bottom_left:(usize, usize) = (box2d.min.x as usize, box2d.max.y as usize);

    [top_left, top_right, bottom_right, bottom_left]
}

/// Create a mosaic tile with the supplied info
///
/// : top_left and bottom_right coord of each tile, tile colour.
/// # Return
///
/// returns 
///  ('Box2D<i32>', RGB)
///  
pub fn create_tile(
    top_left: (i32, i32),
    bot_right: (i32, i32),
    rgb_val: RGB,
) -> (Box2D<i32>, RGB) {
    let p_start: Point2D<i32> = Point2D::new(top_left.0, top_left.1);
    let p_end: Point2D<i32> = Point2D::new(bot_right.0, bot_right.1);
    let tile_box: Box2D<i32> = Box2D {
        min: p_start,
        max: p_end,
    };
    // let rgb: RGB = RGB(rgb_val.0, rgb_val.1, rgb_val.2);
    let rgb: RGB = rgb_val;

    (tile_box, rgb)
}


/// Create an Array2 nd array of booleans.
/// 
/// Each tile has a north, east, south and west edge
/// If a tile matches the colour of its neighbour then corresponding cardinal edge boolean is set to true
/// if it does not or if it is an edge then direction boolean is set to false
/// Lines are drawn for all false edges. No lines are drawn for true edges
/// 
// fn get_cardinal_edge_boolean() -> ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> 
//pub fn get_edge_bools(mosaic_nd_arr: &ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, RGB)>, ndarray::Dim<[usize; 2]>>)  -> ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>
pub fn get_edge_bools(mosaic_nd_arr: &ndarray::ArrayBase<ndarray::OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>, ndarray::Dim<[usize; 2]>>) 
                        -> ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> 
{
    let rows = mosaic_nd_arr.dim().0;
    let cols = mosaic_nd_arr.dim().1;

    // TODO check row_dim and rows below
    // are we using mosaic_nd_arr dimensions or are we using the passed Tiles Per Pane Width Height to construct the boolean
    let mut edges: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = 
                                    get_bool_arr(rows, cols);

    for i in 0..rows {
        for j in 0..cols {
            let curtile_rgb = mosaic_nd_arr[(i, j)].1;
            let top_tile_bool: bool = { if i > 0 {curtile_rgb == mosaic_nd_arr[(i - 1, j)].1 } else { false } };
            let bottom_tile_bool: bool = { if i < rows - 1 { curtile_rgb == mosaic_nd_arr[(i + 1, j)].1 } else { false } };
            let  left_tile_bool: bool = { if j > 0 { curtile_rgb == mosaic_nd_arr[(i, j - 1)].1 } else { false } };
            let  right_tile_bool: bool = { if j < cols - 1 { curtile_rgb == mosaic_nd_arr[(i, j + 1)].1 } else { false } };

            println!("get_edge_bools() ({},{}) \n\tTop {}\n\tRight {}\n\tBottom {}\n\tLeft {}", i,j, top_tile_bool, right_tile_bool, bottom_tile_bool, left_tile_bool);

            edges[[i,j]][TOP] = top_tile_bool;
            edges[[i,j]][RIGHT] = right_tile_bool;
            edges[[i,j]][BOTTOM] = bottom_tile_bool;
            edges[[i,j]][LEFT] = left_tile_bool; 
        
            // if curtile_rgb == north_tile_rgb {println!("north tile same colour");}
            // if curtile_rgb == east_tile_rgb {println!("east tile same colour");}
            // if curtile_rgb == south_tile_rgb {println!("south tile same colour");}
            // if curtile_rgb == west_tile_rgb {println!("west tile same colour");}

        } // cols
    } // rows

    // println!("get_edge_bools = {:?}" , &edges);
    
    edges

}


fn pane_to_2d_vec(pane_nd_arr: &ndarray::ArrayBase<ndarray::OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>, ndarray::Dim<[usize; 2]>>, _tiles_per_pane_height: usize, _tiles_per_pane_width: usize) -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>> {
    
    // Convert the ndarray into a Vec<Vec>
    let v: Vec<Vec<_>> = pane_nd_arr
    .axis_iter(Axis(0))
    .map(|row| row.to_vec())
    .collect();

//    println!("{:?}", &v);
 v

}
