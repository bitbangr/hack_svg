use ndarray::Array2;

pub struct RGB(pub u8, pub u8, pub u8);

pub fn compare_neighbours(array: Array2<(Box2D<i32>, RGB)>) {
    let rows = array.dim().0;
    let cols = array.dim().1;

    for i in 0..rows {
        for j in 0..cols {
            let current_value = array[(i, j)].1;
            let north = if i > 0 { array[(i - 1, j)].1 } else { RGB(0, 0, 0) };
            let south = if i < rows - 1 { array[(i + 1, j)].1 } else { RGB(0, 0, 0) };
            let west = if j > 0 { array[(i, j - 1)].1 } else { RGB(0, 0, 0) };
            let east = if j < cols - 1 { array[(i, j + 1)].1 } else { RGB(0, 0, 0) };

            if current_value == north && current_value == south && current_value == west && current_value == east {
                println!("true");
            } else {
                println!("false");
            }
        }
    }
}
