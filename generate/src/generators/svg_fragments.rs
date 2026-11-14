use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::Write;

use itertools::Itertools;
use num_traits::ToPrimitive;

use common::bit_utils::{print_bits, sort_upper_bits, to_6bit_binary};
use common::consts::{
    A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, B6, IN, N1, N2, N3, N4, N5, N6, NL, NO, WA,
};

use crate::fragments::frag_1::{n1, n2, n3, n4, n5, n6};
use crate::fragments::frag_12::{n12, n13, n23};
use crate::fragments::frag_123::n123;
use crate::fragments::frag_12345::{n12345, n12346, n12356};
use crate::fragments::frag_123456::n123456;
use crate::fragments::frag_1235::{n1234, n1235, n1236};
use crate::fragments::frag_125::{n125, n126, n134, n136, n234, n235};
use crate::fragments::frag_1345::{n1245, n1246, n1345, n1356, n2346, n2356};
use crate::fragments::frag_13456::{n12456, n13456, n23456};
use crate::fragments::frag_1346::{n1256, n1346, n2345};
use crate::fragments::frag_135::{n124, n135, n236};
use crate::fragments::frag_145::{n145, n246, n356};
use crate::fragments::frag_1456::{n1456, n2456, n3456};
use crate::fragments::frag_15::{n14, n15, n24, n26, n35, n36};
use crate::fragments::frag_156::{n146, n156, n245, n256, n345, n346};
use crate::fragments::frag_15_35::{n15_35, n24_14, n36_26};
use crate::fragments::frag_15_dot::{n14_4, n15_5, n24_4, n26_6, n35_5, n36_6};
use crate::fragments::frag_15_straight::{n14_34, n15_25, n24_34, n26_16, n35_25, n36_16};
use crate::fragments::frag_16::{n16, n25, n34};
use crate::fragments::frag_arc_semi::{n45_14, n45_15, n46_24, n46_26, n56_35, n56_36};
use crate::fragments::frag_arcs::{a46, n45, n456, n56};
use crate::fragments::frag_complex_arcs::{n45_24, n45_35, n46_14, n46_35, n56_15, n56_26};
use crate::fragments::frag_corner_hook::{
    n14_14_4, n15_15_5, n24_24_4, n26_26_6, n35_35_5, n36_36_6,
};
use crate::fragments::frag_crescent::{n45_24_35, n46_14_36, n46_15_26};
use crate::fragments::frag_s_curve::{
    n45_14_35, n45_24_15, n46_14_26, n46_24_36, n56_15_36, n56_35_26,
};
use crate::fragments::frag_vertex_s::{n15_14, n24_26, n36_35};
use crate::fragments::frag_wall::l_wall;

