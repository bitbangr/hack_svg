use crate::adjacent_tiles::build_adjacent_map;
use crate::mosaic_tile::{Tile, RGB, MosaicTile};
use crate::{dfs_tiles, pane_to_2d_vec};
use crate::constants::{FLAGGED, TOP, BOTTOM, LEFT, RIGHT, SVG_SCALE_X, SVG_SCALE_Y, SVG_STROKE_WIDTH, SVG_PPI};
use crate::constants::{TOP_LEFT,TOP_RIGHT,BOT_RIGHT, BOT_LEFT};

use euclid::default::{Box2D, Point2D};
use ndarray::{Array2, ArrayBase, OwnedRepr, Dim};



use svg::node::element::path::{Data, Command, Position};
use svg::node::element::{Path, Polygon};
use svg::Document;
use svg::node::element::{Rectangle, Text};


use crate::get_edge_bools;
use crate::pane_vec_to_ndarray;

use crate::mosaic_tile_svg_utils::{combine_data, get_ext_tile_svg_line_data};

use num_traits::Zero;
use std::collections::HashMap;



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
///  10.0) Interior Tile borders are always drawn in counter clockwise fzashion in the output SVG
///  11) SVG Lines(borders) are drawn for tile edges that are marked False. 
///  12) There are 16 possible configurations of borders (tile edges which have been marked false) for a Topbound(Northbound) tile ranging from none to all 4 edges being a border
///  13) A vector containing collections of contiguous tiles has been returned by a Depth First Search Algorithm
///  14) All completely interior tiles (i.e tiles with zero borders, all edges marked true) will be ignored. i.e. no drawing will take place for these
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
///       if tile border is not contiguous then we need to add code to handle interior voids i.e. 
///       starting a new svg path drawing process for the non-contiguous border (false edge)
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
/// The above is a good start for documentation of function travel_contig_svg_refact()


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
    // println!(" create_mosaic_data_fn {:?}", &mosaic_vec);

    // grab the ND Array for the first mosiac pane
    // which is the first element of the mosaic vec
    // TODO In future need to iterate over all panes 
    let pane_nd_arr = pane_vec_to_ndarray(&mosaic_vec[0],tiles_per_pane_height , tiles_per_pane_width ); // rows, cols
    // println!("\n\npane nd array {:?} ", &pane_nd_arr);

    // convert the pane_ds_arr back to a 2D vector so we can use it for the Depth First Search Algorithm
    let pane_2d_vec: Vec<Vec<(Box2D<i32>, RGB)>> = pane_to_2d_vec(&pane_nd_arr, tiles_per_pane_height, tiles_per_pane_width);
    // println!("\n\n2D Pane Vec -> {:?}", pane_2d_vec);


    // get the test boolean array to build our svg path with
    let edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&pane_nd_arr);

    // println!("edge_booleans = {:?}" , &edge_booleans);

    // get Vec of Vec of contiguous tiles
    let contiguous_tiles = dfs_tiles::get_contiguous_tiles_mod(&pane_2d_vec);
    // println!("fn get_contiguous_tiles_mod search results -> {:?}", &contiguous_tiles);

    // combine pane_nd_arr and edge_booleans into Array of MosaicTiles.  
    // Each tile holds its own boolean edge values as well as Box2D and RGB values
    let pane_edge_nd_arr:  Array2<MosaicTile> = combine_pane_edges(&pane_nd_arr, &edge_booleans);

    // println! ("*********\nmosaic_pane_edge_nd_arr\n\n{:?}", &pane_edge_nd_arr);

    // Testing out new code for tile traversal
    // taking into account Exterior (Clockwise) and Interior(Counter Clockwise)
    // as well as tile visited booleans 
    let _ = travel_contig_svg(pane_edge_nd_arr, 
                        contiguous_tiles, 
                        op_svg_file_name ,
                        svg_width as usize,
                        svg_height as usize,
                        tiles_per_pane_height as usize,
                        tiles_per_pane_width as usize 
                    );
    
}

// ****************************** */
// ****************************** */

