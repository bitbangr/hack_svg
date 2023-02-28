use euclid::default::{Box2D, Point2D};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use svg_utils::match_edge_boolean_pattern;

use crate::constants::{TOP, RIGHT,BOTTOM,LEFT,FLAGGED};
use crate::constants::{TOP_LEFT, TOP_RIGHT,BOT_LEFT, BOT_RIGHT};
use crate::svg_utils;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq, Hash, Default)]
// #[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Default)]
pub struct RGB(pub u8, pub u8, pub u8);

impl Display for RGB {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "rgb ({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Zero for RGB {
    fn zero() -> Self {
        RGB(0, 0, 0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0 && self.2 == 0
    }
}

impl Add for RGB {
    type Output = RGB;

    fn add(self, other: RGB) -> RGB {
        RGB(
            self.0.saturating_add(other.0),
            self.1.saturating_add(other.1),
            self.2.saturating_add(other.2),
        )
    }
}

impl RGB {
    pub fn new_with_u8(in_rgb: (u8, u8, u8)) -> RGB {
        RGB(in_rgb.0, in_rgb.1, in_rgb.2)
    }
}

/// Util function to Take a vector of RGB and return an appropriately sized array
///
pub fn rgb_vec_to_array(rgb_vec: Vec<Vec<RGB>>) -> Array2<RGB> {
    let nrows = rgb_vec.len();
    let ncols = rgb_vec[0].len();
    let mut arr = Array2::zeros((nrows, ncols));

    for (i, row) in rgb_vec.iter().enumerate() {
        for (j, &rgb) in row.iter().enumerate() {
            arr[[i, j]] = rgb;
        }
    }

    arr
}

// ********************************************************
// ********************************************************

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct MosaicTile {
    pub tile: Tile,
    pub edge_bool: Vec<bool>,
    pub bpoints: [Box2D<i32>; 2],  // Using Box2D struct to hold Start Point End Point pairs Clockwise And Counter Clockwise
    pub start_point: Point2D<i32>,
    pub end_point: Point2D<i32>,
    pub start_point_two: Point2D<i32>,
    pub end_point_two: Point2D<i32>,
}

impl MosaicTile {
    pub fn new(tile: Tile, edge_bool: Vec<bool>) -> MosaicTile {
        let (sp, ep, sp2, ep2) = get_start_end_points(&edge_bool, tile);

        let sp_points = [
            Box2D::new(sp, ep),    // 
            Box2D::new(sp2, ep2),  // Set to sp2 ep2 are Point2D(FLAGGED,FLAGGED) unless this is a TFTF or FTFT tile
        ];
        
        MosaicTile {
            tile,
            edge_bool,
            bpoints: sp_points ,
            start_point: sp,
            end_point: ep,
            start_point_two: sp2,
            end_point_two: ep2,
        }
    }

