use common::consts::{ADJ_N4, ADJ_N5, ADJ_N6, NODE_1, NODE_2, NODE_3, NODE_4, NODE_5, NODE_6};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

pub fn generate_blank_paths() {
    let mut blank_paths: BTreeMap<u32, Vec<Vec<(usize, bool, u32)>>> = BTreeMap::new();
    // Arc
    blank_paths.insert(
        NODE_4 | NODE_5,
        vec![
            vec![
                (ADJ_N4, true, NODE_1 | NODE_4),
                (ADJ_N5, true, NODE_3 | NODE_5),
            ],
            vec![
                (ADJ_N4, true, NODE_2 | NODE_4),
                (ADJ_N5, true, NODE_3 | NODE_5),
            ],
            vec![(ADJ_N4, true, NODE_1 | NODE_4)],
            vec![(ADJ_N5, true, NODE_1 | NODE_5)],
        ],
    );
    //
    // blank_paths.insert(
    //     NODE_4 | NODE_6,
    //     vec![
    //         vec![
    //             (ADJ_N4, true, NODE_1 | NODE_4),
    //             (ADJ_N6, true, NODE_3 | NODE_6),
    //         ],
    //         vec![(ADJ_N4, true, NODE_2 | NODE_4)],
    //         vec![(ADJ_N6, true, NODE_2 | NODE_6)],
    //     ],
    // );
    //
    // blank_paths.insert(
    //     NODE_5 | NODE_6,
    //     vec![
    //         vec![
    //             (ADJ_N5, true, NODE_1 | NODE_5),
    //             (ADJ_N6, true, NODE_2 | NODE_6),
    //         ],
    //         vec![(ADJ_N5, true, NODE_3 | NODE_5)],
    //         vec![(ADJ_N6, true, NODE_3 | NODE_6)],
    //     ],
    // );

    //  Tight Curve when connecting to an arc, we will draw that from the arcs point of view
    blank_paths.insert(NODE_1 | NODE_4, vec![vec![(ADJ_N4, true, NODE_4 | NODE_5)]]);
    blank_paths.insert(NODE_1 | NODE_5, vec![vec![(ADJ_N5, true, NODE_4 | NODE_5)]]);
    blank_paths.insert(NODE_2 | NODE_4, vec![vec![(ADJ_N4, true, NODE_4 | NODE_6)]]);
    blank_paths.insert(NODE_2 | NODE_6, vec![vec![(ADJ_N6, true, NODE_4 | NODE_6)]]);
    blank_paths.insert(NODE_3 | NODE_5, vec![vec![(ADJ_N5, true, NODE_5 | NODE_6)]]);
    blank_paths.insert(NODE_3 | NODE_6, vec![vec![(ADJ_N6, true, NODE_5 | NODE_6)]]);

    // straight line through one vertex
    blank_paths.insert(
        NODE_1 | NODE_6,
        vec![
            vec![(ADJ_N6, true, NODE_2 | NODE_6)],
            vec![(ADJ_N6, true, NODE_3 | NODE_6)],
        ],
    );
    blank_paths.insert(
        NODE_2 | NODE_5,
        vec![
            vec![(ADJ_N5, true, NODE_1 | NODE_5)],
            vec![(ADJ_N5, true, NODE_3 | NODE_5)],
        ],
    );
    blank_paths.insert(
        NODE_3 | NODE_4,
        vec![
            vec![(ADJ_N4, true, NODE_1 | NODE_4)],
            vec![(ADJ_N4, true, NODE_2 | NODE_4)],
        ],
    );

    // Point
    blank_paths.insert(
        NODE_4,
        vec![
            vec![(ADJ_N4, true, NODE_1 | NODE_4)],
            vec![(ADJ_N4, true, NODE_2 | NODE_4)],
        ],
    );
    blank_paths.insert(
        NODE_5,
        vec![
            vec![(ADJ_N5, true, NODE_1 | NODE_5)],
            vec![(ADJ_N5, true, NODE_3 | NODE_5)],
        ],
    );
    blank_paths.insert(
        NODE_6,
        vec![
            vec![(ADJ_N6, true, NODE_2 | NODE_6)],
            vec![(ADJ_N6, true, NODE_3 | NODE_6)],
        ],
    );

    // Change this to a look up function rather than a class with a HashMap.
    let mut file = File::create("../lib/src/maze/nets_blank.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(b"use ::phf::{Map, phf_map};\n\n");
    let _ = file.write(
        b"pub static BLANK_CHECK_PATHS: Map<u32, &'static [&'static [(usize, bool, u32)]]> = phf_map! {\n",
    );
    for (key, list_index_and_masks) in blank_paths {
        // Extract out possible masks

        let mut subs: Vec<String> = vec![];

        for index_and_masks in list_index_and_masks {
            let masks_as_string = index_and_masks
                .iter()
                .map(|(i, include, n)| format!("({i}usize, {include}, {n}u32)"))
                .collect::<Vec<String>>()
                .join(", ");
            subs.push(format!("&[{masks_as_string}]"));
        }
        let _ = write!(
            file,
            "  {key}u32 =>  &[{subs}], // {key:06b}\n",
            subs = subs.join(", ")
        );
    }
    let _ = file.write(b"};\n\n");
    let _ = file.flush();
}
