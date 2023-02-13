use euclid::default::Box2D;
use ndarray::{Array, Array2};
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

use crate::dfs_tiles::get_contiguous_tiles_mod;
use crate::{pane_vec_to_ndarray, get_bool_arr, NORTH,EAST,SOUTH,WEST};
use crate::{modtile::{RGB, self}, create_data};

// For a two tile horizontal mosiac the dimension are 1 row by 2 col
const TILES_PER_PANE_WIDTH: usize = 2;
const TILES_PER_PANE_HEIGHT: usize = 1;

/*
    This function creates a 1x2 mosaic of two white tiles and then creates an SVG file with this info
*/
pub(crate) fn create_white_white_svg(){

    // Create a simple 1x2 mosaic
    let mosaic_vec: Vec<Vec<(Box2D<i32>, RGB)>> = create_double_white_tile_data(); 
    println!("test of module call create_double_white_tile_data {:?}", &mosaic_vec);

    // grab the ND Array for this mosiac
    let mosaic_nd_arr = get_tile_ndarray(&mosaic_vec[0]);
    println!("Tile NDArray {:?} ", &mosaic_nd_arr);

    // get the test boolean array to build our svg path with
    let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&mosaic_nd_arr);

    println!("edge_booleans[0,0][0] = {:?}" , edge_booleans[[0,0]][0]);
    println!("edge_booleans = {:?}" , &edge_booleans);

    // call get_contiguous_tiles() to get contiguous tiles 
    // need to decide if using 
    //    get_contiguous_tiles (&mosaic_vec) This seems to work  
    //       created get_contiguous_tiles_mod()
    // or get_contiguous_tiles (&mosaic_nd_arr)

    let contiguous_tiles = get_contiguous_tiles_mod(&mosaic_vec);
    println!("fn dfs_mod search results -> {:?}", &contiguous_tiles);

    let svg_file_name_str = "double_tile_horizontal.svg";

    // lets create an svg file
    let _ = write_svg(mosaic_nd_arr, edge_booleans, contiguous_tiles, svg_file_name_str);


}


/*
    This function creates a 1x2 mosaic of one white and one black tile and then creates an SVG file with this info
*/
pub(crate) fn create_white_black_svg(){

    // Create a simple 1x2 mosaic
    let mosaic_vec: Vec<Vec<(Box2D<i32>, RGB)>> = create_white_black_tile_data(); 
    println!("test of module call create_white_black_tile_data {:?}", &mosaic_vec);

    // grab the ND Array for this mosiac
    let mosaic_nd_arr = get_tile_ndarray(&mosaic_vec[0]);
    println!("Tile NDArray {:?} ", &mosaic_nd_arr);

    // get the test boolean array to build our svg path with
    let mut edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&mosaic_nd_arr);

    println!("edge_booleans[0,0][0] = {:?}" , edge_booleans[[0,0]][0]);
    println!("edge_booleans = {:?}" , &edge_booleans);

    // call get_contiguous_tiles() to get contiguous tiles 
    // need to decide if using 
    //    get_contiguous_tiles (&mosaic_vec) This seems to work  
    //       created get_contiguous_tiles_mod()
    // or get_contiguous_tiles (&mosaic_nd_arr)

    let contiguous_tiles = get_contiguous_tiles_mod(&mosaic_vec);
    println!("fn dfs_mod search results -> {:?}", &contiguous_tiles);

    let svg_file_name_str = "white_black_1x2.svg";
    let _ = write_svg(mosaic_nd_arr, edge_booleans, contiguous_tiles, svg_file_name_str);

}



