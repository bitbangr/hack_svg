const BOARD_SIZE: i32 = 8;
const RECT_SIZE: i32 = 50;
const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;
const CELL_SIZE: i32 = 100;
const COLOR_BLK: &str = "black";
const COLOR_WHT: &str = "white";

pub const NORTH: usize = 0;
pub const  EAST: usize = 1;
pub const SOUTH: usize = 2;
pub const  WEST: usize = 3;

pub const NW_CORNER: usize = 0;
pub const NE_CORNER: usize = 1; 
pub const SE_CORNER: usize = 2;
pub const SW_CORNER: usize = 3;

pub const    TOP: usize = 0;
pub const  RIGHT: usize = 1;
pub const BOTTOM: usize = 2;
pub const   LEFT: usize = 3;

pub const  TOP_LEFT: usize = 0 ;
pub const TOP_RIGHT: usize = 1; 
pub const BOT_RIGHT: usize = 2;
pub const  BOT_LEFT: usize = 3;

pub const FLAGGED: usize = 987659; // random value unlikely to be used found in problem domain
// pub const RGB_BLACK: (u8,u8,u8) = (0,0,0);
// pub const RGB_WHITE: (u8,u8,u8) = (255,255,255);
// pub const RGB_RED: (u8,u8,u8) = (255,0,0);
// pub const RGB_GREEN: (u8,u8,u8) = (0,255,0);
// pub const RGB_BLUE: (u8,u8,u8) = (0,0,255);

use crate::mosaic_tile::RGB;
pub const RGB_BLACK: RGB = RGB(0,0,0);
pub const RGB_WHITE: RGB = RGB(255,255,255);
pub const RGB_RED: RGB = RGB(255,0,0);
pub const RGB_GREEN: RGB = RGB(0,255,0);
pub const RGB_BLUE: RGB = RGB(0,0,255);
