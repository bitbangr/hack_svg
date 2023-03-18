// Describe an algorithm that uses a force directed graph to take an svg file of interconnected shapes
// and colours and groups or and rearranges them by the shape fill colour using rust programming language.
// This is similar to a bin packing algorithm but should use force directed graph to determine shape positions.

// Here is an algorithm that uses a force-directed graph to group and rearrange shapes by their fill color:

// 1. Parse the SVG file using the usvg crate to obtain a list of shapes and their fill colors.

// 2. Create a force-directed graph where each node represents a shape and each edge represents a connection between two shapes.
//    The edges can be determined by checking whether two shapes intersect each other or share a common border.

// 3. Assign an initial position to each node. This can be done randomly or by using the centroid of the shape's bounding box.

// 4. Define a force function that calculates the forces acting on each node based on its position and the positions of its neighboring nodes.

//    The force function should include repulsive forces between all nodes and attractive forces between connected nodes.

//    The strength of the forces can be adjusted to balance the desire for shapes of the same color to cluster together while avoiding overlaps and maintaining a pleasing layout.

// 5. Run a simulation to update the position of each node based on the net force acting on it.
//    This can be done using an iterative algorithm such as Euler's method or the Verlet integration method.

// 6. Repeat steps 4 and 5 until the nodes settle into a stable configuration, where the total force acting on each node is below a certain threshold.

// 7. Once the positions of the nodes have been determined, group the shapes by fill color and rearrange them according to their positions.
//    This can be done by sorting the shapes by their x and y coordinates and creating separate SVG files for each color.

// Here is some sample Rust code that implements this algorithm:

use usvg::{ Tree, Node, NodeExt };

struct Shape {
    fill_color: String,
    bounds: (f64, f64, f64, f64), // (x0, y0, x1, y1)
}

struct Edge {
    source: usize,
    target: usize,
}

struct ForceDirectedGraph {
    shapes: Vec<Shape>,
    edges: Vec<Edge>,
    positions: Vec<(f64, f64)>,
}

impl ForceDirectedGraph {
    fn new(shapes: Vec<Shape>, edges: Vec<Edge>) -> Self {
        let positions = vec![(0.0, 0.0); shapes.len()];
        ForceDirectedGraph { shapes, edges, positions }
    }

    fn apply_forces(&mut self) {
        let k_repulsive = 1.0;
        let k_attractive = 1.0;

        // Calculate repulsive forces between nodes
        for i in 0..self.shapes.len() {
            let (x1, y1) = self.positions[i];
            let mut fx = 0.0;
            let mut fy = 0.0;
            for j in 0..self.shapes.len() {
                if i != j {
                    let (x2, y2) = self.positions[j];
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let d2 = dx * dx + dy * dy;
                    if d2 > 0.0 {
                        let f = k_repulsive / d2;
                        fx -= f * dx;
                        fy -= f * dy;
                    }
                }
            }
            self.positions[i].0 += fx;
            self.positions[i].1 += fy;
        }

        // Calculate attractive forces between connected nodes
        for edge in &self.edges {
            let (x1, y1) = self.positions[edge.source];
            let (x2, y2) = self.positions[edge.target];
            let dx = x2 - x1;
            let dy = y2 - y1;
            let d = dx * dx + dy * dy; // mgj CHECK THIS LINE WAS NOT COMPLETED

            let f = k_attractive * d;
            let fx = f * dx;
            let fy = f * dy;
            self.positions[edge.source].0 += fx;
            self.positions[edge.source].1 += fy;
            self.positions[edge.target].0 -= fx;
            self.positions[edge.target].1 -= fy;
        }
    }

    fn run_simulation(&mut self) {
        let max_iterations = 1000;
        let convergence_threshold = 0.001;
        let mut iterations = 0;
        let mut total_force = f64::INFINITY;
        while iterations < max_iterations && total_force > convergence_threshold {
            self.apply_forces();
            total_force = self.calculate_total_force();
            iterations += 1;
        }
    }

