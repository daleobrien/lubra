use crate::frag_12::{a12, a13, a23};
use crate::frag_123::a123;
use crate::frag_15::{a14, a15, a24, a26, a35, a36};
use crate::frag_155::a144;
use crate::frag_15_dot::{a14_4, a15_5, a24_4, a26_6, a35_5, a36_6};
use crate::frag_15_long::{a14_24, a15_35, a24_14, a26_36, a35_15, a36_26};
use crate::frag_15_straight::{a14_34, a15_25, a24_34, a26_16, a35_25, a36_16};
use crate::frag_16::{l16, l25, l34};
use crate::frag_arc_halfs::{
    a45_24_half, a45_35_half, a46_14_half, a46_36_half, a56_15_half, a56_26_half,
};
use crate::frag_arc_long::{
    a45_24_long, a45_35_long, a46_14_long, a46_36_long, a56_15_long, a56_26_long,
};
use crate::frag_arc_semi::{
    a45_14_semi, a45_15_semi, a46_24_semi, a46_26_semi, a56_35_semi, a56_36_semi,
};
use crate::frag_arcs::{a1, a2, a3, a4, a45, a456, a46, a5, a56, a6};
use crate::frag_complex_arcs::{a_14_45_15, a_14_45_35};
use crate::frag_wall::l_wall;
use common::consts::{
    ADJ_1, ADJ_2, ADJ_3, ADJ_4, ADJ_5, ADJ_6, INVERT, NODE_1, NODE_2, NODE_3, NODE_4, NODE_5,
    NODE_6, NORMAL, WALL,
};
use num_traits::ToPrimitive;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

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
            (direction | NODE_4 | NODE_6, a46(invert)),
            (direction | NODE_5 | NODE_6, a56(invert)),
            (direction | NODE_1 | NODE_5, a15(invert)),
            // Complex
            (direction | NODE_1 | NODE_4 | ADJ_2 | ADJ_4, a14_24(invert)),
            (direction | NODE_1 | NODE_4 | ADJ_3 | ADJ_4, a14_34(invert)),
            (direction | NODE_1 | NODE_4 | ADJ_4, a14_4(invert)),
            (
                direction | NODE_1 | NODE_4 | ADJ_4 | ADJ_6,
                a45_24_long(invert),
            ),
            (direction | NODE_1 | NODE_5 | ADJ_2 | ADJ_5, a15_25(invert)),
            (direction | NODE_1 | NODE_5 | ADJ_3 | ADJ_5, a15_35(invert)),
            (direction | NODE_1 | NODE_5 | ADJ_5, a15_5(invert)),
            (direction | NODE_2 | NODE_4 | ADJ_1 | ADJ_4, a24_14(invert)),
            (direction | NODE_2 | NODE_4 | ADJ_3 | ADJ_4, a24_34(invert)),
            (direction | NODE_2 | NODE_4 | ADJ_4, a24_4(invert)),
            (
                direction | NODE_2 | NODE_4 | ADJ_4 | ADJ_5,
                a56_26_long(invert),
            ),
            (direction | NODE_2 | NODE_6 | ADJ_1 | ADJ_6, a26_16(invert)),
            (direction | NODE_2 | NODE_6 | ADJ_3 | ADJ_6, a26_36(invert)),
            (
                direction | NODE_2 | NODE_6 | ADJ_5 | ADJ_6,
                a56_15_long(invert),
            ),
            (direction | NODE_2 | NODE_6 | ADJ_6, a26_6(invert)),
            //
            (direction | NODE_3 | NODE_5 | ADJ_1 | ADJ_5, a35_15(invert)),
            (direction | NODE_3 | NODE_5 | ADJ_2 | ADJ_5, a35_25(invert)),
            (direction | NODE_3 | NODE_5 | ADJ_5, a35_5(invert)),
            (
                direction | NODE_3 | NODE_5 | ADJ_4 | ADJ_5,
                a46_36_long(invert),
            ),
            //
            (direction | NODE_3 | NODE_6 | ADJ_1 | ADJ_6, a36_16(invert)),
            (direction | NODE_3 | NODE_6 | ADJ_2 | ADJ_6, a36_26(invert)),
            (
                direction | NODE_3 | NODE_6 | ADJ_4 | ADJ_6,
                a46_14_long(invert),
            ),
            // arcs
            (
                direction | NODE_4 | NODE_5 | ADJ_1 | ADJ_3 | ADJ_4 | ADJ_5,
                a_14_45_35(invert),
            ),
            (
                direction | NODE_4 | NODE_5 | ADJ_1 | ADJ_4 | ADJ_5,
                a_14_45_15(invert),
            ),
            (direction | NODE_4 | NODE_5, a45(invert)),
            (direction | NODE_4 | NODE_5 | NODE_6, a456(invert)),
            (direction | NODE_4 | NODE_5 | NODE_6, a456(invert)),
            // TODO:
            // (
            //     direction | NODE_1 | NODE_5 | ADJ_5 | ADJ_6,
            //     a45_35_long(invert),
            // ),
            // Half arcs
            (
                direction | NODE_4 | NODE_5 | ADJ_3 | ADJ_5,
                a45_35_half(invert),
            ),
            (
                direction | NODE_4 | NODE_5 | ADJ_2 | ADJ_4,
                a45_24_half(invert),
            ),
            (
                direction | NODE_4 | NODE_5 | ADJ_1 | ADJ_4,
                a45_14_semi(invert),
            ), // New
            (
                direction | NODE_4 | NODE_5 | ADJ_1 | ADJ_5,
                a45_15_semi(invert),
            ), // New
            //
            (
                direction | NODE_4 | NODE_6 | ADJ_1 | ADJ_4,
                a46_14_half(invert),
            ),
            (
                direction | NODE_4 | NODE_6 | ADJ_3 | ADJ_6,
                a46_36_half(invert),
            ),
            (
                direction | NODE_4 | NODE_6 | ADJ_2 | ADJ_4,
                a46_24_semi(invert),
            ), // New
            (
                direction | NODE_4 | NODE_6 | ADJ_2 | ADJ_6,
                a46_26_semi(invert),
            ), // New
            (
                direction | NODE_5 | NODE_6 | ADJ_1 | ADJ_5,
                a56_15_half(invert),
            ),
            (
                direction | NODE_5 | NODE_6 | ADJ_2 | ADJ_6,
                a56_26_half(invert),
            ),
            (
                direction | NODE_5 | NODE_6 | ADJ_3 | ADJ_5,
                a56_35_semi(invert),
            ), // New
            (
                direction | NODE_5 | NODE_6 | ADJ_3 | ADJ_6,
                a56_36_semi(invert),
            ), // New
            //
            (direction | NODE_3 | NODE_6 | ADJ_6, a36_6(invert)),
            (direction | NODE_1 | NODE_4, a14(invert)),
            (direction | WALL, l_wall(invert)),
        ] {
            paths.insert(key.to_usize().unwrap_or_default(), path);
        }
    }

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
}
