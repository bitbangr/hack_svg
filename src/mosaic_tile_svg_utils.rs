use crate::mosaic_tile::MosaicTile;

use crate::constants::{TOP,RIGHT,BOTTOM, LEFT};
use crate::constants::{TOP_LEFT,TOP_RIGHT,BOT_RIGHT, BOT_LEFT};
use crate::svg_utils::TileVisited;

use euclid::default::Point2D;
use ndarray::{ArrayBase, OwnedRepr, Dim};
use svg::node::element::path::Data;



/// This will construct an SVG absolute line to data element
/// 
/// - it is assumed that the caller is already at the start point of this tile so
/// there is no move to start_point of the tile and the first line to will be 
/// to the endpoint of the first segment of the Mosaic tile
/// 
/// It is up to the caller to add the returned line data to the existing line data in order
/// to complete a path
/// 
pub fn get_ext_tile_svg_line_data(m_tile: &MosaicTile, 
                                curr_svg_line_end_point: &Point2D<i32>, 
                                visited_tiles: &mut ArrayBase<OwnedRepr<TileVisited>, Dim<[usize; 2]>>,
                                row: usize, col: usize) -> Data {
    let mut line_data = Data::new();
    let edge_bool = m_tile.edge_bool.clone();

    let top = edge_bool[TOP];
    let right = edge_bool[RIGHT];
    let bottom = edge_bool[BOTTOM];
    let left = edge_bool[LEFT];

    let start_point = m_tile.start_point; 
    let end_point = m_tile.end_point;

    let corner = &m_tile.tile.corners();

    // visited_tiles[[0, 1]].edge_visited[TOP]
    let top_visited = visited_tiles[[row, col]].edge_visited[TOP];
    let right_visited = visited_tiles[[row, col]].edge_visited[RIGHT];
    let bottom_visited = visited_tiles[[row, col]].edge_visited[BOTTOM];
    let left_visited = visited_tiles[[row, col]].edge_visited[LEFT];

    // visited_tiles[[row, col]].edge_visited[TOP] = true;

    println!("\t*************************");
    println!("\tfn get_tile_svg_line_data");

    println!("\n\ttop visited:{:?}",top_visited );
    println!("\tright visited:{:?}",right_visited );
    println!("\tbottom visited:{:?}",bottom_visited );
    println!("\tleft visited:{:?}",left_visited );

    match (top, right, bottom, left) {

        // *******************************************
        // Fully closed tiles are by definition the only element in the contigous tile collection
        // don't need to look for next tile
        // **********************************    
        // Start of four false edge case
        // **********************************

        (false, false, false, false) => { // FFFF
            println!("\nmatch -> false false false false - single tile");
            println!{"start point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
            println!{"end point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 

            // line_data = line_data.move_to(corner[TOP_LEFT])
            line_data = line_data.line_to(corner[TOP_RIGHT])
                                 .line_to(corner[BOT_RIGHT])
                                 .line_to(corner[BOT_LEFT])
                                 .line_to(corner[TOP_LEFT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            }, // FFFF
        // **********************************    
        // Start of three false edge cases 
        // **********************************
        (true, false, false, false) => { //TFFF
            println!("\nmatch -> true false false false - top open");
            println!(" BOTTOM LEFT RIGHT Closed - Top side open tile\n");

            line_data = line_data.line_to(corner[BOT_RIGHT])
                                 .line_to(corner[BOT_LEFT])
                                 .line_to(corner[TOP_LEFT]);

            println!{"start point TOP_RIGHT-> {:?} ", &start_point}; 
            println!{"end point TOP_LEFT-> {:?} ", &end_point};     

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            }, // TFFF
            // **********************************    
        (false, true, false, false) => { //FTFF
            println!("\nmatch -> false true false false - right open");
            println!(" TOP BOTTOM LEFT Closed - Right side open tile\n");

            line_data = line_data.line_to(corner[BOT_LEFT])
                                 .line_to(corner[TOP_LEFT])
                                 .line_to(corner[TOP_RIGHT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point BOT_RIGHT-> {:?} ", &start_point}; 
            println!{"end point TOP_RIGHT-> {:?} ", &end_point};

 
            }, // FTFF
            // **********************************    
        (false, false, true, false) => { //FFTF
            println!("\nmatch -> false false true false - bottom open");
            println!(" TOP/LEFT/RIGHT Closed - bottom side open tile\n");

            line_data = line_data .line_to(corner[TOP_LEFT])
                                  .line_to(corner[TOP_RIGHT])
                                  .line_to(corner[BOT_RIGHT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point BOT_LEFT-> {:?} ", &start_point}; 
            println!{"end point BOT_RIGHT-> {:?} ", &end_point};     
            }, // FFTF            

        // **********************************
        (false, false, false, true) => { //FFFT
            println!("\nmatch -> false false false true - left open");
            println!(" TOP RIGHT BOTTOM Closed - Left side Open tile\n");

            line_data = line_data.line_to(corner[TOP_RIGHT])
            .line_to(corner[BOT_RIGHT])
            .line_to(corner[BOT_LEFT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point TOP_LEFT-> {:?} ", &start_point}; 
            println!{"end point BOT_LEFT-> {:?} ", &end_point};     
            }, // FFFT
        // **********************************  
        // Start of two false edge cases 
        // **********************************
        (false, false, true, true) => { //FFTT
            println!("\nmatch -> false false true true - bottom left open");
            println!(" TOP/RIGHT Closed - Bottom-Left side open tile\n");

            line_data = line_data.line_to(corner[TOP_RIGHT])
                                 .line_to(corner[BOT_RIGHT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point TOP_LEFT-> {:?} ", &start_point}; 
            println!{"end point BOT_RIGHT-> {:?} ", &end_point};     
            }, // FFTT            
        // **********************************    
        (true, false, false, true) => { //TFFT
            println!("\nmatch -> true false false true - top/left open");
            println!(" BOTTOM RIGHT Closed - Top-Left side open tile\n");

            line_data = line_data.line_to(corner[BOT_RIGHT])
                                 .line_to(corner[BOT_LEFT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point TOP_RIGHT-> {:?} ", &start_point}; 
            println!{"end point BOT_LEFT-> {:?} ", &end_point};     
        }, // TFFT        
        // **********************************    
        (true, true, false, false) => { //TTFF
            println!("\nmatch -> true true false false - top/right open");
            println!(" BOTTOM LEFT Closed - Top-Right side open tile\n");

            line_data = line_data.line_to(corner[BOT_LEFT])
                                 .line_to(corner[TOP_LEFT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point BOT_RIGHT-> {:?} ", &start_point}; 
            println!{"end point TOP_LEFT-> {:?} ", &end_point};     

        }, // TTFF        
        // **********************************    
        (false, true, true, false) => { //FTTF
            println!("\nmatch -> false true true false - right/bottom open");
            println!(" TOP LEFT Closed - Right-Bottom side open tile\n");

            line_data = line_data.line_to(corner[TOP_LEFT])
                                 .line_to(corner[TOP_RIGHT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point BOT_LEFT-> {:?} ", &start_point}; 
            println!{"end point TOP_RIGHT-> {:?} ", &end_point};     

        }, // FTTF

        // **********************************  
        // NOTE THESE NEXT TWO CASES HAVE two start points and two end points
        // Need to handle this somehow 
        // TODO REVISIT
        // **********************************
        // **********************************    
        (false, true, false, true) => { //FTFT
            println!("\nmatch -> false true false true - left/right open");
            println!(" TOP BOTTOM Closed - Left-Right side open tile\n");
            println!(" !!!!! Need to Deal with this!!!!!\n");

            // ***** TODO BELOW 
            // TODO use bpoints instead of m_tile.start_point m_tile_end_point 

            if *curr_svg_line_end_point == m_tile.start_point 
            {
                line_data = line_data.line_to(corner[TOP_RIGHT]);
                // Set TOP RIGHT and LEFT as VISITED
                // BOTTOM will default to whatever it was so will remain FALSE if first time through
                visited_tiles[[row, col]].edge_visited[TOP] = true;
                visited_tiles[[row, col]].edge_visited[RIGHT] = true;
                visited_tiles[[row, col]].edge_visited[LEFT] = true;

            } //bottom
            // else if *curr_svg_line_end_point == m_tile.start_point_two // Not sure why this is - REVIEW REVIEW REVIEW
            else if *curr_svg_line_end_point == m_tile.end_point_two // Not sure why this is - REVIEW REVIEW REVIEW
            {
                line_data = line_data.line_to(corner[BOT_LEFT]);

                // Set BOTTOM RIGHT and LEFT as VISITED
                // TOP will default to whatever it was so will remain FALSE if first time through
                visited_tiles[[row, col]].edge_visited[RIGHT] = true;
                visited_tiles[[row, col]].edge_visited[LEFT] = true;
                visited_tiles[[row, col]].edge_visited[BOTTOM] = true;

            }
            else {
                println!("We should never get here!!!");
                panic!();                
            }

            println!{"1st line - start point TOP_LEFT-> {:?} ", &start_point}; 
            println!{"1st line - end point TOP_RIGHT-> {:?} ", &end_point};  
            println!{"2nd line - start point corners[BOT_RIGHT]-> {:?} ", corner[BOT_RIGHT]}; 
            println!{"2nd line - end point corners[BOT_LEFT]-> {:?} ", corner[BOT_LEFT]};  

        }, // FTFT
        // **********************************    
        (true, false, true, false) => { //TFTF
            println!("\nmatch -> true false true false - top/bottom open");
            println!(" LEFT RIGHT Closed - Left-Right side open tile\n");
            println!(" !!!!! Need to Deal with this!!!!!\n");

            println!{"1st line - start point BOT_LEFT-> {:?} ", &start_point}; 
            println!{"1st line - end point TOP_LEFT-> {:?} ", &end_point};  
            println!{"2nd line - start point corners[TOP_RIGHT]-> {:?} ", corner[TOP_RIGHT]}; 
            println!{"2nd line - end point corners[BOT_RIGHT]-> {:?} ", corner[BOT_RIGHT]};  

            panic!();   

        }, // TFTF
        // **********************************  
        // Start of single false edge cases 
        // **********************************
        // **********************************    
        (false, true, true, true) => { //FTTT
            println!("\nmatch -> false true true true - right/left/bottom open");
            println!(" TOP Closed - Right-Left-Bottom side open tile\n");

            line_data = line_data.line_to(corner[TOP_RIGHT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point TOP_LEFT-> {:?} ", &start_point}; 
            println!{"end point TOP_RIGHT-> {:?} ", &end_point};     

        }, // FTTT
        // **********************************    
        (true, false, true, true) => { //TFTT
            println!("\nmatch -> true false true true - top/bottom/left open");
            println!(" RIGHT Closed - Top-Bottom-Left side open tile\n");

            line_data = line_data.line_to(corner[BOT_RIGHT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point TOP_RIGHT-> {:?} ", &start_point}; 
            println!{"end point BOT_RIGHT-> {:?} ", &end_point};     

        }, // TFTT
        // **********************************    
        (true, true, false, true) => { //TTFT
            println!("\nmatch -> true true false true - top/left/right open");
            println!(" BOTTOM Closed - Top-Left-Right side open tile\n");

            line_data = line_data.line_to(corner[BOT_LEFT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point BOT_RIGHT-> {:?} ", &start_point}; 
            println!{"end point BOT_LEFT-> {:?} ", &end_point};     

        }, // TTFT
        // **********************************    
        (true, true, true, false) => { //TTTF
            println!("\nmatch -> true true true false - top/right/bottom open");
            println!(" LEFT Closed - Top-Right-Bottom side open tile\n");

            line_data = line_data.line_to(corner[TOP_LEFT]);

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

            println!{"start point BOT_LEFT-> {:?} ", &start_point}; 
            println!{"end point TOP_LEFT-> {:?} ", &end_point};     

        }, // TTTF

        // **********************************    
        // Start of zere false edge case
        // **********************************

        (true, true, true, true) => { //TTTT
            println!("\nmatch -> true true true true - top/right/bottom/left open");
            println!(" NO EDGES - Top-Right-Bottom-Left side open tile\n");

            // There
            println!("\n !!!! NO EDGES Nothing to draw here.  !!!! \n");

            println!{"start point TOP_RIGHT-> {:?} ", &start_point}; 
            println!{"end point TOP_RIGHT-> {:?} ", &end_point};     

            // set all the visited edges to true
            visited_tiles[[row, col]].set_all_visited_edges_true(); 

        }, // TTTT
            _ => {
                println!("The EDGE Boolean does not match any of the options\n");  
                panic!();
            },

        } // match

    line_data

} // get_ext_tile_svg_line_data


/// Combine two SVG line data elements and return
pub fn combine_data (data1:&Data, data2:&Data) -> Data {
    
    let commands: Vec<_> = data1.iter().chain(data2.iter()).cloned().collect();
    let concatenated_data = Data::from(commands);

    println!("\nCombined data: \n\t{:?} ", &concatenated_data);
    concatenated_data
} // concatenated_data now contains the commands from both data1 and data2


// ****************************** */
// ****************************** */
// DO NOT Touch the code below 
// ****************************** */
// ****************************** */

/// This will construct an SVG absolute line to data element
/// 
/// - it is assumed that the caller is already at the start point of this tile so
/// there is no move to start_point of the tile and the first line to will be 
/// to the endpoint of the first segment of the Mosaic tile
/// 
/// It is up to the caller to add the returned line data to the existing line data in order
/// to complete a path
/// 
pub fn get_tile_svg_line_data(m_tile: &MosaicTile, 
                                curr_svg_line_end_point: &Point2D<i32>, 
                                visited_tiles: &ArrayBase<OwnedRepr<TileVisited>, ndarray::Dim<[usize; 2]>>) -> Data {
    let mut line_data = Data::new();
    let edge_bool = m_tile.edge_bool.clone();

    let top = edge_bool[TOP];
    let right = edge_bool[RIGHT];
    let bottom = edge_bool[BOTTOM];
    let left = edge_bool[LEFT];

    let start_point = m_tile.start_point; 
    let end_point = m_tile.end_point;

    let corner = &m_tile.tile.corners();

    println!("\t*************************");
    println!("\tfn get_tile_svg_line_data");

    match (top, right, bottom, left) {

        // *******************************************
        // Fully closed tiles are by definition the only element in the contigous tile collection
        // don't need to look for next tile
        // **********************************    
        // Start of four false edge case
        // **********************************

        (false, false, false, false) => { // FFFF
            println!("\nmatch -> false false false false - single tile");
            println!{"start point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 
            println!{"end point TOP_LEFT-> {:?} ", corner[TOP_LEFT]}; 

            // line_data = line_data.move_to(corner[TOP_LEFT])
            line_data = line_data.line_to(corner[TOP_RIGHT])
                                 .line_to(corner[BOT_RIGHT])
                                 .line_to(corner[BOT_LEFT])
                                 .line_to(corner[TOP_LEFT]);

            }, // FFFF
        // **********************************    
        // Start of three false edge cases 
        // **********************************
        (true, false, false, false) => { //TFFF
            println!("\nmatch -> true false false false - top open");
            println!(" BOTTOM LEFT RIGHT Closed - Top side open tile\n");

            line_data = line_data.line_to(corner[BOT_RIGHT])
                                 .line_to(corner[BOT_LEFT])
                                 .line_to(corner[TOP_LEFT]);

            println!{"start point TOP_RIGHT-> {:?} ", &start_point}; 
            println!{"end point TOP_LEFT-> {:?} ", &end_point};     

            }, // TFFF
            // **********************************    
        (false, true, false, false) => { //FTFF
            println!("\nmatch -> false true false false - right open");
            println!(" TOP BOTTOM LEFT Closed - Right side open tile\n");

            line_data = line_data.line_to(corner[BOT_LEFT])
                                 .line_to(corner[TOP_LEFT])
                                 .line_to(corner[TOP_RIGHT]);

            println!{"start point BOT_RIGHT-> {:?} ", &start_point}; 
            println!{"end point TOP_RIGHT-> {:?} ", &end_point}; 

            }, // FTFF
            // **********************************    
        (false, false, true, false) => { //FFTF
            println!("\nmatch -> false false true false - bottom open");
            println!(" TOP/LEFT/RIGHT Closed - bottom side open tile\n");

            line_data = line_data .line_to(corner[TOP_LEFT])
                                  .line_to(corner[TOP_RIGHT])
                                  .line_to(corner[BOT_RIGHT]);

            println!{"start point BOT_LEFT-> {:?} ", &start_point}; 
            println!{"end point BOT_RIGHT-> {:?} ", &end_point};     
            }, // FFTF            

        // **********************************
        (false, false, false, true) => { //FFFT
            println!("\nmatch -> false false false true - left open");
            println!(" TOP RIGHT BOTTOM Closed - Left side Open tile\n");

            line_data = line_data.line_to(corner[TOP_RIGHT])
            .line_to(corner[BOT_RIGHT])
            .line_to(corner[BOT_LEFT]);

            println!{"start point TOP_LEFT-> {:?} ", &start_point}; 
            println!{"end point BOT_LEFT-> {:?} ", &end_point};     
            }, // FFFT
        // **********************************  
        // Start of two false edge cases 
        // **********************************
        (false, false, true, true) => { //FFTT
            println!("\nmatch -> false false true true - bottom left open");
            println!(" TOP/RIGHT Closed - Bottom-Left side open tile\n");

            line_data = line_data.line_to(corner[TOP_RIGHT])
                                 .line_to(corner[BOT_RIGHT]);

            println!{"start point TOP_LEFT-> {:?} ", &start_point}; 
            println!{"end point BOT_RIGHT-> {:?} ", &end_point};     
            }, // FFTT            
        // **********************************    
        (true, false, false, true) => { //TFFT
            println!("\nmatch -> true false false true - top/left open");
            println!(" BOTTOM RIGHT Closed - Top-Left side open tile\n");

            line_data = line_data.line_to(corner[BOT_RIGHT])
                                 .line_to(corner[BOT_LEFT]);

            println!{"start point TOP_RIGHT-> {:?} ", &start_point}; 
            println!{"end point BOT_LEFT-> {:?} ", &end_point};     
        }, // TFFT        
        // **********************************    
        (true, true, false, false) => { //TTFF
            println!("\nmatch -> true true false false - top/right open");
            println!(" BOTTOM LEFT Closed - Top-Right side open tile\n");

            line_data = line_data.line_to(corner[BOT_LEFT])
                                 .line_to(corner[TOP_LEFT]);

            println!{"start point BOT_RIGHT-> {:?} ", &start_point}; 
            println!{"end point TOP_LEFT-> {:?} ", &end_point};     

        }, // TTFF        
        // **********************************    
        (false, true, true, false) => { //FTTF
            println!("\nmatch -> false true true false - right/bottom open");
            println!(" TOP LEFT Closed - Right-Bottom side open tile\n");

            line_data = line_data.line_to(corner[TOP_LEFT])
                                 .line_to(corner[TOP_RIGHT]);

            println!{"start point BOT_LEFT-> {:?} ", &start_point}; 
            println!{"end point TOP_RIGHT-> {:?} ", &end_point};     

        }, // FTTF

        // **********************************  
        // NOTE THESE NEXT TWO CASES HAVE two start points and two end points
        // Need to handle this somehow 
        // TODO REVISIT
        // **********************************
        // **********************************    
        (false, true, false, true) => { //FTFT
            println!("\nmatch -> false true false true - left/right open");
            println!(" TOP BOTTOM Closed - Left-Right side open tile\n");
            println!(" !!!!! Need to Deal with this!!!!!\n");

            // top
            if *curr_svg_line_end_point == m_tile.start_point 
            {
                line_data = line_data.line_to(corner[TOP_RIGHT]);
            } //bottom
            else if *curr_svg_line_end_point == m_tile.start_point_two
            {
                line_data = line_data.line_to(corner[BOT_LEFT]);
            }
            else {
                println!("We should never get here!!!");
                panic!();                
            }

            println!{"1st line - start point TOP_LEFT-> {:?} ", &start_point}; 
            println!{"1st line - end point TOP_RIGHT-> {:?} ", &end_point};  
            println!{"2nd line - start point corners[BOT_RIGHT]-> {:?} ", corner[BOT_RIGHT]}; 
            println!{"2nd line - end point corners[BOT_LEFT]-> {:?} ", corner[BOT_LEFT]};  

        }, // FTFT
        // **********************************    
        (true, false, true, false) => { //TFTF
            println!("\nmatch -> true false true false - top/bottom open");
            println!(" LEFT RIGHT Closed - Left-Right side open tile\n");
            println!(" !!!!! Need to Deal with this!!!!!\n");

            println!{"1st line - start point BOT_LEFT-> {:?} ", &start_point}; 
            println!{"1st line - end point TOP_LEFT-> {:?} ", &end_point};  
            println!{"2nd line - start point corners[TOP_RIGHT]-> {:?} ", corner[TOP_RIGHT]}; 
            println!{"2nd line - end point corners[BOT_RIGHT]-> {:?} ", corner[BOT_RIGHT]};  

            panic!();   

        }, // TFTF
        // **********************************  
        // Start of single false edge cases 
        // **********************************
        // **********************************    
        (false, true, true, true) => { //FTTT
            println!("\nmatch -> false true true true - right/left/bottom open");
            println!(" TOP Closed - Right-Left-Bottom side open tile\n");

            line_data = line_data.line_to(corner[TOP_RIGHT]);

            println!{"start point TOP_LEFT-> {:?} ", &start_point}; 
            println!{"end point TOP_RIGHT-> {:?} ", &end_point};     

        }, // FTTT
        // **********************************    
        (true, false, true, true) => { //TFTT
            println!("\nmatch -> true false true true - top/bottom/left open");
            println!(" RIGHT Closed - Top-Bottom-Left side open tile\n");

            line_data = line_data.line_to(corner[BOT_RIGHT]);

            println!{"start point TOP_RIGHT-> {:?} ", &start_point}; 
            println!{"end point BOT_RIGHT-> {:?} ", &end_point};     

        }, // TFTT
        // **********************************    
        (true, true, false, true) => { //TTFT
            println!("\nmatch -> true true false true - top/left/right open");
            println!(" BOTTOM Closed - Top-Left-Right side open tile\n");

            line_data = line_data.line_to(corner[BOT_LEFT]);

            println!{"start point BOT_RIGHT-> {:?} ", &start_point}; 
            println!{"end point BOT_LEFT-> {:?} ", &end_point};     

        }, // TTFT
        // **********************************    
        (true, true, true, false) => { //TTTF
            println!("\nmatch -> true true true false - top/right/bottom open");
            println!(" LEFT Closed - Top-Right-Bottom side open tile\n");

            line_data = line_data.line_to(corner[TOP_LEFT]);

            println!{"start point BOT_LEFT-> {:?} ", &start_point}; 
            println!{"end point TOP_LEFT-> {:?} ", &end_point};     

        }, // TTTF

        // **********************************    
        // Start of zere false edge case
        // **********************************

        (true, true, true, true) => { //TTTT
            println!("\nmatch -> true true true true - top/right/bottom/left open");
            println!(" NO EDGES - Top-Right-Bottom-Left side open tile\n");

            // There
            println!("\n !!!! NO EDGES Nothing to draw here.  !!!! \n");

            println!{"start point TOP_RIGHT-> {:?} ", &start_point}; 
            println!{"end point TOP_RIGHT-> {:?} ", &end_point};     

        }, // TTTT
            _ => {
                println!("The EDGE Boolean does not match any of the options\n");  
                panic!();
            },

        } // match

    line_data

}
