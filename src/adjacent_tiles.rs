use std::collections::HashMap;


/// return all elements in contig_group that are within +/- 1 cur_tile row and +/- 1 cur_tile column 
/// from curtile_row and curtile_col. 
/// Do not include element with value (curtile_row, curtile_col)
pub fn get_adjacent_tiles(curtile_row: usize, curtile_col: usize, contig_group:  &Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut result = Vec::new();

    // Min value for a contig_group element is (0,0)
    // Max value for a contig_group element isis indeterminate

    for &(row, col) in contig_group {

        if (row as isize - curtile_row as isize).abs() <= 1 &&
           (col as isize - curtile_col as isize).abs() <= 1 &&
           !(row == curtile_row as isize && col == curtile_col as isize) {
            result.push((row, col));
        }
    }

    result
}


/// Use get_adjacent_tiles() function to construct a Hashmap for each entry of a contig_group
/// so adjacent_map[row,col] returns the above result for that row, col.  
/// i.e.  set up the map once used again without have to call get_adjacent_tiles repeatedly
pub fn build_adjacent_map( contig_group:  &Vec<(isize, isize)>) -> HashMap<(isize, isize), Vec<(isize, isize)>> 
{
    let mut adjacent_map: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();

    println!("{:?}" , contig_group );

    for (row, col) in contig_group {
        let adjacents: Vec<(isize, isize)> = get_adjacent_tiles(*row as usize, *col as usize, &contig_group);
        let mut adjacents_cloned: Vec<(isize, isize)> = adjacents.iter().cloned().collect();

        // Remove the current tile from adjacents
        adjacents_cloned.retain(|&(r, c)| r != *row || c != *col);

        adjacent_map.insert((*row, *col), adjacents_cloned);
    }

    // Print the adjacent map for debug purposes
    // TODO Remove when development complete
    println!("Adjacent Tiles Map"); 
    for ((row, col), adjacents) in adjacent_map.iter() {
        println!("({},{}) -> {:?}", row, col, adjacents);
    }

    adjacent_map
}

/// quick test to see that the adjacents is working properly
/// Note the R C do need to be a valid element in the contig_group otherwise
/// a None Exception is thrown.
pub fn _test_adjacents(r:isize, c:isize, contig_group:  &Vec<(isize, isize)>) {

    let adjacent_map = build_adjacent_map(&contig_group);
    
    // get the adjacent tiles for tile at (3, 7)
    let adjacents = adjacent_map.get(&(r, c));
    if let Some(adj) = adjacents {
        // adjacent tiles found
        // do something with the adjacent tiles
        println!("Adjacent tiles for ({},{}): {:?}", r,c, adjacents);
    } else {
        // adjacent tiles not found
        // handle the case when there are no adjacent tiles
        println!("No Adjacent tiles for ({},{}):", r,c );
    }

}
