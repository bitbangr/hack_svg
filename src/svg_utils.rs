use crate::mosaic_tile::{Tile, RGB, MosaicTile};
use crate::dfs_tiles;
use crate::constants::{FLAGGED, TOP, BOTTOM, LEFT, RIGHT};
use crate::constants::{TOP_LEFT,TOP_RIGHT,BOT_RIGHT, BOT_LEFT};

use euclid::default::{Box2D, Point2D};
use ndarray::{Array2, ArrayBase, OwnedRepr, Dim};
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

use crate::get_edge_bools;
use crate::pane_to_2d_vec;
use crate::pane_vec_to_ndarray;

use crate::mosaic_tile_svg_utils::{get_tile_svg_line_data, combine_data};

use num_traits::Zero;

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct TileVisited{
    pub edge_visited: Vec<bool>
} 

impl TileVisited {
    pub fn new(edge_visited: Vec<bool>) -> TileVisited {
        TileVisited { edge_visited }
    }
}

impl Zero for TileVisited {
    fn zero() -> Self {
        TileVisited {
            edge_visited: Vec::new(),
        }
    }

    fn is_zero(&self) -> bool {
        self.edge_visited.is_empty()
    }
}

use std::ops::Add;
impl Add for TileVisited {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        TileVisited {
            // WARNING WARNING WARNING
            //
            // THIS ADD FUNCTION IS BORKED.
            //
            // JUST RETURNS THE FIRST ELEMENT
            // This is here so that code compiles
            edge_visited: self.edge_visited,
        }
    }
}




///
/// draw an svg polyline outline around a Vec of contiguous tiles of the same colour
/// Assumptions
///   1) all contiguous tiles are bounded by a closed border (i.e. there no open segments in the border)
///   2) the end point of one line segment is always the start point of another line segent
///   3) There is exactly one line exiting from an endpoint (otherwise it is not a simple enclosed shape)
///   4) every tile has exactly 0, 1 , 2 , 3 or 4 edges that are borders to be drawn
///   5) each tile is considered to be oriented North 
///   6) each tile has a Top, Right, Bottom and Left edge
///   6) Given two adjacent tiles A and B, if tile A is a different colour from tile B then the edge between Tile A and Tile B is marked "FALSE"
///         and a border line is drawn at this edge between these two tiles
///      Edges/Borders of Adjacent Tiles will always be mirrors of each other i.e. Tile A Right = Tile B Left or Tile A Top = Tile B Bottom 
///   7) Conversly if Tile A is the same colour as tile B then the edge between Tile A and Tile B is marked TRUE
///         and no border line is drawn betwee these two tiles
///   8) each tile has an associated array (T,R,B,L) that holds (T/F,T/F,T/F,T/F) corresponding to whether the adjacent tile is the same colour or not
///         a border is drawn for False Edges, A border is not dranw for True edges
///   9) Tiles are never rotated. Top edge cannot become Right edge etc.
///  10) Tile borders are always drawn in clockwise fashion in the output SVG
///  11) SVG Lines(borders) are drawn for tile edges that are marked False. 
///  12) There are 16 possible configurations of borders (tile edges which have been marked false) for a Topbound(Northbound) tile ranging from none to all 4 edges being a border
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
/// The above is a good start for documentation of meth travel_contig_svg_refact()