    pub fn get_start_point_as_i32(&self) -> (i32, i32) {
        let x: i32 = self.start_point.x.try_into().unwrap();
        let y: i32 = self.start_point.y.try_into().unwrap();
        (x, y)
    }
    
}

/// helper function to generate a Point2D from a usize array (x,y)
fn get_point_2d(usize_arr: (usize, usize)) -> Point2D<i32> {
    let start_x: i32 = usize_arr.0.try_into().unwrap();
    let start_y: i32 = usize_arr.1.try_into().unwrap();

    Point2D::new(start_x, start_y)
}






///
/// This function calculate the start endpoints for each Tile configuration 
/// Values are passed back to MosaicTile Constructor

fn get_start_end_points(
    edge_bool: &[bool],
    tile: Tile,
) -> (Point2D<i32>, Point2D<i32>, Point2D<i32>, Point2D<i32>) {
    let top = edge_bool[TOP];
    let right = edge_bool[RIGHT];
    let bottom = edge_bool[BOTTOM];
    let left = edge_bool[LEFT];

    let mut start_point: Point2D<i32> = Point2D::new(0, 0);
    let mut end_point: Point2D<i32> = Point2D::new(0, 0);

    // Second start end points default to Point2D (FLAGGED, FLAGGED)
    // and remain so unless set otherwise in the FTFT and TFTF cases below
    let mut start_point_two: Point2D<i32> = get_point_2d((FLAGGED, FLAGGED));
    let mut end_point_two: Point2D<i32> = get_point_2d((FLAGGED, FLAGGED));

    let corners = &tile.corners();

    println!("\n----------------------- ");
    println!("mosaic_tile Constructor \n\t fn get_start_end_points()");

    match (top, right, bottom, left) {
        // *******************************************
        // Fully closed tiles are by definition the only element in the contigous tile collection
        // don't need to look for next tile
        // **********************************
        // Start of four false edge case
        // **********************************
        (false, false, false, false) => {
            // FFFF
            println!("\nmatch -> false false false false - single tile");
            println!(" TOP RIGHT BOTTOM LEFT fully closed single tile\n");

            start_point = get_point_2d(corners[TOP_LEFT]);
            end_point = get_point_2d(corners[TOP_LEFT]);

            println! {"start point TOP_LEFT-> {:?} ", corners[TOP_LEFT]};
            println! {"end point TOP_LEFT-> {:?} ", corners[TOP_LEFT]};
        } // FFFF
        // **********************************
        // Start of three false edge cases
        // **********************************
        (true, false, false, false) => {
            //TFFF
            println!("\nmatch -> true false false false - top open");
            println!(" BOTTOM LEFT RIGHT Closed - Top side open tile\n");

            start_point = get_point_2d(corners[TOP_RIGHT]);
            end_point = get_point_2d(corners[TOP_LEFT]);

            println! {"start point TOP_RIGHT-> {:?} ", &start_point};
            println! {"end point TOP_LEFT-> {:?} ", &end_point};
        } // TFFF
        // **********************************
        (false, true, false, false) => {
            //FTFF
            println!("\nmatch -> false true false false - right open");
            println!(" TOP BOTTOM LEFT Closed - Right side open tile\n");

            start_point = get_point_2d(corners[BOT_RIGHT]);
            end_point = get_point_2d(corners[TOP_RIGHT]);

            println! {"start point BOT_RIGHT-> {:?} ", &start_point};
            println! {"end point TOP_RIGHT-> {:?} ", &end_point};
        } // FTFF
        // **********************************
        (false, false, true, false) => {
            //FFTF
            println!("\nmatch -> false false true false - bottom open");
            println!(" TOP/LEFT/RIGHT Closed - bottom side open tile\n");

            start_point = get_point_2d(corners[BOT_LEFT]);
            end_point = get_point_2d(corners[BOT_RIGHT]);
            // update the current tile line end point

            println! {"start point BOT_LEFT-> {:?} ", &start_point};
            println! {"end point BOT_RIGHT-> {:?} ", &end_point};
        } // FFTF

        // **********************************
        (false, false, false, true) => {
            //FFFT
            println!("\nmatch -> false false false true - left open");
            println!(" TOP RIGHT BOTTOM Closed - Left side Open tile\n");

            start_point = get_point_2d(corners[TOP_LEFT]);
            end_point = get_point_2d(corners[BOT_LEFT]);

            println! {"start point TOP_LEFT-> {:?} ", &start_point};
            println! {"end point BOT_LEFT-> {:?} ", &end_point};
        } // FFFT
        // **********************************
        // Start of two false edge cases
        // **********************************
        (false, false, true, true) => {
            //FFTT
            println!("\nmatch -> false false true true - bottom left open");
            println!(" TOP/RIGHT Closed - Bottom-Left side open tile\n");

            start_point = get_point_2d(corners[TOP_LEFT]);
            end_point = get_point_2d(corners[BOT_RIGHT]);
            // update the current tile line end point

            println! {"start point TOP_LEFT-> {:?} ", &start_point};
            println! {"end point BOT_RIGHT-> {:?} ", &end_point};
        } // FFTT
        // **********************************
        (true, false, false, true) => {
            //TFFT
            println!("\nmatch -> true false false true - top/left open");
            println!(" BOTTOM RIGHT Closed - Top-Left side open tile\n");

            start_point = get_point_2d(corners[TOP_RIGHT]);
            end_point = get_point_2d(corners[BOT_LEFT]);

            println! {"start point TOP_RIGHT-> {:?} ", &start_point};
            println! {"end point BOT_LEFT-> {:?} ", &end_point};
        } // TFFT
        // **********************************
        (true, true, false, false) => {
            //TTFF
            println!("\nmatch -> true true false false - top/right open");
            println!(" BOTTOM LEFT Closed - Top-Right side open tile\n");

            start_point = get_point_2d(corners[BOT_RIGHT]);
            end_point = get_point_2d(corners[TOP_LEFT]);

            println! {"start point BOT_RIGHT-> {:?} ", &start_point};
            println! {"end point TOP_LEFT-> {:?} ", &end_point};
        } // TTFF
        // **********************************
        (false, true, true, false) => {
            //FTTF
            println!("\nmatch -> false true true false - right/bottom open");
            println!(" TOP LEFT Closed - Right-Bottom side open tile\n");

            start_point = get_point_2d(corners[BOT_LEFT]);
            end_point = get_point_2d(corners[TOP_RIGHT]);

            println! {"start point BOT_LEFT-> {:?} ", &start_point};
            println! {"end point TOP_RIGHT-> {:?} ", &end_point};
        } // FTTF

        // **********************************
        // NOTE THESE NEXT TWO CASES HAVE two start points and two end points
        // Need to handle this somehow
        // TODO REVISIT
        // **********************************
        // **********************************
        (false, true, false, true) => {
            //FTFT
            println!("\nmatch -> false true false true - left/right open");
            println!(" TOP BOTTOM Closed - Left-Right side open tile\n");
            println!(" !!!!! CHECK THIS CODE LOGIC !!!!!\n");

            start_point = get_point_2d(corners[TOP_LEFT]);
            end_point = get_point_2d(corners[TOP_RIGHT]);
            // pairs of start/end points are opposite direction assuming clockwise direction of travel around tile
            // Each start end needs to be evaluated on a case by case basis as direction 
            // of drawn line depends on where in the mosiac this tile is encountered
            start_point_two = get_point_2d(corners[BOT_RIGHT]);
            end_point_two = get_point_2d(corners[BOT_LEFT]);

            println! {"1st line - start point TOP_LEFT-> {:?} ", &start_point};
            println! {"1st line - end point TOP_RIGHT-> {:?} ", &end_point};
            println! {"2nd line - start point corners[BOT_RIGHT]-> {:?} ", &start_point_two};
            println! {"2nd line - end point corners[BOT_LEFT]-> {:?} ", &end_point_two};

            // panic!();
        } // FTFT
        // **********************************
        (true, false, true, false) => {
            //TFTF
            println!("\nmatch -> true false true false - top/bottom open");
            println!(" LEFT RIGHT Closed - Top-Bottom side open tile\n");
            println!(" !!!!! CHECK THIS CODE LOGIC !!!!!\n");

            start_point = get_point_2d(corners[BOT_LEFT]);
            end_point = get_point_2d(corners[TOP_LEFT]);
            // pairs of start/end points are opposite direction assuming clockwise direction of travel around tile
            // Each start end needs to be evaluated on a case by case basis as direction 
            // of drawn line depends on where in the mosiac this tile is encountered
            start_point_two = get_point_2d(corners[TOP_RIGHT]);
            end_point_two = get_point_2d(corners[BOT_RIGHT]);

            println! {"1st line - start point BOT_LEFT-> {:?} ", &start_point};
            println! {"1st line - end point TOP_LEFT-> {:?} ", &end_point};
            println! {"2nd line - start point corners[TOP_RIGHT]-> {:?} ", &start_point_two};
            println! {"2nd line - end point corners[BOT_RIGHT]-> {:?} ", &end_point_two};

            // panic!();
        } // TFTF
        // **********************************
        // Start of single false edge cases
        // **********************************
        // **********************************
        (false, true, true, true) => {
            //FTTT
            println!("\nmatch -> false true true true - right/left/bottom open");
            println!(" TOP Closed - Right-Left-Bottom side open tile\n");

            start_point = get_point_2d(corners[TOP_LEFT]);
            end_point = get_point_2d(corners[TOP_RIGHT]);

            println! {"start point TOP_LEFT-> {:?} ", &start_point};
            println! {"end point TOP_RIGHT-> {:?} ", &end_point};
        } // FTTT
        // **********************************
        (true, false, true, true) => {
            //TFTT
            println!("\nmatch -> true false true true - top/bottom/left open");
            println!(" RIGHT Closed - Top-Bottom-Left side open tile\n");

            start_point = get_point_2d(corners[TOP_RIGHT]);
            end_point = get_point_2d(corners[BOT_RIGHT]);

            println! {"start point TOP_RIGHT-> {:?} ", &start_point};
            println! {"end point BOT_RIGHT-> {:?} ", &end_point};
        } // TFTT
        // **********************************
        (true, true, false, true) => {
            //TTFT
            println!("\nmatch -> true true false true - top/left/right open");
            println!(" BOTTOM Closed - Top-Left-Right side open tile\n");

            start_point = get_point_2d(corners[BOT_RIGHT]);
            end_point = get_point_2d(corners[BOT_LEFT]);

            println! {"start point BOT_RIGHT-> {:?} ", &start_point};
            println! {"end point BOT_LEFT-> {:?} ", &end_point};
        } // TTFT
        // **********************************
        (true, true, true, false) => {
            //TTTF
            println!("\nmatch -> true true true false - top/right/bottom open");
            println!(" LEFT Closed - Top-Right-Bottom side open tile\n");

            start_point = get_point_2d(corners[BOT_LEFT]);
            end_point = get_point_2d(corners[TOP_LEFT]);

            println! {"start point BOT_LEFT-> {:?} ", &start_point};
            println! {"end point TOP_LEFT-> {:?} ", &end_point};
        } // TTTF

        // **********************************
        // Start of zere false edge case
        // **********************************
        (true, true, true, true) => {
            //TTTT
            println!("\nmatch -> true true true true - top/right/bottom/left open");
            println!(" NO EDGES - Top-Right-Bottom-Left side open tile\n");

            // start and points don't matter as there are no edges so just setting to top right for defaul
            start_point = get_point_2d(corners[TOP_RIGHT]);
            end_point = get_point_2d(corners[TOP_RIGHT]);

            println! {"start point TOP_RIGHT-> {:?} ", &start_point};
            println! {"end point TOP_RIGHT-> {:?} ", &end_point};
        } // TTTT
        _ => {
            println!("The EDGE Boolean does not match any of the options\n");
            panic!();
        }
    } // match

    println!("\n----------------------- ");

    (start_point, end_point, start_point_two, end_point_two)
} // get_start_end_points

impl MosaicTile {
    // fn set_hp(&mut self, hp: &i32) {
    //     self.hp = *hp;
    // }

