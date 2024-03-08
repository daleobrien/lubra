use crate::frag_12::{n12, n13, n23};
use crate::frag_123::n123;
use crate::frag_15::{n14, n15, n24, n26, n35, n36};
use crate::frag_15_dot::{a15_5, a24_4, a26_6, a35_5, a36_6, n14_4};
use crate::frag_15_long::{a15_35, a26_36, a35_15, a36_26, n14_24, n24_14};
use crate::frag_15_straight::{a15_25, a24_34, a26_16, a35_25, a36_16, n14_34};
use crate::frag_16::{n16, n25, n34};
use crate::frag_arc_semi::{a45_14, a45_15, a46_24, a46_26, a56_35_semi, a56_36_semi};
use crate::frag_arcs::{a45, a46, a56, n1, n2, n3, n4, n456, n5, n6};
use crate::frag_complex_arcs::{
    n45_14_35, n45_24, n45_24_15, n45_24_35, n45_35, n46_14, n46_14_26, n46_14_36, n46_15_26,
    n46_24_36, n46_35, n56_15, n56_15_36, n56_26, n56_35_26,
};
use crate::frag_wall::l_wall;
use common::consts::{
    A1, A2, A3, A4, A5, A6, B1, B2, B3, B5, B6, IN, N1, N2, N3, N4, N5, N6, NO, WA,
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
            (dir | WA, l_wall(invert)),
            //
            (dir | N1, n1(invert)),
            (dir | N2, n2(invert)),
            (dir | N3, n3(invert)),
            (dir | N4, n4(invert)),
            (dir | N5, n5(invert)),
            (dir | N6, n6(invert)),
            //
            (dir | N1 | N2 | N3, n123(invert)),
            (dir | N4 | N5 | N6, n456(invert)),
            //
            (dir | N1 | N2, n12(invert)),
            (dir | N1 | N3, n13(invert)),
            (dir | N1 | N4, n14(invert)),
            (dir | N1 | N5, n15(invert)),
            (dir | N1 | N6, n16(invert)),
            (dir | N2 | N3, n23(invert)),
            (dir | N2 | N4, n24(invert)),
            (dir | N2 | N5, n25(invert)),
            (dir | N2 | N6, n26(invert)),
            (dir | N3 | N4, n34(invert)),
            (dir | N3 | N5, n35(invert)),
            (dir | N3 | N6, n36(invert)),
            //
            (dir | N1 | N4 | A2 | A4, n14_24(invert)),
            (dir | N1 | N4 | A3 | A4, n14_34(invert)),
            (dir | N1 | N4 | A4 | A6, "".to_string()), // Equivalent to 4-6/1-4
            (dir | N1 | N4 | A4, n14_4(invert)),
            //
            (dir | N1 | N5 | A2 | A5, a15_25(invert)),
            (dir | N1 | N5 | A3 | A5, a15_35(invert)),
            (dir | N1 | N5 | A5 | A6, "".to_string()), // Equivalent to 2-5/5-6
            (dir | N1 | N5 | A5, a15_5(invert)),
            //
            (dir | N2 | N4 | A1 | A4, n24_14(invert)),
            (dir | N2 | N4 | A3 | A4, a24_34(invert)),
            (dir | N2 | N4 | A4 | A5, "".to_string()), // Equivalent to 4-5/2-4
            (dir | N2 | N4 | A4, a24_4(invert)),
            //
            (dir | N2 | N6 | A1 | A6, a26_16(invert)),
            (dir | N2 | N6 | A3 | A6, a26_36(invert)),
            (dir | N2 | N6 | A5 | A6, "".to_string()), // Equivalent to 5-6/2-6
            (dir | N2 | N6 | A6, a26_6(invert)),
            //
            (dir | N3 | N5 | A1 | A5, a35_15(invert)),
            (dir | N3 | N5 | A2 | A5, a35_25(invert)),
            (dir | N3 | N5 | A4 | A5, "".to_string()), // Equivalent to 4-6/3-5
            (dir | N3 | N5 | A5, a35_5(invert)),
            //
            (dir | N3 | N6 | A1 | A6, a36_16(invert)),
            (dir | N3 | N6 | A2 | A6, a36_26(invert)),
            (dir | N3 | N6 | A4 | A6, "".to_string()), // Equivalent to 4-6/3-6
            (dir | N3 | N6 | A6, a36_6(invert)),
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
            // 5-6
            (dir | N5 | N6, a56(invert)),
            (dir | N5 | N6 | A3 | A5, a56_35_semi(invert)),
            (dir | N5 | N6 | A3 | A6, a56_36_semi(invert)),
            (dir | N5 | N6 | A1 | A5 | B2 | B6, n46_15_26(invert)),
            (dir | N5 | N6 | A3 | A5 | B2 | B6, n56_35_26(invert)),
            (dir | N5 | N6 | A1 | A5 | B3 | B6, n56_15_36(invert)),
            (dir | N5 | N6 | A1 | A5, n56_15(invert)),
            (dir | N5 | N6 | A2 | A6, n56_26(invert)),
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