/// General helper function for used for testing
/// 
/// Arguments
/// op_svg_file_name
/// svg_width
/// svg_height
/// pane_rows - how many rows of panes are in whole mosaic 
/// pane_cols - how many columns of panes in whole mosaic 
/// tiles_per_pane_height: usize, <- this is redundant
/// tiles_per_pane_width: usize,  <- this is redundant
pub(crate) fn test_create_svg(op_svg_file_name: &str, 
    svg_width: i32, 
    svg_height: i32, 
    _pane_rows: usize, 
    _pane_cols: usize, 
    tiles_per_pane_height: usize,  // = number of rows
    tiles_per_pane_width: usize,   // = number of cols
    create_mosaic_data_fn: fn() -> Vec<Vec<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>>) 
{
    println!("svg_utils::create_svg");

    // lets call the create data function 
    let mosaic_vec: Vec<Vec<(Box2D<i32>, RGB)>> = create_mosaic_data_fn(); 
    println!(" create_mosaic_data_fn {:?}", &mosaic_vec);

    // grab the ND Array for the first mosiac pane
    // which is the first element of the mosaic vec
    // TODO In future need to iterate over all panes 
    let pane_nd_arr = pane_vec_to_ndarray(&mosaic_vec[0],tiles_per_pane_height , tiles_per_pane_width ); // rows, cols
    println!("\n\npane nd array {:?} ", &pane_nd_arr);

    // convert the pane_ds_arr back to a 2D vector so we can use it for the Depth First Search Algorithm
    let pane_2d_vec: Vec<Vec<(Box2D<i32>, RGB)>> = pane_to_2d_vec(&pane_nd_arr, tiles_per_pane_height, tiles_per_pane_width);
    println!("\n\n2D Pane Vec -> {:?}", pane_2d_vec);


    // get the test boolean array to build our svg path with
    let edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&pane_nd_arr);

    println!("edge_booleans = {:?}" , &edge_booleans);

    // get Vec of Vec of contigous tiles
    let contiguous_tiles = dfs_tiles::get_contiguous_tiles_mod(&pane_2d_vec);
    println!("fn get_contiguous_tiles_mod search results -> {:?}", &contiguous_tiles);

    // combine pane_nd_arr and edge_booleans into Array of MosaicTiles.  
    // Each tile holds its own boolean edge values as well as Box2D and RGB values
    let pane_edge_nd_arr:  Array2<MosaicTile> = combine_pane_edges(&pane_nd_arr, &edge_booleans);

    println! ("*********\nmosaic_pane_edge_nd_arr\n\n{:?}", &pane_edge_nd_arr);

    // testing the travel contigous tiles function
    // let _ = travel_contig_svg(pane_nd_arr, 
    //                     edge_booleans, 
    //                     contiguous_tiles, 
    //                     op_svg_file_name ,
   //                      svg_width as usize,
    //                     svg_height as usize);
    // testing the travel contigous tiles function
    let _ = travel_contig_svg_refact(pane_edge_nd_arr, 
                        contiguous_tiles, 
                        op_svg_file_name ,
                        svg_width as usize,
                        svg_height as usize);

    
}


// ************************************
// ************************************

/// General helper function for used for testing
/// 
/// Arguments
/// op_svg_file_name
/// svg_width
/// svg_height
/// pane_rows - how many rows of panes are in whole mosaic 
/// pane_cols - how many columns of panes in whole mosaic 
/// tiles_per_pane_height: usize, <- this is redundant
/// tiles_per_pane_width: usize,  <- this is redundant
pub(crate) fn create_svg(op_svg_file_name: &str, 
    svg_width: i32, 
    svg_height: i32, 
    _pane_rows: usize, 
    _pane_cols: usize, 
    tiles_per_pane_height: usize,  // = number of rows
    tiles_per_pane_width: usize,   // = number of cols
    mosaic_vec: Vec<Vec<(Box2D<i32>, RGB)>> ) 
{
    println!("svg_utils::create_svg");

    // lets call the create data function 
    // let mosaic_vec: Vec<Vec<(Box2D<i32>, RGB)>> = create_mosaic_data_fn(); 
    println!(" create_mosaic_data_fn {:?}", &mosaic_vec);

    // grab the ND Array for the first mosiac pane
    // which is the first element of the mosaic vec
    // TODO In future need to iterate over all panes 
    let pane_nd_arr = pane_vec_to_ndarray(&mosaic_vec[0],tiles_per_pane_height , tiles_per_pane_width ); // rows, cols
    println!("\n\npane nd array {:?} ", &pane_nd_arr);

    // convert the pane_ds_arr back to a 2D vector so we can use it for the Depth First Search Algorithm
    let pane_2d_vec: Vec<Vec<(Box2D<i32>, RGB)>> = pane_to_2d_vec(&pane_nd_arr, tiles_per_pane_height, tiles_per_pane_width);
    println!("\n\n2D Pane Vec -> {:?}", pane_2d_vec);


    // get the test boolean array to build our svg path with
    let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&pane_nd_arr);

    println!("edge_booleans = {:?}" , &edge_booleans);

    // get Vec of Vec of contigous tiles
    let contiguous_tiles = dfs_tiles::get_contiguous_tiles_mod(&pane_2d_vec);
    println!("fn get_contiguous_tiles_mod search results -> {:?}", &contiguous_tiles);

    // combine pane_nd_arr and edge_booleans into Array of MosaicTiles.  
    // Each tile holds its own boolean edge values as well as Box2D and RGB values
    let pane_edge_nd_arr:  Array2<MosaicTile> = combine_pane_edges(&pane_nd_arr, &edge_booleans);

    println! ("*********\nmosaic_pane_edge_nd_arr\n\n{:?}", &pane_edge_nd_arr);

    // testing the travel contigous tiles function
    // let _ = travel_contig_svg(pane_nd_arr, 
    //                     edge_booleans, 
    //                     contiguous_tiles, 
    //                     op_svg_file_name ,
   //                      svg_width as usize,
    //                     svg_height as usize);
    // testing the travel contigous tiles function
    let _ = travel_contig_svg_refact(pane_edge_nd_arr, 
                        contiguous_tiles, 
                        op_svg_file_name ,
                        svg_width as usize,
                        svg_height as usize);

    
}



