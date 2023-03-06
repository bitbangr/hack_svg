// let curtile_row: usize = 3;
// let curtile_col: usize = 7;
// contig_group: &[(isize, isize)],

// write a function to return all elements in contig_group that are within +/- 1 row and +/1 one column from curtile_row and curtile_col

// Min value for a contig_group element is (0,0)
// Max value for a contig_group element isis indeterminate

// return all elements in contig_group that are within +/- 1 row and +/- 1 column 
// from curtile_row and curtile_col. 
// Do not include element with value (curtile_row, curtile_col)
pub fn get_adjacent_tiles(curtile_row: usize, curtile_col: usize, contig_group: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let mut result = Vec::new();
    for &(row, col) in contig_group {
        if row >= (curtile_row as isize - 1) && row <= (curtile_row as isize + 1)
            && col >= (curtile_col as isize - 1) && col <= (curtile_col as isize + 1)
            && !(row == curtile_row as isize && col == curtile_col as isize)
        {
            result.push((row, col));
        }
    }
    result
}

// Use get_adjacent_tiles() function to construct a rust Map or dictionary for an Row by Column grid of tiles.  
// so adjacent[x,y] returns the above result for that row, col.  This means we set up the map once and can be 
// used again without have to call get_adjacent_tiles again.

// Here's an example implementation using a HashMap to store the adjacent tiles for each tile in an N by N grid:
use std::collections::HashMap;

pub fn build_adjacent_map(R: usize, C:usize) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut adjacent_map: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for row in 0..R {
        for col in 0..C {
            let mut adjacents = get_adjacent_tiles(row, col, N);
            adjacents.retain(|&(r, c)| r != row || c != col); // remove the current tile from adjacents
            adjacent_map.insert((row, col), adjacents);
        }
    }

    adjacent_map
}

fn test() {

    let N = 10;
    let adjacent_map = build_adjacent_map(N);
    
    // get the adjacent tiles for tile at (3, 7)
    let adjacents = adjacent_map.get(&(3, 7)).unwrap();
    
    println!("Adjacent tiles for (3, 7): {:?}", adjacents);
    
}