    pub fn is_ftft ( &self) -> bool {

        // let match_this_tftf = [Some(true), Some(false), Some(true), Some(false)];
        // let tile_is_tftf :bool = match_edge_boolean_pattern(match_this_tftf, &cur_tile_edge_bool);
        // let cur_tile_edge_bool = cur_tile.edge_bool.clone();
        // let tile_is_ftft :bool = match_edge_boolean_pattern(match_this_ftft, &self);

        let match_this_ftft = [Some(false), Some(true), Some(false), Some(true)];        
        match_edge_boolean_pattern(match_this_ftft, &self.edge_bool)

    }

    pub fn set_start_point(&mut self, start_point: &Point2D<i32>) {
        self.start_point = *start_point;
    }

    pub fn set_end_point(&mut self, end_point: &Point2D<i32>) {
        self.end_point = *end_point;
    }

    pub fn set_start_end_point(&mut self, start_point: &Point2D<i32>, end_point: &Point2D<i32>) {
        self.start_point = *start_point;
        self.end_point = *end_point;
    }

    pub fn set_start_end_points_to_zero(&mut self) {
        self.start_point = Point2D::new(0, 0);
        self.end_point = Point2D::new(0, 0);
    }
}

use num_traits::Zero;

impl Zero for MosaicTile {
    fn zero() -> Self {
        MosaicTile {
            tile: Tile {
                coords: Box2D::new(Point2D::new(0, 0), Point2D::new(0, 0)),
                rgb: RGB(0, 0, 0),
            },
            edge_bool: Vec::new(),
            bpoints: [Box2D::new(Point2D::new(0, 0), Point2D::new(0, 0)), Box2D::new(Point2D::new(0, 0), Point2D::new(0, 0))],
            start_point: Point2D::new(0, 0),
            end_point: Point2D::new(0, 0),
            start_point_two: Point2D::new(0, 0),
            end_point_two: Point2D::new(0, 0),
        }
    }