//****************************** */
//****************************** */
fn travel_contig_svg_refact(pane_edge_nd_arr: ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>>, 
                            contiguous_tiles: Vec<Vec<(isize, isize)>>, 
                            op_svg_file_name: &str, 
                            svg_width: usize, 
                            svg_height: usize) -> Result<(), std::io::Error> 
{
    println!("\n***********\nfn travel_contig_svg_refact\n***********");
    println!("\nVector of contigous tiles -> {:?}", contiguous_tiles);

    let mut document = Document::new().set("viewBox", (0, 0, svg_width, svg_height));

    let shape = pane_edge_nd_arr.shape();
    let mut visited_tiles: Array2<TileVisited> = create_visited_bool_arr(shape);

    // visited_tiles[[0,0]] = TileVisited::new(vec![true,true,true,true]);
    // // visited_tiles[[0,1]][0] = true;
    // visited_tiles[[0, 1]].edge_visited[TOP] = true;
    // visited_tiles[[0, 1]].edge_visited[BOTTOM] = true;
    // visited_tiles[[0, 2]].edge_visited[LEFT] = true;
    // visited_tiles[[0, 2]].edge_visited[RIGHT] = true;

    // // *visited_tiles.get_mut((0, 1))[0].unwrap() = true; // modify element at row 0, column 1
    // println!("Visited Tiles {:?} ", &visited_tiles);

    // let match_this_tttt = [Some(true), Some(true), Some(true), Some(true)];
    // let match_this_ftft = [Some(false), Some(true), Some(false), Some(true)];

    // let visited_tile_bool = visited_tiles[[0,0]].edge_visited.clone();
    // let visited_tile_is_tttt :bool = match_edge_boolean_pattern(match_this_tttt, &visited_tile_bool);
    // let visited_tile_is_ftft :bool = match_edge_boolean_pattern(match_this_ftft, &visited_tile_bool);

    // // this is where we need to add logic to handle finding next tile for
    // // FTFT and TFTF tiles
    // if visited_tile_is_tttt == true {
    //     println!("\nvisited_tile_edges are {:?}", &visited_tile_bool);
    // }
    // // } else if tile_is_ftft == true {
    // //     println!("\nnext tile found is FTFT {:?}", &cur_tile);
    // // }

    // Grab a collection of contigous tiles
    for contig_group in &contiguous_tiles{

        // current end location of last line drawn (x,y)
        // need to check this is the start point of the next line 
        // let mut curr_svg_line_end_point: (usize,usize) = (0,0);
        let mut curr_svg_line_end_point: Point2D<i32>;
        // let mut is_first_tile : bool = true;

        // grab the first tile
        let start_tile_idx = contig_group[0];
        println!("\nStart tile index from contiguous tiles -> {:?}" , &start_tile_idx);

        let mut row = *&start_tile_idx.0 as usize;
        let mut col = *&start_tile_idx.1 as usize;

        // grab the first tile and keep track of it
        let start_tile:MosaicTile  = pane_edge_nd_arr[[row,col]].clone(); 
        
        let start_tile_rgb_str = &start_tile.tile.rgb.to_string().replace(" ", "");
        let rgb_str = start_tile_rgb_str.to_string(); 

        let mut more_tiles: bool = true; 

        // Create new SVG line data and move to the start point of the first tile
        let mut line_data = Data::new();
        let start_xy = start_tile.get_start_point_as_i32();
        line_data = line_data.move_to(start_xy);

        // Update the first end point as the first tile end_point
        curr_svg_line_end_point = start_tile.end_point;

        while more_tiles {

            let cur_tile  = &pane_edge_nd_arr[[row,col]]; 
            
            println!("\n\n********** cur_tile INFO **********");
            println!("(row: {} col: {})",row, col);
            println!("\tbox co-ords {:?}", &cur_tile.tile.coords);
            println!("\trgb {:?}", &cur_tile.tile.coords);
            println!("\tedge_bool {:?}", &cur_tile.edge_bool);

            let corner = cur_tile.tile.corners();
            let cur_tile_start_point = cur_tile.start_point;
            let cur_tile_end_point = cur_tile.end_point;
            let cur_tile_start_point_two = cur_tile.start_point_two;
            let cur_tile_end_point_two = cur_tile.end_point_two;

            println!("\n\ttop left corner {:?}", corner[TOP_LEFT]);
            println!("\ttop right corner {:?}", corner[TOP_RIGHT]);
            println!("\tbottom right corner {:?}", corner[BOT_RIGHT]);
            println!("\tbottom left corner {:?}", corner[BOT_LEFT]);
            println!("\tcur_tile_start_point: {:?}", cur_tile_start_point);
            println!("\tcur_tile_end_point: {:?}\n", cur_tile_end_point);
            println!("\tcur_tile_start_point_two: {:?}", cur_tile_start_point_two);
            println!("\tcur_tile_end_point_two: {:?}\n\n", cur_tile_end_point_two);

            // add the current tile data to the line data
            // need to pass the curr_svg_line_end_point so that we can check TFTF and FTFT which lines to draw.
            // let cur_tile_svg_line_data = get_tile_svg_line_data(&cur_tile,&curr_svg_line_end_point);
            let cur_tile_svg_line_data = get_tile_svg_line_data(&cur_tile,&curr_svg_line_end_point,&visited_tiles);

            // visited_tiles[[row,col]] = TileVisited::new(vec![true,true,true,true]);

            // line_data = line_data.extend (cur_tile_svg_line_data);

            line_data = combine_data(&line_data,&cur_tile_svg_line_data );

            if contig_group.len() == 1 {
                println!("length of contig_group is 1 so there are no more tiles to process");
                break;
            }

            let (found_tile_row, found_tile_col) = find_next_tile(row, col, &cur_tile, &contig_group, &pane_edge_nd_arr ); 
            
            // this should never happen. 
            if found_tile_row == FLAGGED && found_tile_col == FLAGGED {
                println!("Did not find next tile.  Panic!");
                panic!();
            }

            // update row col 
            row = found_tile_row;
            col = found_tile_col;

            let next_tile_clone = pane_edge_nd_arr[[found_tile_row,found_tile_col]].clone(); 
            println!("Next Tile using Tile mosaic_tile::Tile struct {:?} ", &next_tile_clone);

        // TODO the below if ep = sp check does not work for FTFT or TFTF next_tile_clone tiles
        // as we don't know which point is the end of the path
        // modify this to check for FTFT || TFTF tiles -> if any visited edge end point (sp or ep) matches then we've completed travel

            // // ----------------------------------------
            // // ----------------------------------------
            // if next_tile_clone.start_point_two.x as usize != FLAGGED {
            //     println!("\n\t FTFT or TFTF -- check all corners to see if end match");

            //     // check to see if any of check_tile corners match start_tile.start_point;
            //     let corners: [euclid::Point2D<i32, euclid::UnknownUnit>; 4] = next_tile_clone.tile.get_tile_corners();
            //     let start_tile_start_point: &euclid::Point2D<i32, euclid::UnknownUnit> = &start_tile.start_point;
                
            //     // find out if curtile endpoint is in corners 
            //     // and which which corner it is [top_left, top_right, bottom_right, bottom_left] 
            //     if let Some(idx) = corners.iter().position(|&corner| corner == *start_tile_start_point) {
            //         println!("Found {:?} at index {}", start_tile_start_point, idx);
                    
            //         println!("FTFT TFTF -> Completed contigous tile group traversal");
            //         println!("need to get_tile_svg_line_data with correct info here ");
            //         more_tiles = false;
            //     } else {
            //         println!("{:?} Cur_tile_end_point not found in check_tile corners array", cur_tile_end_point);
            //     }

            // } else 
            
            if next_tile_clone.end_point == start_tile.start_point { 
                println!("Completed traversal of all tiles in contigous group");

                // add the last tile data to the data 
                // let next_tile_svg_line_data = get_tile_svg_line_data(&next_tile_clone, &start_tile.start_point );
                let next_tile_svg_line_data = get_tile_svg_line_data(&next_tile_clone, &start_tile.start_point, &visited_tiles );
                // line_data = line_data.extend (cur_tile_svg_line_data);
    
                line_data = combine_data(&line_data,&next_tile_svg_line_data );
    
                more_tiles = false;
            }
            else {
                println!("next_tile end_point != start_tile start_point\n Continue processing contigous group tiles");
                more_tiles = true;
            }

             // TODO now (fix the find yourself in the the find_next_tile)

        } // while more_tiles == true

        // finally close the path
        line_data = line_data.close();

        let stroke_colour =  "purple";
        let stroke_width =  0.25; 
    
        // create a path and add it to the svg document
        let tile_path = Path::new().set("fill", rgb_str.to_owned()) // ie -> .set("fill", "rgb(255, 0, 0)")
                                   .set("stroke", stroke_colour)
                                   .set("stroke-width", stroke_width)
                                   .set("d", line_data);
                                
        // add the tile path to the document
        document = document.add(tile_path);

    } // for contig_group in &contiguous_tiles{

    svg::save(op_svg_file_name, &document)   

}

