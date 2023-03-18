use roxmltree::{Document, Node};
use std::collections::HashMap;
use svg::node::element::{Rectangle as SvgRectangle, Path as SvgPath};
use svg::node::element::path::Data;
use svg::node::element::Rectangle;
use usvg::{Options, Tree, roxmltree};
use svgtypes::{Length, LengthUnit};

pub fn get_shape_list(file_path: &str) -> HashMap<String, Vec<Rectangle>> {
    let mut shape_list = HashMap::new();



    let tree = Tree::from_file(file_path, &Options::default()).unwrap();

    for node in tree.root.descendants() {
        if let Some(rect) = rectangle_from_node(node) {
            let fill_color = format!("{:x}", rect.fill);
            if let Some(rect_list) = shape_list.get_mut(&fill_color) {
                rect_list.push(rect);
            } else {
                shape_list.insert(fill_color, vec![rect]);
            }
        }
    }

    shape_list
}

fn rectangle_from_node(node: Node) -> Option<Rectangle> {
    match node.tag_name().name() {
        "rect" => rectangle_from_rect_node(node),
        "path" => rectangle_from_path_node(node),
        _ => None,
    }
}

fn rectangle_from_rect_node(node: Node) -> Option<Rectangle> {
    let x = get_length_attribute(&node, "x")?;
    let y = get_length_attribute(&node, "y")?;
    let width = get_length_attribute(&node, "width")?;
    let height = get_length_attribute(&node, "height")?;
    let fill = get_fill_attribute(&node)?;

    Some(Rectangle::new(x.to_pixels(), y.to_pixels(), width.to_pixels(), height.to_pixels())
        .set("fill", fill))
}

fn rectangle_from_path_node(node: Node) -> Option<Rectangle> {
    let d = node.attribute("d")?;
    let fill = get_fill_attribute(&node)?;

    let data = Data::parse(&d).ok()?;
    let bounds = data
        .iter()
        .fold(None, |b, e| Some(b.map_or(e.bounding_box(), |bb| bb.union(e.bounding_box()))))?;

    Some(Rectangle::new(bounds.x, bounds.y, bounds.width, bounds.height).set("fill", fill))
}

fn get_length_attribute(node: &Node, name: &str) -> Option<Length> {
    node.attribute(name).and_then(|value| Length::from_str(value).ok())
}

fn get_fill_attribute(node: &Node) -> Option<String> {
    node.attribute("fill").map(|fill| fill.to_string())
}

trait ToPixels {
    fn to_pixels(&self) -> f64;
}

impl ToPixels for Length {
    fn to_pixels(&self) -> f64 {
        match self.unit {
            LengthUnit::Px => self.value,
            LengthUnit::Pt => self.value * 1.25,
            LengthUnit::Pc => self.value * 15.0,
            LengthUnit::In => self.value * 90.0,
            LengthUnit::Cm => self.value * 35.433,
            LengthUnit::Mm => self.value * 3.543,
            LengthUnit::Em | LengthUnit::Ex | LengthUnit::Percent => 0.0,
        }
        None {
            println!("j error");
            return 0.0;
        }
    }
}
