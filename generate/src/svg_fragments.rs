use crate::frag_12::{a23, n12, n13};
use crate::frag_123::n123;
use crate::frag_15::{a14, a15, a24, a26, a35, a36};
use crate::frag_155::{a144, a155};
use crate::frag_15_dot::{a14_4, a15_5, a24_4, a26_6, a35_5, a36_6};
use crate::frag_15_long::{a14_24, a15_35, a24_14, a26_36, a35_15, a36_26};
use crate::frag_15_straight::{a14_34, a15_25, a24_34, a26_16, a35_25, a36_16};
use crate::frag_16::{l16, l25, l34};
use crate::frag_arc_halfs::{
    a45_24_half, a45_35_half, a46_14_half, a46_36_half, a56_15_half, a56_26_half,
};
use crate::frag_arc_long::{a45_24_long, a46_14_long, a46_36_long, a56_15_long, a56_26_long};
use crate::frag_arc_semi::{a45_14, a45_15, a46_24, a46_26, a56_35_semi, a56_36_semi};
use crate::frag_arcs::{a1, a2, a3, a4, a45, a456, a46, a5, a56, a6};
use crate::frag_complex_arcs::{
    n45_14_35, n45_24, n45_24_15, n45_24_35, n45_35, n46_14, n46_14_26, n46_14_36, n46_24_36,
    n46_35,
};
use crate::frag_wall::l_wall;
use common::consts::{
    A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, B6, IN, N1, N2, N3, N4, N5, N6, NO, WA,
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

    for dir in [IN, NO] {
        let invert = dir == IN;
        for (key, path) in [
            (dir | N1 | N2 | N3, n123(invert)),
            (dir | N1 | N2, n12(invert)),
            (dir | N1 | N3, n13(invert)),
            (dir | N1 | N4 | A1 | A2 | A3 | A4 | A6, a155(invert)),
            (dir | N1 | N4 | A2 | A4, a14_24(invert)),
            (dir | N1 | N4 | A3 | A4, a14_34(invert)),
            (dir | N1 | N4 | A4 | A6, a45_24_long(invert)),
            (dir | N1 | N4 | A4, a14_4(invert)),
            (dir | N1 | N4, a14(invert)),
            (dir | N1 | N5 | A2 | A5, a15_25(invert)),
            (dir | N1 | N5 | A3 | A5, a15_35(invert)),
            (dir | N1 | N5 | A5, a15_5(invert)),
            (dir | N1 | N5, a15(invert)),
            (dir | N1 | N6, l16(invert)),
            (dir | N1, a1(invert)),
            (dir | N2 | N3, a23(invert)),
            (dir | N2 | N4 | A1 | A4, a24_14(invert)),
            (dir | N2 | N4 | A3 | A4, a24_34(invert)),
            (dir | N2 | N4 | A4 | A5, a56_26_long(invert)),
            (dir | N2 | N4 | A4, a24_4(invert)),
            (dir | N2 | N4, a24(invert)),
            (dir | N2 | N5, l25(invert)),
            (dir | N2 | N6 | A1 | A6, a26_16(invert)),
            (dir | N2 | N6 | A3 | A6, a26_36(invert)),
            (dir | N2 | N6 | A5 | A6, a56_15_long(invert)),
            (dir | N2 | N6 | A6, a26_6(invert)),
            (dir | N2 | N6, a26(invert)),
            (dir | N2, a2(invert)),
            (dir | N3 | N4, l34(invert)),
            (dir | N3 | N5 | A1 | A5, a35_15(invert)),
            (dir | N3 | N5 | A2 | A5, a35_25(invert)),
            (dir | N3 | N5 | A4 | A5, a46_36_long(invert)),
            (dir | N3 | N5 | A5, a35_5(invert)),
            (dir | N3 | N5, a35(invert)),
            (dir | N3 | N6 | A1 | A6, a36_16(invert)),
            (dir | N3 | N6 | A2 | A6, a36_26(invert)),
            (dir | N3 | N6 | A4 | A6, a46_14_long(invert)),
            (dir | N3 | N6 | A6, a36_6(invert)),
            (dir | N3 | N6, a36(invert)),
            (dir | N3, a3(invert)),
            // 4-5
            (dir | N4 | N5, a45(invert)),
            (dir | N4 | N5 | A1 | A4, a45_14(invert)),
            (dir | N4 | N5 | A1 | A5, a45_15(invert)),
            (dir | N4 | N5 | A2 | A4 | B3 | B5, n45_24_35(invert)),
            (dir | N4 | N5 | A1 | A4 | B3 | B5, n45_14_35(invert)),
            (dir | N4 | N5 | A2 | A4 | B1 | B5, n45_24_15(invert)),
            (dir | N4 | N5 | A2 | A4, n45_24(invert)),
            (dir | N4 | N5 | A3 | A5, n45_35(invert)),
            // 4-6
            (dir | N4 | N6, a46(invert)),
            (dir | N4 | N6 | A2 | A6, a46_26(invert)),
            (dir | N4 | N6 | A2 | A4, a46_24(invert)),
            (dir | N4 | N6 | A1 | A4 | B3 | B6, n46_14_36(invert)),
            (dir | N4 | N6 | A1 | A4 | B2 | B6, n46_14_26(invert)),
            (dir | N4 | N6 | A2 | A4 | B3 | B6, n46_24_36(invert)),
            (dir | N4 | N6 | A1 | A4, n46_14(invert)),
            (dir | N4 | N6 | A3 | A6, n46_35(invert)),
            //
            (dir | N4 | N5 | N6, a456(invert)),
            //
            (dir | N4, a4(invert)),
            (dir | N5 | N6 | A1 | A5, a56_15_half(invert)),
            (dir | N5 | N6 | A2 | A6, a56_26_half(invert)),
            (dir | N5 | N6 | A3 | A5, a56_35_semi(invert)), // New
            (dir | N5 | N6 | A3 | A6, a56_36_semi(invert)), // New
            (dir | N5 | N6, a56(invert)),
            (dir | N5, a5(invert)),
            (dir | N6, a6(invert)),
            (dir | WA, l_wall(invert)),
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