/// Create visited boolean array with each edge set to false 
/// shape[0] is rows
/// shape[1] is cols
fn create_visited_bool_arr(shape: &[usize]) -> ArrayBase<OwnedRepr<TileVisited>, Dim<[usize; 2]>> {
    
    let mut result = Array2::<TileVisited>::zeros((shape[0], shape[1]));
        
    for mut row in result.outer_iter_mut() {
        for mut tile in row.iter_mut() {
            *tile = TileVisited::new(vec![false, false, false, false]);
        }
    }
    
    result
}



/// Find the next tile based on the end point of one tile is the start point of the next tile
/// Note tiles must reside in the same contiguous group
/// 
/// TODO!!!! need to deal with cases where search returns a link to yourself 
/// and the start point and end points are the same.  
/// So somehow remove yourself from the contig array or if congtig_row and contig_col match then skip

fn find_next_tile(row: usize, 
    col: usize, 
    cur_tile: &MosaicTile, 
    contig_group: &[(isize, isize)], 
    pane_edge_nd_arr: &ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>>) -> (usize,usize) 
    // pane_edge_nd_arr: &ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>>) -> MosaicTile 
{

    println!("\n******************\nfn find_next_tile\n******************");
    println!( "row {}\ncol {}\ncontig_group {:?}\ncur_tile {:?}", row, col, contig_group, cur_tile ); 
    println!("******************************************");
    println!("******************************************\n");

    let mut contig_row: usize;
    let mut contig_col: usize;
    let mut found:bool = false;

    // a bad way to program but if this routine completes and a next tile has not 
    // been found then return (FLAGGED,FLAGGED) where pub const FLAGGED: usize = 987659; 
    // which will be the signal to panic
    // Look into returning a Result in the future
    let mut res = (FLAGGED,FLAGGED);

    for contig_tile in contig_group{
            
        contig_row = *&contig_tile.0 as usize;
        contig_col = *&contig_tile.1 as usize;

        // don't check for ourselves
        if !((contig_row == row) && (contig_col == col))
        {

            let check_tile: MosaicTile = pane_edge_nd_arr[[contig_row,contig_col]].clone();

            // find the true match and set the new tile accordingly
            let match_this_tftf = [Some(true), Some(false), Some(true), Some(false)];
            let match_this_ftft = [Some(false), Some(true), Some(false), Some(true)];

            let cur_tile_edge_bool = cur_tile.edge_bool.clone();
            let tile_is_tftf :bool = match_edge_boolean_pattern(match_this_tftf, &cur_tile_edge_bool);
            let tile_is_ftft :bool = match_edge_boolean_pattern(match_this_ftft, &cur_tile_edge_bool);

            // this is where we need to add logic to handle finding next tile for
            // FTFT and TFTF tiles
            if tile_is_tftf == true {
                println!("\nCurrent tile is TFTF\n\n {:?}\n", &cur_tile);
            } else if tile_is_ftft == true {
                println!("\nCurrent tile is FTFT\n\n {:?}\n", &cur_tile);
            }

            if check_tile.start_point == cur_tile.end_point {
                println!("Next Tile Found\ncheck_tile.start_point == cur_tile.end_point");

                println!("Next Tile: \n {:?}", &check_tile);

                if check_tile.start_point_two.x as usize != FLAGGED {
                     println!("\n\tThis is a double line tile FTFT or TFTF");
                }

                found = true;
                res = (contig_row,contig_col);
                break;
            } else {
                println!("\n[{},{}]check_tile.start_point Does not match cur_tile.end_point", &contig_row, &contig_col);
                if check_tile.start_point_two.x as usize != FLAGGED {
                    println!("\n\tThis is double line tile FTFT or TFTF \n\n{:?} \n", &check_tile);

                    // check to see if any of check_tile corners match cur_tile.end_point 
                    let corners: [euclid::Point2D<i32, euclid::UnknownUnit>; 4] = check_tile.tile.get_tile_corners();
                    let cur_tile_end_point: &euclid::Point2D<i32, euclid::UnknownUnit> = &cur_tile.end_point;

                    // if corners.contains(cur_tile_end_point) {
                    //     println!("cur_tile_end_point is in corners array");
                    // } else {
                    //     println!("cur_tile_end_point is not in corners array");
                    // }

                    // find out if curtile endpoint is in corners 
                    // and which which corner it is [top_left, top_right, bottom_right, bottom_left] 
                    if let Some(idx) = corners.iter().position(|&corner| corner == *cur_tile_end_point) {
                        println!("Found {:?} at index {}", cur_tile_end_point, idx);

                        found = true;
                        res = (contig_row,contig_col);
                        break;
        
                    } else {
                        println!("{:?} Cur_tile_end_point not found in check_tile corners array", cur_tile_end_point);
                    }

               }

            }
        } else {
            println!("...self...");
        } 
    
    }

    // set up the new tile according to whichever match this came back true
    // pane_edge_nd_arr[[contig_row,contig_col]].clone()
    println!("fn find_next_tile return {:?}\n**********\n ", &res);

    res

} // find_next_tile


