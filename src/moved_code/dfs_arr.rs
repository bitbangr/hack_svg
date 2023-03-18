use euclid::{Box2D, UnknownUnit};

use crate::modtile::RGB;

    // TODO dfs_mod returns a vect of matching entries to the position in the Vec
    //         This does not match the array inde position to the NDarray
    //         So need to modify DFS_MOD to return array index and not Vec position
    // see main.rs write_svg where it crashes

    // fn dfs_mod search results -> [[(0, 0), (0, 1), (0, 2), (0, 3)]]
    // fn write_svg - Vector of contigous tiles -> [[(0, 0), (0, 1), (0, 2), (0, 3)]]



pub fn get_cont_tiles_arr(mosaic_nd_arr: &ndarray::ArrayBase<ndarray::OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>, 
                                                                                        ndarray::Dim<[usize; 2]>>) -> 
                                                                                        Vec<Vec<(isize, isize)>> 
{
    let shape = mosaic_nd_arr.shape();
    println!("shape {:?} ", & shape);

    let rows = shape[0] as isize;
    let cols = shape[1] as isize;

    println!("get_cont_tiles_arr (rows,cols) -> ({},{})", &rows, &cols );
    let mut visited = vec![vec![false; cols as usize]; rows as usize];
    let mut result = vec![];

    for row in 0..rows {
        for col in 0..cols {
            if !visited[row as usize ][col as usize] {
                
                let color:RGB = mosaic_nd_arr[[row as usize,col as usize]].1;
                // let color = &mosaic_nd_arr[row as usize][col as usize];

                let curr_group = dfs_arr(mosaic_nd_arr, row, col, color, &mut visited, rows, cols);
                if !curr_group.is_empty() {
                    result.push(curr_group);
                }
            }
        }
    }

    return result;

}

fn dfs_arr(mosaic_nd_arr: &ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32, UnknownUnit>, RGB)>,ndarray::Dim<[usize; 2]>>, 
                          row: isize, 
                          col: isize, 
                          color: RGB, 
                          visited: &[Vec<bool>], 
                          rows: isize, 
                          cols: isize) 
                    -> Vec<(isize, isize)> {
        
    if row < 0 || row >= rows || col < 0 || col >= cols || visited[row as usize ][col as usize] 
        || mosaic_nd_arr[[row as usize, col as usize]].1 != color 
    {
        return vec![];
    }
    // mgj uncomment and fix line below
    // visited[row as usize][col as usize] = true;
    let dirs: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut curr_group = vec![(row, col)];
    for d in dirs {
        let r = row + d.0;
        let c = col + d.1;
        curr_group.extend(dfs_arr(mosaic_nd_arr, r, c, color, visited, rows, cols));
    }
    return curr_group;


} // dfs_arr