/// Generates SVG output file with the provided pane edge data, contiguous tile coordinates, 
/// dimensions, and output file name.  It also creates a separate svg file for each RGB colour
/// that can be used for laser cutting
///
/// This function processes pane edge data and contiguous tile coordinates to create an SVG file
/// with the specified dimensions and file name. The function returns a Result, which is either
/// an empty tuple (Ok case) or an error (Err case) in case of I/O issues.
///
/// # Arguments
///
/// * `pane_edge_nd_arr` - A 2-dimensional array of MosaicTile objects representing the pane edge data.
/// * `contiguous_tiles` - A vector containing vectors of tuples, where each tuple represents the (row, col) coordinates of a contiguous tile.
/// * `op_svg_file_name` - A reference to a string containing the output SVG file name.
/// * `svg_width` - The width of the SVG document.
/// * `svg_height` - The height of the SVG document.
///
/// # Returns
///
/// * `Result<(), std::io::Error>` - A Result indicating success (empty tuple) or an error in case of I/O issues.
///
/// # Example
///
/// ```rust
/// let pane_edge_nd_arr: ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>> = ...
/// let contiguous_tiles: Vec<Vec<(isize, isize)>> = ...
/// let op_svg_file_name = "output_file.svg";
/// let svg_width = 4000;
/// let svg_height = 4000;
///
/// let result =   travel_contig_svg(pane_edge_nd_arr, contiguous_tiles, op_svg_file_name, svg_width, svg_height);
/// ```
fn travel_contig_svg(pane_edge_nd_arr: ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>>, 
                            contiguous_tiles: Vec<Vec<(isize, isize)>>, 
                            op_svg_file_name: &str, 
                            svg_width: usize, 
                            svg_height: usize,
                            tiles_per_pane_height: usize,
                            tiles_per_pane_width: usize                             
                        ) -> Result<(), std::io::Error> 
{
    let create_laser_files = true;

    println!("\n***********\nfn   travel_contig_svg\n***********");
    println!("\n {} <- Number of contiguous tile groups", contiguous_tiles.len()); 
    println!("\nVector of contiguous tiles -> {:?}", contiguous_tiles);

    // let viewBoxWidth = 4000;
    // let viewBoxHeight = 4000;
    // the desired width and height need to be calcuated based on tile size 1/2" or 12.7mm 
    // and the number of tiles per pane width and pane height.
    let desired_svg_tile_width :f32 = 0.5; // hard coded as 1/2 inch tiles TODO Fix this
    let desired_svg_tile_height :f32  = 0.5; // hard coded as 1/2 inch tiles TODO Fix this

    // our tiles are 100x100 units  and we want each tile to be 1/2 in x 1/2.
    // Standard display is 96px per inch for so 1/2" us 48px,  
    // which makes are scale factor 0.48 * 100 = 48
    // USE CONST SVG_SCALE_X and SVG_SCALE_Y 

    let desired_svg_width_in_inches = tiles_per_pane_width as f32 * desired_svg_tile_width; 
    let desired_svg_height_in_inches = tiles_per_pane_height as f32 * desired_svg_tile_height;  
    // let scale_x = desired_svg_width_in_inches / svg_width as f32  ;
    // let scale_y = desired_svg_height_in_inches / svg_height as f32  ;

    let width_str = desired_svg_width_in_inches.to_string()+"in";
    let height_str = desired_svg_height_in_inches.to_string()+"in";

    let desired_svg_width_in_inches_str = desired_svg_width_in_inches.to_string() +"in";
    let desired_svg_height_in_inches_str = desired_svg_height_in_inches.to_string() +"in";

    let mut document: svg::node::element::SVG = Document::new()
        .set("viewBox", (0, 0, desired_svg_width_in_inches_str, desired_svg_height_in_inches_str))
        .set("width", width_str)
        .set("height", height_str);
        // .set("transform", format!("scale({}, {})", scale_x, scale_y));

        // mgj TO DO try creating a path group
        // create a path group with name of that rgb value
        let mut path_group = svg::node::element::Group::new()
                .set("id", "mosaic")
                .set("transform", format!("scale({}, {})", SVG_SCALE_X, SVG_SCALE_Y));

    // let mut document: svg::node::element::SVG = Document::new()
    //     .set("viewBox", (0, 0, viewBoxWidth, viewBoxHeight))
    //     .set("width", "20in")
    //     .set("height", "20in")
    //     .set("transform", format!("scale({}, {})", scaleX, scaleY));


    // let mut document: svg::node::element::SVG = Document::new().set("viewBox", (0, 0, svg_width, svg_height));
    // let mut document: svg::node::element::SVG = Document::new().set("viewBox", "(0, 0, 20in, 20in)")
                                                            //    .set("transform", format!("scale({}, {})", scaleX, scaleY));

    let shape = pane_edge_nd_arr.shape();
    let mut visited_tiles: Array2<TileVisited> = create_visited_bool_arr(shape);
    // println!("Visited Tiles {:?} ", &visited_tiles);

    // helper function to set all TTTT tiles as visited as we will never process these to create an SVG path
    let _ = set_tttt_visited_tiles(&mut visited_tiles,&pane_edge_nd_arr);

    // for each colour store all the line Data elements
    let mut path_data_hashmap: HashMap<String,Vec<Data>> = HashMap::new();

    // Grab a collection of contiguous tiles
    for contig_group in &contiguous_tiles{

        // build a map of adjacent tiles for each entry in contous_group
        let adjacent_map = build_adjacent_map(&contig_group);

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
        let mut start_tile:MosaicTile  = pane_edge_nd_arr[[row,col]].clone(); 

        // use this value to check if paths are complete as we can have FTFT or TFTF tiles which have two pairs of start/end points
        let mut svg_line_data_begin_point =  start_tile.start_point.clone();
        
        let start_tile_rgb_str = &start_tile.tile.rgb.to_string().replace(" ", "");
        let rgb_str = start_tile_rgb_str.to_string(); 

        // Create new SVG line data and move to the start point of the first tile
        let mut line_data = Data::new();

        let start_xy = start_tile.get_start_point_as_i32();

        line_data = line_data.move_to(start_xy);

        // Update the first end point as the first tile end_point
        curr_svg_line_end_point = start_tile.end_point;

        let mut more_tiles: bool = true; 
        while more_tiles {

            let cur_tile  = &pane_edge_nd_arr[[row,col]]; 
            
            println!("\n\n********** cur_tile INFO **********");
            println!("(row: {} col: {})",row, col);
            println!("\tbox co-ords {:?}", &cur_tile.tile.coords);
            println!("\trgb {:?}", &cur_tile.tile.rgb);
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
            
            // at this point assume we are working on external path so external line date
            // once we find interior tiles we would need to call get_int_tile_svg_line_data
            let (cur_tile_svg_line_data, svg_line_end_point) = get_ext_tile_svg_line_data(&cur_tile,
                                                                    &curr_svg_line_end_point, // CHECK Called with end point
                                                                    &mut visited_tiles,
                                                                    row,
                                                                    col);

            // update the curr_svg_line_end_point to the last svg line position
            curr_svg_line_end_point = Point2D::new(svg_line_end_point.0 as i32, svg_line_end_point.1 as i32, );


            // display the visited_tiles.  Each edge should be marked true unless it is FTFT or TFTF tile
            println!(" ->Visited tile [{},{}] {:?} ", &row, &col, &visited_tiles[[row,col]]);

            // combine the cur tile line data with the existing line data
            line_data = combine_data(&line_data,&cur_tile_svg_line_data );

            if contig_group.len() == 1 {
                println!("length of contig_group is 1 so there are no more tiles to process");
                break;
            }

            // search for the next tile to process
            let (found_tile_row, found_tile_col) = find_next_tile_ext(row, col, 
                                                                      &cur_tile, 
                                                                      &contig_group, 
                                                                      &adjacent_map,
                                                                      &pane_edge_nd_arr, 
                                                                      &mut visited_tiles ); 
            
            // update the find_next_tile_ext method to include error handling
            // so we can avoid panicing below
            // this should never happen. 
            if found_tile_row == FLAGGED && found_tile_col == FLAGGED {
                println!("Did not find next tile.  Panic!");
                panic!();
            }

            // update row col to the found tile row col
            row = found_tile_row;
            col = found_tile_col;

            let next_tile_clone = pane_edge_nd_arr[[found_tile_row,found_tile_col]].clone(); 

            // Determine if path traversal is complete.  Extra handling for FTFT and TFTF tiles as they
            // have two pairs of start end points.

            // Check if next_tile_clone end_point or end_point_one matches the cur_tile start_point.
            // if it does match then we draw the last line segment(s) and set more_tiles to false;

            let next_tile_clone_tftf = next_tile_clone.is_tftf();
            let next_tile_clone_ftft = next_tile_clone.is_ftft();

            let mut path_traversal_complete = false;

            // FTFT 
            if next_tile_clone_ftft && next_tile_clone.start_point == curr_svg_line_end_point &&
                   next_tile_clone.end_point == svg_line_data_begin_point
            {
                path_traversal_complete = true;   
            }
            // FTFT
            else if next_tile_clone_ftft && next_tile_clone.start_point_two == curr_svg_line_end_point &&
            next_tile_clone.end_point_two == svg_line_data_begin_point
            {
                path_traversal_complete = true;   
            } 
            // TFTF
            else if next_tile_clone_tftf && next_tile_clone.start_point == curr_svg_line_end_point &&
                    next_tile_clone.end_point == svg_line_data_begin_point
            {
                path_traversal_complete = true;   
            } 
            // TFTF
            else if next_tile_clone_tftf && next_tile_clone.start_point_two == curr_svg_line_end_point &&
                        next_tile_clone.end_point_two == svg_line_data_begin_point
            {
                path_traversal_complete = true;   
            }
            // TFTF
            else 
            if !next_tile_clone_ftft && !next_tile_clone_tftf && 
                next_tile_clone.end_point == svg_line_data_begin_point
            { 
                path_traversal_complete = true;   
            }

            if path_traversal_complete
            {
                println!("Completed external path traversal for this contiguous group");
                println!("Must check for and draw internal SVG paths");
                
                let (next_tile_svg_line_data, svg_line_end_point) = get_ext_tile_svg_line_data(&next_tile_clone, &curr_svg_line_end_point, &mut visited_tiles, row, col );
            
                // update the curr_svg_line_end_point to the last svg line position
                curr_svg_line_end_point = Point2D::new(svg_line_end_point.0 as i32, svg_line_end_point.1 as i32  );

                line_data = combine_data(&line_data,&next_tile_svg_line_data );

                // check for incomplete tiles in contig_group to see if there are more paths still to be processed
                let incomplete_tile: Option<((isize, isize), MosaicTile)> = get_incomplete_tile(&contig_group, &visited_tiles, &pane_edge_nd_arr); 
                match incomplete_tile {
                    Some((index, tile)) => {
                        println!("\nAn Incomplete tile was found: {:?},\n\t {:?}\n", &index, &tile);
                        
                        // close the above current path line_data in prep to start a new path
                        line_data = line_data.close();

                        // grab the start xy and end point for this new path
                        let (path_start_xy , _svg_path_end_pt): ((i32, i32), euclid::Point2D<i32, euclid::UnknownUnit>) = 
                                get_incomplete_tile_info(&tile, &visited_tiles, &index);

                        // move to the start location of the incomplete edge
                        let start_xy: (i32, i32) = path_start_xy;

                        // move to start of first incomplete tile
                        line_data = line_data.move_to(start_xy);

                        // set to the end point of this tile
                        // curr_svg_line_end_point = svg_path_end_pt;
                        // This is where the path data currently sits after the move to
                        curr_svg_line_end_point = Point2D::new(start_xy.0 as i32, start_xy.1 as i32) ;  
                        
                        // this is the updated start point for the new path that we check against for completion
                        svg_line_data_begin_point = Point2D::new(start_xy.0 as i32, start_xy.1 as i32);  

                        // set the row column values 
                        // update row col to the found tile row col
                        row = index.0 as usize;
                        col = index.1 as usize;

                        let incomplete_tile_clone = pane_edge_nd_arr[[row,col]].clone(); 

                        let (next_tile_svg_line_data, svg_line_end_point)  = get_ext_tile_svg_line_data(&incomplete_tile_clone, 
                            &curr_svg_line_end_point, // 
                            &mut visited_tiles, 
                            row,
                            col );

                        // update the curr_svg_line_end_point to the last svg line position
                        curr_svg_line_end_point = Point2D::new(svg_line_end_point.0 as i32, svg_line_end_point.1 as i32, );

                        line_data = combine_data(&line_data,&next_tile_svg_line_data );

                        // update the start tile that we use to check for end of path
                        start_tile = pane_edge_nd_arr[[row,col]].clone(); 

                        // search for the next tile to process
                        let (found_tile_row, found_tile_col) = find_next_tile_ext(row, col, 
                            &start_tile, 
                            &contig_group, 
                            &adjacent_map,
                            &pane_edge_nd_arr, 
                            &mut visited_tiles ); 

                        // update the find_next_tile_ext method to include error handling
                        // so we can avoid panicing below
                        // this should never happen. 
                        if found_tile_row == FLAGGED && found_tile_col == FLAGGED {
                        println!("Did not find next tile.  Panic!");
                        panic!();
                        }

                        // update row col to the found tile row col
                        row = found_tile_row;
                        col = found_tile_col;

                        // continue to search for more tiles
                        more_tiles = true;

                    // end of Some((index, tile)) returns index and tile if found otherwise None 
                    },
                    None => {
                        // External path completed 
                        println!("\n External Path Completed\nNo Incomplete Tiles found - Wrap it up");
                        more_tiles = false;
                    }

                // end of match incomplete_tile 
                }
            
            // end of if next_tile_clone.end_point == svg_line_data_begin_point
            }
            else {
                println!("next_tile end_point != start_tile start_point\n Continue processing contiguous group tiles");
                more_tiles = true;
            }


        } // while more_tiles == true

        // finally close the path
        line_data = line_data.close();
        let line_data_clone = line_data.clone();
        let line_data_clone1 = line_data.clone();

        let stroke_width =  SVG_STROKE_WIDTH; // currently set to 0.0
    
        // create a path and add it to the svg document
        let tile_path = Path::new().set("fill", rgb_str.to_owned()) // ie -> .set("fill", "rgb(255, 0, 0)")
                                   .set("stroke", rgb_str.to_owned())
                                   .set("stroke-width", stroke_width)
                                   .set("d", line_data);
                                
        // add the tile path to the document 
        // document = document.add(tile_path);
        
        // instead of adding to document lets add to path group
        // add the tile path to the path group
        path_group = path_group.add(tile_path);

        // also add the line data to the the path data hash map with the corresponding rgb_str as the key
        path_data_hashmap.entry(rgb_str.to_owned())
                .and_modify(|v| v.push(line_data_clone)) // clone the line data for path data hash map
                .or_insert(vec![(line_data_clone1)]); // clone the line data for path data hash map
    
    } // for contig_group in &contiguous_tiles{

    // add the path group to the document
    document = document.add(path_group);

    let _ = create_laser_polygon_svg_doc(&path_data_hashmap, svg_width, svg_height, &op_svg_file_name);

    // println!("path_data_hashmap -> {:?}", path_data_hashmap);    
    // doo_eet();

    // Save each tile colour to a new svg doc just for that colour
    // Possible todos bin pack, add legend, and sizing box to each layer doc
    if create_laser_files {
        let _ = create_laser_svg_doc(&path_data_hashmap, svg_width, svg_height, &op_svg_file_name);
    } 
    
    // Output the complete SVG
    println!("Writing Mosaic to SVG output file {}", &op_svg_file_name);
    svg::save(op_svg_file_name, &document)   

    // end   travel_contig_svg
}

