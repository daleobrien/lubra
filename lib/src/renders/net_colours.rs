//! Debug colours for tracks: each net in a tile gets its own colour, and the
//! colours shift from tile to tile so neighbouring tracks are easy to tell apart.

/// Colours that read well on white and stay clear of the red debug overlay.
const PALETTE: [&str; 9] = [
    "#1f77b4", "#ff7f0e", "#2ca02c", "#9467bd", "#8c564b", "#17becf", "#bcbd22", "#000075",
    "#469990",
];

/// The colour of the `index`th net of tile `(i, j)` (its position in the tile's net
/// list). Nets in one tile always differ; neighbouring tiles start from different
/// colours.
pub fn net_colour(i: usize, j: usize, index: usize) -> &'static str {
    PALETTE[(i * 2 + j * 5 + index) % PALETTE.len()]
}

/// The position in `tile_nets` of the net that uses any of the node bits in `nodes`.
pub fn net_index(tile_nets: &[usize], nodes: usize) -> Option<usize> {
    tile_nets.iter().position(|net| net & nodes != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nets_in_a_tile_get_different_colours() {
        for (i, j) in [(0, 0), (3, 7), (19, 11)] {
            let colours: Vec<_> = (0..3).map(|k| net_colour(i, j, k)).collect();
            assert_ne!(colours[0], colours[1]);
            assert_ne!(colours[1], colours[2]);
            assert_ne!(colours[0], colours[2]);
        }
    }

    #[test]
    fn test_neighbouring_tiles_start_on_different_colours() {
        assert_ne!(net_colour(4, 4, 0), net_colour(5, 4, 0));
        assert_ne!(net_colour(4, 4, 0), net_colour(4, 5, 0));
    }

    #[test]
    fn test_net_index() {
        assert_eq!(net_index(&[0b000011, 0b010000], 0b010000), Some(1));
        assert_eq!(net_index(&[0b000011], 0b100000), None);
    }
}
