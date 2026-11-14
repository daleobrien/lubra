//! Clearance for the vertex S (`frag_vertex_s`): which nets around it it is drawn clear of.

pub use super::vertex_s_clearance::VERTEX_S_CLEAR;
use common::consts::{A1, A2, A3, A4, A5, A6, N1, N2, N3, N4, N5, N6};

/// The logical keys of the vertex S that draw (the tile opposite the vertex gets a
/// blank). `VERTEX_S_CLEAR` is indexed in this order.
pub const VERTEX_S_KEYS: [usize; 3] = [N1 | N5 | A1 | A4, N2 | N4 | A2 | A6, N3 | N6 | A3 | A5];

/// The index in `VERTEX_S_KEYS` of a key (any upper bits beyond the S are ignored), if
/// it is a vertex S.
pub fn vertex_s_index(key: usize) -> Option<usize> {
    VERTEX_S_KEYS.iter().position(|&k| k == key)
}
