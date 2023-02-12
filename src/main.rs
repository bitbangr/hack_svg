mod my_mod;
mod modtile;
mod dfs_tiles;

mod single_tile;

use euclid::default::Box2D;
use euclid::default::Point2D;
use ndarray::{Array, Array2};

use svg::node::element::path::Data;
use svg::node::element::Path;

use svg::Document;
use std::fmt::Write;

use std::collections::HashSet;

use crate::dfs_tiles::get_contiguous_tiles_orig;

const BOARD_SIZE: i32 = 8;
const RECT_SIZE: i32 = 50;
const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;
const CELL_SIZE: i32 = 100;
const COLOR_BLK: &str = "black";
const COLOR_WHT: &str = "white";

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;



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

    my_mod::function();
    println!("Hack SVG");

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

    let _ = &svg_it(&input_window);

    // let mut match_colour :modtile::RGB = RGB(0,0,0);

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

    // let first = &input_window[0];
    // // get all the paths for a specific colour from the pane
    // let pane_paths : Vec<euclid::Point2D<i32,i32>> = get_paths (first, match_colour );

    let tile_box: Box2D<i32> = Box2D::new(Point2D::new(0, 0), Point2D::new(50, 50));

    // test out the convert Box2d to a series of points
    let box_points: Vec<Point2D<i32>> = box2d_to_points(tile_box);
    println!("Box2D  {:?}", &tile_box);
    println!("Box2D to points {:?}", &box_points);

    // create lines for each pair of points in
    let tile_lines: Vec<Line> = get_tile_lines(&box_points);
    println!("Lines are {:?} " , tile_lines);

    let input_3x3_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> =  create_3x3_single_pane_data();

    let _ = &svg_it(&input_3x3_window);

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

    // create a single tile mosaic and draw the corresponding SVG diagram    
    let _ = single_tile::create_svg();
    
}





///
/// draw an svg polyline outline around a Vec of contiguous tiles of the same colour
/// Assumptions
///   1) all contiguous tiles are bounded by a closed border (i.e. there no open segments in the border)
///   2) the end point of one line segment is always the start point of another line segent
///   3) There is exactly one line exiting from an endpoint (otherwise it is not a simple enclosed shape)
///   4) every tile has exactly 0, 1 , 2 , 3 or 4 edges that are borders to be drawn
///   5) each tile is considered to be oriented North 
///   6) each tile has a North, East, South and West edge
///   6) Given two adjacent tiles A and B, if tile A is a different colour from tile B then the edge between Tile A and Tile B is marked "FALSE"
///         and a border line is drawn at this edge between these two tiles
///      Edges/Borders of Adjacent Tiles will always be mirrors of each other i.e. Tile A East = Tile B West or Tile A North = Tile B South 
///   7) Conversly if Tile A is the same colour as tile B then the edge between Tile A and Tile B is marked TRUE
///         and no border line is drawn betwee these two tiles
///   8) each tile has an associated array (N,E,S,W) that holds (T/F,T/F,T/F,T/F) corresponding to whether the adjacent tile is the same colour or not
///         a border is drawn for False Edges, A border is not dranw for True edges
///   9) Tiles are never rotated. North edge cannot become east edge etc.
///  10) Tile borders are always drawn in clockwise fashion in the output SVG
///  11) SVG Lines(borders) are drawn for tile edges that are marked False. 
///  12) There are 16 possible configurations of borders (tile edges which have been marked false) for a Northbound tile ranging from none to all 4 edges being a border
///  13) A vector containing collections of contigous tiles has been returned by a Depth First Search Algorithm
///  14) All completely interior tiles (i.e tiles with zero borders, all edges marked true) are to be removed from search collection as there are no lines to be drawn
/// 
/// Drawing Process is
/// 1. Pick a tile from a result from the DFS collection. 
///     1.a This is the "first" tile. This both the start point and end point of the SVG border path
/// 2. Determine which of the 16 border configurations is present.
///     2a. Find the start point for this configuration
///     2b. If this is the very first tile then Store the Start Point 
///     2c. draw the appropriate border for this tile
///     2d. The end point is the end of the last line drawn for this tile
///     2e. Mark all the borders (false edges) for this tile that have been drawn as visited.
///       2e1 - if all borders visited the mark tile as completed and or remove from tiles to be inspected 
///       if tile border is not contigous then we need to add code to handle interior voids i.e. 
///       starting a new svg path drawing process for the non-contigous border (false edge)
///      | | or _
///             _
/// 
/// 3. Find the tile in the result collection that has the same start point and false edge as step 2d above
///     3a.  Note that it's possible to have tiles with same start point and true edge.  These are not borders
///     3b. Repeat step 2 and 3 
///          until the endpoint is equal to the initial start point stored in 2b above
///     3c. At this point the border is now complete so we close() the svg path and add it to the SVG document with the appropriate Fill colour
/// 4. Repeat steps 2 and 3 for all the results returned from depth first search.
///     4. once complete write the SVG document out to a file
/// 

