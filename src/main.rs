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

/// This application will create an SVG file from a window pane containing
/// rectangular tiles.
/// First tile is top left corner and ordered first by rows and then by columns
fn main() {

    println!("Hack SVG");
    
  /********************************************************/
  /** Commenting out much of below code for now to concentrate on last two test */
  /********************************************************

    let p_start: Point2D<i32> = Point2D::new(0, 0);
    let p_end: Point2D<i32> = Point2D::new(10, 10);
    let _box2d = Box2D {
        min: p_start,
        max: p_end,
    };

    // // let test_window: Vec<Vec<(Box2D <Point2D<i32, i32>,Point2D<i32, i32>>, modtile::RGB) = vec:new();

    let top_left: Point2D<i32> = Point2D::new(1, 2);
    let bottom_right: Point2D<i32> = Point2D::new(3, 4);

    let fbox: Box2D<i32> = Box2D::new(top_left, bottom_right);

    // let test_window: Vec<Vec<(Box2D <i32> , modtile::RGB) = vec:new();

    let input_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = create_test_data();

    let b_vec: Vec<(Box2D<i32>, modtile::RGB)> = vec![(
        Box2D::new(Point2D::new(0, 0), Point2D::new(1, 1)),
        modtile::RGB(0, 0, 0),
    )];

    let a_vec: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = vec![
        vec![(
            Box2D::new(Point2D::new(0, 0), Point2D::new(1, 1)),
            modtile::RGB(0, 0, 0),
        )],
        vec![(
            Box2D::new(Point2D::new(2, 2), Point2D::new(3, 3)),
            modtile::RGB(4, 4, 4),
        )],
    ];

    //          // let first_elem = get_first_element(&vec);

    println!("a_vec {:?}", &a_vec);

    println!("b_vec {:?}", &b_vec);
    // // println!("first element {:?}", &first_elem);
    // // assert_eq!(first, (Box2D::new(Point2D::new(0,0), Point2D::new(1,1)), modtile::RGB(0,0,0)));

    let tile_box: Box2D<i32> = Box2D::new(Point2D::new(0, 0), Point2D::new(50, 50));

    // test out the convert Box2d to a series of points
    let box_points: Vec<Point2D<i32>> = box2d_to_points(tile_box);
    println!("Box2D  {:?}", &tile_box);
    println!("Box2D to points {:?}", &box_points);

    // create lines for each pair of points in
    let tile_lines: Vec<Line> = get_tile_lines(&box_points);
    println!("Lines are {:?} " , tile_lines);

    let input_3x3_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> =  create_3x3_single_pane_data();

    let ele = get_first_element(&input_3x3_window);

    println!("******** \n input_3x3_window");
    println!("First element {:?}", ele);

    // The input window is just one long vector. implicetly ordered by row col but not.
    // indexing for "row" "col" is done mathmatically.   
    // i.e. cannot use tile_n = pane[row][col] 
    //            could do tile_n = pane[row*col] 
    println!("Second element {:?}", &input_3x3_window[0][1]);
    println!("Last element {:?}", &input_3x3_window[0][8]);
    println!("input_3x3_window \n ******** ");

    // test to create a 2 dim NDArray from a vector
    // Can use this to turn ordered vector of Tiles into row col array that will 
    // be useful for checking for contiguous tiles.
    // let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let arr = vec_to_ndarray(&vec);
    // println!("Vec {:?}", &vec);
    // println!("NDArray {}", &arr);
    // println!("NDArray [1][1] {}", arr[[1,1]]);

    let tiles_per_pane_width: usize = 3;
    let tiles_per_pane_height: usize = 3;

    println!("tiles per pane width {}" , tiles_per_pane_width);
    println!("tiles per pane height {}" , tiles_per_pane_height);

    let mut pane_3x3_vec: Vec<Vec<(Box2D<i32>, modtile::RGB)>> =  create_3x3_single_pane_data();

    // let elm = pane_3x3_vec[0].remove(8);
    // println!("removed element {:?}", elm);

    println!("pane_3x3_vec len = {}", &pane_3x3_vec[0].len());

    let pane_nd_arr = pane_vec_to_ndarray(&pane_3x3_vec[0], tiles_per_pane_height, tiles_per_pane_width );
    println!("Pane NDArray {:?} ", &pane_nd_arr);
    println!("Pane NDArray [0][0] {:?} ", &pane_nd_arr[[0,0]]);
    println!("Pane NDArray [0][1] {:?} ", &pane_nd_arr[[0,1]]);
    println!("Pane NDArray [0][2] {:?} ", &pane_nd_arr[[0,2]]);
    println!("Pane NDArray [1][0] {:?} ", &pane_nd_arr[[1,0]]);
    println!("Pane NDArray [1][1] {:?} ", &pane_nd_arr[[1,1]]);
    println!("Pane NDArray [1][2] {:?} ", &pane_nd_arr[[1,2]]);
    println!("Pane NDArray [2][0] {:?} ", &pane_nd_arr[[2,0]]);
    println!("Pane NDArray [2][1] {:?} ", &pane_nd_arr[[2,1]]);
    println!("Pane NDArray [2][2] {:?} ", &pane_nd_arr[[2,2]]);

    // get the test boolean array to build our svg path with
    let mut line_bucket = get_test_bool_bucket();

    println!("bucket[0,0][0] = {:?}" , line_bucket[[0,0]][0]);
    println!("bucket = {:?}" , &line_bucket);

    // simple_draw_svg_grid (&line_bucket);

    // draw_svg_grid (line_bucket, pane_nd_arr);

    draw_svg_grid_one(line_bucket, pane_nd_arr);

    let test_arr = vec![vec!["white".to_string(), "white".to_string(),"black".to_string()],
                                         vec!["green".to_string(),"white".to_string(),"white".to_string()],
                                         vec!["white".to_string(),"green".to_string(),"green".to_string()]];

    let blk_test_arr: Vec<Vec<String>> = vec![vec!["white".to_string(); 3]; 3];

    println!("test array {:?}", &test_arr);
    let result = get_contiguous_tiles_orig(&test_arr);
    println!("search array results {:?}", result);

    println!("blk_test_arr {:?}", &blk_test_arr);
    let result = get_contiguous_tiles_orig(&blk_test_arr);
    println!("blk_test_arr search array results {:?}", result);

    *********************************************************/
    /********************************************************/
    /** Commenting out above code for now to concentrate on lower two test */
    /********************************************************/

    // create a single tile mosaic and draw the corresponding SVG diagram    
    let _ = single_tile::create_svg();

    // create a double tile horizontal mosaic of two white tiles and draw the corresponding SVG diagram    
    // let _ = two_tile_horiz::create_white_white_svg();
        // let _ = test_corner();

    let _ = create_white_black_svg();
    
}

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