/// Creates an SVG document with the given path data and dimensions.
///
/// each line_data_element is converted into a svg pPolyline
/// 
/// The function takes a HashMap of path data, the width and height of the SVG, and an output file name.
/// The path data HashMap maps RGB color strings to vectors of path data strings. An output svg document
/// is created for each RGB colour. Each colour layer can then be separately cut on a laser cutter.
///
/// # Arguments
///
/// * `path_data_hashmap` - A reference to a HashMap containing the path data, where the key is an RGB color string and the value is a vector of path data strings.
/// * `svg_width` - The width of the SVG document.
/// * `svg_height` - The height of the SVG document.
/// * `op_svg_file_name` - A reference to a string containing the output SVG file name.
///
/// # Example
///
/// ```rust
/// let path_data: HashMap<String, Vec<String>> = ...
/// create_laser_svg_doc(&path_data, 4000, 4000, "output_file.svg");
/// ```
fn create_laser_polygon_svg_doc(path_data_hashmap: &HashMap<String, Vec<Data>>, svg_width: usize, svg_height: usize, op_svg_file_name: &str)
{
    let op_file_name : String = op_svg_file_name.trim_end_matches(".svg").to_string();
        
    // our tiles are 100x100 units  and we want each tile to be 1/2 in x 1/2.
    // Standard display is 96px per inch for so 1/2" us 48px,  
    // which makes are scale factor 0.48 * 100 = 48 
    let viewbox_width = svg_width as f32 * SVG_SCALE_X;
    let viewbox_height= svg_height as f32 * SVG_SCALE_Y;
    let mut count: i32= 1 ; 

    for (rgb_value_key, line_data_vec) in path_data_hashmap {

        let mut document: svg::node::element::SVG = Document::new()
                .set("viewBox", (0, 0, viewbox_width, viewbox_height));
        
        // convert all path PT units to MM units
        let converted_paths: Vec<Data> = convert_points_to_mm(line_data_vec);
        // println!("Converted paths {:?}", converted_paths);

        for line_data_elements in converted_paths {
        // for line_data_elements  in line_data_vec {

            // Extract points from the path
            let mut points = String::new();
            // println!("{:?} line data", &line_data_elements);

            line_data_elements.to_vec().into_iter().for_each(|el| {
                // println!("{:?}", &el);
            
                match el {
                    Command::Move(position, parameters) => {
                        // println!("MoveTo: {:?} {:?} ", position, parameters);
                        if position == Position::Absolute 
                        {
                            //  parameters.to_vec().into_iter().for_each(|p|{
                            //     println!("individual line -> {}", &p)
                            // } );
                            let pvec = parameters.to_vec();
                            // println!("individual line -> x{} y{}", &pvec[0], &pvec[1]);
                            points.push_str(&format!("{},{} ", &pvec[0], &pvec[1]));
                        }
                    }
                    Command::Line(position, parameters) => {
                        // println!("LineTo: BRRR {:?} {:?} ", position, parameters);
                        if position == Position::Absolute 
                        {
                            // parameters.to_vec().into_iter().for_each(|p|{
                            //     println!("individual line -> {}", &p)
                            // } );
                            let pvec = parameters.to_vec();
                            // println!("individual line -> x{} y{}", &pvec[0], &pvec[1]);
                            points.push_str(&format!("{},{} ", &pvec[0], &pvec[1]));
                        }
                    }
                    Command::HorizontalLine(_, _) =>{
                        println!("HorizontalLine.{:?}", &el);
                    },
                    Command::VerticalLine(_, _)=>{
                        println!("VerticalLine.{:?}", &el);
                    },
                    Command::QuadraticCurve(_, _)=>{
                        println!("QuadraticCurve.{:?}", &el);
                    },
                    Command::SmoothQuadraticCurve(_, _) =>{
                        println!("SmoothQuadraticCurve.{:?}", &el);
                    },
                    Command::CubicCurve(_, _) =>{
                        println!("CubicCurve.{:?}", &el);
                    },
                    Command::SmoothCubicCurve(_, _) =>{
                        println!("SmoothCubicCurve.{:?}", &el);
                    },
                    Command::EllipticalArc(_, _) =>{
                        println!("EllipticalArc.{:?}", &el);
                    },
                    Command::Close =>{
                        // SVG polygons don't have close element so do nothing wrt to points
                        // println!("Close")
                    },
                }
                // Handle other command types if needed
            });            

            // Create a SVG Polygon with the extracted points and rgb_value_key as the fill color
            let path_polygon: Polygon = Polygon::new()
                .set("points", points.trim())
                .set("fill", rgb_value_key.to_owned());

            // add the path polygon to the document
            document = document.add(path_polygon);
        }
    
        let legend_x:f32 = 0.0;
        let legend_y:f32 = svg_height as f32 * SVG_SCALE_Y + 5.0 ;

        // create a group to hold the legend
        let legend_group: svg::node::element::Group = get_svg_legend (&op_svg_file_name, rgb_value_key.clone(), legend_x , legend_y );

        // add the legend group to the document
        document = document.add(legend_group);

        let laser_rect_x :f32 = 0.0;
        let laser_rect_y :f32 = legend_y + 48.0 + 5.0 ; 
        let laser_rect_width :f32 = 12.0 * SVG_PPI  ; // arbitrary 12"x12" piece for packing
        let laser_rect_height :f32 = 12.0 * SVG_PPI ; // arbitrary 12"x12" piece for packing

        // create a deep_nest rect to represent laser material to be cut
         // Create the legend box (rectangle)
        let laser_material_rect = Rectangle::new()
        .set("x", laser_rect_x )
        .set("y", laser_rect_y )  // hard code to test
        .set("width", laser_rect_width) // Assuming 1 inch = 90 units
        .set("height", laser_rect_height)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1.0);

        // add the laser material rectangle to the document
        document = document.add(laser_material_rect);

        let op_svg_file_name = op_file_name.clone() + "_laser_poly" + &count.to_string() + ".svg"; 
            
        println!("Writing to laser polygon output file {}", &op_svg_file_name);
        svg::save(&op_svg_file_name, &document).expect("Error saving SVG file");

        count += 1;
    }

}

/// trying to take pixel units and convert everything directly to mm so we don't
/// have to muck around downstream with import export issues between various programs
/// such as Lightburn Inkscape DeepNest etc. Keep everything in absolute positions
/// with no transforms etc
/// 
/// At this point my code uses 100px by 100px grid and and each tile is to be 1/2" square
/// or 12.7mm square.  So to translate here "safely" we multilply each point by 0.48 to get
/// the correctly scaled point.
fn convert_points_to_mm(line_data_vec: &[Data]) -> Vec<Data> {
    
    let mut converted_ld_vec = vec![];

    for line_data_elements  in line_data_vec {

        // println!("{:?} line data", &line_data_elements);

        let mut line_d = Data::new();

        // Extract points from the path
        let mut points = String::new();
    
        line_data_elements.to_vec().into_iter().for_each(|el| {
            // println!("{:?}", &el);

            match el {
                Command::Move(position, parameters) => {
                    // println!("MoveTo: {:?} {:?} ", position, parameters);
                    if position == Position::Absolute 
                    {
                        let pvec = parameters.to_vec();
                        // println!("individual line -> x{} y{}", &pvec[0], &pvec[1]);
                        points.push_str(&format!("{},{} ", &pvec[0], &pvec[1]));

                        let mut x : f32 = pvec[0].try_into().unwrap();
                        let mut y : f32 = pvec[1].try_into().unwrap();

                        x = x * SVG_SCALE_X;
                        y = y * SVG_SCALE_Y;

                        line_d = line_d.clone().move_to((x, y));
                    }
                }
                Command::Line(position, parameters) => {
                    // println!("LineTo: BRRR {:?} {:?} ", position, parameters);
                    if position == Position::Absolute 
                    {
                        // parameters.to_vec().into_iter().for_each(|p|{
                        //     println!("individual line -> {}", &p)
                        // } );
                        let pvec = parameters.to_vec();
                        // println!("individual line -> x{} y{}", &pvec[0], &pvec[1]);
                        points.push_str(&format!("{},{} ", &pvec[0], &pvec[1]));

                        let mut x : f32 = pvec[0].try_into().unwrap();
                        let mut y : f32 = pvec[1].try_into().unwrap();

                        x = x * SVG_SCALE_X;
                        y = y * SVG_SCALE_Y;

                        line_d = line_d.clone().line_to((x, y));                      
                    }
                }
                Command::HorizontalLine(_, _) =>{
                    println!("HorizontalLine.{:?}", &el);
                },
                Command::VerticalLine(_, _)=>{
                    println!("VerticalLine.{:?}", &el);
                },
                Command::QuadraticCurve(_, _)=>{
                    println!("QuadraticCurve.{:?}", &el);
                },
                Command::SmoothQuadraticCurve(_, _) =>{
                    println!("SmoothQuadraticCurve.{:?}", &el);
                },
                Command::CubicCurve(_, _) =>{
                    println!("CubicCurve.{:?}", &el);
                },
                Command::SmoothCubicCurve(_, _) =>{
                    println!("SmoothCubicCurve.{:?}", &el);
                },
                Command::EllipticalArc(_, _) =>{
                    println!("EllipticalArc.{:?}", &el);
                },
                Command::Close =>{
                    // println!("Close" );
                    line_d = line_d.clone().close();
                },
            }
            // Handle other command types if needed
        });            

        converted_ld_vec.push(line_d);    
        }

        converted_ld_vec
}


