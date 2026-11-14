//! Clearance for the fragments that pass a vertex and swing into the tiles round it:
//! the vertex S (`frag_vertex_s`) and the vertex sweep (`frag_vertex_sweep`). The
//! table says which nets around each of them it is drawn clear of.

pub use super::vertex_s_clearance::VERTEX_S_CLEAR;
use common::consts::{A1, A2, A3, A4, A5, A6, N1, N2, N3, N4, N5, N6};

/// The logical keys that draw: the three vertex S (the tile opposite the vertex gets a
/// blank), then the six vertex sweeps (hook, with the arc in the tile opposite).
/// `VERTEX_S_CLEAR` is indexed in this order.
pub const VERTEX_S_KEYS: [usize; 9] = [
    N1 | N5 | A1 | A4,
    N2 | N4 | A2 | A6,
    N3 | N6 | A3 | A5,
    N1 | N4 | A1 | A2,
    N1 | N5 | A1 | A3,
    N2 | N4 | A2 | A1,
    N2 | N6 | A2 | A3,
    N3 | N5 | A3 | A1,
    N3 | N6 | A3 | A2,
];

/// The index in `VERTEX_S_KEYS` of a key (any upper bits beyond the S are ignored), if
/// it is one of them.
pub fn vertex_s_index(key: usize) -> Option<usize> {
    VERTEX_S_KEYS.iter().position(|&k| k == key)
}

/// Whether `key` is a vertex S (the first three of `VERTEX_S_KEYS`), not a sweep.
pub fn is_vertex_s(key: usize) -> bool {
    vertex_s_index(key).is_some_and(|index| index < 3)
}
