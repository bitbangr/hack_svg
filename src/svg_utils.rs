use crate::{box_corners, dfs_tiles};
use crate::modtile::{RGB, self};
use crate::constants::{NORTH,EAST,SOUTH,WEST,};
use crate::constants::{SE_CORNER,SW_CORNER,NW_CORNER,NE_CORNER};
use crate::constants::{TOP,RIGHT,BOTTOM, LEFT};
use crate::constants::{TOP_LEFT,TOP_RIGHT,BOT_RIGHT, BOT_LEFT};

use euclid::default::Box2D;
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
let pane_2d_vec: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = pane_to_2d_vec(&pane_nd_arr, tiles_per_pane_height, tiles_per_pane_width);
println!("\n\n2D Pane Vec -> {:?}", pane_2d_vec);

// get the test boolean array to build our svg path with
let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&pane_nd_arr);

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
} // create_svg




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
    
        for contig_tile in contig_group{
            
            let row = *&contig_tile.0 as usize;
            let col = *&contig_tile.1 as usize;
        
            println!("*** contigous tile {:?}", &contig_tile);
            println!("*** contig_tile row {}", &row);
            println!("*** contig_tile col {}", &col);

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
                    line_data = line_data.line_to(corner[TOP_RIGHT])
                    .line_to(corner[BOT_RIGHT])
                    .line_to(corner[BOT_LEFT]);

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
                    line_data = line_data.line_to(corner[BOT_LEFT])
                    .line_to(corner[TOP_LEFT]);

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // TTFF
                // **********************************    
                (false, false, true, false) => { //FFTF
                    println!("match -> false false true false - south open");
                    print!(" NORTH/WEST/EAST (top/left/right) Closed - SOUTH (bottom) side open tile\n");
    
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

}




/// iterate over each tile by row col and
/// Draw out all the pane lines matching the cardinal directors for that tile
/// do not worry about duplicate line etc
/// 
pub fn draw_svg_grid_one(line_bucket: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>, 
                     pane_nd_arr: ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, modtile::RGB)>, ndarray::Dim<[usize; 2]>>) {
    
    println!("\n ***********\nFUNCTION draw_svg_grid_one\n");

    // Create the svg document
    // let mut document = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));
    let mut document = Document::new().set("viewBox", (0, 0, 200, 100));

    // let stroke_colour =  "black";
    let stroke_colour =  "purple";
    let stroke_width =  0.25; 

    for (row, rows) in line_bucket.axis_iter(ndarray::Axis(0)).enumerate() {
        for (col, card_dir) in rows.iter().enumerate() {
            println!("\nRow: {}, Col: {},\nCardinal Direction Bool: {:?}", row, col, card_dir);

            let cur_tile:(Box2D<i32>, RGB) = pane_nd_arr[[row,col]];

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

pub fn draw_svg_grid(line_bucket: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>, 
                        pane_nd_arr: ndarray::ArrayBase<ndarray::OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, modtile::RGB)>, ndarray::Dim<[usize; 2]>>,
                        viewbox_width: usize,
                        viewbox_height: usize) 
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
        .set("viewBox", (0, 0, viewbox_width, viewbox_height))
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
        .set("viewBox", (0, 0, 100, 100))
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
