fn dfs(array: &Vec<Vec<String>>, row: isize, col: isize, color: &str, visited: &mut Vec<Vec<bool>>, rows: isize, cols: isize) -> Vec<(isize, isize)> {
    if row < 0 || row >= rows || col < 0 || col >= cols || visited[row as usize ][col as usize] || array[row as usize][col as usize] != color {
        return vec![];
    }
    visited[row as usize][col as usize] = true;
    let dirs: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut curr_group = vec![(row, col)];
    for d in dirs {
        let r = row + d.0;
        let c = col + d.1;
        curr_group.extend(dfs(array, r, c, color, visited, rows, cols));
    }
    return curr_group;
}

pub fn get_contiguous_tiles(array: &Vec<Vec<String>>) -> Vec<Vec<(isize, isize)>> {
    let rows = array.len() as isize;
    let cols = array[0].len() as isize;
    let mut visited = vec![vec![false; cols as usize]; rows as usize];
    let mut result = vec![];

    for row in 0..rows {
        for col in 0..cols {
            if !visited[row as usize ][col as usize] {
                let color = &array[row as usize][col as usize];
                let curr_group = dfs(array, row, col, color, &mut visited, rows, cols);
                if !curr_group.is_empty() {
                    result.push(curr_group);
                }
            }
        }
    }

    return result;
}