/// This function takes two array arguments, pane_nd_arr and edge_booleans, each of which has a shape of [usize; 2]. 
/// pane_nd_arr is an array of tuples containing a Box2D<i32> instance and an RGB instance, 
/// representing the position and color of each tile in the mosaic.
/// edge_booleans is an array of vectors containing boolean flags representing the visibility of the edges for 
/// each tile in the mosaic.
/// 
/// The function returns an array of MosaicTile instances with the same shape as the input arrays, where each 
/// MosaicTile instance corresponds to a tile in the input arrays. The MosaicTile instances include the 
/// Tile data from the input arrays, as well as the edge_bool data from edge_booleans.
/// 
/// Note that the implementation assumes that the input arrays have the same shape, and that the edge_bool vectors in 
///edge_booleans have the same length as the width and height of the tiles in pane_nd_arr. 
fn combine_pane_edges( pane_nd_arr: &ArrayBase<OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>, Dim<[usize; 2]>>,
                     edge_booleans: &ArrayBase<OwnedRepr<Vec<bool>>, Dim<[usize; 2]>> ) -> ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>> 
{
    
    let mut result = Array2::<MosaicTile>::zeros((pane_nd_arr.shape()[0], pane_nd_arr.shape()[1]));

    // (((0, 0), (Box2D((0, 0), (100, 100)), RGB(0, 0, 0))), ((0, 0), [false, false, true, false]))
    for (((row, col), (box2d, rgb)), ((_row1, _col1), edge_bool)) in pane_nd_arr.indexed_iter().zip(edge_booleans.indexed_iter()) 
    {
        
        let tile = Tile::new(Box2D::new(box2d.min, box2d.max), *rgb);
        let mosaic_tile = MosaicTile::new(tile, edge_bool.to_vec());
        result[[row, col]] = mosaic_tile;
    }
    result

} // combine_pane_edges


fn match_edge_boolean_pattern(match_this: [Option<bool>; 4], tile_edge_bool: &[bool]) -> bool {
    let mut res = true;
    
        if let Some(true) = match_this[0] {
            if tile_edge_bool[0] != true {
                res = false;
            }
        } else if let Some(false) = match_this[0] {
            if tile_edge_bool[0] != false {
                res = false;
            }
        }

        if let Some(true) = match_this[1] {
            if tile_edge_bool[1] != true {
                res = false;
            }
        } else if let Some(false) = match_this[1] {
            if tile_edge_bool[1] != false {
                res = false;
            }
        }

        if let Some(true) = match_this[2] {
            if tile_edge_bool[2] != true {
                res = false;
            }
        } else if let Some(false) = match_this[2] {
            if tile_edge_bool[2] != false {
                res = false;
            }
        }

        if let Some(true) = match_this[3] {
            if tile_edge_bool[3] != true {
                res = false;
            }
        } else if let Some(false) = match_this[3] {
            if tile_edge_bool[3] != false {
                res = false;
            }
        }

    // println!("match_edge_bool: {:?} result: {:?}" ,&match_this, &res);        
    res
}
