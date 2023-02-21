use std::iter::Zip;

use crate::mosaic_tile::{Tile, RGB, MosaicTile};
use crate::{box_corners, dfs_tiles};
// use crate::modtile::{RGB, self};
use crate::constants::{NORTH,EAST,SOUTH,WEST,};
use crate::constants::{SE_CORNER,SW_CORNER,NW_CORNER,NE_CORNER};
use crate::constants::{TOP,RIGHT,BOTTOM, LEFT};
use crate::constants::{TOP_LEFT,TOP_RIGHT,BOT_RIGHT, BOT_LEFT};

use euclid::Point2D;
use euclid::default::Box2D;
use ndarray::{Array2, ArrayBase, OwnedRepr, Dim, s, Axis, ViewRepr, Array1};
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

use crate::dfs_tiles::get_contiguous_tiles_mod;
use crate::get_edge_bools;
use crate::pane_to_2d_vec;
use crate::{pane_vec_to_ndarray, get_bool_arr, box2d_to_points};

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


/// General helper function called to create an svg output file
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
    pane_rows: usize, 
    pane_cols: usize, 
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
    let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&pane_nd_arr);

    // let pane_edge_bool_arr_tuple :[(ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, RGB)>,ndarray::Dim<[usize; 2]>>, // pane_nd_arr
    // ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>)] = get_pane_edge_bool_arr(&pane_nd_arr, &edge_booleans);

    // let p_b_tuple = get_pane_edge_bool_tuple(&pane_nd_arr, &edge_booleans);


    println!("edge_booleans = {:?}" , &edge_booleans);

    // get Vec of Vec of contigous tiles
    let contiguous_tiles = dfs_tiles::get_contiguous_tiles_mod(&pane_2d_vec);
    println!("fn get_contiguous_tiles_mod search results -> {:?}", &contiguous_tiles);

    // where the majic happens. lets create an svg file
    let _ = write_svg(pane_nd_arr, 
                        edge_booleans, 
                        contiguous_tiles, 
                        op_svg_file_name ,
                        svg_width as usize,
                        svg_height as usize);

    // test out our new function  TODO THIS IS OUR TEST
    // let _ = write_svg_tup(&pane_edge_bool_arr_tuple, 
    //                         contiguous_tiles, 
    //                         op_svg_file_name ,
    //                         svg_width as usize,
    //                         svg_height as usize);
    
    // fn create_contiguous_path(tiles: &[(Array2<(Box2D<i32>, RGB)>, 
    //                                     ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>)],
                                    
    //                                 ) -> Data {

    // let _ = write_svg(tiles: &[pane_nd_arr, 
    //                     edge_booleans, 
    //                     contiguous_tiles, 
    //                     op_svg_file_name ,
    //                     svg_width as usize,
    //                     svg_height as usize);
    

} // create_svg


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
    pane_rows: usize, 
    pane_cols: usize, 
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
    let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&pane_nd_arr);

    println!("edge_booleans = {:?}" , &edge_booleans);

    // get Vec of Vec of contigous tiles
    let contiguous_tiles = dfs_tiles::get_contiguous_tiles_mod(&pane_2d_vec);
    println!("fn get_contiguous_tiles_mod search results -> {:?}", &contiguous_tiles);

    let pane_edge_nd_arr:  Array2<(MosaicTile)> = combine_pane_edges(&pane_nd_arr, &edge_booleans);

    println! ("*********\nmosaic_pane_edge_nd_arr\n\n{:?}", &pane_edge_nd_arr);

    // testing the travel contigous tiles function
    // let _ = travel_contig_svg(pane_nd_arr, 
    //                     edge_booleans, 
    //                     contiguous_tiles, 
    //                     op_svg_file_name ,
    //                     svg_width as usize,
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

    println!("\n***********\nfn travel_contig_svg_refact");
    println!("\nVector of contigous tiles -> {:?}", contiguous_tiles);

    // Grab a collection of contigous tiles
    for contig_group in &contiguous_tiles{

        let mut rgb_str: String = String::new();
    
        // current end location of last line drawn (x,y)
        // need to check this is the start point of the next line 
        let mut curr_svg_line_end_point: (usize,usize) = (0,0);
        let mut is_first_tile : bool = true;

        // grab the first tile
        let start_tile_idx = contig_group[0];
        println!("\nStart tile index from contiguous tiles -> {:?}" , &start_tile_idx);

        let row = *&start_tile_idx.0 as usize;
        let col = *&start_tile_idx.1 as usize;

        // grab the first tile and keep track of it
        let start_tile:MosaicTile  = pane_edge_nd_arr[[row,col]].clone(); 
        let start_tile_edge_bool:Vec<bool> = start_tile.edge_bool;

        // let edge_bool:Vec<bool> = edge_booleans[[row,col]].clone(); 
        // let tile = Tile { coords: Box2D::new(0, 0), rgb: RGB::new(255, 255, 255) };
        // let edge_bool = vec![true, false, true, false];
        // let first_mosaic_tile = MosaicTile::new(Tile::new(start_tile.0,start_tile.1), edge_bool);
        // println!("first_mosaic_tile -> {:?}", &first_mosaic_tile);
        // let group_line_start_point.

        let mut more_tiles: bool = true; 
        while (more_tiles) {

            println!("\n while more_tiles start_tile_idx -> {:?}" , &start_tile_idx);

            more_tiles = false;

            let cur_tile  = &pane_edge_nd_arr[[row,col]]; 
            let edge_bools = &cur_tile.edge_bool;
            // let cur_tile: (Box2D<i32>, RGB) = mosaic_nd_arr[[row,col]];
            let mut clone_tile  = pane_edge_nd_arr[[row,col]].clone(); 
            // clone_tile.set_start_end_points_to_zero();

            clone_tile.set_start_end_point(&Point2D::new(12,40), &Point2D::new(120,223));
            clone_tile.set_end_point(&Point2D::new(434,400));

            println!("\n(row: {} col: {})\n\tCur Tile Info {:?} ",row, col, &cur_tile);
            println!("\tCur Tile Edge Booleans -> {:?} " , &edge_bools);
            println!("\n(row: {} col: {})\n\tClone Tile Info {:?} ",row, col, &clone_tile);
            // let next_tile: (usize,usize) = get_next_tile( ) 

            let n = edge_bools[NORTH];
            let e = edge_bools[EAST];
            let s = edge_bools[SOUTH];
            let w = edge_bools[WEST];
        
            let tile_box = &cur_tile.tile;
            // let corner:[(usize,usize);4] = box_corners(*tile_box);
            let corner = cur_tile.tile.corners();
            
            println!("\nCorner Co-ords {:?}", corner);
            println!("\ntop left corner {:?}", corner[TOP_LEFT]);
            println!("top right corner {:?}", corner[TOP_RIGHT]);
            println!("bottom right corner {:?}", corner[BOT_RIGHT]);
            println!("bottom left corner {:?}\n\n", corner[BOT_LEFT]);

            let mut cur_tile_start_point: Point2D<i32,i32> = Point2D::new(0,0);
            let mut cur_tile_end_point:Point2D<i32,i32> = Point2D::new(10,10);



            // let mut line_data = Data::new();
            match (n, e, s, w) { //FTFF

                // *******************************************
                // Fully closed tiles are by definition the only element in the contigous tile collection
                // don't need to look for next tile
                (false, false, false, false) => {
                    println!("match -> false false false false - single tile");
                    print!(" NORTH EAST SOUTH WEST fully closed single tile\n");

                    cur_tile_start_point = get_point2D(corner[TOP_LEFT]);
                      cur_tile_end_point = get_point2D(corner[TOP_LEFT]);

                    println!{"start point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
                    println!{"end point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
                    }, // FFFF
                    // **********************************
                (false, true, false, false) => {
                    println!("match -> false true false false - east open");
                    print!(" NORTH SOUTH WEST Closed - East Open tile\n");
    
                    cur_tile_start_point = get_point2D(corner[BOT_LEFT]);
                      cur_tile_end_point = get_point2D(corner[TOP_LEFT]);

                    println!{"start point BOT_LEFT-> {:?} ", corner[BOT_LEFT]}; 
                    println!{"end point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
    
                    }, // FTFF
                    // **********************************    
                (false, false, false, true) => { //FFFT
                        println!("match -> false false false true - west open");
                        print!(" NORTH EAST SOUTH Closed - West/left side Open tile\n");

                        cur_tile_start_point = get_point2D(corner[TOP_LEFT]);
                        cur_tile_end_point = get_point2D(corner[BOT_LEFT]);
      
                        println!{"start point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
                        println!{"end point BOT_LEFT-> {:?} ", corner[BOT_LEFT]};     
    
                    }, // FFFT
                    // **********************************    
                (false, true, true, false) => { //FTTF
                        println!("match -> false true true false - east/south open");
                        print!(" NORTH/WEST (top/left) Closed - EAST/South (right/bottom) side open tile\n");

                        cur_tile_start_point = get_point2D(corner[BOT_LEFT]);
                        cur_tile_end_point = get_point2D(corner[TOP_RIGHT]);

                        println!{"start point BOT_LEFT-> {:?} ", corner[BOT_LEFT]}; 
                        println!{"end point TOP_RIGHT-> {:?} ", corner[TOP_RIGHT]};     
    
                    }, // FTTF
                    // **********************************    
                (false, false, true, true) => { //FFTT
                        println!("match -> false false true true - south/west open");
                        print!(" NORTH/EAST (top/right) Closed - SOUTH/WEST (bottom/left) side open tile\n");

                        cur_tile_start_point = get_point2D(corner[TOP_LEFT]);
                        cur_tile_end_point = get_point2D(corner[BOT_RIGHT]);

                        println!{"start point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
                        println!{"end point BOT_RIGHT-> {:?} ", corner[BOT_RIGHT]};     
    
                    }, // FFTT
                    // **********************************    
                    (true, false, false, true) => { //TFFT
                        println!("match -> true false false true - north/east open");
                        print!(" SOUTH/EAST (bottom/right) Closed - NORTH/WEST (top/left) side open tile\n");
        
                        cur_tile_start_point = get_point2D(corner[TOP_RIGHT]);
                        cur_tile_end_point = get_point2D(corner[BOT_LEFT]);

                        println!{"start point TOP_RIGHT-> {:?} ", corner[TOP_RIGHT]}; 
                        println!{"end point BOT_LEFT-> {:?} ", corner[BOT_LEFT]};     
    
                    }, // TFFT
                    // **********************************    
                    (true, true, false, false) => { //TTFF
                        println!("match -> true true false false - north/west open");
                        print!(" SOUTH/WEST (bottom/left) Closed - NORTH/EAST (top/right) side open tile\n");
        
                        cur_tile_start_point = get_point2D(corner[BOT_RIGHT]);
                        cur_tile_end_point = get_point2D(corner[TOP_LEFT]);

                        println!{"start point BOT_RIGHT-> {:?} ", corner[BOT_RIGHT]}; 
                        println!{"end point TOP_LEFT-> {:?} ", corner[TOP_LEFT]};     

                    }, // TTFF
                    // **********************************    
                    (false, false, true, false) => { //FFTF
                        println!("match -> false false true false - south open");
                        print!(" NORTH/WEST/EAST (top/left/right) Closed - SOUTH (bottom) side open tile\n");

                        cur_tile_start_point = get_point2D(corner[BOT_LEFT]);
                        cur_tile_end_point = get_point2D(corner[BOT_RIGHT]);
                        // update the current tile line end point 
                        curr_svg_line_end_point = corner[BOT_RIGHT];

                        println!{"start point BOT_LEFT-> {:?} ", corner[BOT_LEFT]}; 
                        println!{"end point BOT_RIGHT-> {:?} ", corner[BOT_RIGHT]};     
                        println!("\n\t curr_svg_line_end_point = corner[BOT_RIGHT] {:?}", corner[BOT_RIGHT]);

                        println!("Yoooo Hoooo - Get Next Tile Here");

                        //  = mosaic_nd_arr[[row,col]];
                      // let next_tile:(Box2D<i32>, RGB) = find_next_tile(row, col, curr_svg_line_end_point, BOT_RIGHT,cur_tile, &contig_group, &mosaic_nd_arr ); 
                        let next_tile:MosaicTile =  find_next_tile_refact(row, col, curr_svg_line_end_point, BOT_RIGHT,&cur_tile, &contig_group, &pane_edge_nd_arr ); 

                        println!("Next Tile using Tile mosaic_tile::Tile struct {:?} ", &next_tile);


                    }, // FFTF
                    // **********************************    
                    (true, false, false, false) => { //TFFF
                        println!("match -> true false false false - north open");
                        print!(" SOUTH/EAST/WEST (bottom/right/left) Closed - NORTH (top) side open tile\n");

                        cur_tile_start_point = get_point2D(corner[TOP_RIGHT]);
                        cur_tile_end_point = get_point2D(corner[TOP_LEFT]);

                        println!{"start point TOP_RIGHT-> {:?} ", corner[TOP_RIGHT]}; 
                        println!{"end point TOP_LEFT-> {:?} ", corner[TOP_LEFT]};     
    
                    }, // TFFF
    
                    // **********************************
                    _ => {
                        println!("The EDGE Boolean does not match any of the options\n");  
                    },
    
                } // match
                
                println!("cur_tile_start_point {:?}", cur_tile_start_point);
                println!("cur_tile_end_point {:?}", cur_tile_end_point);

        } // while moretiles


        // instead of iterating through each tile in conf lets
        // grab the first tile and draw it
        //    if not yet arrived at start point 
        //      then find next_tile = get_next_tile(cur_end_point).  ie get the tile that starts with the end point of this tile
        // first tile
        // 
        // grab the first tile
        // let start_tile = contig_group[0];
        // let row = *&contig_tile.0 as usize;
        // let col = *&contig_tile.1 as usize;

        // let mut more_tiles: bool = true; 
        // while (more_tiles) {
        //     // keep drawing 
        //     if first_tile {

        //         // move to first tile (starting point based on tile shape (tile boolean)
        //         //
        //         // line_data = draw_startingtile, (line_data, start_tile,row,col);
        //     }
            


        // } // while (more_tiles)

        // for contig_tile in contig_group{
            
        //     let row = *&contig_tile.0 as usize;
        //     let col = *&contig_tile.1 as usize;
        
        //     println!("*** contigous tile {:?}", &contig_tile);
        //     println!("*** contig_tile row {}", &row);
        //     println!("*** contig_tile col {}", &col);
        //     // println!("\n\tfirst_tile {}", &first_tile);
        //     println!("\n\tcurrent curr_svg_line_end_point {:?}", &curr_svg_line_end_point);

        //     // println!("mosaic_nd_arr [x][y] -> {:?} ",mosaic_nd_arr[[row,col]] );

        //     let cur_tile: (Box2D<i32>, RGB) = mosaic_nd_arr[[row,col]];
        //     println!("\n(row: {} col: {})\n\tCur Tile Info {:?} ",row, col, &cur_tile);
        //     println!("\tTile Edge Booleans -> {:?} " , edge_booleans[[row,col]]);
        //     println!("\tTile rgb -> {:?} " , &cur_tile.1);
        
        //     let n = edge_booleans[[row,col]][NORTH];
        //     let e = edge_booleans[[row,col]][EAST];
        //     let s = edge_booleans[[row,col]][SOUTH];
        //     let w = edge_booleans[[row,col]][WEST];
        
        //     let tile_box = &cur_tile.0;
        //     let corner:[(usize,usize);4] = box_corners(*tile_box);
            
        //     println!("\nCorner Co-ords {:?}", corner);
        //     println!("\ntop left corner {:?}", corner[TOP_LEFT]);
        //     println!("top right corner {:?}", corner[TOP_RIGHT]);
        //     println!("bottom right corner {:?}", corner[BOT_RIGHT]);
        //     println!("bottom left corner {:?}", corner[BOT_LEFT]);

        //     // println!("\nNorth West corner {:?}", corner[NW_CORNER]);
        //     // println!("North East corner {:?}", corner[NE_CORNER]);
        //     // println!("South East corner {:?}", corner[SE_CORNER]);
        //     // println!("South West corner {:?}", corner[SW_CORNER]);

        //     let atile_rgb = &cur_tile.1;
        //     let atile_rgb_str = &atile_rgb.to_string().replace(" ", "");
        //     rgb_str = atile_rgb_str.to_string(); 
        //     println!("\nrgb string  {} ", rgb_str);        
        //     // TODO Feb 12 - See notes 
    
        //     }
        }

    let mut document = Document::new().set("viewBox", (0, 0, svg_width, svg_height));
    svg::save(op_svg_file_name, &document)   

}

fn find_next_tile_refact(row: usize, 
                        col: usize, 
                        curr_svg_line_end_point: (usize, usize), 
                        bot_right: usize, 
                        cur_tile: &MosaicTile, 
                        contig_group: &[(isize, isize)], 
                        pane_edge_nd_arr: &ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>>) -> MosaicTile 
{

       
        println!( "find_next_tile(\n\trow {}
            \n\tcol {}        
            \n\tcurr_svg_line_end_point {:?}
            \n\tcur_tile {:?}
            \n\tcontig_group {:?}
            \n\tpane_edge_nd_arr) {:?}\n\n", row, 
                                        col, 
                                        curr_svg_line_end_point ,  
                                        cur_tile,
                                        contig_group,
                                        pane_edge_nd_arr ); 

        let cur_tile = pane_edge_nd_arr[[row,col]].tile;
    
       let coords = cur_tile.coords;
       let rgb = cur_tile.rgb; 
    
       // find the true match and set the new tile accordingly
       let match_this = [Some(true), Some(false), None, None];

       let tile_edge_bool1 = pane_edge_nd_arr[[row+1,col]].edge_bool.clone();
       let result1 :bool =match_edge_boolean_pattern(match_this, &tile_edge_bool1);
    
       if result1 == true {
    
       }
    
       let match_this = [Some(false), None, None, Some(true)];
       let tile_edge_bool2 = pane_edge_nd_arr[[row+1,col+1]].edge_bool.clone();
       let result2 :bool =match_edge_boolean_pattern(match_this, &tile_edge_bool2);
    
        // set up the new tile according to whichever match this came back true
       MosaicTile::new(Tile::new(coords, rgb),tile_edge_bool2)
    

} // find_next_tile_refact

//****************************** */
//****************************** */


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
    for (((row, col), (box2d, rgb)), ((row1, col1), edge_bool)) in pane_nd_arr.indexed_iter().zip(edge_booleans.indexed_iter()) 
    {
        
        let tile = Tile::new(Box2D::new(box2d.min, box2d.max), *rgb);
        let mosaic_tile = MosaicTile::new(tile, edge_bool.to_vec());
        result[[row, col]] = mosaic_tile;
    }
    result

} // combine_pane_edges


// what does the function -> [(Array2< (Box2D<i32>, RGB, Array1<[bool;4]>)] return
// The function returns a tuple containing a single 2-dimensional array 
// of type Array2<(Box2D<i32>, RGB, Array1<[bool; 4]>)>. 

// Each element of the array is a tuple of (Box2D<i32>, RGB, Array1<[bool; 4]>).

// fn get_pane_edge_bool_tuple(
//       pane_nd_arr: &Array2<(Box2D<i32>, RGB)>,
//     edge_booleans: &Array2<Array1<[bool;4]>> ) -> [Array2<(Box2D<i32>, RGB, Array1<[bool;4]>)>; 4] 
// {
//     let mut pane_edge_bool_arr_tuple = [Array2::default(); 4];

//     for (index, (pane, edge_bool)) in pane_nd_arr.iter().zip(edge_booleans.iter()).enumerate() {
//         let pane_edge_bool = Array2::from_elem((pane.0, pane.1, edge_bool.to_owned()), [4, 4]);
//         pane_edge_bool_arr_tuple[index] = pane_edge_bool;
//     }
//     pane_edge_bool_arr_tuple
// }




// fn get_pane_edge_bool_tuple( pane_nd_arr: &Array2<(Box2D<i32>, RGB)>,
//     edge_booleans: &Array2<Array1<[bool;4]>>
// ) ->  [Array2<(Box2D<i32>, RGB, Array1<[bool;4]>)>]
// {
    
    
//     let mut tile: Array2::<(Box2D<i32>, RGB)> = Array2![()] ;

//     let mut tile = Array2::<(Box2D<i32>, RGB)>::default((10, 10));

//     let mut tile1: ArrayBase<OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>, Dim<[usize; 2]>> = Array2::<(Box2D<i32>, RGB), bool>::default::<_>((10, 10));

//      let mut pane_edge_bool_arr_tuple = [(Array2::default(), Array2::default()); 4];

//     let mut pane_edge_bool_arr_tuple = Vec![(Array2::default(), Array2::default()); 4];

//     for ((pane_edge, ((pane, _), edge)), bool_vec) in pane_edge_bool_arr_tuple.iter_mut()
//         .zip(pane_nd_arr.axis_iter(ndarray::Axis(0))
//             .zip(edge_booleans.axis_iter(ndarray::Axis(0)))
//             .zip(edge_booleans.axis_iter(ndarray::Axis(1))))
//         .zip(pane_nd_arr.axis_iter(ndarray::Axis(0)).enumerate())
//     {
//         let (mut pane_arr, mut bool_arr) = pane_edge;

//         let shape = pane.shape();
//         let mut data = Vec::with_capacity(shape[0] * shape[1]);
//         for (i, (box2d, rgb)) in pane.iter().enumerate() {
//             data.push((*box2d, *rgb));
//             bool_arr[i] = edge[i].clone();
//         }
//         pane_arr = Array2::from_shape_vec(shape, data).unwrap();

//         *bool_vec.iter().zip(bool_arr.outer_iter_mut()).for_each(|(b, a)| *a = b.clone());
//     }

//     // pane_edge_bool_arr_tuple
// }

/// trying to use the same style of call used in 
/// create_contiguous_path(tiles: &[(Point2D<i32>, [bool; 4], Rect<usize>)]) -> Data {}
/// 
/// where tiles and edge booleans are a tuple that can be searched together using a single iterator as in the following code snippet
/// 
///   let start_tile = tiles
///     .iter()
///     .find(|(p, _, _)| !tiles.iter().any(|(q, _, _)| q.x == p.x && q.y < p.y))
///     .unwrap()
///     .0;
/// 
/// 
pub fn write_svg_tup(pane_tiles: &[(ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, RGB)>,ndarray::Dim<[usize; 2]>>, // pane_nd_arr
                                    ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>)],        // edge_booleans
            contiguous_tiles: Vec<Vec<(isize, isize)>>,
            svg_file_name_str: &str,
            viewbox_width: usize,
            viewbox_height: usize ) -> Result<(), std::io::Error> 
{
 

    // let start_tile = pane_tiles
    // .iter()
    // .find(|(p, _ )| !pane_tiles.iter().any(|(q, _)| q.x == p.x && q.y < p.y))
    // .unwrap()
    // .0;


    // let start_tile = pane_tiles
    // .iter()
    // .find(|(p, _ )| !pane_tiles.iter().any(|(q, _)| q.0.bottom_right == p.0.top_left && q.0.top_left.y < p.0.top_left.y))
    // .unwrap()
    // .0;

    for (tile, edge_bools) in pane_tiles.iter() {
        println!("Tile: {:?}", tile);
        println!("Edge bools: {:?}", edge_bools);
    }
    

    let mut document = Document::new().set("viewBox", (0, 0, viewbox_width, viewbox_height));
    println!("writing to file {} ", &svg_file_name_str);
    svg::save(svg_file_name_str, &document)


} // write_svg_tup



/// This is the code to be updated to new approach/ possibly use write_svg_tup function style with tuple
/// 
pub fn write_svg(mosaic_nd_arr: ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, RGB)>,ndarray::Dim<[usize; 2]>>,
                 edge_booleans: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>, 
              contiguous_tiles: Vec<Vec<(isize, isize)>>,
             svg_file_name_str: &str,
                 viewbox_width: usize,
                viewbox_height: usize ) -> Result<(), std::io::Error> 
{
    // not sure if SVG specific code should reside here or in svg_utils.rs
    
    // Create the svg document
    // TODO set width and heigh to match rows/cols * tile size
    let mut document = Document::new().set("viewBox", (0, 0, viewbox_width, viewbox_height));

    // let stroke_colour =  "black";
    let stroke_colour =  "purple";
    let stroke_width =  0.25; 

    // TODO dfs_mod returns a vect of matching entries to the position in the Vec
    //         This does not match the array inde position to the NDarray
    //         So need to modify DFS_MOD to return array index and not Vec position
    // see dfs_tiles.rs for implimentaion

    // fn dfs_mod search results -> [[(0, 0), (0, 1), (0, 2), (0, 3)]]
    // fn write_svg - Vector of contigous tiles -> [[(0, 0), (0, 1), (0, 2), (0, 3)]]

    //***********
    // **********
    println!("\nfn write_svg - Vector of contigous tiles -> {:?}", contiguous_tiles);

    // store all the edges 

    // Grab a collection of contigous tiles
    for contig_group in &contiguous_tiles{

        let mut line_data = Data::new();
        let mut rgb_str: String = String::new();
    
        // current end location of last line drawn (x,y)
        // need to check this is the start point of the next line 
        let mut curr_end_point: (usize,usize) = (0,0);
        let mut first_tile : bool = true;


        // instead of iterating through each tile in conf lets
        // grab the first tile and draw it
        //    if not yet arrived at start point 
        //      then find next_tile = get_next_tile(cur_end_point).  ie get the tile that starts with the end point of this tile
        // first tile
        // 
        // grab the first tile
        // let start_tile = contig_group[0];
        // let row = *&contig_tile.0 as usize;
        // let col = *&contig_tile.1 as usize;

        // let mut more_tiles: bool = true; 
        // while (more_tiles) {
        //     // keep drawing 
        //     if first_tile {

        //         // move to first tile (starting point based on tile shape (tile boolean)
        //         //
        //         // line_data = draw_startingtile, (line_data, start_tile,row,col);
        //     }
            


        // } // while (more_tiles)

        for contig_tile in contig_group{
            
            let row = *&contig_tile.0 as usize;
            let col = *&contig_tile.1 as usize;
        
            println!("*** contigous tile {:?}", &contig_tile);
            println!("*** contig_tile row {}", &row);
            println!("*** contig_tile col {}", &col);
            println!("\n\tfirst_tile {}", &first_tile);
            println!("\n\tcurrent endpoint {:?}", &curr_end_point);

            // println!("mosaic_nd_arr [x][y] -> {:?} ",mosaic_nd_arr[[row,col]] );

            let cur_tile: (Box2D<i32>, RGB) = mosaic_nd_arr[[row,col]];
            println!("\n(row: {} col: {})\n\tCur Tile Info {:?} ",row, col, &cur_tile);
            println!("\tTile Edge Booleans -> {:?} " , edge_booleans[[row,col]]);
        
            let n = edge_booleans[[row,col]][NORTH];
            let e = edge_booleans[[row,col]][EAST];
            let s = edge_booleans[[row,col]][SOUTH];
            let w = edge_booleans[[row,col]][WEST];
        
            let tile_box = &cur_tile.0;
            let corner:[(usize,usize);4] = box_corners(*tile_box);
            
            println!("\nCorner Co-ords {:?}", corner);
            println!("\ntop left corner {:?}", corner[TOP_LEFT]);
            println!("top right corner {:?}", corner[TOP_RIGHT]);
            println!("bottom right corner {:?}", corner[BOT_RIGHT]);
            println!("bottom left corner {:?}", corner[BOT_LEFT]);

            println!("\nNorth West corner {:?}", corner[NW_CORNER]);
            println!("North East corner {:?}", corner[NE_CORNER]);
            println!("South East corner {:?}", corner[SE_CORNER]);
            println!("South West corner {:?}", corner[SW_CORNER]);

            let atile_rgb = &cur_tile.1;
            let atile_rgb_str = &atile_rgb.to_string().replace(" ", "");
            rgb_str = atile_rgb_str.to_string(); 
            println!("\nrgb string  {} ", rgb_str);        
            // TODO Feb 12 - See notes 

            // let mut line_data = Data::new();
            match (n, e, s, w) { //FTFF

            // *******************************************
            // Fully closed tiles are by definition the only element in the contigous tile collection
            (false, false, false, false) => {
                println!("match -> false false false false - single tile");
                print!(" NORTH EAST SOUTH WEST fully closed single tile\n");

                line_data = line_data.move_to(corner[TOP_LEFT])
                                    .line_to(corner[TOP_RIGHT])
                                    .line_to(corner[BOT_RIGHT])
                                    .line_to(corner[BOT_LEFT])
                                    .line_to(corner[TOP_LEFT]);

                // same as above but harder to visualize
                // line_data = line_data.move_to(corner[NW_CORNER])
                //                     .line_to(corner[NE_CORNER])
                //                     .line_to(corner[SE_CORNER])
                //                     .line_to(corner[SW_CORNER])
                //                     .line_to(corner[NW_CORNER]);

                // same as above but easy to mess up x and y so use corner array
                // line_data = line_data.move_to((x0,y0))
                //                     .line_to((x1,y0))
                //                     .line_to((x1,y1))
                //                     .line_to((x0,y1))
                //                     .line_to((x0,y0));
                                    // .close();                           // will double close crap out
                println!("line data {:?}" , &line_data);
                }, // FFFF
                // **********************************
            (false, true, false, false) => {
                println!("match -> false true false false - east open");
                print!(" NORTH SOUTH WEST Closed - East Open tile\n");

                line_data = line_data.move_to(corner[BOT_RIGHT])
                    .line_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT])
                    .line_to(corner[TOP_RIGHT]);

                    println!("line data {:?}\n ----------- " , &line_data);

                }, // FFFF
                // **********************************    
            (false, false, false, true) => { //FFFT
                    println!("match -> false false false true - west open");
                    print!(" NORTH EAST SOUTH Closed - West/left side Open tile\n");
    
                    // open West tiles cannot be first tile in results so no need for absolute 'move_to'.
                    // just continue to draw from last point

                    if curr_end_point != corner[TOP_LEFT]{
                        line_data = line_data.move_to(corner[TOP_LEFT]);
                    }
                    
                    line_data = line_data.line_to(corner[TOP_RIGHT])
                    .line_to(corner[BOT_RIGHT])
                    .line_to(corner[BOT_LEFT]);

                    curr_end_point = corner[BOT_LEFT];

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // FFFT
                // **********************************    
                (false, true, true, false) => { //FTTF
                    println!("match -> false true true false - east/south open");
                    print!(" NORTH/WEST (top/left) Closed - EAST/South (right/bottom) side open tile\n");
    
                    // TODO mgj Feb 15th See Test Case notes pg 12
                    // closed West tiles may not be first tile so need to check if first for absolute 'move_to'.
                    // otherewise just continue to draw from last point
                    line_data = line_data.move_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT])
                    .line_to(corner[TOP_RIGHT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // FTTF
                // **********************************    
                (false, false, true, true) => { //FFTT
                    println!("match -> false false true true - south/west open");
                    print!(" NORTH/EAST (top/right) Closed - SOUTH/WEST (bottom/left) side open tile\n");
    
                    // can't be first tile so no need absolute 'move_to'.
                    // continue to draw from last point
                    line_data = line_data.line_to(corner[TOP_RIGHT])
                    .line_to(corner[BOT_RIGHT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // FFTT
                // **********************************    
                (true, false, false, true) => { //TFFT
                    println!("match -> true false false true - north/east open");
                    print!(" SOUTH/EAST (bottom/right) Closed - NORTH/WEST (top/left) side open tile\n");
    
                    // can't be first tile so no need absolute 'move_to'.
                    // continue to draw from last point
                    line_data = line_data.line_to(corner[BOT_RIGHT])
                    .line_to(corner[BOT_LEFT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // TFFT
                // **********************************    
                (true, true, false, false) => { //TTFF
                    println!("match -> true true false false - north/west open");
                    print!(" SOUTH/WEST (bottom/left) Closed - NORTH/EAST (top/right) side open tile\n");
    
                    // can't be first tile so no need absolute 'move_to'.
                    // continue to draw from last point
                    // above assumption is wrong
                    //  FFTF -> TTFF requires a moveto as last point of FFTF is not start of TTFF  
                    if curr_end_point != corner[BOT_RIGHT]{
                        line_data = line_data.move_to(corner[BOT_RIGHT]);
                    }
                    line_data = line_data.line_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT]);

                    curr_end_point = corner[TOP_LEFT];

                    println!("line data {:?}\n curr_end_point = {:?}---------- " , &line_data, & curr_end_point);

                }, // TTFF
                // **********************************    
                (false, false, true, false) => { //FFTF
                    println!("match -> false false true false - south open");
                    print!(" NORTH/WEST/EAST (top/left/right) Closed - SOUTH (bottom) side open tile\n");
    
                    if first_tile {
                        println!("We have the first tile!") ;
                        first_tile = false;
                        // update the endpoint to last point of this shape
                        curr_end_point = corner[BOT_RIGHT];

                    } 
                    // west closed so may be a move too here if no  'move_to'.
                    // continue to draw from last point
                    // if not first tile then don't do absolute 'move_to'.  TODO CHECK THIS 
                    line_data = line_data.move_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT])
                    .line_to(corner[TOP_RIGHT])
                    .line_to(corner[BOT_RIGHT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // FFTF
                // **********************************    
                (true, false, false, false) => { //TFFF
                    println!("match -> true false false false - north open");
                    print!(" SOUTH/EAST/WEST (bottom/right/left) Closed - NORTH (top) side open tile\n");
    
                    // continue to draw from last point
                    // can't be first tile so no need absolute 'move_to'.
                    // line_data.move_to(corner[BOT_LEFT])
                    line_data = line_data.line_to(corner[BOT_RIGHT])
                    .line_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // TFFF

                // **********************************
                _ => {
                    println!("The EDGE Boolean does not match any of the options\n");  
                },

            } // match

        } // tile in contig_group
        
        // at this point all the tiles of the contig group have been processed so close the line 
        line_data = line_data.close();

        println!(" ^^^^^^^^^^^^^\n after contig_group line_data close() {:?}\n ---------- " , &line_data);

        // create a path and add it to the svg document
        let tile_path = Path::new()
        .set("fill", rgb_str.to_owned()) // ie -> .set("fill", "rgb(255, 0, 0)")
        .set("stroke", stroke_colour)
        .set("stroke-width", stroke_width)
        .set("d", line_data);

        // add the tile path to the document
        document = document.add(tile_path);
    
    } // contig_group

    // let cur_tile: (Box2D<i32>, RGB) = mosaic_nd_arr[[row,col]];
    // println!("\n(row: {} col: {})\n\tTile Info {:?} ",row, col, &cur_tile);
    // println!("\tTile Edge Booleans -> {:?} " , edge_booleans[[row,col]]);

    // Write the svg document to a file
    println!("writing to file {} ", &svg_file_name_str);
    svg::save(svg_file_name_str, &document)

} // write_svg


/// given set of contiguous tiles find the ntext tile and print it out
/// 
pub fn travel_contig_svg(mosaic_nd_arr: ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, RGB)>,ndarray::Dim<[usize; 2]>>,
                 edge_booleans: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>, 
              contiguous_tiles: Vec<Vec<(isize, isize)>>,
             svg_file_name_str: &str,
                 viewbox_width: usize,
                viewbox_height: usize ) -> Result<(), std::io::Error> 
{


    // TODO dfs_mod returns a vect of matching entries to the position in the Vec
    //         This does not match the array index position to the NDarray
    //         So need to modify DFS_MOD to return array index and not Vec position
    // see dfs_tiles.rs for implimentaion

    // fn dfs_mod search results -> [[(0, 0), (0, 1), (0, 2), (0, 3)]]
    // fn write_svg - Vector of contigous tiles -> [[(0, 0), (0, 1), (0, 2), (0, 3)]]

    //***********
    // **********
    println!("\n***********\nfn travel_contig_svg");
    println!("\nVector of contigous tiles -> {:?}", contiguous_tiles);

    // Grab a collection of contigous tiles
    for contig_group in &contiguous_tiles{

        let mut rgb_str: String = String::new();
    
        // current end location of last line drawn (x,y)
        // need to check this is the start point of the next line 
        let mut curr_svg_line_end_point: (usize,usize) = (0,0);
        let mut is_first_tile : bool = true;

        // grab the first tile
        let start_tile_idx = contig_group[0];
        let row = *&start_tile_idx.0 as usize;
        let col = *&start_tile_idx.1 as usize;

        // grab the first tile
        let start_tile: (Box2D<i32>, RGB) = mosaic_nd_arr[[row,col]]; 
        let edge_bool:Vec<bool> = edge_booleans[[row,col]].clone(); 


        // let tile = Tile { coords: Box2D::new(0, 0), rgb: RGB::new(255, 255, 255) };
        // let edge_bool = vec![true, false, true, false];
        let first_mosaic_tile = MosaicTile::new(Tile::new(start_tile.0,start_tile.1), edge_bool);

        println!("first_mosaic_tile -> {:?}", &first_mosaic_tile);
        // let group_line_start_point.



        println!("\nStart tile index from contiguous tiles -> {:?}" , &start_tile_idx);

        let mut more_tiles: bool = true; 
        while (more_tiles) {

            println!("\n while more_tiles start_tile_idx -> {:?}" , &start_tile_idx);

            more_tiles = false;

            let cur_tile: (Box2D<i32>, RGB) = mosaic_nd_arr[[row,col]];
            println!("\n(row: {} col: {})\n\tCur Tile Info {:?} ",row, col, &cur_tile);
            println!("\tTile Edge Booleans -> {:?} " , edge_booleans[[row,col]]);
        
            // let next_tile: (usize,usize) = get_next_tile( ) 

            let n = edge_booleans[[row,col]][NORTH];
            let e = edge_booleans[[row,col]][EAST];
            let s = edge_booleans[[row,col]][SOUTH];
            let w = edge_booleans[[row,col]][WEST];
        
            let tile_box = &cur_tile.0;
            let corner:[(usize,usize);4] = box_corners(*tile_box);
            
            println!("\nCorner Co-ords {:?}", corner);
            println!("\ntop left corner {:?}", corner[TOP_LEFT]);
            println!("top right corner {:?}", corner[TOP_RIGHT]);
            println!("bottom right corner {:?}", corner[BOT_RIGHT]);
            println!("bottom left corner {:?}\n\n", corner[BOT_LEFT]);

            let mut cur_tile_start_point: Point2D<i32,i32> = Point2D::new(0,0);
            let mut cur_tile_end_point:Point2D<i32,i32> = Point2D::new(10,10);

            // let mut line_data = Data::new();
            match (n, e, s, w) { //FTFF

                // *******************************************
                // Fully closed tiles are by definition the only element in the contigous tile collection
                // don't need to look for next tile
                (false, false, false, false) => {
                    println!("match -> false false false false - single tile");
                    print!(" NORTH EAST SOUTH WEST fully closed single tile\n");

                    cur_tile_start_point = get_point2D(corner[TOP_LEFT]);
                      cur_tile_end_point = get_point2D(corner[TOP_LEFT]);

                    println!{"start point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
                    println!{"end point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
                    }, // FFFF
                    // **********************************
                (false, true, false, false) => {
                    println!("match -> false true false false - east open");
                    print!(" NORTH SOUTH WEST Closed - East Open tile\n");
    
                    cur_tile_start_point = get_point2D(corner[BOT_LEFT]);
                      cur_tile_end_point = get_point2D(corner[TOP_LEFT]);

                    println!{"start point BOT_LEFT-> {:?} ", corner[BOT_LEFT]}; 
                    println!{"end point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
    
                    }, // FTFF
                    // **********************************    
                (false, false, false, true) => { //FFFT
                        println!("match -> false false false true - west open");
                        print!(" NORTH EAST SOUTH Closed - West/left side Open tile\n");

                        cur_tile_start_point = get_point2D(corner[TOP_LEFT]);
                        cur_tile_end_point = get_point2D(corner[BOT_LEFT]);
      
                        println!{"start point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
                        println!{"end point BOT_LEFT-> {:?} ", corner[BOT_LEFT]};     
    
                    }, // FFFT
                    // **********************************    
                (false, true, true, false) => { //FTTF
                        println!("match -> false true true false - east/south open");
                        print!(" NORTH/WEST (top/left) Closed - EAST/South (right/bottom) side open tile\n");

                        cur_tile_start_point = get_point2D(corner[BOT_LEFT]);
                        cur_tile_end_point = get_point2D(corner[TOP_RIGHT]);

                        println!{"start point BOT_LEFT-> {:?} ", corner[BOT_LEFT]}; 
                        println!{"end point TOP_RIGHT-> {:?} ", corner[TOP_RIGHT]};     
    
                    }, // FTTF
                    // **********************************    
                (false, false, true, true) => { //FFTT
                        println!("match -> false false true true - south/west open");
                        print!(" NORTH/EAST (top/right) Closed - SOUTH/WEST (bottom/left) side open tile\n");

                        cur_tile_start_point = get_point2D(corner[TOP_LEFT]);
                        cur_tile_end_point = get_point2D(corner[BOT_RIGHT]);

                        println!{"start point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
                        println!{"end point BOT_RIGHT-> {:?} ", corner[BOT_RIGHT]};     
    
                    }, // FFTT
                    // **********************************    
                    (true, false, false, true) => { //TFFT
                        println!("match -> true false false true - north/east open");
                        print!(" SOUTH/EAST (bottom/right) Closed - NORTH/WEST (top/left) side open tile\n");
        
                        cur_tile_start_point = get_point2D(corner[TOP_RIGHT]);
                        cur_tile_end_point = get_point2D(corner[BOT_LEFT]);

                        println!{"start point TOP_RIGHT-> {:?} ", corner[TOP_RIGHT]}; 
                        println!{"end point BOT_LEFT-> {:?} ", corner[BOT_LEFT]};     
    
                    }, // TFFT
                    // **********************************    
                    (true, true, false, false) => { //TTFF
                        println!("match -> true true false false - north/west open");
                        print!(" SOUTH/WEST (bottom/left) Closed - NORTH/EAST (top/right) side open tile\n");
        
                        cur_tile_start_point = get_point2D(corner[BOT_RIGHT]);
                        cur_tile_end_point = get_point2D(corner[TOP_LEFT]);

                        println!{"start point BOT_RIGHT-> {:?} ", corner[BOT_RIGHT]}; 
                        println!{"end point TOP_LEFT-> {:?} ", corner[TOP_LEFT]};     

                    }, // TTFF
                    // **********************************    
                    (false, false, true, false) => { //FFTF
                        println!("match -> false false true false - south open");
                        print!(" NORTH/WEST/EAST (top/left/right) Closed - SOUTH (bottom) side open tile\n");

                        cur_tile_start_point = get_point2D(corner[BOT_LEFT]);
                        cur_tile_end_point = get_point2D(corner[BOT_RIGHT]);
                        // update the current tile line end point 
                        curr_svg_line_end_point = corner[BOT_RIGHT];

                        println!{"start point BOT_LEFT-> {:?} ", corner[BOT_LEFT]}; 
                        println!{"end point BOT_RIGHT-> {:?} ", corner[BOT_RIGHT]};     
                        println!("\n\t curr_svg_line_end_point = corner[BOT_RIGHT] {:?}", corner[BOT_RIGHT]);

                        println!("Yoooo Hoooo - Get Next Tile Here");

                        //  = mosaic_nd_arr[[row,col]];
                        // let next_tile:(Box2D<i32>, RGB) =  find_next_tile(row, col,curr_svg_line_end_point, BOT_RIGHT,cur_tile, &contig_group, &mosaic_nd_arr ); 

                        let next_tile:Tile =  find_next_tile(row, col,curr_svg_line_end_point, BOT_RIGHT,    
                            &cur_tile, &contig_group, &mosaic_nd_arr , &edge_booleans); 

                        println!("Next Tile using Tile mosaic_tile::Tile struct {:?} ", &next_tile);


                    }, // FFTF
                    // **********************************    
                    (true, false, false, false) => { //TFFF
                        println!("match -> true false false false - north open");
                        print!(" SOUTH/EAST/WEST (bottom/right/left) Closed - NORTH (top) side open tile\n");

                        cur_tile_start_point = get_point2D(corner[TOP_RIGHT]);
                        cur_tile_end_point = get_point2D(corner[TOP_LEFT]);

                        println!{"start point TOP_RIGHT-> {:?} ", corner[TOP_RIGHT]}; 
                        println!{"end point TOP_LEFT-> {:?} ", corner[TOP_LEFT]};     
    
                    }, // TFFF
    
                    // **********************************
                    _ => {
                        println!("The EDGE Boolean does not match any of the options\n");  
                    },
    
                } // match
                
                println!("cur_tile_start_point {:?}", cur_tile_start_point);
                println!("cur_tile_end_point {:?}", cur_tile_end_point);

        } // while moretiles


        // instead of iterating through each tile in conf lets
        // grab the first tile and draw it
        //    if not yet arrived at start point 
        //      then find next_tile = get_next_tile(cur_end_point).  ie get the tile that starts with the end point of this tile
        // first tile
        // 
        // grab the first tile
        // let start_tile = contig_group[0];
        // let row = *&contig_tile.0 as usize;
        // let col = *&contig_tile.1 as usize;

        // let mut more_tiles: bool = true; 
        // while (more_tiles) {
        //     // keep drawing 
        //     if first_tile {

        //         // move to first tile (starting point based on tile shape (tile boolean)
        //         //
        //         // line_data = draw_startingtile, (line_data, start_tile,row,col);
        //     }
            


        // } // while (more_tiles)

        for contig_tile in contig_group{
            
            let row = *&contig_tile.0 as usize;
            let col = *&contig_tile.1 as usize;
        
            println!("*** contigous tile {:?}", &contig_tile);
            println!("*** contig_tile row {}", &row);
            println!("*** contig_tile col {}", &col);
            // println!("\n\tfirst_tile {}", &first_tile);
            println!("\n\tcurrent curr_svg_line_end_point {:?}", &curr_svg_line_end_point);

            // println!("mosaic_nd_arr [x][y] -> {:?} ",mosaic_nd_arr[[row,col]] );

            let cur_tile: (Box2D<i32>, RGB) = mosaic_nd_arr[[row,col]];
            println!("\n(row: {} col: {})\n\tCur Tile Info {:?} ",row, col, &cur_tile);
            println!("\tTile Edge Booleans -> {:?} " , edge_booleans[[row,col]]);
            println!("\tTile rgb -> {:?} " , &cur_tile.1);
        
            let n = edge_booleans[[row,col]][NORTH];
            let e = edge_booleans[[row,col]][EAST];
            let s = edge_booleans[[row,col]][SOUTH];
            let w = edge_booleans[[row,col]][WEST];
        
            let tile_box = &cur_tile.0;
            let corner:[(usize,usize);4] = box_corners(*tile_box);
            
            println!("\nCorner Co-ords {:?}", corner);
            println!("\ntop left corner {:?}", corner[TOP_LEFT]);
            println!("top right corner {:?}", corner[TOP_RIGHT]);
            println!("bottom right corner {:?}", corner[BOT_RIGHT]);
            println!("bottom left corner {:?}", corner[BOT_LEFT]);

            // println!("\nNorth West corner {:?}", corner[NW_CORNER]);
            // println!("North East corner {:?}", corner[NE_CORNER]);
            // println!("South East corner {:?}", corner[SE_CORNER]);
            // println!("South West corner {:?}", corner[SW_CORNER]);

            let atile_rgb = &cur_tile.1;
            let atile_rgb_str = &atile_rgb.to_string().replace(" ", "");
            rgb_str = atile_rgb_str.to_string(); 
            println!("\nrgb string  {} ", rgb_str);        
            // TODO Feb 12 - See notes 
    
            }
        }


    let mut document = Document::new().set("viewBox", (0, 0, viewbox_width, viewbox_height));
    svg::save(svg_file_name_str, &document)   
}



fn find_next_tile(
    row: usize, 
    col: usize, 
    curr_svg_line_end_point: (usize, usize), 
    end_point_corner: usize, 
    cur_tile: &(Box2D<i32>, RGB), 
    contig_group: &[(isize, isize)], 
    mosaic_nd_arr: &ArrayBase<OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>, Dim<[usize; 2]>>,
    edge_booleans: &ArrayBase<OwnedRepr<Vec<bool>>, Dim<[usize; 2]>>) 
        // -> (euclid::Box2D<i32, euclid::UnknownUnit>, RGB) {
            -> Tile {
   
    println!( "find_next_tile(\n\trow {}
        \n\tcol {}        
        \n\tcurr_svg_line_end_point {:?}
        \n\tend_point_corner {}
        \n\tcur_tile {:?}
        \n\tcontig_group {:?}
        \n\tedge_booleans_ {:?}
        \n\tmosaic_nd_arr) {:?}\n\n", row, 
                                    col, 
                                    curr_svg_line_end_point , 
                                    end_point_corner, 
                                    cur_tile,
                                    contig_group,
                                    edge_booleans, 
                                    mosaic_nd_arr ); 

   let coords = cur_tile.0;
   let rgb = cur_tile.1; 

   // find the true match and set the new tile accordingly
   let match_this = [Some(true), Some(false), None, None];
   let tile_edge_bool = &edge_booleans[[row+1,col]];
   let result1 :bool =match_edge_boolean_pattern(match_this, &tile_edge_bool);

   if result1 == true {


   }


   let match_this = [Some(false), None, None, Some(true)];
   let tile_edge_bool = &edge_booleans[[row+1,col+1]];
   let result2 :bool =match_edge_boolean_pattern(match_this, &tile_edge_bool);

    // set up the new tile according to whichever match this came back true
   Tile::new(coords, rgb)
}

// fn get_rows_with_pattern_v2(match_this: [Option<bool>; 4], tile_edge_bool: &[bool]) -> bool {
//     todo!()
// }




// fn get_rows_with_pattern_v2(match_this: [Option<bool>; 4], tile_edge_bools: &Vec[<bool>;4>]) -> bool {
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

    println!("match_edge_boolean_pattern result {:?}" , &res);        
    res
}


/// helper function to generate a Point2D from a usize array (x,y)
/// 
fn get_point2D( usize_arr : (usize, usize)) -> Point2D<i32, i32> {
    
    let start_x:i32 = usize_arr.0.try_into().unwrap();
    let start_y:i32 = usize_arr.1.try_into().unwrap();

    Point2D::new(start_x,start_y)
}




/// The write_svgvfunction will create an output SVG file with the supplied input data.
/// 
/// # Arguments
///
/// `mosaic_nd_arr: ArrayBase<OwnedRepr<(Box2D<i32>, RGB)>, Dim<[usize; 2]>>` - Array of all tiles with Box Coordinates and associated tile colour
/// 'edge_booleans: ArrayBase<OwnedRepr<Vec<bool>>, Dim<[usize; 2]>>' - Edge boolean for each tile
/// 'contiguous_tiles: Vec<Vec<(isize, isize)>>'  - vector containing collections of contigous tiles
/// 'svg_file_name_str'": &str string holding name of SVG file to write to
///
/// # Return
///
/// returns a result 
///  
/// # Examples
///
/// ```
/// ```
pub fn write_svg_orig(mosaic_nd_arr: ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, RGB)>,ndarray::Dim<[usize; 2]>>,
            edge_booleans: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>, 
            contiguous_tiles: Vec<Vec<(isize, isize)>>,
            svg_file_name_str: &str,
            viewbox_width: usize,
            viewbox_height: usize ) -> Result<(), std::io::Error> 
{
    // not sure if SVG specific code should reside here or in svg_utils.rs
    
    // Create the svg document
    // TODO set width and heigh to match rows/cols * tile size
    let mut document = Document::new().set("viewBox", (0, 0, viewbox_width, viewbox_height));

    // let stroke_colour =  "black";
    let stroke_colour =  "purple";
    let stroke_width =  0.25; 

    // TODO dfs_mod returns a vect of matching entries to the position in the Vec
    //         This does not match the array inde position to the NDarray
    //         So need to modify DFS_MOD to return array index and not Vec position
    // see dfs_tiles.rs for implimentaion

    // fn dfs_mod search results -> [[(0, 0), (0, 1), (0, 2), (0, 3)]]
    // fn write_svg - Vector of contigous tiles -> [[(0, 0), (0, 1), (0, 2), (0, 3)]]

    //***********
    // **********
    println!("\nfn write_svg - Vector of contigous tiles -> {:?}", contiguous_tiles);

    // store all the edges 

    // Grab a collection of contigous tiles
    for contig_group in &contiguous_tiles{

        let mut line_data = Data::new();
        let mut rgb_str: String = String::new();
    
        // current end location of last line drawn (x,y)
        // need to check this is the start point of the next line 
        let mut curr_end_point: (usize,usize) = (0,0);
        let mut first_tile : bool = true;

        for contig_tile in contig_group{
            
            let row = *&contig_tile.0 as usize;
            let col = *&contig_tile.1 as usize;
        
            println!("*** contigous tile {:?}", &contig_tile);
            println!("*** contig_tile row {}", &row);
            println!("*** contig_tile col {}", &col);
            println!("\n\tfirst_tile {}", &first_tile);
            println!("\n\tcurrent endpoint {:?}", &curr_end_point);

            // println!("mosaic_nd_arr [x][y] -> {:?} ",mosaic_nd_arr[[row,col]] );

            let cur_tile: (Box2D<i32>, RGB) = mosaic_nd_arr[[row,col]];
            println!("\n(row: {} col: {})\n\tCur Tile Info {:?} ",row, col, &cur_tile);
            println!("\tTile Edge Booleans -> {:?} " , edge_booleans[[row,col]]);
        
            let n = edge_booleans[[row,col]][NORTH];
            let e = edge_booleans[[row,col]][EAST];
            let s = edge_booleans[[row,col]][SOUTH];
            let w = edge_booleans[[row,col]][WEST];
        
            let tile_box = &cur_tile.0;
            let corner:[(usize,usize);4] = box_corners(*tile_box);
            
            println!("\nCorner Co-ords {:?}", corner);
            println!("\ntop left corner {:?}", corner[TOP_LEFT]);
            println!("top right corner {:?}", corner[TOP_RIGHT]);
            println!("bottom right corner {:?}", corner[BOT_RIGHT]);
            println!("bottom left corner {:?}", corner[BOT_LEFT]);

            println!("\nNorth West corner {:?}", corner[NW_CORNER]);
            println!("North East corner {:?}", corner[NE_CORNER]);
            println!("South East corner {:?}", corner[SE_CORNER]);
            println!("South West corner {:?}", corner[SW_CORNER]);

            let atile_rgb = &cur_tile.1;
            let atile_rgb_str = &atile_rgb.to_string().replace(" ", "");
            rgb_str = atile_rgb_str.to_string(); 
            println!("\nrgb string  {} ", rgb_str);        
            // TODO Feb 12 - See notes 

            // let mut line_data = Data::new();
            match (n, e, s, w) { //FTFF

            // *******************************************
            // Fully closed tiles are by definition the only element in the contigous tile collection
            (false, false, false, false) => {
                println!("match -> false false false false - single tile");
                print!(" NORTH EAST SOUTH WEST fully closed single tile\n");

                line_data = line_data.move_to(corner[TOP_LEFT])
                                    .line_to(corner[TOP_RIGHT])
                                    .line_to(corner[BOT_RIGHT])
                                    .line_to(corner[BOT_LEFT])
                                    .line_to(corner[TOP_LEFT]);

                // same as above but harder to visualize
                // line_data = line_data.move_to(corner[NW_CORNER])
                //                     .line_to(corner[NE_CORNER])
                //                     .line_to(corner[SE_CORNER])
                //                     .line_to(corner[SW_CORNER])
                //                     .line_to(corner[NW_CORNER]);

                // same as above but easy to mess up x and y so use corner array
                // line_data = line_data.move_to((x0,y0))
                //                     .line_to((x1,y0))
                //                     .line_to((x1,y1))
                //                     .line_to((x0,y1))
                //                     .line_to((x0,y0));
                                    // .close();                           // will double close crap out
                println!("line data {:?}" , &line_data);
                }, // FFFF
                // **********************************
            (false, true, false, false) => {
                println!("match -> false true false false - east open");
                print!(" NORTH SOUTH WEST Closed - East Open tile\n");

                line_data = line_data.move_to(corner[BOT_RIGHT])
                    .line_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT])
                    .line_to(corner[TOP_RIGHT]);

                    println!("line data {:?}\n ----------- " , &line_data);

                }, // FFFF
                // **********************************    
            (false, false, false, true) => { //FFFT
                    println!("match -> false false false true - west open");
                    print!(" NORTH EAST SOUTH Closed - West/left side Open tile\n");
    
                    // open West tiles cannot be first tile in results so no need for absolute 'move_to'.
                    // just continue to draw from last point

                    if curr_end_point != corner[TOP_LEFT]{
                        line_data = line_data.move_to(corner[TOP_LEFT]);
                    }
                    
                    line_data = line_data.line_to(corner[TOP_RIGHT])
                    .line_to(corner[BOT_RIGHT])
                    .line_to(corner[BOT_LEFT]);

                    curr_end_point = corner[BOT_LEFT];

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // FFFT
                // **********************************    
                (false, true, true, false) => { //FTTF
                    println!("match -> false true true false - east/south open");
                    print!(" NORTH/WEST (top/left) Closed - EAST/South (right/bottom) side open tile\n");
    
                    // TODO mgj Feb 15th See Test Case notes pg 12
                    // closed West tiles may not be first tile so need to check if first for absolute 'move_to'.
                    // otherewise just continue to draw from last point
                    line_data = line_data.move_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT])
                    .line_to(corner[TOP_RIGHT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // FTTF
                // **********************************    
                (false, false, true, true) => { //FFTT
                    println!("match -> false false true true - south/west open");
                    print!(" NORTH/EAST (top/right) Closed - SOUTH/WEST (bottom/left) side open tile\n");
    
                    // can't be first tile so no need absolute 'move_to'.
                    // continue to draw from last point
                    line_data = line_data.line_to(corner[TOP_RIGHT])
                    .line_to(corner[BOT_RIGHT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // FFTT
                // **********************************    
                (true, false, false, true) => { //TFFT
                    println!("match -> true false false true - north/east open");
                    print!(" SOUTH/EAST (bottom/right) Closed - NORTH/WEST (top/left) side open tile\n");
    
                    // can't be first tile so no need absolute 'move_to'.
                    // continue to draw from last point
                    line_data = line_data.line_to(corner[BOT_RIGHT])
                    .line_to(corner[BOT_LEFT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // TFFT
                // **********************************    
                (true, true, false, false) => { //TTFF
                    println!("match -> true true false false - north/west open");
                    print!(" SOUTH/WEST (bottom/left) Closed - NORTH/EAST (top/right) side open tile\n");
    
                    // can't be first tile so no need absolute 'move_to'.
                    // continue to draw from last point
                    // above assumption is wrong
                    //  FFTF -> TTFF requires a moveto as last point of FFTF is not start of TTFF  
                    if curr_end_point != corner[BOT_RIGHT]{
                        line_data = line_data.move_to(corner[BOT_RIGHT]);
                    }
                    line_data = line_data.line_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT]);

                    curr_end_point = corner[TOP_LEFT];

                    println!("line data {:?}\n curr_end_point = {:?}---------- " , &line_data, & curr_end_point);

                }, // TTFF
                // **********************************    
                (false, false, true, false) => { //FFTF
                    println!("match -> false false true false - south open");
                    print!(" NORTH/WEST/EAST (top/left/right) Closed - SOUTH (bottom) side open tile\n");
    
                    if first_tile {
                        println!("We have the first tile!") ;
                        first_tile = false;
                        // update the endpoint to last point of this shape
                        curr_end_point = corner[BOT_RIGHT];

                    } 
                    // west closed so may be a move too here if no  'move_to'.
                    // continue to draw from last point
                    // if not first tile then don't do absolute 'move_to'.  TODO CHECK THIS 
                    line_data = line_data.move_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT])
                    .line_to(corner[TOP_RIGHT])
                    .line_to(corner[BOT_RIGHT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // FFTF
                // **********************************    
                (true, false, false, false) => { //TFFF
                    println!("match -> true false false false - north open");
                    print!(" SOUTH/EAST/WEST (bottom/right/left) Closed - NORTH (top) side open tile\n");
    
                    // continue to draw from last point
                    // can't be first tile so no need absolute 'move_to'.
                    // line_data.move_to(corner[BOT_LEFT])
                    line_data = line_data.line_to(corner[BOT_RIGHT])
                    .line_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // TFFF

                // **********************************
                _ => {
                    println!("The EDGE Boolean does not match any of the options\n");  
                },

            } // match

        } // tile in contig_group
        
        // at this point all the tiles of the contig group have been processed so close the line 
        line_data = line_data.close();

        println!(" ^^^^^^^^^^^^^\n after contig_group line_data close() {:?}\n ---------- " , &line_data);

        // create a path and add it to the svg document
        let tile_path = Path::new()
        .set("fill", rgb_str.to_owned()) // ie -> .set("fill", "rgb(255, 0, 0)")
        .set("stroke", stroke_colour)
        .set("stroke-width", stroke_width)
        .set("d", line_data);

        // add the tile path to the document
        document = document.add(tile_path);
    
    } // contig_group

    // let cur_tile: (Box2D<i32>, RGB) = mosaic_nd_arr[[row,col]];
    // println!("\n(row: {} col: {})\n\tTile Info {:?} ",row, col, &cur_tile);
    // println!("\tTile Edge Booleans -> {:?} " , edge_booleans[[row,col]]);

    // Write the svg document to a file
    println!("writing to file {} ", &svg_file_name_str);
    svg::save(svg_file_name_str, &document)

} // write_svg_orig
