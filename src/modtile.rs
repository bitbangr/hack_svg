// THIS FILE SHOULD NOT BE MERGED BACK TO MAIN BRANCH 
// the version in IMG_TILE should be kept

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::io::{Read};
// use std::fmt::{self, Formatter, Display};
use crate::mosaic_tile::RGB;

// #[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq, Hash, Default)]
// pub struct RGB(pub u8,pub u8,pub u8);

// impl Display for RGB {
//     // `f` is a buffer, and this method must write the formatted string into it
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {

//         // `write!` is like `format!`, but it will write the formatted string
//         // into a buffer (the first argument)
//         write!(f, "rgb ({}, {}, {})", self.0, self.1, self.2)
//     }
// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TileColor {
    pub rgb: RGB,
    pub name: String,
    pub number: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AllColors{
    pub name: String,
    pub url: String,
    pub description: String,
    pub colors: Vec<TileColor>
}

// load tile color files from a json file
// todo need throw exception for missing or deformed file
pub fn _load_all_colors(path_str: &str) -> AllColors {
    let path = Path::new(path_str);

    match File::open(path) {
       Ok(mut file) => {
           println!();
           println!("Tile Colour File was successfully opened");
           let mut buf = vec![];
           if file.read_to_end(&mut buf).is_ok() {
               match serde_json::from_slice(&buf[..]) {
                   Ok(all_colors) => return all_colors,
                   Err(e) => {
                       eprintln!("Could not read Colour file {:?} \n  {}", path, e );
                       panic!("Improperly formed JSON file");
                   }
               } // match serde_json
           }
           else {
               eprintln!("Could not read tile colors file {:?}", path);
               panic!("Improperly formed JSON file");
           }
       },
       Err(e) => {
           eprintln!(" Could not open Colors file {:?} \n  {}", path, e );
           panic!("Missing Tile Colors File");
        },
     }

    // if let Ok(mut file) = File::open(path) {
    //     let mut buf = vec![];
    //     if file.read_to_end(&mut buf).is_ok() {
    //         if let Ok(all_colors) = serde_json::from_slice(&buf[..]) {
    //             return all_colors;
    //         }
    //     }
    // }
    // // todo  update error handling
    // // See load_configs
    // // There was no file, or the file failed to load, create a new All_Colors.
    // println!("no file, or the file failed to load, create a new All_Colors\n*****\n*****\nThere was a problem \n*****\n*****" );
    //
    // let tc1 = TileColor { rgb: RGB(0,0,0), name: "black".to_owned() , number: "0".to_owned() };
    // AllColors{name:"Hack".to_owned(),url:"none".to_owned(),description:"MadeUp".to_owned(), colors: vec![tc1] }
}