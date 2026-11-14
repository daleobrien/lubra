use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

use crate::fragments::frag_hook_fork::hook_forks;
use common::bit_utils::print_bits;
use common::consts::{ADJ_N1, ADJ_N2, ADJ_N3, ADJ_N4, ADJ_N5, ADJ_N6, N1, N2, N3, N4, N5, N6};

/// One neighbour check: (neighbour index, must match, expected net in that neighbour).
type AdjCheck = (usize, bool, usize);

pub fn generate_adjacent_paths() {
    // This is used to look up matching adjacent paths.
    let mut adj_paths: BTreeMap<usize, Vec<Vec<AdjCheck>>> = BTreeMap::new();

    // e.g. for a NODE_1-4 we can handle a few different matching tiles
    // The booleans is to make sure it matches, if false, it must not be the tile
    adj_paths.insert(
        N1 | N4,
        vec![
            // A dead end across the edge, reached from the tile opposite the vertex:
            // drawn as one U-turn round the vertex (frag_corner_hook).
            vec![(ADJ_N4, true, N4), (ADJ_N1, true, N1 | N4)],
            // The track goes on through the vertex into the tile opposite: one S (frag_vertex_s).
            vec![(ADJ_N1, true, N1 | N5)],
            vec![(ADJ_N4, true, N4)], // Adjacent tile to look up, then the expected tile
            vec![(ADJ_N4, true, N2 | N4)],
            vec![(ADJ_N4, true, N3 | N4)],
            vec![(ADJ_N4, true, N4 | N5)], // Will be blank
            vec![(ADJ_N4, true, N4 | N6)], // Will be blank
        ],
    );
    adj_paths.insert(
        N1 | N5,
        vec![
            // A dead end across the edge, reached from the tile opposite the vertex:
            // drawn as one U-turn round the vertex (frag_corner_hook).
            vec![(ADJ_N5, true, N5), (ADJ_N1, true, N1 | N5)],
            // The track goes on through the vertex into the tile opposite: one S (frag_vertex_s).
            vec![(ADJ_N1, true, N1 | N4)],
            vec![(ADJ_N5, true, N5)],
            vec![(ADJ_N5, true, N2 | N5)],
            vec![(ADJ_N5, true, N3 | N5)],
            vec![(ADJ_N5, true, N4 | N5)], // Will be blank
            vec![(ADJ_N5, true, N5 | N6)], // Will be blank
        ],
    );

    adj_paths.insert(
        N1 | N6,
        vec![
            vec![(ADJ_N6, true, N2 | N6)], // Will be blank
            vec![(ADJ_N6, true, N3 | N6)], // Will be blank
        ],
    );

    adj_paths.insert(
        N2 | N4,
        vec![
            // A dead end across the edge, reached from the tile opposite the vertex:
            // drawn as one U-turn round the vertex (frag_corner_hook).
            vec![(ADJ_N4, true, N4), (ADJ_N2, true, N2 | N4)],
            // The track goes on through the vertex into the tile opposite: one S (frag_vertex_s).
            vec![(ADJ_N2, true, N2 | N6)],
            vec![(ADJ_N4, true, N4)],
            vec![(ADJ_N4, true, N1 | N4)],
            vec![(ADJ_N4, true, N3 | N4)],
            vec![(ADJ_N4, true, N4 | N5)], // Will be blank
            vec![(ADJ_N4, true, N4 | N6)], // Will be blank
        ],
    );

    adj_paths.insert(
        N2 | N5,
        vec![
            vec![(ADJ_N5, true, N1 | N5)], // Will be blank
            vec![(ADJ_N5, true, N3 | N5)], // Will be blank
        ],
    );

    adj_paths.insert(
        N2 | N6,
        vec![
            // A dead end across the edge, reached from the tile opposite the vertex:
            // drawn as one U-turn round the vertex (frag_corner_hook).
            vec![(ADJ_N6, true, N6), (ADJ_N2, true, N2 | N6)],
            // The track goes on through the vertex into the tile opposite: one S (frag_vertex_s).
            vec![(ADJ_N2, true, N2 | N4)],
            vec![(ADJ_N6, true, N6)],
            vec![(ADJ_N6, true, N1 | N6)],
            vec![(ADJ_N6, true, N3 | N6)],
            vec![(ADJ_N6, true, N4 | N6)], // Will be blank
                                           // vec![(ADJ_N6, true, N5 | N6)], // Will be blank
        ],
    );

    adj_paths.insert(
        N3 | N4,
        vec![
            vec![(ADJ_N4, true, N1 | N4)], // Will be blank
            vec![(ADJ_N4, true, N2 | N4)], // Will be blank
        ],
    );

    adj_paths.insert(
        N3 | N5,
        vec![
            // A dead end across the edge, reached from the tile opposite the vertex:
            // drawn as one U-turn round the vertex (frag_corner_hook).
            vec![(ADJ_N5, true, N5), (ADJ_N3, true, N3 | N5)],
            // The track goes on through the vertex into the tile opposite: one S (frag_vertex_s).
            vec![(ADJ_N3, true, N3 | N6)],
            vec![(ADJ_N5, true, N5)],
            vec![(ADJ_N5, true, N1 | N5)],
            vec![(ADJ_N5, true, N2 | N5)],
            vec![(ADJ_N5, true, N4 | N5)], // Will be blank
            vec![(ADJ_N5, true, N5 | N6)], // Will be blank
        ],
    );
    adj_paths.insert(
        N3 | N6,
        vec![
            // A dead end across the edge, reached from the tile opposite the vertex:
            // drawn as one U-turn round the vertex (frag_corner_hook).
            vec![(ADJ_N6, true, N6), (ADJ_N3, true, N3 | N6)],
            // The track goes on through the vertex into the tile opposite: one S (frag_vertex_s).
            vec![(ADJ_N3, true, N3 | N5)],
            vec![(ADJ_N6, true, N6)],
            vec![(ADJ_N6, true, N1 | N6)],
            vec![(ADJ_N6, true, N2 | N6)],
            vec![(ADJ_N6, true, N4 | N6)], // Will be blank
            vec![(ADJ_N6, true, N5 | N6)], // Will be blank
        ],
    );

    adj_paths.insert(
        N4 | N5,
        vec![
            vec![(ADJ_N4, true, N1 | N4), (ADJ_N5, true, N3 | N5)],
            vec![(ADJ_N4, true, N2 | N4), (ADJ_N5, true, N1 | N5)],
            vec![(ADJ_N4, true, N2 | N4), (ADJ_N5, true, N3 | N5)],
            vec![(ADJ_N4, true, N1 | N4)],
            vec![(ADJ_N4, true, N2 | N4)],
            vec![(ADJ_N5, true, N1 | N5)],
            vec![(ADJ_N5, true, N3 | N5)],
        ],
    );
    adj_paths.insert(
        N4 | N6,
        vec![
            vec![(ADJ_N4, true, N1 | N4), (ADJ_N6, true, N2 | N6)],
            vec![(ADJ_N4, true, N1 | N4), (ADJ_N6, true, N3 | N6)],
            vec![(ADJ_N4, true, N2 | N4), (ADJ_N6, true, N3 | N6)],
            vec![(ADJ_N4, true, N1 | N4)],
            vec![(ADJ_N4, true, N2 | N4)],
            vec![(ADJ_N6, true, N2 | N6)],
            vec![(ADJ_N6, true, N3 | N6)],
        ],
    );
    adj_paths.insert(
        N5 | N6,
        vec![
            vec![(ADJ_N5, true, N1 | N5), (ADJ_N6, true, N2 | N6)],
            vec![(ADJ_N5, true, N1 | N5), (ADJ_N6, true, N3 | N6)],
            vec![(ADJ_N5, true, N3 | N5), (ADJ_N6, true, N2 | N6)],
            vec![(ADJ_N5, true, N1 | N5)],
            vec![(ADJ_N5, true, N3 | N5)],
            vec![(ADJ_N6, true, N2 | N6)],
            vec![(ADJ_N6, true, N3 | N6)],
        ],
    );

    // A junction across an edge that hooks round the arc's own vertex: the hook and
    // the arc are drawn as one half-circle (frag_hook_fork). Tried after the rules
    // above, so a plain hook on the arc's other side still comes first.
    for (arc, junction, _, edge, _) in hook_forks() {
        adj_paths
            .entry(arc)
            .or_default()
            .push(vec![(edge - 1, true, junction)]);
    }

    // Change this to a look up function rather than a class with a HashMap.
    let mut file =
        File::create("../lib/src/maze/generated/nets_adjacent.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(b"use ::phf::{Map, phf_map};\n\n");
    let _ = file.write(
        b"pub static ADJ_CHECKS: Map<usize, &'static [&'static [(usize, bool, usize)]]> = phf_map! {\n",
    );
    for (key, list_index_and_masks) in adj_paths {
        let mut subs: Vec<String> = vec![];

        for index_and_masks in list_index_and_masks {
            let masks_as_string = index_and_masks
                .iter()
                .map(|(i, include, n)| format!("({i}usize, {include}, {n}usize)"))
                .collect::<Vec<String>>()
                .join(", ");
            subs.push(format!("&[{masks_as_string}]"));
        }
        let _ = writeln!(
            file,
            "  /* {key6bit:^4} */ {key}usize =>  &[{subs}],",
            subs = subs.join(", "),
            key6bit = print_bits(key)
        );
    }
    let _ = file.write(b"};\n\n");
    let _ = file.flush();
}
