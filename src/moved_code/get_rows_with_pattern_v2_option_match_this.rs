// write this function again with a  'match_this' parameter which is 4 element array [m1,m2,m3,m4]
// where m1,m2,m3,m4 are true false or nan.
// If nan then we return rows containing both true or false in that position

// Here's an implementation of the function get_rows_with_pattern_v2 with a match_this parameter:

use ndarray::{Array2, ArrayBase, ArrayView1, Dim, OwnedRepr};

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
        
    } // for
    res
}

// This version uses an array of Option<bool> instead of an array of bool.
// An element of this array is Some(value) if we want to match value at the corresponding position in the row,
// and None if we don't care about the value at that position.

// For example, if we want to find rows that match the pattern
// [true, false, x, x] or [false, x, x, true], we can call the function like this:

fn main() {
    let match_this = [Some(true), Some(false), None, None];
    let result = get_rows_with_pattern_v2(match_this, &array);

    // If we want to find rows that match the pattern [true, x, false, x] or [false, x, true, x], we can call the function like this:

    let match_this = [Some(true), None, Some(false), None];
    let result = get_rows_with_pattern_v2(match_this, &array);

    // f we want to find rows that contain both true and false in the third position, we can call the function like this:
    let match_this = [None, None, None, None];
    let result = get_rows_with_pattern_v2(match_this, &array);
}
