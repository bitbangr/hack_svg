// create an ArrayBase<OwnedRepr<Vec<bool>>, Dim<[usize; 2]>> with the following elements
// [[[false, false, true, false], [false, false, false, false]],
//  [[true, true, false, false], [false, false, false, true]]]

// Here's how you can create an ArrayBase<OwnedRepr<Vec<bool>>, Dim<[usize; 2]>> with the specified elements:

use ndarray::{ArrayBase, Dim, OwnedRepr};

fn main() {
    let data = vec![
        vec![false, false, true, false],
        vec![false, false, false, false],
        vec![true, true, false, false],
        vec![false, false, false, true],
    ];

    let array = ArrayBase::from_shape_vec((2, 2), data).unwrap();
}
// Here, we first create a 2D vector data that contains the elements we want to store in the array.
// Then we create the array by calling ArrayBase::from_shape_vec and passing in the shape (2, 2) and the vector data.
// We also call unwrap() to unwrap the Result returned by from_shape_vec,
// which gives us the ArrayBase<OwnedRepr<Vec<bool>>, Dim<[usize; 2]>> that we want.

// using this array write a function that return the indexes of
// rows that contains true as the first element in the row AND false as the second element,
// or rows that contain false as the first element in the row and true as the last element of the row.

use ndarray::{ArrayBase, Dim, OwnedRepr};

fn get_matching_row_indices(arr: &ArrayBase<OwnedRepr<Vec<bool>>, Dim<[usize; 2]>>) -> Vec<usize> {
    let mut matching_indices = vec![];
    for (i, row) in arr.outer_iter().enumerate() {
        if (row[0] == true && row[1] == false) || (row[0] == false && row[row.len() - 1] == true) {
            matching_indices.push(i);
        }
    }
    matching_indices
}
