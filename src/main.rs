mod modtile;

use euclid::{Box2D, Point2D};
use modtile::RGB;

/// This function takes a Box2D and returns a vector of Point2D containing
/// the coordinates of each corner in the following order
///  [top_left, top_right, bottom_right, bottom_left]
///
/// # Arguments
///
/// * `box2d` - This is the Box2D to get the corner coordinates for
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
pub fn box2d_to_points(box2d: Box2D<i32, i32>) -> Vec<Point2D<i32, i32>> {
    let top_left: Point2D<i32, i32> = box2d.min;
    let top_right: Point2D<i32, i32> = Point2D::new(box2d.max.x, box2d.min.y);
    let bottom_right: Point2D<i32, i32> = box2d.max;
    let bottom_left: Point2D<i32, i32> = Point2D::new(box2d.min.x, box2d.max.y);

    vec![top_left, top_right, bottom_right, bottom_left]
}
/// This application will create an SVG file from a window pane containing
/// rectangular tiles.
/// First tile is top left corner and ordered first by rows and then by columns
fn main() {
    println!("Hello, world!");

    let mut input_window: Vec<Vec<(Box2D<i32, i32>, modtile::RGB)>> = create_test_data();

    let _ = &svg_it(&input_window);

    // test out the convert Box2d to a series of points
    // TODO mgj 
    // let x: Vec<Point2D<i32, i32>> = box2d_to_points(tile_box);
    // println!("Box2D to points {:?}", x);
}

/// Create sample data to test out the SVG creation algorith
/// 
/// # Return
///
/// returns a vector of Vectors of Box2D containing top_left and bottom_right coord of each tile along 
/// with the RGB value of that tile.  Each pane contains a specific number of rows and columns of tiles
///
/// 
fn create_data(top_left: (i32, i32), bot_right: (i32, i32), rgb_val: (u8, u8, u8)) -> (Box2D<i32, i32>,modtile::RGB ) {

    let p_start: Point2D<i32, i32> = Point2D::new(top_left.0, top_left.1);
    let p_end: Point2D<i32, i32> = Point2D::new(bot_right.0, bot_right.1);
    let tile_box:Box2D<i32, i32> = Box2D {
        min: p_start,
        max: p_end,
    };
    let rgb:modtile::RGB = modtile::RGB(rgb_val.0, rgb_val.1, rgb_val.2);

    (tile_box,rgb)
}

