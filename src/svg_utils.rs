use crate::modtile::{RGB, self};


use euclid::default::Box2D;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

// TODO get rid of duplications.  Centralize contants in a single file
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
pub fn draw_svg_grid_one(line_bucket: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>, 
                     pane_nd_arr: ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, modtile::RGB)>, ndarray::Dim<[usize; 2]>>) {
    
    println!("\n ***********\nFUNCTION draw_svg_grid_one\n");

    // Create the svg document
    let mut document = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));

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