fn draw_polyline_borders()
{
    todo!()
}

/// iterate over each tile by row col and
/// Draw out all the pane lines matching the cardinal directors for that tile
/// do not worry about duplicate line etc
/// 
fn draw_svg_grid_one(line_bucket: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>, 
                     pane_nd_arr: ndarray::ArrayBase<ndarray::OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, modtile::RGB)>, ndarray::Dim<[usize; 2]>>) {
    
    println!("\n ***********\nFUNCTION draw_svg_grid_one\n");

    // Create the svg document
    let mut document = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));

    // let stroke_colour =  "black";
    let stroke_colour =  "purple";
    let stroke_width =  0.25; 

    for (row, rows) in line_bucket.axis_iter(ndarray::Axis(0)).enumerate() {
        for (col, card_dir) in rows.iter().enumerate() {
            println!("\nRow: {}, Col: {},\nCardinal Direction Bool: {:?}", row, col, card_dir);

            let cur_tile = pane_nd_arr[[row,col]];

            println!("Tile info {:?}", &cur_tile);  

            let N = card_dir[NORTH];
            let E = card_dir[EAST];
            let S = card_dir[SOUTH];
            let W = card_dir[WEST];

            let tile_box = &cur_tile.0;
            let x0 = tile_box.min.x as usize;
            let y0 = tile_box.min.y as usize;
            let x1 = tile_box.max.x as usize;
            let y1 = tile_box.max.y as usize;

            let tile_rgb = &cur_tile.1;
            let rgb_str = tile_rgb.to_string().replace(" ", "");
            println!("rgb string  {} ", rgb_str);
            match (N, E, S, W) { //FFFF
                (false, false, false, false) => {
                        println!("match -> false false false false - SQUARE");
                        print!(" NORTH EAST SOUTH WEST fully closed single tile\n");
    
                        let mut line_data = Data::new()
                                             .move_to((x0,y0))
                                             .line_to((x1,y0))
                                             .line_to((x1,y1))
                                             .line_to((x0,y1))
                                             .line_to((x0,y0))
                                             .close();
        
                        println!("line data {:?}" , &line_data);

                            let tile_path = Path::new()
                            // .set("fill", "rgb(255, 0, 0)")
                            .set("fill", rgb_str.to_owned())
                            .set("stroke", stroke_colour)
                            .set("stroke-width", stroke_width)
                            .set("d", line_data);
                        
                        

                        // add the tile path to the document
                        document = document.add(tile_path);
        
                        }, // FFFF
                (false, true,  false, false) => {  // FTFF
                        println!("match -> false true false false EAST OPEN ");
                        let mut line_data = Data::new()
                        .move_to((x1,y1))
                        .line_to((x0,y1))
                        .line_to((x0,y0))
                        .line_to((x1,y0));
    
                        let tile_path = Path::new()
                        .set("fill", rgb_str.to_owned())
                        .set("stroke", stroke_colour)
                        .set("stroke-width", stroke_width)
                        .set("d", line_data);
    
                        // add the tile path to the document
                        document = document.add(tile_path);
    
                        }, // FTFF
                (false, false, false, true) => { // FFFT
                        println!("match -> false false false true WEST OPEN");
                        let mut line_data = Data::new()
                        .move_to((x0,y0))
                        .line_to((x1,y0))
                        .line_to((x1,y1))
                        .line_to((x0,y1));
    
                        let tile_path = Path::new()
                        .set("fill", rgb_str.to_owned())
                        .set("stroke", stroke_colour)
                        .set("stroke-width", stroke_width)
                        .set("d", line_data);
    
                        // add the tile path to the document
                        document = document.add(tile_path);

                        }, // FFFT
                (false, false,  true, false) => {
                        println!("match -> false false true false SOUTH OPEN")
                        },
                ( true, false, false, false) => {
                        println!("match -> true false false false NORTH OPEN")
                        },
                (false,  true,  true, false) => {
                    println!("match -> false, true , true, false NORTH WEST CORNER")
                    },
                (false, false,  true, true) => { // FFTT
                    println!("match -> false, false , true, true NORTH EAST CORNER");

                    let mut line_data = Data::new()
                    .move_to((x0,y0))
                    .line_to((x1,y0))
                    .line_to((x1,y1));

                    let tile_path = Path::new()
                    .set("fill", rgb_str.to_owned())
                    .set("stroke", stroke_colour)
                    .set("stroke-width", stroke_width)
                    .set("d", line_data);

                    // add the tile path to the document
                    document = document.add(tile_path);

                    }, // FFTT
                ( true,  true, false, false) => { //TTFF
                    println!("match -> true, true , false, false SOUTH WEST CORNER");
                    let mut line_data = Data::new()
                    .move_to((x1,y1))
                    .line_to((x0,y1))
                    .line_to((x0,y0));

                    let tile_path = Path::new()
                    .set("fill", rgb_str.to_owned())
                    .set("stroke", stroke_colour)
                    .set("stroke-width", stroke_width)
                    .set("d", line_data);

                    // add the tile path to the document
                    document = document.add(tile_path);

                    }, // TTFF
                ( true, false, false, true) => {
                    println!("match -> true, false,  false, true SOUTH EAST CORNER")
                    },
                ( false, true, false, true) => {
                    println!("match -> false, true, false, true NORTH SOUTH 2 HORIZONTAL LINES")
                    },
                ( true, false, true, false) => {
                    println!("match -> true, false, true, false EAST WEST 2 VERTICAL LINES")
                    },
                ( false, true, true, true) => {
                    println!("match -> false, true, true, true NORTH SINGLE LINE")
                    },
                ( true, false, true, true) => {
                    println!("match -> true, false, true, true EAST SINGLE LINE")
                    },
                ( true, true, false, true) => {
                    println!("match -> true, true, false, true SOUTH SINGLE LINE")
                    },
                ( true, true, true, false) => {
                    println!("match -> true, true, true, false WEST SINGLE LINE")
                    },
                ( true, true, true, true) => {
                    println!("match -> true, true, true, true OPEN INTERNAL TILE NO LINES ")
                    },

                _ => println!("The value does not match any of the options\n"),
            } // match 

            

            
            // print!("Draw ->");

            // if !card_dir[NORTH]&&!card_dir[EAST]&&!card_dir[SOUTH]&&!card_dir[WEST]{ // FFFF NO COLOUR MATCH FOR ALL DIRECTIONS
            //     // print!(" NORTH EAST SOUTH WEST fully closed single tile\n");
    
            //     // let mut line_data = Data::new()
            //     //                      .move_to((x0,y0))
            //     //                      .line_to((x1,y0))
            //     //                      .line_to((x1,y1))
            //     //                      .line_to((x0,y1))
            //     //                      .line_to((x0,y0))
            //     //                      .close();

            //     //     let tile_path = Path::new()
            //     //     .set("fill", "blue")
            //     //     .set("stroke", "red")
            //     //     .set("stroke-width", 0.25)
            //     //     .set("d", line_data);
                
            //     // // add the tile path to the document
            //     // document = document.add(tile_path);

            // } else {

            //     if !card_dir[NORTH]&&card_dir[EAST]&&!card_dir[SOUTH]&&!card_dir[WEST]{ //FTFF  EAST MATCHES NEXT TILE COLOR
            //         print!(" FFTF EAST is OPEN ");
            //     } // FTFF

            //     if !card_dir[NORTH] {
            //         print!(" NORTH ");
    
            //         // let tile_box = &cur_tile.0;
            //         // let x0 = tile_box.min.x as usize;
            //         // let y0 = tile_box.min.y as usize;
            //         // let x1 = tile_box.max.x as usize;
            //         // let y1 = tile_box.max.y as usize;
                
            //         // east_line_data = east_line_data.move_to((x1,y0)).line_to((x1,y1));           
    
            //     }


            //     if !card_dir[EAST] {
            //         print!(" EAST ");
    
            //         // let tile_box = &cur_tile.0;
            //         // let x0 = tile_box.min.x as usize;
            //         // let y0 = tile_box.min.y as usize;
            //         // let x1 = tile_box.max.x as usize;
            //         // let y1 = tile_box.max.y as usize;
                
            //         // east_line_data = east_line_data.move_to((x1,y0)).line_to((x1,y1));           
    
            //     }
            //     if !card_dir[SOUTH] {
            //         print!(" SOUTH ");
    
            //         // let tile_box = &cur_tile.0;
            //         // let x0 = tile_box.min.x as usize;
            //         // let y0 = tile_box.min.y as usize;
            //         // let x1 = tile_box.max.x as usize;
            //         // let y1 = tile_box.max.y as usize;
                
            //         // south_line_data = south_line_data.move_to((x0,y1)).line_to((x1,y1)); 
    
            //     }
            //     if !card_dir[WEST] {
            //         print!(" WEST ");
    
            //         // let tile_box = &cur_tile.0;
            //         // let x0 = tile_box.min.x as usize;
            //         // let y0 = tile_box.min.y as usize;
            //         // let x1 = tile_box.max.x as usize;
            //         // let y1 = tile_box.max.y as usize;
                
            //         // west_line_data = west_line_data.move_to((x0,y0)).line_to((x0,y1));           
                 
            //     }
            //     println!("edges\n");
    

            // }
            
        }// col iterator
    } // row iterator