pub fn trim_whitespace(s: &str) -> String {
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

pub fn generate_all_shapes() {
    let mut paths: BTreeMap<usize, String> = BTreeMap::new();

    // TODO:!!!! 4-5 5 and 4-5 4 !!!! appears in docs! and doesn't make sense

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
            // 3 ways ...
            //
            (dir | N1 | N2 | N3, n123(invert)),
            (dir | N4 | N5 | N6, n456(invert)),
            //
            // 3 way, one complete side
            (dir | N1 | N3 | N5, n135(invert)),
            (dir | N1 | N2 | N4, n124(invert)),
            (dir | N2 | N3 | N6, n236(invert)),
            //
            //
            (dir | N1 | N2 | N5, n125(invert)),
            (dir | N1 | N3 | N4, n134(invert)),
            (dir | N1 | N2 | N6, n126(invert)),
            (dir | N2 | N3 | N4, n234(invert)),
            (dir | N2 | N3 | N5, n235(invert)),
            (dir | N1 | N3 | N6, n136(invert)),
            //
            (dir | N1 | N4 | N5, n145(invert)),
            (dir | N2 | N4 | N6, n246(invert)),
            (dir | N3 | N5 | N6, n356(invert)),
            //
            //
            (dir | N1 | N4 | N6, n146(invert)),
            (dir | N1 | N5 | N6, n156(invert)),
            (dir | N2 | N4 | N5, n245(invert)),
            (dir | N2 | N5 | N6, n256(invert)),
            (dir | N3 | N4 | N5, n345(invert)),
            (dir | N3 | N4 | N6, n346(invert)),
            //
            // 4 ways
            //
            (dir | N1 | N4 | N5 | N6, n1456(invert)),
            (dir | N2 | N4 | N5 | N6, n2456(invert)),
            (dir | N3 | N4 | N5 | N6, n3456(invert)),
            //
            (dir | N1 | N3 | N4 | N5, n1345(invert)),
            (dir | N1 | N2 | N4 | N5, n1245(invert)),
            (dir | N1 | N2 | N4 | N6, n1246(invert)),
            (dir | N2 | N3 | N4 | N6, n2346(invert)),
            (dir | N2 | N3 | N5 | N6, n2356(invert)),
            (dir | N1 | N3 | N5 | N6, n1356(invert)),
            //
            (dir | N1 | N3 | N4 | N6, n1346(invert)),
            (dir | N1 | N2 | N5 | N6, n1256(invert)),
            (dir | N2 | N3 | N4 | N5, n2345(invert)),
            //
            (dir | N1 | N2 | N3 | N5, n1235(invert)),
            (dir | N1 | N2 | N3 | N4, n1234(invert)),
            (dir | N1 | N2 | N3 | N6, n1236(invert)),
            //
            // 5 ways
            (dir | N1 | N3 | N4 | N5 | N6, n13456(invert)),
            (dir | N1 | N2 | N4 | N5 | N6, n12456(invert)),
            (dir | N2 | N3 | N4 | N5 | N6, n23456(invert)),
            //
            (dir | N1 | N2 | N3 | N4 | N5, n12345(invert)),
            (dir | N1 | N2 | N3 | N4 | N6, n12346(invert)),
            (dir | N1 | N2 | N3 | N5 | N6, n12356(invert)),
            //
            // 6 ways
            (dir | N1 | N3 | N2 | N4 | N5 | N6, n123456(invert)),
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
            (dir | N1 | N4 | A3 | A4, n14_34(invert)),
            (dir | N1 | N4 | A1 | A4 | B4, n14_14_4(invert)),
            (dir | N1 | N4 | A4, n14_4(invert)),
            //
            (dir | N1 | N5 | A2 | A5, n15_25(invert)),
            (dir | N1 | N5 | A3 | A5, n15_35(invert)),
            (dir | N1 | N5 | A1 | A5 | B5, n15_15_5(invert)),
            (dir | N1 | N5 | A1 | A4, n15_14(invert)),
            (dir | N1 | N5 | A5, n15_5(invert)),
            //
            (dir | N2 | N4 | A1 | A4, n24_14(invert)),
            (dir | N2 | N4 | A3 | A4, n24_34(invert)),
            (dir | N2 | N4 | A2 | A4 | B4, n24_24_4(invert)),
            (dir | N2 | N4 | A2 | A6, n24_26(invert)),
            (dir | N2 | N4 | A4, n24_4(invert)),
            //
            (dir | N2 | N6 | A1 | A6, n26_16(invert)),
            (dir | N2 | N6 | A2 | A6 | B6, n26_26_6(invert)),
            (dir | N2 | N6 | A6, n26_6(invert)),
            //
            (dir | N3 | N5 | A2 | A5, n35_25(invert)),
            (dir | N3 | N5 | A3 | A5 | B5, n35_35_5(invert)),
            (dir | N3 | N5 | A5, n35_5(invert)),
            //
            (dir | N3 | N6 | A1 | A6, n36_16(invert)),
            (dir | N3 | N6 | A2 | A6, n36_26(invert)),
            (dir | N3 | N6 | A3 | A6 | B6, n36_36_6(invert)),
            (dir | N3 | N6 | A3 | A5, n36_35(invert)),
            (dir | N3 | N6 | A6, n36_6(invert)),
            // 4-5
            (dir | N4 | N5, n45(invert)),
            (dir | N4 | N5 | A1 | A4, n45_14(invert)),
            (dir | N4 | N5 | A1 | A5, n45_15(invert)),
            (dir | N4 | N5 | A2 | A4 | B3 | B5, n45_24_35(invert)),
            (dir | N4 | N5 | A1 | A4 | B3 | B5, n45_14_35(invert)),
            (dir | N4 | N5 | A2 | A4 | B1 | B5, n45_24_15(invert)),
            (dir | N4 | N5 | A2 | A4, n45_24(invert)),
            (dir | N4 | N5 | A3 | A5, n45_35(invert)),
            // 4-6
            (dir | N4 | N6, a46(invert)),
            (dir | N4 | N6 | A2 | A6, n46_26(invert)),
            (dir | N4 | N6 | A2 | A4, n46_24(invert)),
            (dir | N4 | N6 | A1 | A4 | B3 | B6, n46_14_36(invert)),
            (dir | N4 | N6 | A1 | A4 | B2 | B6, n46_14_26(invert)),
            (dir | N4 | N6 | A2 | A4 | B3 | B6, n46_24_36(invert)),
            (dir | N4 | N6 | A1 | A4, n46_14(invert)),
            (dir | N4 | N6 | A3 | A6, n46_35(invert)),
            // 5-6
            (dir | N5 | N6, n56(invert)),
            (dir | N5 | N6 | A3 | A5, n56_35(invert)),
            (dir | N5 | N6 | A3 | A6, n56_36(invert)),
            (dir | N5 | N6 | A1 | A5 | B2 | B6, n46_15_26(invert)),
            (dir | N5 | N6 | A3 | A5 | B2 | B6, n56_35_26(invert)),
            (dir | N5 | N6 | A1 | A5 | B3 | B6, n56_15_36(invert)),
            (dir | N5 | N6 | A1 | A5, n56_15(invert)),
            (dir | N5 | N6 | A2 | A6, n56_26(invert)),
        ] {
            let sorted_key = sort_upper_bits(key);
            paths.insert(sorted_key.to_usize().unwrap_or_default(), path.clone());

            // Automatically generate Blank fragments
            let key_base = key & NL;
            let key_a = (key >> 6) & NL;
            let key_b = (key >> 12) & NL;
            if key_a != 0 && key_b == 0 {
                let blank_key = ((key_base << 6) | key_a | (key & WA & IN))
                    .to_usize()
                    .unwrap_or_default();
                if let std::collections::btree_map::Entry::Vacant(e) = paths.entry(blank_key) {
                    e.insert("".to_string());
                } else if !paths[&blank_key].is_empty() {
                    println!(
                        "Not adding blank key: {}",
                        print_bits(blank_key.to_usize().unwrap())
                    );
                }
            }
        }
    }

    // Change this to a look up function rather than a class with a HashMap.
    let mut file =
        File::create("../lib/src/renders/generated/fragment.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(b"use ::phf::{Map, phf_map};\n\n");
    let _ = file.write(b"pub static MAP_PATHS: Map<usize, &'static str> = phf_map! {\n");

    let mut keys: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut empty_fragments: HashSet<usize> = HashSet::new();

    for (key, value) in paths {
        // Store to render library
        let key = key.to_usize().unwrap();
        let _ = writeln!(
            file,
            " /* {:<10} */ {}usize =>  \"{}\",",
            print_bits(key),
            key,
            trim_whitespace(value.as_str())
        );

        if value.is_empty() {
            empty_fragments.insert(key);
            continue;
        }

        // Store to documentation library
        let index = key & (IN | NL);

        if !keys.contains_key(&index) {
            keys.entry(index).or_default().push(key);
        } else {
            if !keys[&index].contains(&key) {
                keys.entry(index).or_default().push(key);
            }
        }
    }
    let _ = file.write(b"};\n\n");
    let _ = file.flush();

    println!("Generating variants.rs");
    let mut file = File::create("../document/src/variants.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(
        b"pub fn list_of_variants (mask:usize) -> Vec<usize> {
  match mask {\n",
    );

    let variant_keys = keys.keys().sorted().collect::<Vec<_>>();
    for key in variant_keys {
        let value = keys.get(key).unwrap();
        // Store to render library

        let value = value
            .iter()
            .map(|x| to_6bit_binary(x, false))
            .sorted()
            .unique()
            .join(", ");

        let _ = writeln!(
            file,
            "/* {:<10} */    {} =>  vec![{}],",
            print_bits(*key),
            to_6bit_binary(key, false),
            value,
        );
    }
    let _ = file.write(b"    _ => vec![mask]\n   }\n");
    let _ = file.write(b"}\n\n");
    // Add in empty fragments, by key

    let mut file = File::create("../document/src/empty.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");

    let _ = file.write(
        b"pub fn empty_fragments (mask:usize) -> bool {
  match mask {\n",
    );

    for key in empty_fragments.iter().sorted() {
        let _ = writeln!(
            file,
            " /*{:<10}*/   {} =>  true,",
            print_bits(*key),
            to_6bit_binary(key, false)
        );
    }
    let _ = file.write(b"    _ => false\n   }\n");
    let _ = file.write(b"}\n\n");
    let _ = file.flush();
}