/// The write_svgvfunction will create an output SVG file with the supplied input data.
/// 
/// # Arguments
///
/// `mosaic_nd_arr: ArrayBase<OwnedRepr<(Box2D<i32>, RGB)>, Dim<[usize; 2]>>` - Array of all tiles with Box Coordinates and associated tile colour
/// 'edge_booleans: ArrayBase<OwnedRepr<Vec<bool>>, Dim<[usize; 2]>>' - Edge boolean for each tile
/// 'contiguous_tiles: Vec<Vec<(isize, isize)>>'  - vector containing collections of contigous tiles
///
/// # Return
///
/// returns a result 
///  
/// # Examples
///
/// ```
/// ```
fn write_svg(mosaic_nd_arr: ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, RGB)>,ndarray::Dim<[usize; 2]>>, 
            edge_booleans: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>, 
            contiguous_tiles: Vec<Vec<(isize, isize)>>,
            svg_file_name_str: &str ) -> Result<(), std::io::Error> 
{
    // not sure if SVG specific code should reside here or in svg_utils.rs
    
    // Create the svg document
    // TODO set width and heigh to match rows/cols * tile size
    let mut document = Document::new().set("viewBox", (0, 0, 200, 100));

    // let stroke_colour =  "black";
    let stroke_colour =  "purple";
    let stroke_width =  0.25; 


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
            let x0 = tile_box.min.x as usize;
            let y0 = tile_box.min.y as usize;
            let x1 = tile_box.max.x as usize;
            let y1 = tile_box.max.y as usize;
        
            let atile_rgb = &cur_tile.1;
            let atile_rgb_str = &atile_rgb.to_string().replace(" ", "");
            rgb_str = atile_rgb_str.to_string(); 
            println!("rgb string  {} ", rgb_str);        
            // TODO Feb 12 - See notes 

            // let mut line_data = Data::new();
            match (n, e, s, w) { //FTFF

            // *******************************************
            // Fully closed tiles are by definition the only element in the contigous tile collection
            (false, false, false, false) => {
                println!("match -> false false false false - single tile");
                print!(" NORTH EAST SOUTH WEST fully closed single tile\n");

                line_data = line_data.move_to((x0,y0))
                                    .line_to((x1,y0))
                                    .line_to((x1,y1))
                                    .line_to((x0,y1))
                                    .line_to((x0,y0));
                                    // .close();                           // will double close crap out
                println!("line data {:?}" , &line_data);
                }, // FFFF
                // **********************************
            (false, true, false, false) => {
                println!("match -> false true false false - east open");
                print!(" NORTH SOUTH WEST Closed - East Open tile\n");

                line_data = line_data.move_to((x1,y1))
                    .line_to((x0,y1))
                    .line_to((x0,y0))
                    .line_to((x1,y0));

                    println!("line data {:?}\n ----------- " , &line_data);


                }, // FFFF
                // **********************************    
            (false, false, false, true) => { //FFFT
                    println!("match -> false false false true - west open");
                    print!(" NORTH EAST SOUTH Closed - West Open tile\n");
    
                    // line_data = line_data.move_to((x0,y0))
                    line_data = line_data.line_to((x1,y0))
                    .line_to((x1,y1))
                    .line_to((x0,y1));

                    println!("line data {:?}\n ---------- " , &line_data);

                }, // FFFT
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

    // let svg_file_name_str = "double_tile_horizontal.svg";
    svg::save(svg_file_name_str, &document)

}

///
/// Get the Array2 ND array for the tile 
fn get_tile_ndarray (vec: &Vec<(Box2D<i32>, modtile::RGB)>) -> Array2<(Box2D<i32>, modtile::RGB)> {

   let pane_nd_array =  pane_vec_to_ndarray(&vec, TILES_PER_PANE_HEIGHT, TILES_PER_PANE_WIDTH );
   
   pane_nd_array
}

///  This function creates mosaic which consists of one window of one pane with two tiles
/// 100 by 200 UnknownUnits size
pub fn create_double_white_tile_data() -> Vec<Vec<(Box2D<i32>, modtile::RGB)>> {

    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((0, 0), (100, 100), (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // (Box2D((100, 0), (200, 100)), RGB(255, 255, 255)),
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((100, 0), (200, 100), (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window
}


///  This function creates mosaic which consists of one window of one pane with two tiles
/// left tile is white, right tile is black
/// 100 by 200 UnknownUnits size
pub fn create_white_black_tile_data() -> Vec<Vec<(Box2D<i32>, modtile::RGB)>> {

    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // [(Box2D((0, 0), (100, 100)), RGB(255, 255, 255)),
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((0, 0), (100, 100), (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));
    
    // (Box2D((100, 0), (200, 100)), RGB(255, 255, 255)),
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((100, 0), (200, 100), (0, 0, 0));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window
}


/// Create an Array2 nd array of booleans.
/// 
/// Each tile has a north, east, south and west edge
/// If a tile matches the colour of its neighbour then corresponding cardinal edge boolean is set to true
/// if it does not or if it is an edge then direction boolean is set to false
/// Lines are drawn for all false edges. No lines are drawn for true edges
/// 
// fn get_cardinal_edge_boolean() -> ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> 
fn get_edge_bools(mosaic_nd_arr: &ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, RGB)>, ndarray::Dim<[usize; 2]>>)  -> ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>
{
    let mut edges: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = 
                                    get_bool_arr(TILES_PER_PANE_HEIGHT, TILES_PER_PANE_WIDTH);

    let rows = mosaic_nd_arr.dim().0;
    let cols = mosaic_nd_arr.dim().1;

    for i in 0..rows {
        for j in 0..cols {
            let curtile_rgb = mosaic_nd_arr[(i, j)].1;
            let north_tile_bool: bool = { if i > 0 {curtile_rgb == mosaic_nd_arr[(i - 1, j)].1 } else { false } };
            let south_tile_bool: bool = { if i < rows - 1 { curtile_rgb == mosaic_nd_arr[(i + 1, j)].1 } else { false } };
            let  west_tile_bool: bool = { if j > 0 { curtile_rgb == mosaic_nd_arr[(i, j - 1)].1 } else { false } };
            let  east_tile_bool: bool = { if j < cols - 1 { curtile_rgb == mosaic_nd_arr[(i, j + 1)].1 } else { false } };

            println!("get_edge_bools() ({},{}) \n\tNorth {}\n\tEast {}\n\tSouth {}\n\tWest {}", i,j, north_tile_bool, east_tile_bool, south_tile_bool, west_tile_bool);

            edges[[i,j]][NORTH] = north_tile_bool;
            edges[[i,j]][EAST] = east_tile_bool;
            edges[[i,j]][SOUTH] = south_tile_bool;
            edges[[i,j]][WEST] = west_tile_bool;
        
            // if curtile_rgb == north_tile_rgb {println!("north tile same colour");}
            // if curtile_rgb == east_tile_rgb {println!("east tile same colour");}
            // if curtile_rgb == south_tile_rgb {println!("south tile same colour");}
            // if curtile_rgb == west_tile_rgb {println!("west tile same colour");}

        } // cols
    } // rows

    // println!("get_edge_bools = {:?}" , &edges);
    
    edges

}
