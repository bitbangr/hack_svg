use euclid::{Box2D, UnknownUnit};

use crate::mosaic_tile::RGB;

// use crate::mosiac_tilemodtile::RGB;

fn dfs_mod(pane_vec: &Vec<Vec<(Box2D<i32, UnknownUnit>, RGB)>>, row: isize, col: isize, color: &RGB, visited: &mut Vec<Vec<bool>>, rows: isize, cols: isize) -> Vec<(isize, isize)> {    

    if row < 0 || row >= rows || col < 0 || col >= cols || visited[row as usize ][col as usize] 
        || pane_vec[row as usize][col as usize].1 != *color 
    {
        return vec![];
    }
    visited[row as usize][col as usize] = true;
    let dirs: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut curr_group = vec![(row, col)];
    for d in dirs {
        let r = row + d.0;
        let c = col + d.1;
        curr_group.extend(dfs_mod(pane_vec, r, c, color, visited, rows, cols));
    }
    return curr_group;
}

pub fn get_contiguous_tiles_mod(pane_vec: &Vec<Vec<(Box2D<i32, UnknownUnit>, RGB)>>) -> Vec<Vec<(isize, isize)>> {

    let rows = pane_vec.len() as isize;
    let cols = pane_vec[0].len() as isize;
    let mut visited = vec![vec![false; cols as usize]; rows as usize];
    let mut result = vec![];

    for row in 0..rows {
        for col in 0..cols {
            if !visited[row as usize ][col as usize] {
                
                let color = &pane_vec[row as usize][col as usize].1;
                let curr_group = dfs_mod(pane_vec, row, col, color, &mut visited, rows, cols);
                if !curr_group.is_empty() {
                    result.push(curr_group);
                }
            }
        }
    }

    return result;
}


/***********************************************
 * ********************************************
 * do not touch below
 */
// CODE Below works but needed to refactor

fn _dfs_orig(array: &Vec<Vec<String>>, row: isize, col: isize, color: &str, visited: &mut Vec<Vec<bool>>, rows: isize, cols: isize) -> Vec<(isize, isize)> {
    if row < 0 || row >= rows || col < 0 || col >= cols || visited[row as usize ][col as usize] || array[row as usize][col as usize] != color {
        return vec![];
    }
    visited[row as usize][col as usize] = true;
    let dirs: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut curr_group = vec![(row, col)];
    for d in dirs {
        let r = row + d.0;
        let c = col + d.1;
        curr_group.extend(_dfs_orig(array, r, c, color, visited, rows, cols));
    }
    return curr_group;
}

pub fn _get_contiguous_tiles_orig(array: &Vec<Vec<String>>) -> Vec<Vec<(isize, isize)>> {
    let rows = array.len() as isize;
    let cols = array[0].len() as isize;
    let mut visited = vec![vec![false; cols as usize]; rows as usize];
    let mut result = vec![];

    for row in 0..rows {
        for col in 0..cols {
            if !visited[row as usize ][col as usize] {
                let color = &array[row as usize][col as usize];
                let curr_group = _dfs_orig(array, row, col, color, &mut visited, rows, cols);
                if !curr_group.is_empty() {
                    result.push(curr_group);
                }
            }
        }
    }

    return result;
}
