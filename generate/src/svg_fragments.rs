use crate::frag_12::{a12, a13, a23};
use crate::frag_123::a123;
use crate::frag_15::{a14, a15, a24, a26, a35, a36};
use crate::frag_16::{l16, l25, l34};
use crate::frag_arcs::{a1, a2, a3, a4, a45, a456, a46, a5, a56, a6};
use crate::frag_wall::l_wall;
use common::consts::{INVERT, NODE_1, NODE_2, NODE_3, NODE_4, NODE_5, NODE_6, NORMAL, WALL};
use num_traits::ToPrimitive;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

// const ADJ_N1: usize = 0usize;
// const ADJ_N2: usize = 1usize;
// const ADJ_N3: usize = 2usize;
// const ADJ_N4: usize = 3usize;
// const ADJ_N5: usize = 4usize;
// const ADJ_N6: usize = 5usize;

pub fn trim_whitespace(s: &str) -> String {
    let words: Vec<_> = s.split_whitespace().collect();
    return words.join(" ");
}

pub fn generate_all_shapes() {
    let mut paths: BTreeMap<usize, String> = BTreeMap::new();

    for direction in [INVERT, NORMAL] {
        let invert = direction == INVERT;
        for (key, path) in [
            (direction | NODE_1 | NODE_6, l16(invert)),
            (direction | NODE_2 | NODE_5, l25(invert)),
            (direction | NODE_3 | NODE_4, l34(invert)),
            (direction | NODE_1, a1(invert)),
            (direction | NODE_2, a2(invert)),
            (direction | NODE_3, a3(invert)),
            (direction | NODE_4, a4(invert)),
            (direction | NODE_5, a5(invert)),
            (direction | NODE_6, a6(invert)),
            (direction | NODE_1 | NODE_2 | NODE_3, a123(invert)),
            (direction | NODE_1 | NODE_2, a12(invert)),
            (direction | NODE_1 | NODE_3, a13(invert)),
            (direction | NODE_2 | NODE_3, a23(invert)),
            (direction | NODE_2 | NODE_4, a24(invert)),
            (direction | NODE_2 | NODE_6, a26(invert)),
            (direction | NODE_3 | NODE_5, a35(invert)),
            (direction | NODE_3 | NODE_6, a36(invert)),
            (direction | NODE_4 | NODE_5, a45(invert)),
            (direction | NODE_4 | NODE_6, a46(invert)),
            (direction | NODE_5 | NODE_6, a56(invert)),
            (direction | NODE_4 | NODE_5 | NODE_6, a456(invert)),
            (direction | NODE_1 | NODE_5, a15(invert)),
            (direction | NODE_1 | NODE_4, a14(invert)),
            (direction | WALL, l_wall(invert)),
        ] {
            paths.insert(key.to_usize().unwrap_or_default(), path);
        }
    }

    // This is used to look up mathcing adjacent paths.
    let mut adj_paths: BTreeMap<u32, (usize, Vec<u32>)> = BTreeMap::new();
    // e.g. for a NODE_1-4 we can handle a few different matching tiles

    // Change this to a look up function rather than a class with a HashMap.
    let mut file = File::create("../lib/src/renders/fragment.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(b"use ::phf::{Map, phf_map};\n\n");
    let _ = file.write(b"pub static MAP_PATHS: Map<u32, &'static str> = phf_map! {\n");
    for (key, value) in paths {
        let _ = write!(
            file,
            "  {}u32 =>  \"{}\",\n",
            key,
            trim_whitespace(value.as_str())
        );
    }
    let _ = file.write(b"};\n\n");
    let _ = file.flush();

    // Change this to a look up function rather than a class with a HashMap.
    let mut file = File::create("../lib/src/maze/nets_adjacent.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(b"use ::phf::{Map, phf_map};\n\n");
    let _ = file.write(b"pub static ADJ_CHECKS: Map<u32, &'static (usize, &[u32])> = phf_map! {\n");
    for (key, (index, masks)) in adj_paths {
        // Extract out possible masks
        let masks_as_string = masks
            .iter()
            .map(|n| format!("{n}u32"))
            .collect::<Vec<String>>()
            .join(", ");

        let _ = write!(
            file,
            "  {key}u32 =>  &({index}usize, &[{masks_as_string}]), // {key:06b}\n"
        );
    }
    let _ = file.write(b"};\n\n");
    let _ = file.flush();
}
