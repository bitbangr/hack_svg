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
pub const RGB_YELLOW: RGB = RGB(250,237,39);

// Kroma Colours
pub const RGB_HA_LITE_YEL: RGB = RGB(245, 232, 18);
pub const RGB_CAD_LITE_YEL: RGB = RGB(245, 231, 11);
pub const RGB_BENZI_YEL: RGB = RGB(245, 231, 14);
pub const RGB_HA_MED_YEL: RGB = RGB(246, 227, 9);
pub const RGB_CAD_MED_YEL: RGB = RGB(249, 205, 18);
pub const RGB_NKL_AZO_YEL: RGB = RGB(185, 114, 41);
pub const RGB_AYLDE_YEL: RGB = RGB(247, 152, 32);
pub const RGB_CAD_ORNG: RGB = RGB(243, 116, 35);
pub const RGB_BENZ_ORNG: RGB = RGB(240, 84, 37);
pub const RGB_CAD_MED_RED: RGB = RGB(229, 31, 37);
pub const RGB_NPTHL_LITE_RED: RGB = RGB(201, 33, 40);
pub const RGB_NPTHL_MED_RED: RGB = RGB(167, 30, 35);
pub const RGB_CAD_DARK_RED: RGB = RGB(173, 30, 35);
pub const RGB_QCRI_RED: RGB = RGB(183, 33, 39);
pub const RGB_BNZI_BRNT_ORNG: RGB = RGB(117, 19, 24);
pub const RGB_QCRI_VIOLET: RGB = RGB(82, 19, 31);
pub const RGB_QCRI_MAGENTA: RGB = RGB(161, 28, 71);
pub const RGB_ULTM_VIOLET: RGB = RGB(68, 76, 159);
pub const RGB_DIOX_VIOLET: RGB = RGB(13, 16, 16);
pub const RGB_PAYNES_GREY: RGB = RGB(7, 8, 10);
pub const RGB_ULTM_BLUE: RGB = RGB(45, 54, 147);
pub const RGB_COBLT_BLUE: RGB = RGB(40, 49, 137);
pub const RGB_CULN_BLUE: RGB = RGB(30, 54, 105);
pub const RGB_COBLT_TEAL: RGB = RGB(23, 147, 173);
pub const RGB_PHTO_BLUE: RGB = RGB(31, 27, 79);
pub const RGB_PHTO_GREEN: RGB = RGB(8, 29, 34);
pub const RGB_PERM_GREEN: RGB = RGB(24, 159, 72);
pub const RGB_CHRM_OXIDE_GREEN: RGB = RGB(41, 115, 56);
pub const RGB_SAP_GREEN: RGB = RGB(34, 35, 31);
pub const RGB_YEL_OXIDE: RGB = RGB(178, 137, 45);
pub const RGB_TNSPNT_YEL_OXIDE: RGB = RGB(218, 133, 40);
pub const RGB_RAW_SIENNA: RGB = RGB(131, 86, 35);
pub const RGB_TNSPNT_RED_OXIDE: RGB = RGB(127, 39, 24);
pub const RGB_BRNT_SIENNA: RGB = RGB(116, 63, 27);
pub const RGB_RED_OXIDE: RGB = RGB(115, 36, 21);
pub const RGB_ALIZ_CRIM_HUE: RGB = RGB(103, 15, 24);
pub const RGB_VIOLT_OXIDE: RGB = RGB(97, 25, 26);
pub const RGB_BRNT_UMBER: RGB = RGB(58, 24, 24);
pub const RGB_RAW_UMBER: RGB = RGB(36, 26, 27);
pub const RGB_VAN_GREY: RGB = RGB(109, 97, 91);
pub const RGB_RAW_TITA: RGB = RGB(199, 174, 145);
pub const RGB_IRID_GOLD: RGB = RGB(209, 147, 70);
pub const RGB_PERL_WHITE: RGB = RGB(220, 214, 210);

// SVG scale factors for creating tile sizes of 1/2" or 12.7mm based on 96 PPI Points to Inches conversion
pub const SVG_PPI :f32 = 96.0;       
pub const SVG_SCALE_X :f32 = 0.48; 
pub const SVG_SCALE_Y :f32 = 0.48; 
pub const SVG_STROKE_WIDTH :f32 = 0.0;