pub fn create_test_data() -> Vec<Vec<(euclid::Box2D<i32, i32>, modtile::RGB)>> {
   
    let mut result_window: Vec<Vec<(euclid::Box2D<i32, i32>, modtile::RGB)>> = Vec::new();

    // let p_start : Point2D<i32,i32> = Point2D::new(tile_top_left_x as i32, tile_top_left_y as i32);
    // let p_end : Point2D<i32,i32> = Point2D::new(tile_bot_right_x as i32, tile_bot_right_y as i32);

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32, i32>, modtile::RGB)> = Vec::new();
    // [
    //     [(Box2D((0, 0), (24, 24)), RGB(45, 54, 147)),
    //      (Box2D((25, 0), (49, 24)), RGB(45, 54, 147)),
    //      (Box2D((0, 25), (24, 49)), RGB(245, 232, 18)),
    //      (Box2D((25, 25), (49, 49)), RGB(109, 97, 91))],
    
    // [(Box2D((0, 0), (24, 24)), RGB(45, 54, 147)),
    let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((0, 0),(24, 24), (45, 54, 147)) ;
    let _ = &pane_grid.push((tile_box, rgb));
    // (Box2D((25, 0), (49, 24)), RGB(45, 54, 147)),
    let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((25, 25), (49, 49), (109, 97, 91)) ;
    let _ = &pane_grid.push((tile_box, rgb));
    // (Box2D((0, 25), (24, 49)), RGB(245, 232, 18)),
    let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((0, 25), (24, 49),(245, 232, 18)) ;
    let _ = &pane_grid.push((tile_box, rgb));
    // (Box2D((25, 25), (49, 49)), RGB(109, 97, 91))],
    let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((25, 25), (49, 49),(109, 97, 91)) ;
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    // ****************************
    // start the second pane
    let mut pane_grid: Vec<(Box2D<i32, i32>, modtile::RGB)> = Vec::new();
    //     [(Box2D((50, 0), (74, 24)), RGB(68, 76, 159)),
    //      (Box2D((75, 0), (99, 24)), RGB(161, 28, 71)),
    //      (Box2D((50, 25), (74, 49)), RGB(243, 116, 35)),
    //      (Box2D((75, 25), (99, 49)), RGB(247, 152, 32))],

  // [(Box2D((50, 0), (74, 24)), RGB(68, 76, 159)),
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((50, 0), (74, 24),(68, 76, 159)) ;
  let _ = &pane_grid.push((tile_box, rgb));
  // (Box2D((75, 0), (99, 24)), RGB(161, 28, 71)),
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((75, 0), (99, 24),(161, 28, 71)) ;
  let _ = &pane_grid.push((tile_box, rgb));
  // (Box2D((50, 25), (74, 49)), RGB(243, 116, 35)),
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((50, 25), (74, 49), (243, 116, 35)) ;
  let _ = &pane_grid.push((tile_box, rgb));
  // (Box2D((75, 25), (99, 49)), RGB(247, 152, 32))],
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((75, 25), (99, 49), (247, 152, 32)) ;
  let _ = &pane_grid.push((tile_box, rgb));

  // save the pane to the result window
  let _ = &result_window.push(pane_grid);


     // ****************************
    // start the third pane
    let mut pane_grid: Vec<(Box2D<i32, i32>, modtile::RGB)> = Vec::new();   
    //     [(Box2D((0, 50), (24, 74)), RGB(24, 159, 72)),
    //      (Box2D((25, 50), (49, 74)), RGB(199, 174, 145)),
    //      (Box2D((0, 75), (24, 99)), RGB(7, 8, 10)),
    //      (Box2D((25, 75), (49, 99)), RGB(41, 115, 56))],

  // [(Box2D((0, 50), (24, 74)), RGB(24, 159, 72)),
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((0, 50), (24, 74), (24, 159, 72)) ;
  let _ = &pane_grid.push((tile_box, rgb));
  //  (Box2D((25, 50), (49, 74)), RGB(199, 174, 145)),
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((25, 50), (49, 74), (199, 174, 145)) ;
  let _ = &pane_grid.push((tile_box, rgb));
  // (Box2D((0, 75), (24, 99)), RGB(7, 8, 10)),
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((0, 75), (24, 99), (7, 8, 10)) ;
  let _ = &pane_grid.push((tile_box, rgb));
  // (Box2D((25, 75), (49, 99)), RGB(41, 115, 56))],
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((25, 75), (49, 99), (41, 115, 56)) ;
  let _ = &pane_grid.push((tile_box, rgb));

  // save the pane to the result window
  let _ = &result_window.push(pane_grid);

     // ****************************
    // start the fourth pane
    let mut pane_grid: Vec<(Box2D<i32, i32>, modtile::RGB)> = Vec::new();   
    //     [(Box2D((50, 50), (74, 74)), RGB(23, 147, 173)),
    //      (Box2D((75, 50), (99, 74)), RGB(36, 26, 27)),
    //      (Box2D((50, 75), (74, 99)), RGB(249, 205, 18)),
    //      (Box2D((75, 75), (99, 99)), RGB(245, 231, 14))]]

     // (Box2D((50, 50), (74, 74)), RGB(23, 147, 173)),
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((50, 50), (74, 74),(23, 147, 173)) ;
  let _ = &pane_grid.push((tile_box, rgb));
  //  (Box2D((75, 50), (99, 74)), RGB(36, 26, 27)),
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((75, 50), (99, 74), (36, 26, 27)) ;
  let _ = &pane_grid.push((tile_box, rgb));
  // (Box2D((50, 75), (74, 99)), RGB(249, 205, 18)),
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((50, 75), (74, 99),(249, 205, 18)) ;
  let _ = &pane_grid.push((tile_box, rgb));
  // (Box2D((75, 75), (99, 99)), RGB(245, 231, 14))]]
  let (tile_box, rgb): (Box2D<i32, i32>, modtile::RGB) = create_data((75, 75), (99, 99),(245, 231, 14)) ;
  let _ = &pane_grid.push((tile_box, rgb));

  // save the pane to the result window
  let _ = &result_window.push(pane_grid);
 

    result_window
}


fn svg_it(input_window: &Vec<Vec<(euclid::Box2D<i32, i32>, modtile::RGB)>>) {
    println!("\n YO ********* \n\n {:?} \n ********* \n\n", input_window)
}
