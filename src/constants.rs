pub const    TOP: usize = 0;
pub const  RIGHT: usize = 1;
pub const BOTTOM: usize = 2;
pub const   LEFT: usize = 3;

pub const  TOP_LEFT: usize = 0 ;
pub const TOP_RIGHT: usize = 1; 
pub const BOT_RIGHT: usize = 2;
pub const  BOT_LEFT: usize = 3;

pub const NOT_VISITED: usize = 0;
pub const     VISITED: usize = 1;

pub const FLAGGED: usize = 987659; // random value unlikely to be used found in problem domain
use crate::mosaic_tile::RGB;
pub const RGB_BLACK: RGB = RGB(0,0,0);
pub const RGB_WHITE: RGB = RGB(255,255,255);
pub const RGB_RED: RGB = RGB(255,0,0);
pub const RGB_GREEN: RGB = RGB(0,255,0);
pub const RGB_BLUE: RGB = RGB(0,0,255);