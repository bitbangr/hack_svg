
// write this function again with a  'match_this' parameter which is 4 element array [m1,m2,m3,m4] 
// where m1,m2,m3,m4 are true false or nan. If nan then we return rows containing both true or false in that position 

// Here's an implementation of the function get_rows_with_pattern_v2 with a match_this parameter:



use ndarray::{ArrayBase, OwnedRepr, Array2, Dim, ArrayView1};

fn get_rows_with_pattern_v2(match_this: [Option<bool>; 4], array: &Array2<bool>) -> Vec<usize> {
    let mut res = Vec::new();
    for (i, row) in array.outer_iter().enumerate() {
        if let Some(true) = match_this[0] {
            if row[0] != true {
                continue;
            }
        } else if let Some(false) = match_this[0] {
            if row[0] != false {
                continue;
            }
        }

        if let Some(true) = match_this[1] {
            if row[1] != true {
                continue;
            }
        } else if let Some(false) = match_this[1] {
            if row[1] != false {
                continue;
            }
        }

        if let Some(true) = match_this[2] {
            if row[2] != true {
                continue;
            }
        } else if let Some(false) = match_this[2] {
            if row[2] != false {
                continue;
            }
        }

        if let Some(true) = match_this[3] {
            if row[3] != true {
                continue;
            }
        } else if let Some(false) = match_this[3] {
            if row[3] != false {
                continue;
            }
        }

        res.push(i);
    }
    res
}
