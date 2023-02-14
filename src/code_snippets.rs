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

fn vec_to_ndarray(vec: &Vec<i32>) -> Array2<i32> {
    let data = vec.as_slice();
    Array::from_shape_vec((3, 3), data.to_vec()).unwrap()
}