/// Create an Array2 nd array of booleans
/// 
/// Each tile has a north, east, SOUTH and west direction
/// If a tile matches the colour of its neighbour then corresponding direction boolean is set to true
/// if it does not or if it is an edge then direction boolean is set to false
/// Lines are drawn for all false edges. No lines are drawn for true edges
/// 
/// set the booleans to match the colour below
/// all edges are automatically false.
/// 3x3 pane of 9 tiles with the following colours 
/// white, white, black
/// green, white, white
/// white, green, green
// fn get_test_bool_bucket() -> ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> 
// {

//     let tiles_per_pane_width: usize = 3;
//     let tiles_per_pane_height: usize = 3;

//     // let mut initf = vec![vec![false ; 4] ; row_dim * col_dim] ;
//     // let bucket = Array::from_shape_vec((3,3), initf.to_vec()).unwrap();
//     let mut bucket: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = 
//                                     get_bool_arr(tiles_per_pane_height, tiles_per_pane_width);
//     // println!("bucket = {:?}" , &bucket);

//     bucket[[0,0]][NORTH] = false;
//     bucket[[0,0]][EAST] = true;
//     bucket[[0,0]][SOUTH] = false;
//     bucket[[0,0]][WEST] = false;

//     bucket[[0,1]][NORTH] = false;
//     bucket[[0,1]][EAST] = false;
//     bucket[[0,1]][SOUTH] = true;
//     bucket[[0,1]][WEST] = true;