    fn is_zero(&self) -> bool {
        self.tile.coords.min == Point2D::new(0, 0)
            && self.tile.coords.max == Point2D::new(0, 0)
            && self.tile.rgb == RGB(0, 0, 0)
            && self.start_point == Point2D::new(0, 0)
            && self.end_point == Point2D::new(0, 0)
            && self.edge_bool.is_empty()
    }
}

use std::ops::Add;



// use crate::constants::{EAST, FLAGGED, NORTH, SOUTH, WEST};

impl Add for MosaicTile {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        MosaicTile {
            // WARNING WARNING WARNING
            //
            // THIS ADD FUNCTION IS BORKED.
            //
            // JUST RETURNS THE FIRST ELEMENT
            // This is here so that code compiles
            //
            // tile: self.tile + other.tile,
            // edge_bool: self.edge_bool + other.edge_bool,
            tile: self.tile,
            edge_bool: self.edge_bool,
            bpoints: self.bpoints,
            start_point: self.start_point,
            end_point: self.end_point,
            start_point_two: self.start_point,
            end_point_two: self.end_point,
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone, Hash)]
pub struct Tile {
    pub coords: Box2D<i32>,
    pub rgb: RGB,
}

impl Tile {
    pub fn new(coords: Box2D<i32>, rgb: RGB) -> Tile {
        Tile { coords, rgb }
    }

