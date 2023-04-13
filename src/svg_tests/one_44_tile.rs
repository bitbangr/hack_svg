#![allow(unused)]

use euclid::default::Box2D;
use crate::constants::{RGB_BLACK,RGB_WHITE,RGB_RED,RGB_GREEN,RGB_BLUE, RGB_YELLOW, RGB_RAW_TITA, RGB_VAN_GREY, RGB_SAP_GREEN, RGB_BRNT_SIENNA, RGB_BRNT_UMBER, RGB_QCRI_VIOLET, RGB_CULN_BLUE, RGB_DIOX_VIOLET, RGB_RAW_UMBER, RGB_RAW_SIENNA, RGB_PAYNES_GREY, RGB_VIOLT_OXIDE, RGB_YEL_OXIDE};
use crate::create_tile;
use crate::mosaic_tile::{RGB, rgb_vec_to_array};
use crate::pane_build_utils::create_pane_test_data;
use crate::svg_utils::create_svg;

// This file holds a 12x12 square mosaic with mosiac self portrait
// using Kroma Colours

pub fn svg_1() {
    let op_svg_file_name = "./svg_output/twelveXtwelve/output_1.svg";
    let rows: usize = 12;
    let cols: usize = 12;
    let tiles_per_pane_height: usize = 12;
    let tiles_per_pane_width: usize = 12;
    let svg_width = 1200;
    let svg_height = 1200 ;

    let rgb_vec: Vec<Vec<(RGB)>> = vec![
        //   [0,0],        [0,1],        [0,2],          [0,3],         [0,4],          [0,5],           [0,6],            [0,7],           [0,8],           [0,9],            [0,10],          [0,11],
        vec![RGB_RAW_TITA, RGB_VAN_GREY, RGB_SAP_GREEN,  RGB_VAN_GREY,  RGB_VAN_GREY,   RGB_VAN_GREY,    RGB_QCRI_VIOLET,  RGB_BRNT_UMBER,  RGB_QCRI_VIOLET, RGB_BRNT_SIENNA, RGB_BRNT_SIENNA, RGB_BRNT_UMBER],
        //   [1,0],        [1,1],        [1,2],          [1,3],         [1,4],          [1,5],           [1,6],            [1,7],           [1,8],           [1,9],            [1,10],           [1,11],
        vec![RGB_VAN_GREY, RGB_VAN_GREY, RGB_SAP_GREEN,  RGB_CULN_BLUE, RGB_SAP_GREEN,  RGB_BRNT_SIENNA,  RGB_BRNT_SIENNA, RGB_BRNT_SIENNA, RGB_BRNT_UMBER,  RGB_BRNT_UMBER,   RGB_BRNT_UMBER,    RGB_SAP_GREEN],
        
        //   [2,0],          [2,1],          [2,2],           [2,3],           [2,4],          [2,5],         [2,6],        [2,7],          [2,8],        [2,9],           [2,10],         [2,11],
        vec![RGB_RAW_UMBER,  RGB_SAP_GREEN,  RGB_BRNT_UMBER,  RGB_DIOX_VIOLET,  RGB_VAN_GREY,  RGB_VAN_GREY,  RGB_VAN_GREY, RGB_RAW_SIENNA, RGB_VAN_GREY, RGB_QCRI_VIOLET, RGB_BRNT_UMBER, RGB_BRNT_UMBER],
        //   [3,0],           [3,1],         [3,2],           [3,3],         [3,4],         [3,5],         [3,6],          [3,7],         [3,8],        [3,9],          [3,10],        [3,11],
        vec![RGB_DIOX_VIOLET, RGB_RAW_UMBER, RGB_BRNT_UMBER,  RGB_VAN_GREY,  RGB_VAN_GREY,  RGB_VAN_GREY,  RGB_RAW_SIENNA, RGB_VAN_GREY, RGB_VAN_GREY, RGB_BRNT_SIENNA, RGB_SAP_GREEN, RGB_CULN_BLUE],
        
        //   [4,0],          [4,1],          [4,2],          [4,3],         [4,4],           [4,5],            [4,6],        [4,7],          [4,8],         [4,9],           [4,10],          [4,11],
        vec![RGB_BRNT_UMBER, RGB_BRNT_UMBER, RGB_SAP_GREEN,  RGB_VAN_GREY,  RGB_VAN_GREY,    RGB_VAN_GREY,     RGB_VAN_GREY, RGB_VAN_GREY,    RGB_VAN_GREY, RGB_VAN_GREY,    RGB_PAYNES_GREY, RGB_BRNT_UMBER],
        //   [5,0],         [5,1],           [5,2],          [5,3],         [5,4],           [5,5],            [5,6],        [5,7],           [5,8],        [5,9],           [5,10],          [5,11],
        vec![RGB_SAP_GREEN, RGB_SAP_GREEN, RGB_SAP_GREEN,  RGB_SAP_GREEN,  RGB_QCRI_VIOLET,  RGB_BRNT_SIENNA,  RGB_VAN_GREY, RGB_BRNT_SIENNA, RGB_VAN_GREY, RGB_BRNT_SIENNA, RGB_DIOX_VIOLET, RGB_BRNT_SIENNA],
        
        //   [6,0],          [6,1],           [6,2],            [6,3],            [6,4],            [6,5],            [6,6],        [6,7],          [6,8],        [6,9],          [6,10],          [6,11],
        vec![RGB_BRNT_UMBER, RGB_SAP_GREEN,   RGB_DIOX_VIOLET,  RGB_BRNT_UMBER,   RGB_BRNT_SIENNA,  RGB_BRNT_SIENNA,  RGB_VAN_GREY, RGB_VAN_GREY,   RGB_VAN_GREY, RGB_VAN_GREY,   RGB_PAYNES_GREY, RGB_BRNT_SIENNA],
        //   [7,0],          [7,1],           [7,2],            [7,3],            [7,4],            [7,5],            [7,6],        [7,7],          [7,8],        [7,9],          [7,10],          [7,11],
        vec![RGB_BRNT_UMBER, RGB_DIOX_VIOLET, RGB_DIOX_VIOLET,  RGB_QCRI_VIOLET,  RGB_VAN_GREY,     RGB_BRNT_SIENNA,  RGB_VAN_GREY, RGB_RAW_SIENNA, RGB_VAN_GREY, RGB_RAW_SIENNA, RGB_BRNT_UMBER,  RGB_BRNT_UMBER],


        //   [8,0],           [8,1],           [8,2],            [8,3],           [8,4],           [8,5],           [8,6],           [8,7],        [8,8],        [8,9],         [8,10],          [8,11],
        vec![RGB_PAYNES_GREY, RGB_PAYNES_GREY, RGB_PAYNES_GREY,  RGB_BRNT_UMBER,  RGB_BRNT_UMBER,  RGB_BRNT_UMBER,  RGB_BRNT_SIENNA, RGB_VAN_GREY, RGB_VAN_GREY, RGB_VAN_GREY,  RGB_BRNT_SIENNA, RGB_BRNT_UMBER],
        //   [9,0],           [9,1],           [9,2],            [9,3],           [9,4],           [9,5],           [9,6],           [9,7],        [9,8],        [9,9],         [9,10],          [9,11],
        vec![RGB_PAYNES_GREY, RGB_DIOX_VIOLET, RGB_PAYNES_GREY,  RGB_SAP_GREEN,   RGB_BRNT_UMBER,  RGB_BRNT_UMBER,  RGB_VIOLT_OXIDE, RGB_VAN_GREY, RGB_VAN_GREY, RGB_YEL_OXIDE, RGB_VAN_GREY,    RGB_QCRI_VIOLET],


        //   [10,0],          [10,1],          [10,2],           [10,3],           [10,4],         [10,5],          [10,6],          [10,7],       [10,8],       [10,9],         [10,10],          [10,11],
        vec![RGB_PAYNES_GREY, RGB_PAYNES_GREY, RGB_PAYNES_GREY,  RGB_PAYNES_GREY,  RGB_RAW_UMBER,  RGB_BRNT_UMBER,  RGB_BRNT_SIENNA, RGB_VAN_GREY, RGB_VAN_GREY, RGB_VAN_GREY,    RGB_VAN_GREY,    RGB_BRNT_UMBER],
        //   [11,0],          [11,1],          [11,2],           [11,3],           [11,4],         [11,5],          [11,6],          [11,7],       [11,8],       [11,9],          [11,10],         [11,11],
        vec![RGB_PAYNES_GREY, RGB_PAYNES_GREY, RGB_PAYNES_GREY,  RGB_PAYNES_GREY,  RGB_SAP_GREEN,  RGB_BRNT_UMBER,  RGB_QCRI_VIOLET, RGB_VAN_GREY, RGB_VAN_GREY, RGB_QCRI_VIOLET, RGB_PAYNES_GREY, RGB_DIOX_VIOLET],                
    ];

    let rgb_arr = rgb_vec_to_array(rgb_vec);
    // println!("rgb_arr {:?}", &rgb_arr);

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
} // svg1


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