//     bucket[[0,2]][NORTH] = false;
//     bucket[[0,2]][EAST] = false;
//     bucket[[0,2]][SOUTH] = false;
//     bucket[[0,2]][WEST] = false;

//     bucket[[1,0]][NORTH] = false;
//     bucket[[1,0]][EAST] = false;
//     bucket[[1,0]][SOUTH] = false;
//     bucket[[1,0]][WEST] = false;

//     bucket[[1,1]][NORTH] = true;
//     bucket[[1,1]][EAST] = true;
//     bucket[[1,1]][SOUTH] = false;
//     bucket[[1,1]][WEST] = false;

//     bucket[[1,2]][NORTH] = false;
//     bucket[[1,2]][EAST] = false;
//     bucket[[1,2]][SOUTH] = false;
//     bucket[[1,2]][WEST] = true;
// // 
//     bucket[[2,0]][NORTH] = false;
//     bucket[[2,0]][EAST] = false;
//     bucket[[2,0]][SOUTH] = false;
//     bucket[[2,0]][WEST] = false;

//     bucket[[2,1]][NORTH] = false;
//     bucket[[2,1]][EAST] = true;
//     bucket[[2,1]][SOUTH] = false;
//     bucket[[2,1]][WEST] = false;

//     bucket[[2,2]][NORTH] = false;
//     bucket[[2,2]][EAST] = false;
//     bucket[[2,2]][SOUTH] = false;
//     bucket[[2,2]][WEST] = true;

//     // println!("bucket[0,0][0] = {:?}" , bucket[[0,0]][0]);
//     // println!("bucket = {:?}" , &bucket);

//     bucket

// }


///
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

fn vec_to_ndarray(vec: &Vec<i32>) -> Array2<i32> {
    let data = vec.as_slice();
    Array::from_shape_vec((3, 3), data.to_vec()).unwrap()
}

/// .
///
/// # Panics
///
/// Panics if .
fn pane_vec_to_ndarray(vec: &Vec<(Box2D<i32>, modtile::RGB)>, row_dim:usize, col_dim:usize) -> Array2<(Box2D<i32>, modtile::RGB)> {
    let data = vec.as_slice();
    // Array::from_shape_vec((3, 3), data.to_vec()).unwrap()
    // Array::from_shape_vec((arr_dim, arr_dim), data.to_vec()).unwrap()
       Array::from_shape_vec((row_dim, col_dim), data.to_vec()).unwrap()
}


