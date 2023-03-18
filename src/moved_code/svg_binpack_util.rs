// Describe a bin packing algorithm that takes an svg file of interconnected shapes and separates them so no shape is touching another
// shape and all shapes are grouped by fill colour using rust programming language
// To accomplish this task, we can use a variant of the bin packing algorithm called the "guillotine algorithm,"
// which divides the available space into smaller rectangles using vertical or horizontal cuts.
// In addition, we can use Rust's built-in SVG parsing and manipulation libraries, such as usvg and svgtypes,
// to read in the SVG file and separate the shapes based on their fill color.

// Here are the steps we can follow to implement this algorithm:

// 1. Parse the SVG file using usvg to extract the shapes and their fill colors.
// We can also calculate the dimensions of each shape using the svgtypes library.

// 2. Sort the shapes based on their fill color. This can be done using Rust's built-in sorting functions.

// 3. Initialize a rectangle that represents the available space, starting with the dimensions of the largest shape.

// 4. For each shape, try to fit it into the available space using the guillotine algorithm. If it fits, place it in the space and update the available space.

// 5. If the shape doesn't fit, create a new rectangle with the dimensions of the shape and repeat step 4.

// 6. Continue this process for all shapes in the SVG file.

// 7. Output the separated shapes as individual SVG files, with each file containing all the shapes of a single fill color.

// Here is some sample Rust code that implements this algorithm:

use ndarray::Shape;
use usvg::{Tree, roxmltree};
use svgtypes::{ Length, LengthUnit };
use std::collections::HashMap;

#[derive(Clone)]
struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
        Rectangle { x, y, width, height }
    }
}

#[derive(Clone)]
struct Bin {
    width: f64,
    height: f64,
    rectangles: Vec<Rectangle>,
}

impl Bin {
    fn new(width: f64, height: f64) -> Bin {
        Bin { width, height, rectangles: vec![] }
    }

    fn fits(&self, rect: &Rectangle) -> bool {
        rect.width <= self.width && rect.height <= self.height
    }

    fn split_horizontally(&self, split: f64) -> (Bin, Bin) {
        let bin1 = Bin::new(split, self.height);
        let bin2 = Bin::new(self.width - split, self.height);
        (bin1, bin2)
    }

    fn split_vertically(&self, split: f64) -> (Bin, Bin) {
        let bin1 = Bin::new(self.width, split);
        let bin2 = Bin::new(self.width, self.height - split);
        (bin1, bin2)
    }

    fn insert(&mut self, rect: Rectangle) {
        self.rectangles.push(rect);
    }
}




// use usvg::prelude::Shape;

fn collect_rectangles_by_color(file_path: &str) -> Result<HashMap<String, Vec<Rectangle>>, String> {

    let mut shape_list = HashMap::new();

    let opt = roxmltree::ParsingOptions {
        allow_dtd: true,
        ..roxmltree::ParsingOptions::default()
    };

    let svg_input = std::fs::read_to_string(file_path);
    let svg_text = match svg_input {
        Ok(svg_text) => svg_text,
        Err(e) => {
            println!("Error: {}.", e);
            panic!();
        }
    };
    
    let doc = match roxmltree::Document::parse_with_options(&svg_text, opt) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error: {}.", e);
            panic!();
        }
    };



    // let tree = Tree::from_file(file_path, &usvg::Options::default()).map_err(|e| e.to_string())?;
    let tree = Tree::from_xmltree(&doc, &usvg::Options::default()).map_err(|e| e.to_string())?;
    for node in tree.root.descendants() {
        
        match Shape::try_from(node.clone()) {
            Some(shape) => {
                if let Some(color) = shape.fill_color() {
                    let fill_color = format!("{:x}", color.to_rgba());
                    let rect = Rectangle::new(
                        shape.bounds().x(),
                        shape.bounds().y(),
                        shape.bounds().width(),
                        shape.bounds().height()
                    );
                    if let Some(rect_list) = shape_list.get_mut(&fill_color) {
                        rect_list.push(rect);
                    } else {
                        shape_list.insert(fill_color, vec![rect]);
                    }
                }
            }
            None => return Err("No Shape found".to_string()),
            Ok(_) => {
                todo!()
            },
            Err(_) => todo!(),
        }
    }
    Ok(shape_list)
}


// fn parse_svg_file(file_path: &str) -> HashMap<String, Vec<Rectangle>> {
//     // let tree = Tree::from_file(file_path, &usvg::Options::default()).unwrap();

//     let opt = roxmltree::ParsingOptions {
//         allow_dtd: true,
//         ..roxmltree::ParsingOptions::default()
//     };

//     let svg_input = std::fs::read_to_string(file_path);
//     let svg_text = match svg_input {
//         Ok(svg_text) => svg_text,
//         Err(e) => {
//             println!("Error: {}.", e);
//             panic!();
//         }
//     };
    
//     let doc = match roxmltree::Document::parse_with_options(&svg_text, opt) {
//         Ok(doc) => doc,
//         Err(e) => {
//             println!("Error: {}.", e);
//             panic!();
//         }
//     };


//     // let tree = Tree::from_xmltree(file_path, &usvg::Options::default()).unwrap();
//     let tree = Tree::from_xmltree( &doc, &usvg::Options::default()).unwrap();
//     let mut shape_list: HashMap<String, Vec<Rectangle>> = HashMap::new();

