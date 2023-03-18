use std::collections::HashMap;

struct Shape {
    bounds: (f32, f32, f32, f32),
    fill_color: String,
    // additional fields as needed
}

struct ForceDirectedGraph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    k: f32,
    c1: f32,
    c2: f32,
    t: f32,
}

#[derive(Clone)]
struct Node {
    x: f32,
    y: f32,
    fx: f32,
    fy: f32,
}

struct Edge {
    source: usize,
    target: usize,
}

impl ForceDirectedGraph {
    fn new(num_nodes: usize, k: f32, c1: f32, c2: f32) -> ForceDirectedGraph {
        ForceDirectedGraph {
            nodes: vec![Node { x: 0.0, y: 0.0, fx: 0.0, fy: 0.0 }; num_nodes],
            edges: vec![],
            k,
            c1,
            c2,
            t: 0.0,
        }
    }

    fn add_node(&mut self, x: f32, y: f32) {
        self.nodes.push(Node { x, y, fx: 0.0, fy: 0.0 });
    }

    fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.push(Edge { source, target });
    }

    fn add_edges(&mut self) {
        let mut edge_map: HashMap<(usize, usize), f32> = HashMap::new();
        for i in 0..self.nodes.len() {
            for j in i + 1..self.nodes.len() {
                let distance = self.distance(i, j);
                edge_map.insert((i, j), distance);
            }
        }
        for edge in edge_map.iter_mut() {
            if edge.1 < &mut self.k {
                self.add_edge(edge.0.0, edge.0.1);
            }
        }
    }

    fn distance(&self, i: usize, j: usize) -> f32 {
        let dx = self.nodes[i].x - self.nodes[j].x;
        let dy = self.nodes[i].y - self.nodes[j].y;
        (dx * dx + dy * dy).sqrt()
    }

    fn repulsive_force(&self, distance: f32) -> f32 {
        self.c1 / distance
    }

    fn attractive_force(&self, distance: f32) -> f32 {
        self.c2 * (distance - self.k)
    }

    fn run_simulation(&mut self) {
        let mut rng = rand::thread_rng();
        let mut forces = vec![Node { x: 0.0, y: 0.0, fx: 0.0, fy: 0.0 }; self.nodes.len()];
        while self.t < 1.0 {
            // calculate repulsive forces
            for i in 0..self.nodes.len() {
                for j in i + 1..self.nodes.len() {
                    let distance = self.distance(i, j);
                    if distance > 0.0 {
                        let force = self.repulsive_force(distance);
                        let dx = self.nodes[i].x - self.nodes[j].x;
                        let dy = self.nodes[i].y - self.nodes[j].y;
                        forces[i].fx += (force * dx) / distance;
                        forces[i].fy += (force * dy) / distance;
                        forces[j].fx -= (force * dx) / distance;
                        forces[j].fy -= (force * dy) / distance;
                    }
                }
            }
            // calculate attractive forces
            for edge in &self.edges {
                let distance = self.distance(edge.source, edge.target);
                let force = self.attractive_force(distance);
                let dx = self.nodes[edge.target].x - self.nodes[edge.source].x;
                let dy = self.nodes[edge.target].y - self.nodes[edge.source].y;
                forces[edge.source].fx += (force * dx) / distance;
                forces[edge.source].fy += (force * dy) / distance;
                forces[edge.target].fx -= (force * dx) / distance;
                forces[edge.target].fy -= (force * dy) / distance;
            }
            // update node positions
            for i in 0..self.nodes.len() {
                let force = forces[i].x * forces[i].x + forces[i].y * forces[i].y;
                let speed = force.sqrt();
                let max_speed = (1.0 - self.t) / (1.0 + self.t);
                let displacement = if speed > 0.0 {
                    let dx = (forces[i].x * max_speed) / speed;
                    let dy = (forces[i].y * max_speed) / speed;
                    (dx, dy)
                } else {
                    (0.0, 0.0)
                };
                
                let bounds = self.nodes[i].bounds;
                let new_x = (self.nodes[i].x + displacement.0).max(bounds.0).min(bounds.2);
                let new_y = (self.nodes[i].y + displacement.1).max(bounds.1).min(bounds.3);
                self.nodes[i].x = new_x;
                self.nodes[i].y = new_y;
                forces[i].fx = 0.0;
                forces[i].fy = 0.0;
            }
            // cool down
            self.t += 0.05;
            if rng.gen::<f32>() < 0.1 {
                self.k *= 0.95;
            }
        }
    }
}