//     let north_path = Path::new()
//     .set("fill", "green")
//     .set("stroke", "green")
//     .set("stroke-width", 1)
//     .set("d", north_line_data)
//     .set("fill-rule", "evenodd");

//     let east_path = Path::new()
//     .set("fill", "red")
//     .set("stroke", "red")
//     .set("stroke-width", 1)
//     .set("d", east_line_data)
//     .set("fill-rule", "evenodd");

//     let south_path = Path::new()
//     .set("fill", "blue")
//     .set("stroke", "blue")
//     .set("stroke-width", 1)
//     .set("d", south_line_data)
//     .set("fill-rule", "evenodd");

//     let west_path = Path::new()
//     .set("fill", "orange")
//     .set("stroke", "orange")
//     .set("stroke-width", 1)
//     .set("d", west_line_data)
//     .set("fill-rule", "evenodd");


// // Create the svg document
//     let document = Document::new()
//         .set("viewBox", (0, 0, WIDTH, HEIGHT))
//         .add(north_path)
//         .add(east_path)
//         .add(south_path)
//         .add(west_path);

    // Write the svg document to a file
    svg::save("test_1.svg", &document);
   
} // draw_sgv_grid_one

///
/// Smush everthing together
/// we take line_bool bucket
/// pane ndarray
/// check the booleans and draw and svg line in the cardinal direction 
/// create one path for all
/// and write out to document

