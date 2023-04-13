#![allow(unused)]

use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE,RGB_RED,RGB_GREEN,RGB_BLUE, RGB_YELLOW, RGB_RAW_TITA, RGB_VAN_GREY, RGB_SAP_GREEN, RGB_BRNT_SIENNA, RGB_BRNT_UMBER, RGB_QCRI_VIOLET, RGB_CULN_BLUE, RGB_DIOX_VIOLET, RGB_RAW_UMBER, RGB_RAW_SIENNA, RGB_PAYNES_GREY, RGB_VIOLT_OXIDE, RGB_YEL_OXIDE};
use crate::create_tile;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::create_svg;

// This file used to create a  60x60 tile mosaic of mary blaze
// using Kroma Colours



pub fn svg_frank() {
    let op_svg_file_name = "./svg_output/twelveXtwelve/frank.svg";
    let rows: usize = 40;
    let cols: usize = 40;
    let tiles_per_pane_height: usize = 40;
    let tiles_per_pane_width: usize = 40;
    let svg_width = 4000;
    let svg_height = 4000 ;

    let rgb_vec_result = read_config("./rgb_json/franky.json");
    let rgb_vec = match rgb_vec_result {
        Ok(rgb_vec) => rgb_vec,
        Err(e) => {
            println!("Error reading config: {:?}", e);
            panic!();
        }
    };
    
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    // println!("svg1 data_results = {:?}", data_results);

    let _ = create_svg(
        op_svg_file_name,
        svg_width,
        svg_height,
        rows,
        cols,
        tiles_per_pane_height,
        tiles_per_pane_width,
        data_results,
    );
} // svg_frank

pub fn svg_frank_monet_ver6() {
    let op_svg_file_name = "./svg_output/_frank_mosaic/monet_green_bkgnd/frank_monet_ver6.svg";
    let rows: usize = 40;
    let cols: usize = 40;
    let tiles_per_pane_height: usize = 40;
    let tiles_per_pane_width: usize = 40;
    let svg_width = 4000;
    let svg_height = 4000 ;

    let rgb_vec_result = read_config("./rgb_json/FrankFolkFest_GreenBkgnd_Ver6.json");
    let rgb_vec = match rgb_vec_result {
        Ok(rgb_vec) => rgb_vec,
        Err(e) => {
            println!("Error reading config: {:?}", e);
            panic!();
        }
    };
    
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    // println!("svg1 data_results = {:?}", data_results);

    let _ = create_svg(
        op_svg_file_name,
        svg_width,
        svg_height,
        rows,
        cols,
        tiles_per_pane_height,
        tiles_per_pane_width,
        data_results,
    );
} // svg_frank_monet_ver6


pub fn svg_mary_blaze_crayola_ver7() {
    let op_svg_file_name = "./svg_output/_mary_blaze/ver7/mb_crayola_v7.svg";
    let rows: usize = 60;
    let cols: usize = 60;
    let tiles_per_pane_height: usize = 60;
    let tiles_per_pane_width: usize = 60;
    let svg_width = 6000;
    let svg_height = 6000 ;

    let rgb_vec_result = read_config("./rgb_json/mb_cray_apr_23_v7_op.json");
    let rgb_vec = match rgb_vec_result {
        Ok(rgb_vec) => rgb_vec,
        Err(e) => {
            println!("Error reading config: {:?}", e);
            panic!();
        }
    };
    
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 100;
    let row_height: i32 = 100;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    // println!("svg1 data_results = {:?}", data_results);

    let _ = create_svg(
        op_svg_file_name,
        svg_width,
        svg_height,
        rows,
        cols,
        tiles_per_pane_height,
        tiles_per_pane_width,
        data_results,
    );
} // svg_mary_blaze_crayola_ver7


pub fn svg_mary_blaze_crayola_ver7_small_tiles() {
    let op_svg_file_name = "./svg_output/_mary_blaze/ver7_small_tile/mb_crayola_v7.svg";
    let rows: usize = 60;
    let cols: usize = 60;
    let tiles_per_pane_height: usize = 60;
    let tiles_per_pane_width: usize = 60;
    let svg_width = 4020;
    let svg_height = 4020 ;

    let rgb_vec_result = read_config("./rgb_json/mb_cray_apr_23_v7_op.json");
    let rgb_vec = match rgb_vec_result {
        Ok(rgb_vec) => rgb_vec,
        Err(e) => {
            println!("Error reading config: {:?}", e);
            panic!();
        }
    };
    
    let rgb_arr = rgb_vec_to_array(rgb_vec);

    let col_width: i32 = 67;
    let row_height: i32 = 67;
    let data_results = create_pane_test_data(rgb_arr, row_height, col_width);

    // println!("svg1 data_results = {:?}", data_results);

    let _ = create_svg(
        op_svg_file_name,
        svg_width,
        svg_height,
        rows,
        cols,
        tiles_per_pane_height,
        tiles_per_pane_width,
        data_results,
    );
} // svg_mary_blaze_crayola_ver7_small_tiles


use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    tiles: Vec<Vec<RGB>>,
}

fn read_config(file_name: &str) -> Result<Vec<Vec<RGB>>, Box<dyn Error>> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_json::from_str(&contents)?;
    Ok(config.tiles)
}