/// Creates an SVG document with the given path data and dimensions.
///
/// The function takes a HashMap of path data, the width and height of the SVG, and an output file name.
/// The path data HashMap maps RGB color strings to vectors of path data strings. An output svg document
/// is created for each RGB colour. Each colour layer can then be separately cut on a laser cutter.
///
/// # Arguments
///
/// * `path_data_hashmap` - A reference to a HashMap containing the path data, where the key is an RGB color string and the value is a vector of path data strings.
/// * `svg_width` - The width of the SVG document.
/// * `svg_height` - The height of the SVG document.
/// * `op_svg_file_name` - A reference to a string containing the output SVG file name.
///
/// # Example
///
/// ```rust
/// let path_data: HashMap<String, Vec<String>> = ...
/// create_laser_svg_doc(&path_data, 4000, 4000, "output_file.svg");
/// ```
fn create_laser_svg_doc(path_data_hashmap: &HashMap<String, Vec<Data>>, svg_width: usize, svg_height: usize, op_svg_file_name:&str) 
{
    //  "./svg_output/twelveXtwelve/fy_laser_org".to_string();   
    let op_file_name : String = op_svg_file_name.trim_end_matches(".svg").to_string();
 
    // our tiles are 100x100 units  and we want each tile to be 1/2 in x 1/2.
    // Standard display is 96px per inch for so 1/2" us 48px,  
    // which makes are scale factor 0.48 * 100 = 48 
    let viewbox_width = svg_width as f32 * SVG_SCALE_X;
    let viewbox_height= svg_height as f32 * SVG_SCALE_Y;

    let mut count: i32= 1 ; 

    for (rgb_value_key, line_data_vec) in path_data_hashmap {

        let mut document: svg::node::element::SVG = Document::new()
                .set("viewBox", (0, 0, viewbox_width, viewbox_height));
                // .set("width", viewBoxWidth)
                // .set("height", viewBoxHeight);
                // .set("transform", format!("scale({}, {})", scale_x, scale_y));
        
        // create a path group with name of that rgb value
        let mut path_group = svg::node::element::Group::new()
                .set("id", rgb_value_key.clone())
                .set("fill", rgb_value_key.to_owned())
                .set("stroke", rgb_value_key.to_owned())
                .set("stroke-width", 0.0);

        // convert all path Point Units to MM units
        let converted_paths: Vec<Data> = convert_points_to_mm(line_data_vec);
        // println!("Converted paths {:?}", converted_paths);

        // using converted line data elements now
        for line_data_element in converted_paths {
            
            // create a path and add it to the svg document
            let tile_path = Path::new().set("d", line_data_element.to_owned());

            // as we are grouping we don't add fill, stroke and stroke_width and to each path
            // create a path and add it to the svg document
            // let tile_path = Path::new()
            //     .set("fill", rgb_value_key.to_owned())
            //     .set("stroke", rgb_value_key.to_owned())
            //     .set("stroke-width", 0.0)
            //     .set("d", line_data_element.to_owned());

            // add the tile path to the path group
            path_group = path_group.add(tile_path);
        }

        let op_svg_file_name = op_file_name.clone() + "_lry" + &count.to_string() + ".svg"; 

        // add the path group to the document
        document = document.add(path_group);
        
        let legend_x:f32 = 0.0;
        let legend_y:f32 = svg_height as f32 * SVG_SCALE_Y + 5.0 ;

        // create a group to hold the legend
        let legend_group: svg::node::element::Group = get_svg_legend (&op_svg_file_name, rgb_value_key.clone(), legend_x , legend_y );

        // add the legend group to the document
        document = document.add(legend_group);

        let laser_rect_x :f32 = 0.0;
        let laser_rect_y :f32 = legend_y + 48.0 + 5.0 ; 
        let laser_rect_width :f32 = 12.0 * SVG_PPI ; // arbitrary 12"x12" piece for packing
        let laser_rect_height :f32 = 12.0 * SVG_PPI; // arbitrary 12"x12" piece for packing

        // create a deep_nest rect to represent laser material to be cut
         // Create the legend box (rectangle)
        let laser_material_rect = Rectangle::new()
        .set("x", laser_rect_x )
        .set("y", laser_rect_y )  // hard code to test
        .set("width", laser_rect_width) // Assuming 1 inch = 90 units
        .set("height", laser_rect_height)
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1.0);

        // add the laser material rectangle to the document
        document = document.add(laser_material_rect);

        println!("Writing to laser paths output file {}", &op_svg_file_name);
        svg::save(&op_svg_file_name, &document).expect("Error saving SVG file");

        count += 1;
    }

}

fn get_svg_legend(op_svg_file_name: &str, rgb_value_key: String, x_loc :f32, y_loc:f32) -> svg::node::element::Group {
    
    let mut legend_group = svg::node::element::Group::new().set("id", "legend");
    
    let legend_file_name_str = op_svg_file_name.rsplit('/').next().unwrap_or(op_svg_file_name);

    // Create the legend box (rectangle)
    let legend_box = Rectangle::new()
        .set("x", x_loc)
        .set("y", y_loc)
        .set("width", "96") // Assuming 1 inch = 90 units
        .set("height", "48")
        .set("fill", "none")
        .set("stroke", "black");

    // Add the text inside the legend box
    let legend_text_file = Text::new()
        .set("x", x_loc + 5.0) // Add some padding
        .set("y", y_loc + 20.0) // Adjust the y position to center the text
        .set("font-size", 10)
        .set("font-family", "Arial")
        .add(svg::node::Text::new(legend_file_name_str));
    
    // Add the text inside the legend box
    let legend_text_rgb = Text::new()
        .set("x", x_loc + 5.0) // Add some padding
        .set("y", y_loc + 30.0) // Adjust the y position to center the text
        .set("font-size", 10)
        .set("font-family", "Arial")
        .add(svg::node::Text::new(rgb_value_key));

    legend_group = legend_group.add(legend_box).add(legend_text_file).add(legend_text_rgb);

    legend_group
}


/*
// START BLOCK
// START BLOCK

use rosvgtree::AttributeId;
use rosvgtree::svgtypes::Paint;
use rosvgtree::{AttributeId, Attribute};
use usvg::{PathData, PathSegment, Rect};
use regex::Regex;

/// Examines an SVG file to understand the rosvgtree module and extract relevant information.
///
/// This function reads an SVG file and creates a `rosvgtree::Document` from it. It then explores
/// the document structure and extracts information such as fill color, single tile count, and
/// single tile occurrences in the SVG. The function also demonstrates the use of regular expressions
/// to match specific patterns in the SVG data.
///
/// Meant as an exercise to figure out how rosvgtree module works
/// # Example
///
/// ```rust
/// doo_eet();
/// ```
fn _doo_eet() {

    // let input = std::fs::read_to_string("./svg_output/twoXtwo/output_7.svg").unwrap();        
    let input = std::fs::read_to_string("./svg_output/twelveXtwelve/frank_mar15.svg").unwrap();        
    let rosvg_doc: rosvgtree::Document = rosvgtree::Document::parse_str(&input).unwrap();

    println!("rosvgtree -> {:?}", &rosvg_doc);

    let root = &rosvg_doc.root();
    println!(" root -> {:?}", &root);

    let mut single_tile_count: i32 = 0; 

    // let mut single_tile_hashmap: HashMap<String,i32> = HashMap::new();

    // let mut single_svg_tiles: HashMap<modtile::RGB, i32> = HashMap::new();
    let mut single_svg_tiles: HashMap<String, i32> = HashMap::new();
    // for (_i, tile) in pane.iter().enumerate() {
    //     let tile_rgb = tile.1;
    //     *single_svg_tiles.entry(tile_rgb).or_insert(0) += 1;
    // }

    for descendant in rosvg_doc.descendants(){
        
        let tag_name = descendant.tag_name();
        // println!("\ndescendant {:?}" , &descendant);
        println!("\ndescendant {:?}" , &tag_name);

        // let attributes: &[rosvgtree::Attribute] = descendant.attributes();
        // println!("\ndec attributes {:?}" , &attributes);

        match descendant.has_attribute(AttributeId::Fill) {
            true => {
                println!("\tdec has a fill attribute" ); 

                // match rosvgtree::Node::attribute::<Paint>(&descendant, AttributeId::Fill) {    
                //     Some(fill_color) => {
                //         println!("fill color: {:?}", fill_color);
                //     }
                //     None => {
                //         println!("fill attribute is not a valid color");
                //     }
                // }

                match rosvgtree::Node::attribute::<&str>(&descendant, AttributeId::Fill) {    
                    Some(fill_color_str) => {
                        println!("\tfill color string: {:?}", fill_color_str);
                        *single_svg_tiles.entry(fill_color_str.to_owned()).or_insert(0) += 1;
                    }
                    None => {
                        println!("\tfill attribute is not a valid color");
                    }
                }

                // let regex_str :String = "Attribute \\{ name: d, value: M\\d{1,5},\\d{1,5} L\\d{1,5},\\d{1,5} L\\d{1,5},\\d{1,5} L\\d{1,5},\\d{1,5} L\\d{1,5},\\d{1,5} z \\}".to_owned();
                let regex_str :String = "^M\\d{1,5},\\d{1,5} L\\d{1,5},\\d{1,5} L\\d{1,5},\\d{1,5} L\\d{1,5},\\d{1,5} L\\d{1,5},\\d{1,5} z".to_owned();
                let pattern = Regex::new(&regex_str).unwrap();

                // grab the d attribute and regex match for single tile
                match rosvgtree::Node::attribute::<&str>(&descendant, AttributeId::D) {    
                    Some(data) => {
                        println!("data: {:?}", &data);

                        // Perform a regular expression match on the data string
                        match pattern.find(&data) {
                            Some(matched) => {
                                // println!("\tSingle Tile Match found: {:?}", matched.as_str());
                                println!("\tSingle Tile Match found:");
                                single_tile_count  += 1;
                            }
                            None => {
                                println!("\tSingle Tile Match Not found");
                            }
                        }

                    }
                    None => {
                        println!("no valid data attribute found");
                    }
                }

            }
            // false => (),
            false => {
                println!("\ndec does not have a fill attribute" );   
            },
        }
    
    }
    println!("\n {} <- Single Tile Count in mosaic", single_tile_count);
    println!("Single Tiles -> {:?}", &single_svg_tiles);

    test_trans_bound();

}


/// Translates the given path data by a specified amount and computes its bounding box.
///
/// This function takes a path data string, and starting x and y coordinates as input. It then
/// translates the path data to the specified coordinates and computes its axis-aligned
/// bounding box.
///
/// # Arguments
///
/// * `path_data` - The input path data as a string.
/// * `startx` - The starting x coordinate to translate the path to.
/// * `starty` - The starting y coordinate to translate the path to.
///
/// # Returns
///
/// An `Option` containing a tuple of the translated `PathData` and the computed `Rect` (bounding box),
/// or `None` if the path data cannot be processed.
///
/// # Example
///
/// ```rust
/// let path_data_str = "M 10,10 L 100,10 L 100,100 L 10,100 Z";
/// let result = translate_path_and_compute_bounding_box(path_data_str, 20.0, 20.0);
/// if let Some((translated_path_data, bounding_box)) = result {
///     println!("Translated path data: {:?}", translated_path_data);
///     println!("Bounding box: {:?}", bounding_box);
/// }
/// ```
fn translate_path_and_compute_bounding_box(path_data: &str, startx: f64, starty: f64) -> Option<(PathData, Rect)> {
    
    let mut path_data: PathData = PathData::new(); 
    // {
    //     Ok(data) => data,
    //     Err(_) => return None,
    // };
    let x:f64 = 10.0;
    let y:f64 = 10.0;
    PathData::push_move_to(&mut path_data, x, y);
    PathData::push_line_to(&mut path_data, 100.0, 10.0 );
    PathData::push_line_to(&mut path_data, 100.0, 100.0 );
    PathData::push_line_to(&mut path_data, 10.0, 100.0 );
    PathData::push_close_path(&mut path_data);

    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for segment in path_data.segments() {
        let (x, y) = match segment {
            PathSegment::MoveTo { x, y } | PathSegment::LineTo { x, y } => (x, y),
            _ => continue,
        };

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    let mut translated_path_data: PathData = PathData::new();
    let mut prev_x = min_x;
    let mut prev_y = min_y;

    for segment in path_data.segments() {
        match segment {
            PathSegment::MoveTo { x, y } => {
                let new_x = x - min_x + startx;
                let new_y = y - min_y + starty;

                // translated_path_data.push(PathSegment::MoveTo { x: new_x, y: new_y });
                PathData::push_move_to(&mut translated_path_data,  new_x , new_y);
                prev_x = new_x;
                prev_y = new_y;
            }
            PathSegment::LineTo { x, y } => {
                let new_x = x - prev_x;
                let new_y = y - prev_y;

                // translated_path_data.push(PathSegment::LineTo { x: new_x, y: new_y });
                PathData::push_line_to(&mut translated_path_data,  new_x , new_y);
                prev_x = x - min_x + startx;
                prev_y = y - min_y + starty;
            }
            // other => translated_path_data.push(other),
            other => PathData::push_close_path(&mut translated_path_data),
        }
    }

    if min_x.is_infinite() || min_y.is_infinite() || max_x.is_infinite() || max_y.is_infinite() {
        None
    } else {
        let bounding_box = Rect::new(startx, starty, max_x - min_x, max_y - min_y).unwrap();
        Some((translated_path_data, bounding_box))
    }
}


fn test_trans_bound() {
    let path_data = "M500,4000 L400,4000 L400,3900 L500,3900 L600,3900 L600,4000 L500,4000 z";
    let startx = 10.0;
    let starty = 20.0;
    if let Some((translated_path_data, bounding_box)) = translate_path_and_compute_bounding_box(path_data, startx, starty) {
        println!("Translated path data: {:?}", translated_path_data);
        println!("Bounding box: {:?}", bounding_box);
    } else {
        println!("Failed to translate path data and compute bounding box");
    }
}

// END BLOCK
// END BLOCK
 */