fn draw_svg_grid(line_bucket: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>, 
                        pane_nd_arr: ndarray::ArrayBase<ndarray::OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, modtile::RGB)>, ndarray::Dim<[usize; 2]>>) 
{
    println!("draw_svg_grid");

    // all the lines to be held here
    let mut north_line_data = Data::new();
    let mut east_line_data = Data::new();
    let mut south_line_data = Data::new();
    let mut west_line_data = Data::new();

  
    for (row, rows) in line_bucket.axis_iter(ndarray::Axis(0)).enumerate() {
        for (col, card_dir) in rows.iter().enumerate() {
            println!("Row: {}, Col: {}, Cardinal Direction Bool: {:?}", row, col, card_dir);

            let cur_tile = pane_nd_arr[[row,col]];

            println!("Tile info {:?}", &cur_tile);  
            print!("Draw ->");

            // only do white tiles
            if cur_tile.1.0 == 255 {
                            
            if !card_dir[NORTH] {
                print!(" NORTH ");

                let tile_box = &cur_tile.0;
                let x0 = tile_box.min.x as usize;
                let y0 = tile_box.min.y as usize;
                let x1 = tile_box.max.x as usize;
                let y1 = tile_box.max.y as usize;
            
                north_line_data = north_line_data.move_to((x0,y0)).line_to((x1,y0));           

            }
            if !card_dir[EAST] {
                print!(" EAST ");

                let tile_box = &cur_tile.0;
                let x0 = tile_box.min.x as usize;
                let y0 = tile_box.min.y as usize;
                let x1 = tile_box.max.x as usize;
                let y1 = tile_box.max.y as usize;
            
                east_line_data = east_line_data.move_to((x1,y0)).line_to((x1,y1));           

            }
            if !card_dir[SOUTH] {
                print!(" SOUTH ");

                let tile_box = &cur_tile.0;
                let x0 = tile_box.min.x as usize;
                let y0 = tile_box.min.y as usize;
                let x1 = tile_box.max.x as usize;
                let y1 = tile_box.max.y as usize;
            
                south_line_data = south_line_data.move_to((x0,y1)).line_to((x1,y1)); 

            }
            if !card_dir[WEST] {
                print!(" WEST ");

                let tile_box = &cur_tile.0;
                let x0 = tile_box.min.x as usize;
                let y0 = tile_box.min.y as usize;
                let x1 = tile_box.max.x as usize;
                let y1 = tile_box.max.y as usize;
            
                west_line_data = west_line_data.move_to((x0,y0)).line_to((x0,y1));           
             
            }
            println!("edges\n");

        } // white tiles 
        }
    }


    let north_path = Path::new()
    .set("fill", "green")
    .set("stroke", "green")
    .set("stroke-width", 1)
    .set("d", north_line_data)
    .set("fill-rule", "evenodd");

    let east_path = Path::new()
    .set("fill", "red")
    .set("stroke", "red")
    .set("stroke-width", 1)
    .set("d", east_line_data)
    .set("fill-rule", "evenodd");

    let south_path = Path::new()
    .set("fill", "blue")
    .set("stroke", "blue")
    .set("stroke-width", 1)
    .set("d", south_line_data)
    .set("fill-rule", "evenodd");

    let west_path = Path::new()
    .set("fill", "orange")
    .set("stroke", "orange")
    .set("stroke-width", 1)
    .set("d", west_line_data)
    .set("fill-rule", "evenodd");


// Create the svg document
    let document = Document::new()
        .set("viewBox", (0, 0, WIDTH, HEIGHT))
        .add(north_path)
        .add(east_path)
        .add(south_path)
        .add(west_path);

    // Write the svg document to a file
    svg::save("hack_svg_test.svg", &document);
   


}