/// Takes a vector of ordered box points and returns vector of lines joining each of the
/// points in the vector. 
fn get_tile_lines(box_points: &[Point2D<i32>]) -> Vec<Line> {
    let mut lines: Vec<Line> = vec![];
    for i in 0..box_points.len() {
        let start: Point2D<i32> = box_points[i];
        let end:Point2D<i32> = box_points[(i + 1) % box_points.len()];
        let edge_line = Line::new(start, end);
        lines.push(edge_line);
    }
    lines
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

/// This function returns all the tile paths in the pane for the specified RGB colour
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
///
/// # Examples
///
/// ```
/// let p_start: Point2D<i32, i32> = Point2D::new(0, 0);
/// let p_end: Point2D<i32, i32> = Point2D::new(10,10);
/// let box2d = Box2D { min: p_start, max: p_end,};
/// // convert Box2d to a vector Point2D one for each corner points
/// let corner_coords:Vec<Point2D<i32,i32>> = box2d_to_points(box2d);
/// println!("Box2D corner coordinates {:?}", corner_coords);
///
/// let eq_ans: Vec<Point2D<i32,i32>> = vec [Point2D::new(0,0),Point2D::new(0,10), Point2D::new(10,0), Point2D::new(10,10),  ]
/// assert_eq!(corner_coords, eq_ans);
///
/// ```

/// Create sample data to test out the SVG creation algorith
///
/// # Return
///
/// returns a vector of Vectors of Box2D containing top_left and bottom_right coord of each tile along
/// with the RGB value of that tile.  Each pane contains a specific number of rows and columns of tiles
fn create_data(
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



/// Create a 3x3 single pane of three colors white, black, green tiles for testing path
/// creation.  To be used to generate SVG output doc
/// hard code box width height for now as 10 
pub fn create_3x3_single_pane_data() -> Vec<Vec<(Box2D<i32>, modtile::RGB)>> {
    let tile_width:i32 = 10;
    let tile_height:i32 = 10;

    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();
    // [
    //     [(Box2D((0, 0), (24, 24)), RGB(45, 54, 147)),
    //      (Box2D((25, 0), (49, 24)), RGB(45, 54, 147)),
    //      (Box2D((0, 25), (24, 49)), RGB(245, 232, 18)),
    //      (Box2D((25, 25), (49, 49)), RGB(109, 97, 91))],

    println!("Function create_3x3_single_pane_data");
    let mut x1:i32 = 0;
    for row in 0..3 {
        let mut y1:i32 = 0;
        for col in 0..3 {            
            println!("(row,col) -> ({},{})", row,col);
            // let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((0, 0), (24, 24), (45, 54, 147));
            x1 = col * tile_width;
            y1 = row * tile_height;
            let x2 = x1 + tile_width;
            let y2 = y1 + tile_height;

            println!("(x1,y1) -> ({},{})", x1,y1);
            println!("(x2,y2) -> ({},{})\n", x2,y2);
            
            let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data(
                            (x1, y1), 
                            (x2, y2), 
                            (255, 255, 255));
            let _ = &pane_grid.push((tile_box, rgb));
           
        }

    }

    let rgb_white: modtile::RGB = modtile::RGB(255, 255, 255); // white
    let rgb_black: modtile::RGB = modtile::RGB(0, 0, 0); // black
    let rgb_green: modtile::RGB = modtile::RGB(0, 255, 0); // green

    // set the colours
    // 3x3 pane of 9 tiles with the following colours 
    // white, white, black
    // green, white, white
    // white, green, green

    pane_grid[0].1 = rgb_white;
    pane_grid[1].1 = rgb_white;
    pane_grid[2].1 = rgb_black;
    pane_grid[3].1 = rgb_green;
    pane_grid[4].1 = rgb_white;
    pane_grid[5].1 = rgb_white;
    pane_grid[6].1 = rgb_white;
    pane_grid[7].1 = rgb_green;
    pane_grid[8].1 = rgb_green;

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window

}

pub fn create_test_data() -> Vec<Vec<(Box2D<i32>, modtile::RGB)>> {

    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // let p_start : Point2D<i32,i32> = Point2D::new(tile_top_left_x as i32, tile_top_left_y as i32);
    // let p_end : Point2D<i32,i32> = Point2D::new(tile_bot_right_x as i32, tile_bot_right_y as i32);

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();
    // [
    //     [(Box2D((0, 0), (24, 24)), RGB(45, 54, 147)),
    //      (Box2D((25, 0), (49, 24)), RGB(45, 54, 147)),
    //      (Box2D((0, 25), (24, 49)), RGB(245, 232, 18)),
    //      (Box2D((25, 25), (49, 49)), RGB(109, 97, 91))],

    // [(Box2D((0, 0), (24, 24)), RGB(45, 54, 147)),
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((0, 0), (24, 24), (45, 54, 147));
    let _ = &pane_grid.push((tile_box, rgb));
    // (Box2D((25, 0), (49, 24)), RGB(45, 54, 147)),
    // let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((25, 25), (49, 49), (109, 97, 91)) ;
    // make the top two entries the same color
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) =
        create_data((25, 25), (49, 49), (45, 54, 147));
    let _ = &pane_grid.push((tile_box, rgb));
    // (Box2D((0, 25), (24, 49)), RGB(245, 232, 18)),
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) =
        create_data((0, 25), (24, 49), (245, 232, 18));
    let _ = &pane_grid.push((tile_box, rgb));
    // (Box2D((25, 25), (49, 49)), RGB(109, 97, 91))],
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) =
        create_data((25, 25), (49, 49), (109, 97, 91));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    // // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    //     // ****************************
    //     // start the second pane
    //     let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();
    //     //     [(Box2D((50, 0), (74, 24)), RGB(68, 76, 159)),
    //     //      (Box2D((75, 0), (99, 24)), RGB(161, 28, 71)),
    //     //      (Box2D((50, 25), (74, 49)), RGB(243, 116, 35)),
    //     //      (Box2D((75, 25), (99, 49)), RGB(247, 152, 32))],

    //   // [(Box2D((50, 0), (74, 24)), RGB(68, 76, 159)),
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((50, 0), (74, 24),(68, 76, 159)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));
    //   // (Box2D((75, 0), (99, 24)), RGB(161, 28, 71)),
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((75, 0), (99, 24),(161, 28, 71)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));
    //   // (Box2D((50, 25), (74, 49)), RGB(243, 116, 35)),
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((50, 25), (74, 49), (243, 116, 35)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));
    //   // (Box2D((75, 25), (99, 49)), RGB(247, 152, 32))],
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((75, 25), (99, 49), (247, 152, 32)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));

    //   // save the pane to the result window
    //   let _ = &result_window.push(pane_grid);

    //      // ****************************
    //     // start the third pane
    //     let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();
    //     //     [(Box2D((0, 50), (24, 74)), RGB(24, 159, 72)),
    //     //      (Box2D((25, 50), (49, 74)), RGB(199, 174, 145)),
    //     //      (Box2D((0, 75), (24, 99)), RGB(7, 8, 10)),
    //     //      (Box2D((25, 75), (49, 99)), RGB(41, 115, 56))],

    //   // [(Box2D((0, 50), (24, 74)), RGB(24, 159, 72)),
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((0, 50), (24, 74), (24, 159, 72)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));
    //   //  (Box2D((25, 50), (49, 74)), RGB(199, 174, 145)),
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((25, 50), (49, 74), (199, 174, 145)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));
    //   // (Box2D((0, 75), (24, 99)), RGB(7, 8, 10)),
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((0, 75), (24, 99), (7, 8, 10)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));
    //   // (Box2D((25, 75), (49, 99)), RGB(41, 115, 56))],
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((25, 75), (49, 99), (41, 115, 56)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));

    //   // save the pane to the result window
    //   let _ = &result_window.push(pane_grid);

    //      // ****************************
    //     // start the fourth pane
    //     let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();
    //     //     [(Box2D((50, 50), (74, 74)), RGB(23, 147, 173)),
    //     //      (Box2D((75, 50), (99, 74)), RGB(36, 26, 27)),
    //     //      (Box2D((50, 75), (74, 99)), RGB(249, 205, 18)),
    //     //      (Box2D((75, 75), (99, 99)), RGB(245, 231, 14))]]

    //      // (Box2D((50, 50), (74, 74)), RGB(23, 147, 173)),
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((50, 50), (74, 74),(23, 147, 173)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));
    //   //  (Box2D((75, 50), (99, 74)), RGB(36, 26, 27)),
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((75, 50), (99, 74), (36, 26, 27)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));
    //   // (Box2D((50, 75), (74, 99)), RGB(249, 205, 18)),
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((50, 75), (74, 99),(249, 205, 18)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));
    //   // (Box2D((75, 75), (99, 99)), RGB(245, 231, 14))]]
    //   let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((75, 75), (99, 99),(245, 231, 14)) ;
    //   let _ = &pane_grid.push((tile_box, rgb));

    //   // save the pane to the result window
    //   let _ = &result_window.push(pane_grid);

    // // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    result_window
}


/// Returns the first element of a 2D vector of tuples containing a Box2D and an RGB value.
///
/// # Arguments
///
/// * `vec` - A 2D vector of tuples where each tuple contains a Box2D with `i32` values and an RGB value from the `modtile` module.
///
/// # Example
///
/// ```
/// let vec = vec![vec![(Box2D::new((0,0), (1,1)), modtile::RGB::new(0,0,0)),(Box2D::new((2,2), (3,3)), modtile::RGB::new(4,4,4))]];
/// let first = get_first_element(vec);
/// assert_eq!(first, (Box2D::new((0,0), (1,1)), modtile::RGB::new(0,0,0)));
/// ```
pub fn get_first_element(vec: &Vec<Vec<(Box2D<i32>, modtile::RGB)>>) -> (Box2D<i32>, modtile::RGB) {
    vec[0][0]
}