// use usvg::{roxmltree, Tree, NodeKind};
// fn sort_paths(document: &svg::node::element::SVG)
// {
//     println!(" %^%^%^%");

//     let opt = roxmltree::ParsingOptions { allow_dtd: true, ..roxmltree::ParsingOptions::default() };

//     let binding = document.to_string();
//     let doc = match roxmltree::Document::parse_with_options(&binding, opt) {
//         Ok(doc) => doc,
//         Err(e) => {
//             println!("Error: {}.", e);
//             panic!();
//         }
//     };

//     // let tree = Tree::from_file(file_path, &usvg::Options::default()).map_err(|e| e.to_string())?;
//     // let tree = Tree::from_xmltree(&doc, &usvg::Options::default()).map_err(|e| e.to_string())?;
//     // let tree = Tree::from_xmltree(&doc, &usvg::Options::default()).map_err(|e| e.to_string());
//     let tree = Tree::from_xmltree(&doc, &usvg::Options::default()).map_err(|e| e.to_string()).unwrap();

//     iterate_svg_elements(&tree);
//     // println!("svg doc tree - > {}", &tree);
// }

// fn iterate_svg_elements(tree: &usvg::Tree) {
//     let root = &tree.root;
//     traverse_node(&root);
// }



// fn traverse_node(node: &usvg::Node) {
//     match &node. {
//         NodeKind::Element(ref elem) => {
//             println!("Element: {:?}", elem);
//         }
//         _ => {}
//     }

//     for child in node.children() {
//         traverse_node(&child);
//     }
// }




// given the first tile incomplete tile return the first start point and tile end 
fn get_incomplete_tile_info(tile: &MosaicTile, 
                            visited_tiles: &ArrayBase<OwnedRepr<TileVisited>, Dim<[usize; 2]>>, 
                            index: &(isize, isize)) -> ((i32, i32), euclid::Point2D<i32, euclid::UnknownUnit>) 
{

    let start_xy : (i32,i32); 
    let curr_svg_line_end_point: euclid::Point2D<i32, euclid::UnknownUnit>;

    let row = index.0 as usize;
    let col = index.1 as usize;

    if tile.is_ftft(){
        println!("tile.is_ftft()");

        let vbool = visited_tiles[[row,col]].edge_visited.clone();
        let match_top_visited = [Some(true), None, None, None];        
        let top_edge_visited = match_edge_boolean_pattern(match_top_visited, &vbool);
        println!("----- match_top_visited = {:?}" , &top_edge_visited);
        
        let match_bot_visited = [None, None, Some(true), None ];        
        let bot_edge_visited = match_edge_boolean_pattern(match_bot_visited, &vbool);
        println!("----- bot_edge_visited = {:?}" , &bot_edge_visited);
        
        if !top_edge_visited   // Bottom edge must be the incomplete edge
        {  
            println!("top_edge_visited == false");
            start_xy = tile.get_start_point_as_i32();
            curr_svg_line_end_point = tile.end_point;
        } 
        else if !bot_edge_visited // Top edge must be the incomplete edge
        {        
            println!("bot_edge_visited == false");
            start_xy = tile.get_start_point_two_as_i32();
            curr_svg_line_end_point = tile.end_point_two;
        } else {
            println!("Houston we have a problem");
            // commented out below two line to remove compiler warning
            // start_xy = tile.get_start_point_two_as_i32();
            // curr_svg_line_end_point = tile.end_point_two;
            panic!();
        }                    
    }
    else if tile.is_tftf(){

        println!("tile.is_tftf()");

        let vbool = visited_tiles[[row,col]].edge_visited.clone();
        let match_right_visited = [None,Some(true),None, None];        
        let right_edge_visited = match_edge_boolean_pattern(match_right_visited, &vbool);
        println!("----- match_right_visited = {:?}" , &right_edge_visited);
        
        let match_left_visited = [None, None, None, Some(true)];        
        let left_edge_visited = match_edge_boolean_pattern(match_left_visited, &vbool);
        println!("----- left_edge_visited = {:?}" , &left_edge_visited);
        
        if !left_edge_visited   // Right edge must be the incomplete edge
        {  
            println!("left_edge_visited");
            start_xy = tile.get_start_point_two_as_i32();
            curr_svg_line_end_point = tile.end_point_two;
        } 
        else if !right_edge_visited // Left edge must be the incomplete edge
        {        
            println!("right_edge_visited");
            start_xy = tile.get_start_point_as_i32();
            curr_svg_line_end_point = tile.end_point;
        } else {
            println!("Houston we have a problem");
            // commented out below two line to remove compiler warning
            // start_xy = tile.get_start_point_two_as_i32();
            // curr_svg_line_end_point = tile.end_point_two;
            panic!();
        }                    

    } else {
        // for non FTFT or TFTF tiles 
        // just move to start location of tile and set the curr_svg_line_end_point to the tile end point
        start_xy  = tile.get_start_point_as_i32();
        curr_svg_line_end_point = tile.end_point;

        return (start_xy,curr_svg_line_end_point)
    }

    
    return (start_xy,curr_svg_line_end_point)
    
}


// ****************************** */
// ****************************** */

