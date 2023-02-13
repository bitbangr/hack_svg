use euclid::default::Box2D;
use ndarray::{Array, Array2};
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

use crate::dfs_tiles::get_contiguous_tiles_mod;
use crate::{pane_vec_to_ndarray, get_bool_arr, NORTH,EAST,SOUTH,WEST, WIDTH, HEIGHT};
use crate::{modtile::{RGB, self}, create_data};

// For a single tile mosiac the dimension are 1 row by 1 col
const TILES_PER_PANE_WIDTH: usize = 1;
const TILES_PER_PANE_HEIGHT: usize = 1;

/*
    This function creates a 1x1 mosaic of a single tile and then creates an SVG file with this info
*/
pub(crate) fn create_svg(){

    // Create a simple 1x1 mosaic
    let mosaic_vec: Vec<Vec<(Box2D<i32>, RGB)>> = create_single_tile_data();
    println!("test of module call create_single_tile_data {:?}", &mosaic_vec);

    // pane_vec_to_ndarray(&pane_3x3_vec[0], TILES_PER_PANE_HEIGHT, TILES_PER_PANE_WIDTH );

    // grab the ND Array for this mosiac
    let mosaic_nd_arr = get_single_tile_ndarray(&mosaic_vec[0]);
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
    println!("dfs_mod search results - with mosaic_vec -> {:?}", &contiguous_tiles);

    // lets create an svg file
    let _ = write_svg(mosaic_nd_arr, edge_booleans, contiguous_tiles);


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
            contiguous_tiles: Vec<Vec<(isize, isize)>>) -> Result<(), std::io::Error> 
{
    // not sure if SVG specific code should reside here or in svg_utils.rs
    
    // Create the svg document
    let mut document = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));

    // let stroke_colour =  "black";
    let stroke_colour =  "purple";
    let stroke_width =  0.25; 

    println!("write SVG -Contigous tiles -> {:?}", contiguous_tiles);
    let row = contiguous_tiles[0][0].0 as usize;
    let col = contiguous_tiles[0][0].1 as usize;
    println!(" (row: {} col: {}) Tile data {:?} ",row, col, mosaic_nd_arr[[row,col]]);
    println!("Tile Edge Booleans -> {:?} " , edge_booleans[[row,col]]);

    //***********
    // **********
    let cur_tile: (Box2D<i32>, RGB) = mosaic_nd_arr[[row,col]];

    println!("Tile info {:?}", &cur_tile);  

    let n = edge_booleans[[0,0]][NORTH];
    let e = edge_booleans[[0,0]][EAST];
    let s = edge_booleans[[0,0]][SOUTH];
    let w = edge_booleans[[0,0]][WEST];

    let tile_box = &cur_tile.0;
    let x0 = tile_box.min.x as usize;
    let y0 = tile_box.min.y as usize;
    let x1 = tile_box.max.x as usize;
    let y1 = tile_box.max.y as usize;

    let tile_rgb = &cur_tile.1;
    let rgb_str = tile_rgb.to_string().replace(" ", "");
    println!("rgb string  {} ", rgb_str);
    match (n, e, s, w) { //FFFF
        (false, false, false, false) => {
        println!("match -> false false false false - single tile");
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
        _ => println!("The value does not match any of the options\n"),

    }
    // **********
    // **********

    // Write the svg document to a file
    svg::save("single_tile.svg", &document)

}

///
/// Get the Array2 ND array for the tile 
fn get_single_tile_ndarray (vec: &Vec<(Box2D<i32>, modtile::RGB)>) -> Array2<(Box2D<i32>, modtile::RGB)> {

   let pane_nd_array =  pane_vec_to_ndarray(&vec, TILES_PER_PANE_HEIGHT, TILES_PER_PANE_WIDTH );
   
   pane_nd_array
}


///  This function creates the simplest possible mosaic which consists of one window of one pane with a single tile
/// 100 by 100 UnknownUnits size
pub fn create_single_tile_data() -> Vec<Vec<( Box2D<i32>, modtile::RGB)>> {

    let mut result_window: Vec<Vec<(Box2D<i32>, modtile::RGB)>> = Vec::new();

    // ****************************
    // Start the first pane
    let mut pane_grid: Vec<(Box2D<i32>, modtile::RGB)> = Vec::new();

    // [(Box2D((0, 0), (100, 100)), RGB(45, 54, 147)),]
    let (tile_box, rgb): (Box2D<i32>, modtile::RGB) = create_data((0, 0), (100, 100), 
                                                                                                    (255, 255, 255));
    let _ = &pane_grid.push((tile_box, rgb));

    // save the pane to the result window
    let _ = &result_window.push(pane_grid);

    result_window
}


/// Create an Array2 nd array of booleans
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

    // go through mosaic_nd_arr and set the corresponding boolean edge 
    // As we only have a single tile with no edges we shall just set all the values to false.
    // For anything more complex we need to visit each tile and compare to neighbour to set the values properly 
    // **********
    // **********
    // TODO need to impliment this algorithm to iterate over rows and cols and compare colours to adjacent cells
    // and set the edges accordingly. As we only have one tile here we can set these manually

    edges[[0,0]][NORTH] = false;
    edges[[0,0]][EAST] = false;
    edges[[0,0]][SOUTH] = false;
    edges[[0,0]][WEST] = false;

    println!("get_edge_bools = {:?}" , &edges);
    // println!("edges[0,0][0] = {:?}" , edges[[0,0]][0]);

    edges

}
