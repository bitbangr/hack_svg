
use euclid::default::{Box2D, Point2D};


use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::io::{Read};
use std::fmt::{self, Formatter, Display};



pub const    TOP: usize = 0;
pub const  RIGHT: usize = 1;
pub const BOTTOM: usize = 2;
pub const   LEFT: usize = 3;

pub const  TOP_LEFT: usize = 0 ;
pub const TOP_RIGHT: usize = 1; 
pub const BOT_RIGHT: usize = 2;
pub const  BOT_LEFT: usize = 3;

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq, Hash, Default)]
// #[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Default)]
pub struct RGB(pub u8,pub u8,pub u8);

impl Display for RGB {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "rgb ({}, {}, {})", self.0, self.1, self.2)
    }
}

// (Box2D<i32>, modtile::RGB)

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct MosaicTile {
    tile: Tile,
    edge_bool: Vec<bool>,
}

impl MosaicTile {
    pub fn new(tile: Tile, edge_bool: Vec<bool>) -> MosaicTile {
        MosaicTile { tile, edge_bool }
    }
}



use num_traits::Zero;

impl Zero for MosaicTile {
    fn zero() -> Self {
        MosaicTile {
            tile: Tile {
                coords: Box2D::new(Point2D::new(0,0),Point2D::new(0,0)),
                rgb: RGB(0, 0, 0),
            },
            edge_bool: Vec::new(),
        }
    }

    fn is_zero(&self) -> bool {
        self.tile.coords.min == Point2D::new(0,0) && self.tile.coords.max == Point2D::new(0,0) && self.tile.rgb == RGB(0, 0, 0) && self.edge_bool.is_empty()
    }
}

use std::ops::Add;

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
            tile:self.tile,
            edge_bool: self.edge_bool,
            
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

    pub fn as_tuple(&self) -> (Box2D<i32>, RGB)  {
        (self.coords, self.rgb)
    }

    pub fn corners(&self) -> [(usize, usize); 4] {
        let x0 = self.coords.min.x.try_into().unwrap();
        let y0 = self.coords.min.y.try_into().unwrap();
        let x1 = self.coords.max.x.try_into().unwrap();
        let y1 = self.coords.max.y.try_into().unwrap();
        [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
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
