use euclid::default::Box2D;
use ndarray::{Array2, ArrayView2, ArrayViewMut2, OwnedRepr, ViewRepr, Zip};
use crate::{modtile::{self, RGB}, four_tile_square, pane_vec_to_ndarray, get_edge_bools};

// pv_eb_hack.rs collecting bits together to construct pane_vec edge_bool
fn test_pane_edge_bool_mashup() {

    let rows: usize = 2;
    let cols: usize = 2;
    let tiles_per_pane_height: usize = 2; 
    let tiles_per_pane_width: usize = 2; 

    let pane_vec: Vec<Vec<(Box2D<i32>, modtile::RGB)>> =
        four_tile_square::create_bot_left_grn_rest_blk_tile_data();

    let pane_nd_arr: ndarray::ArrayBase<OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>, ndarray::Dim<[usize; 2]>> = pane_vec_to_ndarray(&pane_vec[0],tiles_per_pane_height , tiles_per_pane_width ); // rows, cols
    println!("\n\npane nd array {:?} ", &pane_nd_arr);

    // let edge_booleans: ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> =
    let edge_booleans : ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>> = get_edge_bools(&pane_nd_arr);

    let pane_edge_bool_arr_tuple: [(
        ndarray::ArrayBase<ndarray::OwnedRepr<(Box2D<i32>, RGB)>, ndarray::Dim<[usize; 2]>>,
        ndarray::ArrayBase<ndarray::OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>,
    )] = get_pane_edge_bool_arr_one(&pane_nd_arr, &edge_booleans);

    // return (pane_nd_arr: &Array2<(Box2D<i32>, modtile::RGB)>, edge_bools: edge_booleans: &Array2<Vec<bool>>)
    
}

fn get_pane_edge_bool_arr_one(pane_nd_arr: &ndarray::ArrayBase<OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>, ndarray::Dim<[usize; 2]>>,
                            edge_booleans: &ndarray::ArrayBase<OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>) 
                        -> [(ndarray::ArrayBase<OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>, ndarray::Dim<[usize; 2]>>, 
                             ndarray::ArrayBase<OwnedRepr<Vec<bool>>, ndarray::Dim<[usize; 2]>>)] {
    todo!()
}

// write function get_pane_edge_bool_arr to return a new 2 dimensional Array2 where element is a tuple of the (Box2D<i32>, RGB) and
// corresponding <Vec<bool>> in the same array location

// Here's an implementation of get_pane_edge_bool_arr that creates a new 2-dimensional Array2 where each element is a tuple of the
//  (Box2D<i32>, RGB) and corresponding Vec<bool> in the same array location as the input arrays:


fn get_pane_edge_bool_arr(
    pane_nd_arr: &Array2<(Box2D<i32>, modtile::RGB)>,
    edge_booleans: &Array2<Vec<bool>>,
) -> Array2<((Box2D<i32>, modtile::RGB), Vec<bool>)> {
    let mut pane_edge_bool_arr = Array2::default((pane_nd_arr.shape()[0], pane_nd_arr.shape()[1]));
    Zip::from(
        pane_edge_bool_arr
            .axis_iter_mut(ndarray::Axis(0))
            .into_iter()
            .zip(pane_nd_arr.axis_iter(ndarray::Axis(0)))
            .zip(edge_booleans.axis_iter(ndarray::Axis(0))),
    )
    .for_each(|((mut pane_edge_arr, pane_arr), edge_arr)| {
        Zip::from(
            pane_edge_arr
                .axis_iter_mut(ndarray::Axis(0))
                .into_iter()
                .zip(pane_arr.iter().zip(edge_arr.iter())),
        )
        .for_each(|(pane_edge, (pane, edge))| *pane_edge = ((*pane), edge.clone()));
    });
    pane_edge_bool_arr
}

// This implementation creates a new Array2 with the same shape as pane_nd_arr and edge_booleans,
// but where each element is a tuple of the corresponding (Box2D<i32>, modtile::RGB) and Vec<bool>
// in the same array location. The implementation uses Zip to iterate over the rows of the new array,
// as well as the corresponding rows of pane_nd_arr and edge_booleans.

// It then uses Zip again to iterate over the rows of the new array
// and corresponding (Box2D<i32>, modtile::RGB) and Vec<bool> tuples in pane_nd_arr and edge_booleans,
// and assigns the (Box2D<i32>, modtile::RGB) and Vec<bool> tuple to the corresponding element in the new array.