    fn calculate_total_force(&self) -> f64 {
        let mut total_force = 0.0;
        for i in 0..self.shapes.len() {
            let (x, y) = self.positions[i];
            let mut fx = 0.0;
            let mut fy = 0.0;
            for j in 0..self.shapes.len() {
                if i != j {
                    let (x2, y2) = self.positions[j];
                    let dx = x2 - x;
                    let dy = y2 - y;
                    let d2 = dx * dx + dy * dy;
                    fx += dx / d2;
                    fy += dy / d2;
                }
            }
            total_force += fx * fx + fy * fy;
        }
        total_force
    }
    // *************** manual paste
}

fn parse_svg_file(file_path: &str) -> Vec<Shape> {
    let mut shapes = Vec::new();
    let tree = Tree::from_file(file_path, &usvg::Options::default()).unwrap();

    for node in tree.root().descendants() {
        if let Node::Path(path) = node {
            let bounds = path.svg_bbox();
            let fill_color = path.fill().unwrap_or("none").to_string();
            shapes.push(Shape { fill_color, bounds });
        }
    }
    shapes
}

fn group_shapes_by_color(shapes: Vec<Shape>) -> Vec<Vec<Shape>> {
    let mut groups: Vec<Vec<Shape>> = Vec::new();
    for shape in shapes {
        if let Some(group) = groups.iter_mut().find(|g| g[0].fill_color == shape.fill_color) {
            group.push(shape);
        } else {
            groups.push(vec![shape]);
        }
    }
    groups
}

fn rearrange_shapes_by_position(shapes: &mut [Shape], positions: &[(f64, f64)]) {
    for (shape, position) in shapes.iter_mut().zip(positions.iter()) {
        let (x, y) = position;
        let (x0, y0, x1, y1) = shape.bounds;
        let dx = (x1 - x0) / 2.0;
        let dy = (y1 - y0) / 2.0;
        shape.bounds = (x - dx, y - dy, x + dx, y + dy);
    }
}

fn create_svg_file(shapes: &[Shape], file_name: &str) {
    let mut svg_content = String::new();

    svg_content.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 1000 1000\">\n");
    for shape in shapes {
        let (x0, y0, x1, y1) = shape.bounds;
        let width = x1 - x0;
        let height = y1 - y0;
        // svg

        // ********* end manual paste

        let svg_rect = format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
            x0,
            y0,
            width,
            height,
            shape.fill_color
        );
        svg_content.push_str(&svg_rect);
    }

    svg_content.push_str("</svg>\n");
    std::fs::write(file_name, svg_content).unwrap();

    //** manual paste */
}

pub fn test_svg_fdg() {
    let file_path = "./svg_output/frank_tr12.svg";
    let shapes = parse_svg_file(file_path);
    let groups = group_shapes_by_color(shapes);

    for group in groups {
        let mut force_directed_graph = ForceDirectedGraph::new(group.len(), 0.1, 0.5, 0.05);

        for shape in &group {
            let (x0, y0, x1, y1) = shape.bounds;
            let x = (x0 + x1) / 2.0;
            let y = (y0 + y1) / 2.0;
            force_directed_graph.add_node(x, y);
        }

        force_directed_graph.add_edges();
        force_directed_graph.run_simulation();
        let positions = force_directed_graph.get_positions();
        rearrange_shapes_by_position(&mut group.clone(), &positions);

        let file_name = format!("{}.svg", group[0].fill_color);
        create_svg_file(&group, &file_name);
    }
}

// This algorithm starts by parsing an SVG file and extracting the shapes and their fill colors.
// Then, it groups the shapes by fill color, so that all shapes with the same color are in the same group.
// For each group, it creates a force-directed graph and adds the shapes as nodes to the graph, using the
// center of the shape bounding box as the node position.

// It then adds edges between nodes that are close to each other, using a distance threshold.
// The algorithm then runs the force-directed simulation, which iteratively updates the positions of the nodes
// until they reach a stable equilibrium.

// Finally, the algorithm rearranges the shapes in the group based on the updated node positions and saves the rearranged
// shapes as a new SVG file with the name based on the fill color.

// Note that this algorithm assumes that the shapes in each group are interconnected, i.e., there are edges between
// any two shapes that are close to each other. If the shapes are not interconnected, the force-directed simulation
// may not produce satisfactory results. In that case, a different algorithm, such as a packing algorithm, may be more appropriate.