/* TODO Wed Feb 8ty 2023
    MGJ Need to figure out how to add move_to co_ords to line data to be 
    passed back to calling function so that we can write out the svg lines to a file

*/

// fn add_north_line(line_data: &mut Data, cur_tile: (Box2D<i32>, RGB))
//  {
//     println!("add_north_line cur_tile -> {:?}" , cur_tile );
//     let tile_box = cur_tile.0;
//     println! ("tile box min (top right) -> {:?}", tile_box.min);
//     println! ("tile box max (bot left) -> {:?}", tile_box.max);
//     let x0 = tile_box.min.x as usize;
//     let y0 = tile_box.min.y as usize;
//     let x1 = tile_box.max.x as usize;
//     let y1 = tile_box.max.y as usize;

//     line_data.move_to((x0,y0)).line_to((x1,y0));
// }


fn test_add_north_line(line_data:Data, cur_tile: (Box2D<i32>, modtile::RGB)) -> Data  {
    println!("test_add_north_line cur_tile -> {:?}" , cur_tile );

    let mut tile_data = line_data;
    
    let tile_box = cur_tile.0;
    let x0 = tile_box.min.x as usize;
    let y0 = tile_box.min.y as usize;
    let x1 = tile_box.max.x as usize;
    let y1 = tile_box.max.y as usize;

    tile_data = tile_data.move_to((x0,y0)).line_to((x1,y0));

    tile_data
}