pub fn svg_frank_monet_ver7() {
    let op_svg_file_name = "./svg_output/_frank_mosaic/monet_ver7/frank_monet_ver7.svg";
    let rows: usize = 40;
    let cols: usize = 40;
    let tiles_per_pane_height: usize = 40;
    let tiles_per_pane_width: usize = 40;
    let svg_width = 4000;
    let svg_height = 4000 ;

    let rgb_vec_result = read_config("./rgb_json/FrankFolkFest_mar30_Ver7.json");
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
} // svg_frank_monet_ver7


pub fn svg_frank_monet_mar25() {
    let op_svg_file_name = "./svg_output/_frank_mosaic/monet/franky_monet_mar25.svg";
    let rows: usize = 40;
    let cols: usize = 40;
    let tiles_per_pane_height: usize = 40;
    let tiles_per_pane_width: usize = 40;
    let svg_width = 4000;
    let svg_height = 4000 ;

    let rgb_vec_result = read_config("./rgb_json/franky_monet_mar25.json");
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
} // svg_frank_monet_mar25


pub fn svg_frank_mar22() {
    let op_svg_file_name = "./svg_output/_frank_mosaic/frank_mar22.svg";
    let rows: usize = 40;
    let cols: usize = 40;
    let tiles_per_pane_height: usize = 40;
    let tiles_per_pane_width: usize = 40;
    let svg_width = 4000;
    let svg_height = 4000 ;

    let rgb_vec_result = read_config("./rgb_json/franky_mar16.json");
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
} // svg_frank_mar22


pub fn svg_frank_tr12() {
    let op_svg_file_name = "./svg_output/twelveXtwelve/frank_tr12.svg";
    let rows: usize = 12;
    let cols: usize = 12;
    let tiles_per_pane_height: usize = 12;
    let tiles_per_pane_width: usize = 12;
    let svg_width = 1200;
    let svg_height = 1200 ;

    let rgb_vec_result = read_config("./rgb_json/frank_tr12.json");
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

