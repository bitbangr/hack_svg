use euclid::default::Rect;
use euclid::default::Box2D;
use euclid::default::Point2D;

use svg::node::element::path::{Command, Data, Number};
use svg::Document;

use std::collections::HashSet;




fn create_contiguous_path(tiles: &[(Point2D<i32>, [bool; 4], Rect<usize>)]) -> Data {
    // Find a starting tile
    let start_tile = tiles
        .iter()
        .find(|(p, _, _)| !tiles.iter().any(|(q, _, _)| q.x == p.x && q.y < p.y))
        .unwrap()
        .0;

    // Create a set of visited tiles
    let mut visited = HashSet::new();
    visited.insert(start_tile);

    // Traverse neighboring tiles to form the path
    let mut current_tile = start_tile;
    let mut path_data = vec![
        Command::MoveTo(
            vec![current_tile.x, current_tile.y]
        )
    ];

    loop {
        let next_tile = tiles
            .iter()
            .find(|(p, edges, _)| {
                (p.x == current_tile.x && p.y == current_tile.y - 1 && edges[0]) ||
                (p.x == current_tile.x + 1 && p.y == current_tile.y && edges[1]) ||
                (p.x == current_tile.x && p.y == current_tile.y + 1 && edges[2]) ||
                (p.x == current_tile.x - 1 && p.y == current_tile.y && edges[3])
            })
            .map(|(p, _, _)| *p);

        if let Some(next_tile) = next_tile {
            if next_tile == start_tile {
                // We've come full circle, close the path
                path_data.push(Command::ClosePath);
                break;
            } else if !visited.contains(&next_tile) {
                // Move to the next tile and mark it as visited
                path_data.push(Command::LineTo(
                    vec![next_tile.x, next_tile.y]
                ));
                visited.insert(next_tile);
                current_tile = next_tile;
            } else {
                // We've encountered a visited tile, terminate the path
                break;
            }
        } else {
            // We've reached a dead end, terminate the path
            break;
        }
    }

    Data::from(path_data)
}

use euclid::{TypedRect, Point2D};
use svg::node::element::path::Data;

fn main() {
    let tiles = vec![
        (Point2D::new(0, 0), [true, false, false, true], TypedRect::new(0.0, 0.0, 10.0, 10.0)),
        (Point2D::new(0, 1), [true, true, false, false], TypedRect::new(0.0, 10.0, 10.0, 10.0)),
        (Point2D::new(1, 1), [false, true, true, false], TypedRect::new(10.0, 10.0, 10.0, 10.0)),
        (Point2D::new(1, 0), [false, false, true, true], TypedRect::new(10.0, 0.0, 10.0, 10.0)),
    ];

    let path_data = create_contiguous_path(&tiles);
    println!("{:?}", path_data);
}