fn create_round_path(data: Data) -> Data {
    let mut square_data = data;
    square_data = square_data.move_to((30., 30.)).line_to((40., 40.)).line_to((50., 50.));
    square_data
}


// fn update_square_data(square_data: &mut Data) {
//     square_data.move_to((0, 0));
//     square_data.line_to((0, 100));
//     square_data.line_to((100, 100));
//     square_data.line_to((100, 0));
//     square_data.line_to((0, 0));
// }

// let mut square_data = Data::new();
// update_square_data(&mut square_data);

// // You can call update_square_data multiple times with the same square_data value
// update_square_data(&mut square_data);


// fn create_square(mut square_data: Data) -> Data {
//     square_data.move_to((0.0, 0.0))
//         .line_by((0.0, 100.0))
//         .line_by((100.0, 0.0))
//         .line_by((0.0, -100.0))
//         .line_by((-100.0, 0.0))
//         .close();

//     square_data
// }



///
///  Issues.  Two overlappping polygons
/// 
fn create_non_overlapping_squares() -> Result<(), std::io::Error> {

    let mut square_data = Data::new()
    .move_to((0,0))
    .line_to((100,0))
    .line_to((100,100))
    .line_to((0,100))
    .line_to((0,0))
    .close()
    .move_to((25,25))
    .line_to((75,25))
    .line_to((75,75))
    .line_to((25,75))
    .line_to((25,25))
    .close();

    let path = Path::new()
    .set("fill", "green")
    .set("stroke", "black")
    .set("stroke-width", 1)
    .set("d", square_data)
    .set("fill-rule", "evenodd");

    let mut inner_square_data = Data::new()
    .move_to((25,25))
    .line_to((75,25))
    .line_to((75,75))
    .line_to((25,75))
    .line_to((25,25))
    .close();

    let inner_sqr_path = Path::new()
    .set("fill", "blue")
    .set("stroke", "black")
    .set("stroke-width", 1)
    .set("d", inner_square_data)
    .set("fill-rule", "evenodd");


    // // Create the svg document
    let document = Document::new()
        .set("viewBox", (0, 0, WIDTH, HEIGHT))
        .add(path)
        .add(inner_sqr_path);

    // Write the svg document to a file
    svg::save("hack_svg_square_da_bug.svg", &document)


}