/// Find the next tile based on the end point of one tile is the start point of the next tile
/// Note tiles must reside in the same contiguous group
fn find_next_tile_ext(curtile_row: usize, 
    curtile_col: usize, 
    cur_tile: &MosaicTile, 
    _contig_group: &[(isize, isize)],
    adjacent_map: &HashMap<(isize, isize), Vec<(isize, isize)>>, 
    pane_edge_nd_arr: &ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>>,
    visited_tiles: &ArrayBase<OwnedRepr<TileVisited>, Dim<[usize; 2]>>) -> (usize,usize) 
{
    println!("\n******************\nfn find_next_tile for [{},{}]\n******************", curtile_row, curtile_col);
    println!( "cur_tile row {}\ncur_tile col {} \n\ncur_tile {:?}\n", curtile_row, curtile_col, cur_tile ); 
    println!("******************************************");
    println!("******************************************\n");

    println!( "visited_bool [{},{}] -> {:?}", curtile_row, curtile_col, visited_tiles[[curtile_row,curtile_col]] ); 

    let mut contig_row: usize; 
    let mut contig_col: usize; 

    // a bad way to program but if this routine completes and a next tile has not 
    // been found then return (FLAGGED,FLAGGED) where pub const FLAGGED: usize = 987659; 
    // which will be the signal to panic
    // Look into returning a Result in the future
    let mut res = (FLAGGED,FLAGGED);


    // // this is a working snippet
    // if let Some(adjacents) = adjacent_map.get(&(0,2)) {
    //     println!("878 Adjacent tiles for location (0, 2): {:?}", adjacents);
    //     for tile in adjacents {
    //         // println!("880 : tile {},{}", &tile.0 as usize, &tile.1 as usize);
    //         println!("880 : tile {},{}", &tile.0 , &tile.1 );
    //     }

    // } else {
    //     println!("879 Nothing for Adjacent tiles for location (0, 2)");
    // }


// // /* 
// // 8888 
// // Another working snippet - want to embed logic below into this
// // Getting Keys and not Values 
//
//     for adjacent_tiles in adjacents
//     {

//         if let Some(adjacent_tiles) = adjacent_tiles.iter().map(|v| Some(v.clone())).next() {
//             // do something with `adjacent_tiles`
//             for (row, col) in adjacent_tiles {
//                 // do something with `row` and `col`
//                 println!(" 950  Row: {}, Col: {}", row, col);
//             }
//         } else {
//             println!(" 950 The `adjacent_tiles` vector is empty or `None`.");
//         }
        
//         // end for adjacent_tiles in adjacents
//     }
// // 8888 */



    // instead of looking through tiles in contig_group 
    // just look at the adjacent tiles for curtile_row and curtile_col
    // let adjacents = adjacent_map.get(&(curtile_row.try_into().unwrap(), curtile_col.try_into().unwrap()));

    // this is a working snippet
    // instead of looking through tiles in contig_group 
    // just look at the adjacent tiles for curtile_row and curtile_col
    if let Some(adjacents) = adjacent_map.get(&(curtile_row.try_into().unwrap(), curtile_col.try_into().unwrap()))
    {
        println!("878 Adjacent tiles for location ({},{}): {:?}",curtile_row, curtile_col,adjacents);
        for tile in adjacents {
            // println!("880 : tile {},{}", &tile.0 as usize, &tile.1 as usize);
            println!("880 : tile {},{}", &tile.0 , &tile.1 );

                println!("---------------- ");
                println!("---- tile {:?})", &tile);
                println!("---------------- ");
        
    
                    // println!("890 for adjacent_tiles.iter (row, col) -> ({},{})", &row, &col);
                    // if let Some((row, _)) = tiles {
                    //     contig_row = row as usize;
                    // }
    
                    // if let Some((col, _)) = tiles {
                    //     contig_col = col as usize;
                    // }
    
                    contig_row = tile.0 as usize;
                    contig_col = tile.1 as usize;
                    println!(" 950  Row: {}, Col: {}", &contig_row, &contig_col);
    
                    println!("960");    
                    // contig_row = *&contig_tile.0 as usize;
                    // contig_col = *&contig_tile.1 as usize;
                    // contig_row = adjacent_tile[0].0 as usize;
                    // contig_col = adjacent_tile[0].1 as usize;
            
                    let tile_prev_visited = visited_tiles[[contig_row,contig_col]].visited();
                    if tile_prev_visited {
                        println!("We've visited tile [{},{}]", &contig_row , &contig_col);
                    } else {
                        println!("We've NOT visited tile [{},{}]", &contig_row , &contig_col);
                    }

                    let cur_tile_top_visited = visited_tiles[[curtile_row, curtile_col]].edge_visited[TOP];
                    let cur_tile_right_visited = visited_tiles[[curtile_row, curtile_col]].edge_visited[RIGHT];
                    let cur_tile_bot_visited = visited_tiles[[curtile_row, curtile_col]].edge_visited[BOTTOM];
                    let cur_tile_left_visited = visited_tiles[[curtile_row, curtile_col]].edge_visited[LEFT];
        
                    if !tile_prev_visited
                    {
                        let check_tile: MosaicTile = pane_edge_nd_arr[[contig_row,contig_col]].clone();

                        // get the edge visited booleans.
                        let check_tile_top_visited = visited_tiles[[contig_row, contig_col]].edge_visited[TOP];
                        let check_tile_right_visited = visited_tiles[[contig_row, contig_col]].edge_visited[RIGHT];
                        let check_tile_bot_visited = visited_tiles[[contig_row, contig_col]].edge_visited[BOTTOM];
                        let check_tile_left_visited = visited_tiles[[contig_row, contig_col]].edge_visited[LEFT];
    
                        let cur_tile_is_tftf :bool = cur_tile.is_tftf();
                        let cur_tile_is_ftft :bool = cur_tile.is_ftft();
                        let check_tile_is_tftf:bool = check_tile.is_tftf();
                        let check_tile_is_ftft:bool = check_tile.is_ftft();
            
                        println!("match find_next_tile_ext "); 
                        println!("(cur_tile check_tile is_tftf() is_ftft()) "); 
            
                        println!("\n\tCurrent tile TFTF -> {}", &cur_tile_is_tftf );
                        println!("\tCurrent tile FTFT -> {}", &cur_tile_is_ftft);
                        println!("\n\tCheck tile TFTF -> {}", &check_tile_is_tftf);
                        println!("\tCheck tile is FTFT -> {}\n", &check_tile_is_ftft);
            
                        // if cur_tile is TFTF or FTFT then we need to determine which line endpoint
                        // has the current svg_line_end_point that we're drawing from.
                        // otherwise just use cur_tile.end_point to match with check_tile
                        // or
                        // if check tile is TFTF or FTFT then we need to determine which line start point
                        // matches the previous line end point and use this to find the next tile
            
                        match (cur_tile_is_tftf, cur_tile_is_ftft, check_tile_is_tftf ,check_tile_is_ftft) 
                        {
                            // just process tiles as regular tiles
                            // cur_tile NOT tftf 
                            // cur_tile NOT ftft, 
                            // check_tile NOT tftf 
                            // check_tile NOT ftft
                            (false, false, false, false) => {
                                println!(" ----- 1 ------- cur_tile/check_tile not TFTF or FTFT ");
                                if check_tile.start_point == cur_tile.end_point {
            
                                    println!(" ----- 1a ------- cur_tile and check_tile both not TFTF or FTFT ");
                                    println!(" ----- 1a Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 1a {:?} <- check_tile.start_point == cur_tile.end_point", &cur_tile.end_point );
                        
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                // end match false, false, false, false) 
                            }
                            // cur_tile NOT tftf 
                            // cur_tile NOT ftft, 
                            // check_tile NOT tftf 
                            // check_tile IS ftft
                            (false, false, false, true) => {
                                println!(" ----- 2 -------- FTFT check_tile only");   
                                if cur_tile.end_point == check_tile.start_point
                                {   
                                    println!(" ----- 2a ------- FTFT ");
                                    println!(" ----- 2a Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 2a {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
                            
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                else if cur_tile.end_point == check_tile.start_point_two
                                {
                                    println!(" ----- 2b ------- FTFT ");
                                    println!(" ----- 2b Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 2b {:?} <- cur_tile.end_point == check_tile.start_point_two", &cur_tile.end_point );
            
                                    res = (contig_row,contig_col);
                                    break;
            
                                } else {
                                    println!(" ----- 2c ------- FTFT ");
                                    println!(" ----- 2c no match - keep looking");
                                }                    
                                // end match false, false, false, true) 
                            }
                            // cur_tile NOT tftf 
                            // cur_tile NOT ftft, 
                            // check_tile IS tftf 
                            // check_tile NOT ftft
                            (false, false, true, false) => {
                                println!(" ----- 3 -------- TFTF check_tile only");
                                println!( "----- 3   check_tile Tile visited edges visited_bool [{},{}] -> {:?}", contig_row, contig_col, visited_tiles[[contig_row,contig_col]] ); 
                                
                                println!("----- 3 check_tile_left_visited = {:?}" , &check_tile_left_visited);
                                println!("----- 3 check_tile_right_visited = {:?}" , &check_tile_right_visited);
                                
                                // first time through this tile
                                if !check_tile_left_visited && !check_tile_right_visited &&
                                    cur_tile.end_point == check_tile.start_point 
                                {
                                        println!(" ----- 3a ------- TFTF check_tile only");
                                        println!(" ----- 3a Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                        println!(" ----- 3a {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
                
                                        res = (contig_row,contig_col);
                                        break;
                                } else if !check_tile_left_visited && !check_tile_right_visited &&
                                    cur_tile.end_point == check_tile.start_point_two
                                {
                                            println!(" ----- 3b ------- TFTF check_tile only");
                                            println!(" ----- 3b Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                            println!(" ----- 3b {:?} <- cur_tile.end_point == check_tile.start_point_two", &cur_tile.end_point );
                    
                                            res = (contig_row,contig_col);
                                            break;
                                }
                                 else if check_tile_left_visited && !check_tile_right_visited && 
                                                cur_tile.end_point == check_tile.start_point_two
                                {  
                                    println!(" ----- 3c -------TFTF check_tile only");
                                    println!(" ----- 3c Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 3c {:?} <- check_tile_left_visited && cur_tile.end_point == check_tile.start_point_two", &cur_tile.end_point );
            
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                else if !check_tile_left_visited && check_tile_right_visited && 
                                            cur_tile.end_point == check_tile.start_point
                                {
                                    println!(" ----- 3d ------- TFTF check_tile only");
                                    println!(" ----- 3d Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 3d {:?} <- check_tile_right_visited && cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
                                    
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                // else if right_edge_visited && left_edge_visited // 
                                else if check_tile_right_visited && check_tile_left_visited  
                                {
                                    println!(" ----- 3e ------- TFTF check_tile only");
                                    println!(" ----- 3e Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 3e {:?} <- check_tile_right_visited && check_tile_left_visited both true ", &cur_tile.end_point );
                                    
                                    // TODO REVIEW main test for let _ = twenty5_tile_square::svg_4(); 
                                    // 8x10  mosaic with 
                                    // Last tile has both Left and Right edges visited.
                                    // Perhaps we need to find next tile before setting the edges?
                                    println!(" ----- 3e Not sure why this works - really need to have a look at this logic");
                                    panic!();
                                    // res = (contig_row,contig_col);
                                    // break;
                                }
                                else {
                                    println!(" ----- 3f ------- TFTF check_tile only");
                                    println!(" ----- 3f no match - keep looking");
                                }
                                // end match false, false, true, false)                     
                            }
                            // cur_tile NOT tftf 
                            // cur_tile IS ftft, 
                            // check_tile NOT tftf 
                            // check_tile NOT ftft
                            (false, true, false, false) => {
                                println!(" ----- 4 -------- FTFT cur_tile only");   
                                if cur_tile_top_visited && !cur_tile_bot_visited && 
                                    cur_tile.end_point == check_tile.start_point
                                {  
                                    println!(" ----- 4a ------- FTFT ");
                                    println!(" ----- 4a Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 4a {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
            
                                    res = (contig_row,contig_col);
                                    break;
                                } else 
                                if !cur_tile_top_visited && cur_tile_bot_visited && 
                                    cur_tile.end_point_two == check_tile.start_point
                                {  
                                    println!(" ----- 4b ------- FTFT ");
                                    println!(" ----- 4b Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 4b {:?} <- cur_tile.end_point_two == check_tile.start_point", &cur_tile.end_point_two );
            
                                    res = (contig_row,contig_col);
                                    break;
                                } else 
                                if cur_tile_top_visited && cur_tile_bot_visited && 
                                    cur_tile.end_point == check_tile.start_point
                                {  
                                    println!(" ----- 4c ------- FTFT ");
                                    println!(" ----- 4c Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 4c {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
            
                                    res = (contig_row,contig_col);
                                    break;
                                }
                                else 
                                if cur_tile_top_visited && cur_tile_bot_visited && 
                                    cur_tile.end_point_two == check_tile.start_point
                                {  
                                    println!(" ----- 4d ------- FTFT ");
                                    println!(" ----- 4d Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 4d {:?} <- cur_tile.end_point_two == check_tile.start_point", &cur_tile.end_point_two );
            
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                else {
                                    println!(" ----- 4e ------- FTFT ");
                                    println!(" ----- 4e no match - keep looking");
                                }                    
                            }
                            // cur_tile NOT tftf 
                            // cur_tile IS ftft, 
                            // check_tile NOT tftf 
                            // check_tile IS ftft
                            (false, true, false, true) => {
                                // We want to find next tile for a visited edge only
                                // otherwise we will get a match for two adjacent FTFT tiles
                                // NOTE THAT at least one of TOP or BOT MUST be visited otherwise we cannot be looking for the next tile 
                                // for this tile
                                println!(" ----- 5 -------- FTFT cur_tile FTFT check_tile");   
                                if cur_tile_top_visited && !cur_tile_bot_visited && 
                                    cur_tile.end_point == check_tile.start_point
                                {  
                                    println!(" ----- 5a ------- FTFT ");
                                    println!(" ----- 5a Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 5a {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
            
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                else if !cur_tile_top_visited && cur_tile_bot_visited && 
                                    cur_tile.end_point_two == check_tile.start_point_two
                                {
                                    println!(" ----- 5b ------- FTFT ");
                                    println!(" ----- 5b Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 5b {:?} <- cur_tile.end_point_two == check_tile.start_point_two", &cur_tile.end_point );
                                    
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                else if cur_tile_top_visited && cur_tile_bot_visited && 
                                        check_tile_top_visited && !check_tile_bot_visited &&
                                    cur_tile.end_point_two == check_tile.start_point_two
                                {
                                    println!(" ----- 5c ------- FTFT ");
                                    println!(" ----- 5c Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 5c {:?} <- cur_tile.end_point_two == check_tile.start_point_two", &cur_tile.end_point );
                                    
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                else if cur_tile_top_visited && cur_tile_bot_visited && 
                                        !check_tile_top_visited && check_tile_bot_visited &&
                                    cur_tile.end_point == check_tile.start_point
                                {
                                    println!(" ----- 5c ------- FTFT ");
                                    println!(" ----- 5c Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 5c {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
                                    
                                    res = (contig_row,contig_col);
                                    break;
                                }                                 
                                else {
                                    println!(" ----- 5d ------- FTFT ");
                                    println!(" ----- 5d no match - keep looking");
                                }                    
                                // end match false, true, false, true) 
                            }
                            // cur_tile NOT tftf 
                            // cur_tile IS ftft, 
                            // check_tile IS tftf 
                            // check_tile NOT ftft
                            (false, true, true, false) => {
                                println!(" ----- 6 -------- FTFT cur_tile TFTF check_tile");   
            
                                // let vbool = visited_tiles[[curtile_row,curtile_col]].edge_visited.clone();
                                // let match_top_visited = [Some(true), None, None, None];        
                                // let top_edge_visited = match_edge_boolean_pattern(match_top_visited, &vbool);
                                // println!("----- 6 top_edge_visited = {:?}" , &top_edge_visited);
            
                                // let match_bot_visited = [None, None, Some(true), None ];        
                                // let bot_edge_visited = match_edge_boolean_pattern(match_bot_visited, &vbool);
                                // println!("----- 6 bot_edge_visited = {:?}" , &bot_edge_visited);
            
                                println!("----- 6 cur_tile_top_visited = {:?}" , &cur_tile_top_visited);
                                println!("----- 6 cur_tile_bot_visited = {:?}" , &cur_tile_bot_visited);

                                // if top_edge_visited  && cur_tile.end_point == check_tile.start_point
                                if cur_tile_top_visited && cur_tile.end_point == check_tile.start_point
                                {  
                                    println!(" ----- 6a ------- FTFT/TFTF ");
                                    println!(" ----- 6a Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 6a {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
            
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                // else if bot_edge_visited && cur_tile.end_point_two == check_tile.start_point_two
                                else if cur_tile_bot_visited && cur_tile.end_point_two == check_tile.start_point_two
                                {
                                    println!(" ----- 6b ------- FTFT/TFTF ");
                                    println!(" ----- 6b Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 6b {:?} <- cur_tile.end_point_two == check_tile.start_point_two", &cur_tile.end_point );
                                    
                                    res = (contig_row,contig_col);
                                    break;
                                } else {
                                    println!(" ----- 6c ------- FTFT/TFTF ");
                                    println!(" ----- 6c no match - keep looking");
                                }                    
                                // end match false, true, true, false) 
                            }
                            // cur_tile IS tftf 
                            // cur_tile NOT ftft, 
                            // check_tile NOT tftf 
                            // check_tile NOT ftft
                            (true, false, false, false) => {
                                // there is an issue here in that there are two possible paths to draw out for this cur_tile
                                // so it can possibly match two tiles.
                                println!(" ----- 7 --------- TFTF cur_tile only");
                                println!( "----- 7   Cur Tile visited edges visited_bool [{},{}] -> {:?}", curtile_row, curtile_col, visited_tiles[[curtile_row,curtile_col]] ); 
                                
                                println!("----- 7 cur_tile_right_visited = {:?}" , &cur_tile_right_visited);
                                println!("----- 7 cur_tile_left_visited = {:?}" , &cur_tile_left_visited);
                                
                                if cur_tile_left_visited && cur_tile.end_point == check_tile.start_point
                                {  
                                    println!(" ----- 7a ------- TFTF ");
                                    println!(" ----- 7a Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 7a {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
            
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                else if cur_tile_right_visited && cur_tile.end_point_two == check_tile.start_point
                                {
                                    println!(" ----- 7b ------- TFTF/FTFT ");
                                    println!(" ----- 7b Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 7b {:?} <- cur_tile.end_point_two == check_tile.start_point", &cur_tile.end_point_two );
                                    
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                // else if right_edge_visited && left_edge_visited // 
                                else if cur_tile_right_visited && cur_tile_left_visited &&  
                                    ( cur_tile.end_point_two == check_tile.start_point || cur_tile.end_point == check_tile.start_point)// 
                                {
                                    println!(" ----- 7c ------- TFTF/FTFT ");
                                    println!(" ----- 7c Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 7c {:?} <- right_edge_visited && left_edge_visited both true ", &check_tile.start_point );
                                    println!(" ----- 7c one of two end points matches the check_tile start point" );
                                    
                                    // TODO REVIEW main test for let _ = twenty5_tile_square::svg_4(); 
                                    // 8x10  mosaic with 
                                    // Last tile has both Left and Right edges visited.
                                    // Perhaps we need to find next tile before setting the edges?
                                    println!(" ----- 7c New Logic implemented - rerun all tests");

                                    res = (contig_row,contig_col);
                                    break;

                                }
                                else {
                                    println!(" ----- 7d ------- TFTF ");
                                    println!(" ----- 7d no match - keep looking");
                                }                    
                                // end match true, false, false, false) 
                            }
                            // cur_tile IS tftf 
                            // cur_tile NOT ftft, 
                            // check_tile NOT tftf 
                            // check_tile IS ftft
                            (true, false, false, true) => {
                                // there is an issue here in that there are two possible paths to draw out for this cur_tile
                                // so it can possibly match two tiles.
            
                                println!(" ----- 8 -------- TFTF cur_tile, FTFT check_tile");   
            
                                println!(" ----- 8 Check Tile visited edges visited_bool [{},{}] -> {:?}", contig_row, contig_col,visited_tiles[[contig_row,contig_col]]);
                                println!( "----- 8   Cur Tile visited edges visited_bool [{},{}] -> {:?}", curtile_row, curtile_col, visited_tiles[[curtile_row,curtile_col]] ); 
                                
                                let vbool = visited_tiles[[curtile_row,curtile_col]].edge_visited.clone();
                                let match_right_visited = [None,Some(true),None, None];        
                                let right_edge_visited = match_edge_boolean_pattern(match_right_visited, &vbool);
                                println!("----- 8 match_right_visited = {:?}" , &right_edge_visited);
            
                                let match_left_visited = [None, None, None, Some(true)];        
                                let left_edge_visited = match_edge_boolean_pattern(match_left_visited, &vbool);
                                println!("----- 8 left_edge_visited = {:?}" , &left_edge_visited);
                                
                                if left_edge_visited  && cur_tile.end_point == check_tile.start_point_two
                                {  
                                    println!(" ----- 8a ------- TFTF/FTFT ");
                                    println!(" ----- 8a Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 8a {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
            
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                else if right_edge_visited && cur_tile.end_point_two == check_tile.start_point
                                {
                                    println!(" ----- 8b ------- TFTF/FTFT ");
                                    println!(" ----- 8b Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 8b {:?} <- cur_tile.end_point_two == check_tile.start_point", &cur_tile.end_point );
                                    
                                    res = (contig_row,contig_col);
                                    break;
                                } else {
                                    println!(" ----- 8c ------- TFTF/FTFT ");
                                    println!(" ----- 8c no match - keep looking");
                                }                    
                                // end match true, false, false, true) 
                            }
                            // cur_tile IS tftf 
                            // cur_tile NOT ftft, 
                            // check_tile IS tftf 
                            // check_tile NOT ftft

                            (true, false, true, false) => {

                            // if both cur_tile_left_visited and cur_tile_right_visited are true
                            // then this means that we must see which edge of check_tile has not been visited
                            // and match this to the corresponding cur_tile start end point 

                                println!(" ----- 9 -------- TFTF cur_tile, TFTF check_tile");
                                if cur_tile_left_visited && !cur_tile_right_visited &&   
                                    cur_tile.end_point == check_tile.start_point
                                {  
                                    println!(" ----- 9a ------- TFTF/TFTF ");
                                    println!(" ----- 9a Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 9a {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
            
                                    res = (contig_row,contig_col);
                                    break;
                                } 
                                else if !cur_tile_left_visited && cur_tile_right_visited && 
                                    cur_tile.end_point_two == check_tile.start_point_two
                                {
                                    println!(" ----- 9b ------- TFTF/TFTF ");
                                    println!(" ----- 9b Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                    println!(" ----- 9b {:?} <- cur_tile.end_point_two == check_tile.start_point_two", &cur_tile.end_point );
                                    
                                    res = (contig_row,contig_col);
                                    break;
                                } // we've just completed both edges of this tile and are now looking for next tile
                                else if cur_tile_left_visited && cur_tile_right_visited &&
                                        check_tile_left_visited && !check_tile_right_visited &&
                                    cur_tile.end_point_two == check_tile.start_point_two
                            {
                                println!(" ----- 9c ------- TFTF/TFTF ");
                                println!(" ----- 9c Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                println!(" ----- 9c {:?} <- cur_tile.end_point_two == check_tile.start_point_two", &cur_tile.end_point );
                                
                                res = (contig_row,contig_col);
                                break;
                            }else if cur_tile_left_visited && cur_tile_right_visited &&
                                        !check_tile_left_visited && check_tile_right_visited &&
                                    cur_tile.end_point == check_tile.start_point
                            {
                                println!(" ----- 9d ------- TFTF/TFTF ");
                                println!(" ----- 9d Next Tile Index: [{},{}]", &contig_row,&contig_col);
                                println!(" ----- 9d {:?} <- cur_tile.end_point == check_tile.start_point", &cur_tile.end_point );
                                
                                res = (contig_row,contig_col);
                                break;
                            }                                 
                                else {
                                    println!(" ----- 9e ------- TFTF/TFTF ");
                                    println!(" ----- 9e no match - keep looking");
                                }                    
                                // end match true, false, true, false) 
                            }
                            _ => {
                                // want to explicitly match all cases and panic for unknown ones as these need to be checked
                                println!(" ----- 99a ------- FTFT TFTF cur_tile check_tile  ");
                                println!(" ----- 99a unhandled match case ");
                                println!(" ----- ");
                            panic!();
                            }
                        }
            
            
            // ====================================
            // ====================================
            
                        }
                        else {
                        println!(" ----- 99b --------- ");
                        // println!(" ----- 99b !((contig_row == curtile_row) && (contig_col == curtile_col)) && !tile_prev_visited");
                        println!(" ----- 99b  !tile_prev_visited");
                        println!("\t all edges visited tile\n");
                        }
    
                // end if let Some(adjacent_tiles) = adjacent_tiles.iter().map(|v| Some(v.clone())).next() {
            } 
            // for tile in adjacents
        }

    
        println!("879 Adjacent tiles for location ({:?},{:?})", &curtile_row , &curtile_col);

 // set up the new tile according to whichever match this came back true
    // pane_edge_nd_arr[[contig_row,contig_col]].clone()
    println!(" \n***** {:?} <- find_next_tile_ext result *******\n ", &res);

    res        
}

    
     // find_next_tile

// code below from find_next_tile
//
            // if check_tile.start_point == cur_tile.end_point {
            //     println!("Next Tile Found\ncheck_tile.start_point == cur_tile.end_point");

            //     println!("Next Tile: \n {:?}", &check_tile);

            //     if check_tile.start_point_two.x as usize != FLAGGED {
            //          println!("\n\tThis is a double line tile FTFT or TFTF");
            //     }

            //     res = (contig_row,contig_col);
            //     break;
            // } 
            // else {
            //     println!("\n[{},{}]check_tile.start_point Does not match cur_tile.end_point", &contig_row, &contig_col);
            //     if check_tile.start_point_two.x as usize != FLAGGED {
            //         println!("\n\tThis is double line tile FTFT or TFTF \n\n{:?} \n", &check_tile);

            //         // check to see if any of check_tile corners match cur_tile.end_point 
            //         let corners: [euclid::Point2D<i32, euclid::UnknownUnit>; 4] = check_tile.tile.get_tile_corners();
            //         let cur_tile_end_point: &euclid::Point2D<i32, euclid::UnknownUnit> = &cur_tile.end_point;

            //         // if corners.contains(cur_tile_end_point) {
            //         //     println!("cur_tile_end_point is in corners array");
            //         // } else {
            //         //     println!("cur_tile_end_point is not in corners array");
            //         // }

            //         // find out if curtile endpoint is in corners 
            //         // and which which corner it is [top_left, top_right, bottom_right, bottom_left] 
            //         // THIS DOES NOT WORK AS YES ONE OF THE CORNERS WILL ALWAYS MATCH THE FIRST TILE
            //         if let Some(idx) = corners.iter().position(|&corner| corner == *cur_tile_end_point) {
            //             println!("Found {:?} at index {}", cur_tile_end_point, idx);

            //             res = (contig_row,contig_col);
            //             break;
        
                    // } 


// ****************************** */
// ****************************** */




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

        // println!("combine_pane_edges Mosaic tile bpoints {:?}", &mosaic_tile.bpoints);

        result[[row, col]] = mosaic_tile;
    }
    result

} // combine_pane_edges

// ****************************** */
// ****************************** */


/// Create visited boolean array with each edge set to false 
/// shape[0] is rows
/// shape[1] is cols
fn create_visited_bool_arr(shape: &[usize]) -> ArrayBase<OwnedRepr<TileVisited>, Dim<[usize; 2]>> {
    
    let mut result = Array2::<TileVisited>::zeros((shape[0], shape[1]));
        
    for mut row in result.outer_iter_mut() {
        for tile in row.iter_mut() {
            *tile = TileVisited::new(vec![false, false, false, false]);
        }
    }
    
    result
}

/// iterate over the visited tiles and set all TTTT tiles (i.e. tiles with no edges) as visited
fn set_tttt_visited_tiles(visited_tiles: &mut ArrayBase<OwnedRepr<TileVisited>, Dim<[usize; 2]>>,  pane_edge_nd_arr: &ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>>) 
{
    // for all tttt tiles set visited tiles to [true,true,true,true]
    let match_tttt = [Some(true), Some(true), Some(true), Some(true)];

    for (((_row, _col), m_tile), ((_row1, _col1), edge_visited_bool)) in pane_edge_nd_arr.indexed_iter().zip(visited_tiles.indexed_iter_mut()) 
    {
        let m_tile_edge_bool = m_tile.edge_bool.clone();
        let tile_is_tttt :bool = match_edge_boolean_pattern(match_tttt, &m_tile_edge_bool);

        if tile_is_tttt {
            *edge_visited_bool = TileVisited::new(vec![true, true, true, true]);
        }
        // println!("[{},{}] - {:?}" , row, col, &m_tile.edge_bool);
        // println!("{:?}", &edge_visited_bool);
    }
}

// Return True if there are more tiles to be processed
// false otherwise
fn _check_un_visited(contig_group: &[(isize, isize)], visited_tiles: &ArrayBase<OwnedRepr<TileVisited>, Dim<[usize; 2]>>) -> bool {
    for (row, col) in contig_group {
        let tile_visited = &visited_tiles[[*row as usize, *col as usize]];
        if !tile_visited.edge_visited.iter().all(|&v| v) {
            return true;
        }
    }
    false
}
///
/// Return the (row,col) location and Mosaic tile of first not completely visited tile 
fn get_incomplete_tile(contig_group: &[(isize, isize)], 
                             visited_tiles: &ArrayBase<OwnedRepr<TileVisited>, Dim<[usize; 2]>>, 
                             pane_edge_nd_arr: &ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>>) -> Option<((isize,isize), MosaicTile)> {
    for (row, col) in contig_group {
        let tile_visited = &visited_tiles[[*row as usize, *col as usize]];
        if !tile_visited.edge_visited.iter().all(|&v| v) {
            let tile: MosaicTile = pane_edge_nd_arr[[*row as usize, *col as usize]].clone();
           // return index of item that has not had all edges set to visited 
           return Some(((*row,*col),tile));
        }
    }
    // nothing found so return none
    None 
}


// ****************************** */
// ****************************** */


/// Function to compare passed Option to the tile edge boolean 
/// returns true if matches all the positions where Options != None
/// 
/// for example find the true match and set the new tile accordingly
/// 'let match_this_tftf = [Some(true), Some(false), Some(true), Some(false)];
/// 'let match_this_ftft = [Some(false), Some(true), Some(false), Some(true)];
/// 'let match_this_ftXt = [Some(false), Some(true), None, Some(true)]; // don't care whats in 3rd position
/// 'let cur_tile_edge_bool = cur_tile.edge_bool.clone();
/// 'let tile_is_tftf :bool = match_edge_boolean_pattern(match_this_tftf, &cur_tile_edge_bool);
/// 'let tile_is_ftft :bool = match_edge_boolean_pattern(match_this_ftft, &cur_tile_edge_bool);

pub fn match_edge_boolean_pattern(match_this: [Option<bool>; 4], tile_edge_bool: &[bool]) -> bool {
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

// ****************************** */
// ****************************** */
#[derive(PartialEq, Debug, Clone, Hash)]
pub struct TileVisited{
    pub edge_visited: Vec<bool>
} 

impl TileVisited {
    pub fn new(edge_visited: Vec<bool>) -> TileVisited {
        TileVisited { edge_visited }
    }

    pub(crate) fn set_all_visited_edges_true(&mut self)
    {
        self.edge_visited[TOP] = true;
        self.edge_visited[RIGHT]= true;
        self.edge_visited[BOTTOM]= true;
        self.edge_visited[LEFT]= true;

    }

    // if all edges marked visited then return true otherwise return false
    pub(crate) fn visited(&self) -> bool {
        let res:bool  = self.edge_visited[TOP] == true && self.edge_visited[RIGHT] == true && self.edge_visited[BOTTOM] == true && self.edge_visited[LEFT] == true;
        res
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

    fn add(self, _other: Self) -> Self {
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
