fn combine_pane_edges(
    pane_nd_arr: &ArrayBase<OwnedRepr<(euclid::Box2D<i32, euclid::UnknownUnit>, RGB)>, Dim<[usize; 2]>>,
    edge_booleans: &ArrayBase<OwnedRepr<Vec<bool>>, Dim<[usize; 2]>>
) -> ArrayBase<OwnedRepr<MosaicTile>, Dim<[usize; 2]>> {
    let mut result = Array2::<MosaicTile>::zeros((pane_nd_arr.shape()[0], pane_nd_arr.shape()[1]));

    for ((row, col), ((coords, rgb), edge_bool)) in pane_nd_arr.indexed_iter().zip(edge_booleans.indexed_iter()) {
        let tile = Tile::new(Box2D::new(coords.min, coords.max), *rgb);
        let mosaic_tile = MosaicTile::new(tile, edge_bool.to_vec());
        result[[row, col]] = mosaic_tile;
    }

    result
}

/*

This function takes two array arguments, pane_nd_arr and edge_booleans, each of which has a shape of [usize; 2]. pane_nd_arr is an
 array of tuples containing a Box2D<i32> instance and an RGB instance, representing the position and color of each tile in the mosaic.
  edge_booleans is an array of vectors containing boolean flags representing the visibility of the edges for each tile in the mosaic.

The function returns an array of MosaicTile instances with the same shape as the input arrays, where each MosaicTile instance
 corresponds to a tile in the input arrays. The MosaicTile instances include the Tile data from the input arrays, as well as 
 the edge_bool data from edge_booleans.

The function first creates a new 2D array result of MosaicTile instances with the same shape as the input arrays, using the
 Array2::zeros method. It then iterates over the input arrays using the indexed_iter method, which returns an iterator over 
 the elements of each array along with their indices.

For each element in the input arrays, the function creates a new Tile instance using the Box2D and RGB data, and a new 
MosaicTile instance using the Tile instance and the corresponding edge_bool vector. It then stores the MosaicTile instance 
in the result array at the corresponding index.

Finally, the function returns the result array containing the MosaicTile instances.

Note that the implementation assumes that the input arrays have the same shape, and that the edge_bool vectors in 
edge_booleans have the same length as the width and height of the tiles in pane_nd_arr. You may need to modify the 
function to handle other cases, depending on your specific use case.

 */