/// Use the boolean file to draw SVG lines for each 
/// of the tiles based on the boolean values of the cardinal directions
fn simple_draw_svg_grid(line_bucket: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>) {
    // let shape = &line_bucket.shape();
    // let rows = &line_bucket.shape()[0];
    // let cols = &line_bucket.shape()[1];
    println!("draw_svg_grid");
    // println!("rows {} , cols {} ", rows, cols);

    // for (i, row) in line_bucket.axis_iter(ndarray::Axis(0)).enumerate() {
    //     for (j, col) in row.iter().enumerate() {
    //         println!("Row: {}, Col: {}, Value: {:?}", i, j, col);

    //     }
    // }

    for (row, rows) in line_bucket.axis_iter(ndarray::Axis(0)).enumerate() {
        for (col, cardinal_dirs) in rows.iter().enumerate() {
            println!("Row: {}, Col: {}, Cardinal Direction Bool: {:?}", row, col, cardinal_dirs);            
        }
    }
         
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
fn get_test_bool_bucket() -> ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> 
{

    let tiles_per_pane_width: usize = 3;
    let tiles_per_pane_height: usize = 3;

    // let mut initf = vec![vec![false ; 4] ; row_dim * col_dim] ;
    // let bucket = Array::from_shape_vec((3,3), initf.to_vec()).unwrap();
    let mut bucket: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = 
                                    get_bool_arr(tiles_per_pane_height, tiles_per_pane_width);
    // println!("bucket = {:?}" , &bucket);

    bucket[[0,0]][NORTH] = false;
    bucket[[0,0]][EAST] = true;
    bucket[[0,0]][SOUTH] = false;
    bucket[[0,0]][WEST] = false;

    bucket[[0,1]][NORTH] = false;
    bucket[[0,1]][EAST] = false;
    bucket[[0,1]][SOUTH] = true;
    bucket[[0,1]][WEST] = true;

    bucket[[0,2]][NORTH] = false;
    bucket[[0,2]][EAST] = false;
    bucket[[0,2]][SOUTH] = false;
    bucket[[0,2]][WEST] = false;

    bucket[[1,0]][NORTH] = false;
    bucket[[1,0]][EAST] = false;
    bucket[[1,0]][SOUTH] = false;
    bucket[[1,0]][WEST] = false;

    bucket[[1,1]][NORTH] = true;
    bucket[[1,1]][EAST] = true;
    bucket[[1,1]][SOUTH] = false;
    bucket[[1,1]][WEST] = false;

    bucket[[1,2]][NORTH] = false;
    bucket[[1,2]][EAST] = false;
    bucket[[1,2]][SOUTH] = false;
    bucket[[1,2]][WEST] = true;
// 
    bucket[[2,0]][NORTH] = false;
    bucket[[2,0]][EAST] = false;
    bucket[[2,0]][SOUTH] = false;
    bucket[[2,0]][WEST] = false;

    bucket[[2,1]][NORTH] = false;
    bucket[[2,1]][EAST] = true;
    bucket[[2,1]][SOUTH] = false;
    bucket[[2,1]][WEST] = false;

    bucket[[2,2]][NORTH] = false;
    bucket[[2,2]][EAST] = false;
    bucket[[2,2]][SOUTH] = false;
    bucket[[2,2]][WEST] = true;

    // println!("bucket[0,0][0] = {:?}" , bucket[[0,0]][0]);
    // println!("bucket = {:?}" , &bucket);

    bucket

}


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

fn get_paths(first: &[(Box2D<i32>, modtile::RGB)], match_colour: modtile::RGB) -> Vec<Point2D<i32>> {
    todo!()
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


/// . This function will create the SVG File 
fn svg_it(input_window: &Vec<Vec<(Box2D<i32>, modtile::RGB)>>) {
    println!("\n YO svg_it*********\n{:?}\n ********* \n\n", input_window)
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