//     for node in tree.root.descendants() {
//         match Shape::try_from(node.clone()) {
//             Some(shape) => {
//                 if let Some(color) = shape.fill_color() {
//                     let fill_color = format!("{:x}", color.to_rgba());
//                     let rect = Rectangle::new(
//                         shape.bounds().x(),
//                         shape.bounds().y(),
//                         shape.bounds().width(),
//                         shape.bounds().height()
//                     );
//                     if let Some(rect_list) = shape_list.get_mut(&fill_color) {
//                         rect_list.push(rect);
//                     } else {
//                         shape_list.insert(fill_color, vec![rect]);
//                     }
//                 }
//                 //    None => { println!("No Shape found"); }
//                 None => { panic!(); }
//             }   None => {println!("No Shape found"); }
            
//             Ok(_) => todo!(),
//             Err(_) => todo!(),
//         }
//     }

//     shape_list
// }

fn parse_svg_file(file_path: &str) -> HashMap<String, Vec<Rectangle>> {

    let tree = Tree::from_xmltree(file_path, &usvg::Options::default()).unwrap();

    for node in tree.root.descendants() {
        match Shape::try_from(node.clone()) {
            Some(shape) => {
                if let Some(color) = shape.fill_color() {
                    println!("some shite");
                    
                }
                
            }   None => {println!("No Shape found"); }            
        }
    }

    shape_list
}



fn separate_shapes_by_color(bin: &mut Bin, shapes: &[Rectangle], color: &str) {
    let mut color_shapes = Vec::new();
    for shape in shapes {
        if bin.fits(shape) {
            bin.insert(shape.clone());
        } else {
            let (bin1, bin2) = if bin.width - shape.width > bin.height - shape.height {
                bin.split_horizontally(shape.width)
            } else {
                bin.split_vertically(shape.height)
            };

            separate_shapes_by_color(&mut bin1, &[shape.clone()], color);
            separate_shapes_by_color(&mut bin2, &[shape.clone()], color);
        }
    }

    if !bin.rectangles.is_empty() {
        color_shapes.extend(bin.rectangles.clone());
        bin.rectangles.clear();
    }

    if !color_shapes.is_empty() {
        let file_name = format!("{}.svg", color);
        let mut svg = String::new();
        svg.push_str(
            &format!(
                r#"<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
                bin.width,
                bin.height
            )
        );

        for shape in color_shapes {
            svg.push_str(
                &format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="none" stroke="{}" stroke-width="{}"/>"#,
                    shape.x,
                    shape.y,
                    shape.width,
                    shape.height,
                    color,
                    1
                )
            );
        }

        svg.push_str("</svg>");

        std::fs::write(file_name, svg).expect("Unable to write file");
    }

}

    pub fn test_bin_pack() {
        let file_path = "./svg_output/twelveXtwelve/frank_tr12.svg";
        let shape_list = parse_svg_file(file_path);

        shape_list.into_iter().for_each(|(color, shapes)| {
            let mut bin = Bin::new(
                shapes
                    .iter()
                    .map(|s| s.width)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
                shapes
                    .iter()
                    .map(|s| s.height)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
            );
            separate_shapes_by_color(&mut bin, &shapes, &color);
        });
    }



    use std::convert::TryFrom;
    use roxmltree::{Node, NodeData};
    use usvg::NodeKind;
    
    pub(crate) use crate::errors::SvgError;



    use crate::shapes::{Shape, Circle, Ellipse, Path, Rectangle};
    use crate::parsers::SvgPathParser;
    


    impl<'a> TryFrom<Node<'a, NodeData>> for Shape<'a> {
        type Error = SvgError;
    
        fn try_from(node: Node<'a, NodeData>) -> Result<Self, Self::Error> {
            match node.data.kind {
                NodeKind::Path => {
                    let mut parser = SvgPathParser::new(node.attribute("d").unwrap_or(""));
                    let path = parser.parse()?;
                    Ok(Shape::Path(path))
                }
                NodeKind::Rect => {
                    let x = node.attribute("x").unwrap_or("0").parse::<f32>()?;
                    let y = node.attribute("y").unwrap_or("0").parse::<f32>()?;
                    let w = node.attribute("width").unwrap_or("0").parse::<f32>()?;
                    let h = node.attribute("height").unwrap_or("0").parse::<f32>()?;
                    let rect = Rectangle::new(x, y, w, h);
                    Ok(Shape::Rectangle(rect))
                }
                NodeKind::Circle => {
                    let cx = node.attribute("cx").unwrap_or("0").parse::<f32>()?;
                    let cy = node.attribute("cy").unwrap_or("0").parse::<f32>()?;
                    let r = node.attribute("r").unwrap_or("0").parse::<f32>()?;
                    let circle = Circle::new(cx, cy, r);
                    Ok(Shape::Circle(circle))
                }
                NodeKind::Ellipse => {
                    let cx = node.attribute("cx").unwrap_or("0").parse::<f32>()?;
                    let cy = node.attribute("cy").unwrap_or("0").parse::<f32>()?;
                    let rx = node.attribute("rx").unwrap_or("0").parse::<f32>()?;
                    let ry = node.attribute("ry").unwrap_or("0").parse::<f32>()?;
                    let ellipse = Ellipse::new(cx, cy, rx, ry);
                    Ok(Shape::Ellipse(ellipse))
                }
                _ => Err(SvgError::UnsupportedShape),
            }
        }
    }
    