fn group_and_arrange_shapes(svg_file: &str) -> Vec<Vec<Shape>> {
    // parse SVG file and extract shapes
    let mut shapes: Vec<Shape> = vec![];
    // ...
    // group shapes by fill color
    let mut color_map: HashMap<String, Vec<Shape>> = HashMap::new();

    for shape in shapes {
        let group = color_map.entry(shape.fill_color).or_insert(vec![]);
        group.push(shape);
    }
    // initialize force-directed graph
    let num_groups = color_map.len();
    let mut graph = ForceDirectedGraph::new(num_groups, 100.0, 100.0, 1000.0);
    let mut group_centers: Vec<(f32, f32)> = vec![];

    for group in color_map.values() {
        let mut bounds = (std::f32::MAX, std::f32::MAX, std::f32::MIN, std::f32::MIN);
        for shape in group {
            bounds.0 = bounds.0.min(shape.bounds.0);
            bounds.1 = bounds.1.min(shape.bounds.1);
            bounds.2 = bounds.2.max(shape.bounds.2);
            bounds.3 = bounds.3.max(shape.bounds.3);
        }
        let center_x = (bounds.0 + bounds.2) / 2.0;
        let center_y = (bounds.1 + bounds.3) / 2.0;
        group_centers.push((center_x, center_y));
        graph.add_node(center_x, center_y);
    }

    // add edges to force-directed graph
    for i in 0..num_groups {
        for j in i + 1..num_groups {
            let dx = group_centers[i].0 - group_centers[j].0;
            let dy = group_centers[i].1 - group_centers[j].1;
            let distance = (dx * dx + dy * dy).sqrt();
            graph.add_edge(i, j, distance);
        }
    }

    // run force-directed layout algorithm
    graph.run_layout_algorithm();
    // update shape positions based on group centers
    let mut arranged_shapes: Vec<Vec<Shape>> = vec![vec![]; num_groups];
    for (group_index, group) in color_map.values().enumerate() {
        let (center_x, center_y) = group_centers[group_index];
        for shape in group {
            let new_x = shape.bounds.0 + center_x;
            let new_y = shape.bounds.1 + center_y;
            arranged_shapes[group_index].push(Shape {
                fill_color: shape.fill_color,
                bounds: (
                    new_x,
                    new_y,
                    new_x + shape.bounds.2 - shape.bounds.0,
                    new_y + shape.bounds.3 - shape.bounds.1,
                ),
            });
        }
    }
    arranged_shapes
}

use printpdf::rand;
use xml::writer::{EmitterConfig, XmlEvent};
use std::fs::File;

fn write_arranged_shapes_to_svg(arranged_shapes: Vec<Vec<Shape>>, output_file: &str) {
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(File::create(output_file).unwrap());
    writer.write(
        XmlEvent::start_element("svg")
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("version", "1.1")
    );
    for shape_group in arranged_shapes {
        for shape in shape_group {
            writer.write(
                XmlEvent::start_element("rect")
                    .attr("x", shape.bounds.0.to_string())
                    .attr("y", shape.bounds.1.to_string())
                    .attr("width", (shape.bounds.2 - shape.bounds.0).to_string())
                    .attr("height", (shape.bounds.3 - shape.bounds.1).to_string())
                    .attr("fill", shape.fill_color)
            );
        }
    }
    writer.write(XmlEvent::end_element());
}


pub fn test_dfs_layout() {
    // let svg_file = "my_svg_file.svg";
    // let arranged_shapes = group_and_arrange_shapes(svg_file);

    let arranged_shapes = group_and_arrange_shapes("my_svg_file.svg");
    write_arranged_shapes_to_svg(arranged_shapes, "arranged_svg_file.svg");

    // ...
}

/* 
This algorithm parses an SVG file and extracts the shapes. 
 
 It then groups the shapes by fill color and initializes a force-directed graph with a node for each color group. 
 
 The algorithm adds edges to the graph based on the distances between the centers of the color groups, 
 and then runs the force-directed layout algorithm to determine the positions of the color group nodes. 
 
 Finally, the algorithm updates the positions of the shapes based on the new positions of the color group nodes. 
 
 The output is a vector of vectors, where each inner vector contains all the shapes with the same fill color and arranged in a non-overlapping way.

*/