    pub fn new_withpoints(
        top_left: (i32, i32),
        bot_right: (i32, i32),
        rgb_val: (u8, u8, u8),
    ) -> Tile {
        let p_start: Point2D<i32> = Point2D::new(top_left.0, top_left.1);
        let p_end: Point2D<i32> = Point2D::new(bot_right.0, bot_right.1);
        let coords: Box2D<i32> = Box2D {
            min: p_start,
            max: p_end,
        };
        let rgb: RGB = RGB(rgb_val.0, rgb_val.1, rgb_val.2);
        Tile { coords, rgb }
    }

    pub fn as_tuple(&self) -> (Box2D<i32>, RGB) {
        (self.coords, self.rgb)
    }

    pub fn corners(&self) -> [(usize, usize); 4] {
        let x0 = self.coords.min.x.try_into().unwrap();
        let y0 = self.coords.min.y.try_into().unwrap();
        let x1 = self.coords.max.x.try_into().unwrap();
        let y1 = self.coords.max.y.try_into().unwrap();
        [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
    }

    pub fn get_tile_corners(&self) -> [Point2D<i32>; 4] {
        let top_left = self.coords.min;
        let top_right = Point2D::new(self.coords.max.x, self.coords.min.y);
        let bottom_right = self.coords.max;
        let bottom_left = Point2D::new(self.coords.min.x, self.coords.max.y);
        [top_left, top_right, bottom_right, bottom_left]
    }

    pub fn top_left(&self) -> (usize, usize) {
        let x0 = self.coords.min.x.try_into().unwrap();
        let y0 = self.coords.min.y.try_into().unwrap();
        // let x1 = self.coords.max.x.try_into().unwrap();
        // let y1 = self.coords.max.y.try_into().unwrap();
        (x0, y0)
    }

    pub fn top_right(&self) -> (usize, usize) {
        // let x0 = self.coords.min.x.try_into().unwrap();
        let y0 = self.coords.min.y.try_into().unwrap();
        let x1 = self.coords.max.x.try_into().unwrap();
        // let y1 = self.coords.max.y.try_into().unwrap();
        (x1, y0)
    }

    pub fn bot_right(&self) -> (usize, usize) {
        // let x0 = self.coords.min.x.try_into().unwrap();
        // let y0 = self.coords.min.y.try_into().unwrap();
        let x1 = self.coords.max.x.try_into().unwrap();
        let y1 = self.coords.max.y.try_into().unwrap();
        (x1, y1)
    }

    pub fn bot_left(&self) -> (usize, usize) {
        let x0 = self.coords.min.x.try_into().unwrap();
        // let y0 = self.coords.min.y.try_into().unwrap();
        // let x1 = self.coords.max.x.try_into().unwrap();
        let y1 = self.coords.max.y.try_into().unwrap();
        (x0, y1)
    }
}
