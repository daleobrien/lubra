use crate::consts::NODE_TO_COORD;
use crate::types::coordinate::{add, Coordinate};

// Move this to a util somewhere ...
pub fn get_connected_coords_at_coords(
    coordinate: Coordinate,
    node_idx: usize,
) -> Option<Coordinate> {
    // Each row is indexed:  1,2,3,4,5,6,...don't care about the rest
    let k = coordinate.i + (3 * (coordinate.j % 2));
    let coord = NODE_TO_COORD[k % 6][node_idx];
    let i = add(coordinate.i, coord.0);
    let j = add(coordinate.j, coord.1);

    if i.is_some() && j.is_some() {
        let coord = Coordinate {
            i: add(coordinate.i, coord.0).unwrap(),
            j: add(coordinate.j, coord.1).unwrap(),
        };
        return Some(coord);
    }
    None
}
