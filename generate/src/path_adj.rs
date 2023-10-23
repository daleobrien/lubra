use common::consts::{ADJ_N4, ADJ_N5, ADJ_N6, NODE_1, NODE_2, NODE_3, NODE_4, NODE_5, NODE_6};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

pub fn generate_adjacent_paths() {
    // This is used to look up matching adjacent paths.
    let mut adj_paths: BTreeMap<u32, Vec<Vec<(usize, bool, u32)>>> = BTreeMap::new();

    // e.g. for a NODE_1-4 we can handle a few different matching tiles
    adj_paths.insert(
        NODE_1 | NODE_4,
        vec![
            vec![(ADJ_N4, true, NODE_4)], // Adjacent tile to look up, then the expected tile
            vec![(ADJ_N4, true, NODE_4)],
            vec![(ADJ_N4, true, NODE_2 | NODE_4)],
            vec![(ADJ_N4, true, NODE_3 | NODE_4)],
            vec![(ADJ_N4, true, NODE_4 | NODE_5)],
            vec![(ADJ_N4, true, NODE_4 | NODE_6)],
        ],
    );
    adj_paths.insert(
        NODE_1 | NODE_5,
        vec![
            vec![(ADJ_N5, true, NODE_5)],
            vec![(ADJ_N5, true, NODE_2 | NODE_5)],
            vec![(ADJ_N5, true, NODE_3 | NODE_5)],
            //vec![(ADJ_N5, true, NODE_4 | NODE_5)],
            vec![(ADJ_N5, true, NODE_5 | NODE_6)],
        ],
    );

    adj_paths.insert(
        NODE_2 | NODE_4,
        vec![
            vec![(ADJ_N4, true, NODE_4)],
            vec![(ADJ_N4, true, NODE_1 | NODE_4)],
            vec![(ADJ_N4, true, NODE_3 | NODE_4)],
            vec![(ADJ_N4, true, NODE_4 | NODE_5)],
            vec![(ADJ_N4, true, NODE_4 | NODE_6)],
        ],
    );

    adj_paths.insert(
        NODE_2 | NODE_6,
        vec![
            vec![(ADJ_N6, true, NODE_6)],
            vec![(ADJ_N6, true, NODE_1 | NODE_6)],
            vec![(ADJ_N6, true, NODE_3 | NODE_6)],
            vec![(ADJ_N6, true, NODE_4 | NODE_6)],
            vec![(ADJ_N6, true, NODE_5 | NODE_6)],
        ],
    );

    adj_paths.insert(
        NODE_3 | NODE_5,
        vec![
            vec![(ADJ_N5, true, NODE_5)],
            vec![(ADJ_N5, true, NODE_1 | NODE_5)],
            vec![(ADJ_N5, true, NODE_2 | NODE_5)],
            vec![(ADJ_N5, true, NODE_5 | NODE_6)],
            vec![(ADJ_N5, true, NODE_4 | NODE_5)],
        ],
    );
    adj_paths.insert(
        NODE_3 | NODE_6,
        vec![
            vec![(ADJ_N6, true, NODE_6)],
            vec![(ADJ_N6, true, NODE_1 | NODE_6)],
            vec![(ADJ_N6, true, NODE_2 | NODE_6)],
            vec![(ADJ_N6, true, NODE_4 | NODE_6)],
            vec![(ADJ_N6, true, NODE_5 | NODE_6)],
        ],
    );

    // Blanks
    adj_paths.insert(
        NODE_6,
        vec![
            vec![(ADJ_N6, true, NODE_2 | NODE_6)],
            vec![(ADJ_N6, true, NODE_3 | NODE_6)],
        ],
    );
    adj_paths.insert(
        NODE_5,
        vec![
            vec![(ADJ_N5, true, NODE_1 | NODE_5)],
            vec![(ADJ_N5, true, NODE_3 | NODE_5)],
        ],
    );
    adj_paths.insert(
        NODE_4,
        vec![
            vec![(ADJ_N4, true, NODE_1 | NODE_4)],
            vec![(ADJ_N4, true, NODE_2 | NODE_4)],
        ],
    );
    adj_paths.insert(
        NODE_4 | NODE_5,
        vec![
            vec![
                (ADJ_N4, true, NODE_1 | NODE_4),
                (ADJ_N5, true, NODE_3 | NODE_5),
            ],
            vec![
                (ADJ_N4, true, NODE_2 | NODE_4),
                (ADJ_N5, true, NODE_1 | NODE_5),
            ],
            vec![
                (ADJ_N4, false, NODE_1 | NODE_4),
                (ADJ_N5, true, NODE_3 | NODE_5),
            ],
            vec![
                (ADJ_N5, false, NODE_1 | NODE_5),
                (ADJ_N4, true, NODE_2 | NODE_4),
            ],
            vec![
                (ADJ_N4, true, NODE_1 | NODE_4),
                //(ADJ_N5, false, NODE_3 | NODE_5),
            ],
        ],
    );
    adj_paths.insert(
        NODE_4 | NODE_6,
        vec![
            vec![
                (ADJ_N4, true, NODE_2 | NODE_4),
                (ADJ_N6, true, NODE_3 | NODE_6),
            ],
            vec![
                (ADJ_N4, true, NODE_1 | NODE_4),
                (ADJ_N6, true, NODE_2 | NODE_6),
            ],
            vec![(ADJ_N4, true, NODE_1 | NODE_4)],
            vec![(ADJ_N6, true, NODE_3 | NODE_6)],
        ],
    );
    adj_paths.insert(
        NODE_5 | NODE_6,
        vec![
            vec![
                (ADJ_N5, true, NODE_1 | NODE_5),
                (ADJ_N6, true, NODE_3 | NODE_6),
            ],
            vec![
                (ADJ_N5, true, NODE_3 | NODE_5),
                (ADJ_N6, true, NODE_2 | NODE_6),
            ],
            vec![(ADJ_N5, true, NODE_1 | NODE_5)],
            vec![(ADJ_N6, true, NODE_2 | NODE_6)],
        ],
    );
    // Change this to a look up function rather than a class with a HashMap.
    let mut file = File::create("../lib/src/maze/nets_adjacent.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(b"use ::phf::{Map, phf_map};\n\n");
    let _ = file.write(
        b"pub static ADJ_CHECKS: Map<u32, &'static [&'static [(usize, bool, u32)]]> = phf_map! {\n",
    );
    for (key, list_index_and_masks) in adj_paths {
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
