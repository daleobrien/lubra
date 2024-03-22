use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::Write;

use itertools::Itertools;
use num_traits::ToPrimitive;

use common::bit_utils::to_6bit_binary;
use common::consts::{
  A1, A2, A3, A4, A5, A6, B1, B2, B3, B4, B5, B6, IN, N1, N2, N3, N4, N5, N6, NL, NO, NX, WA,
};

use crate::frag_12::{n12, n13, n23};
use crate::frag_123::n123;
use crate::frag_15::{n14, n15, n24, n26, n35, n36};
use crate::frag_15_dot::{n14_4, n15_5, n24_4, n26_6, n35_5, n36_6};
use crate::frag_15_long::{n14_24, n15_35, n24_14, n26_36, n35_15, n36_26};
use crate::frag_15_straight::{n14_34, n15_25, n24_34, n26_16, n35_25, n36_16};
use crate::frag_16::{n16, n25, n34};
use crate::frag_arc_semi::{n45_14, n45_15, n46_24, n46_26, n56_35, n56_36};
use crate::frag_arcs::{a46, n1, n2, n3, n4, n45, n456, n5, n56, n6};
use crate::frag_complex_arcs::{
  n45_14_35, n45_24, n45_24_15, n45_24_35, n45_35, n46_14, n46_14_26, n46_14_36, n46_15_26,
  n46_24_36, n46_35, n56_15, n56_15_36, n56_26, n56_35_26,
};
use crate::frag_wall::l_wall;

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
            (dir | N1 | A1, "".to_string()),
            (dir | N2 | A2, "".to_string()),
            (dir | N3 | A3, "".to_string()),
            (dir | N4 | A4, "".to_string()),
            (dir | N5 | A5, "".to_string()),
            (dir | N6 | A6, "".to_string()),
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
            (dir | N1 | N6 | A2 | A6, "".to_string()),
            (dir | N1 | N6 | A3 | A6, "".to_string()),
            (dir | N2 | N5 | A1 | A5, "".to_string()),
            (dir | N2 | N5 | A3 | A5, "".to_string()),
            (dir | N3 | N4 | A1 | A4, "".to_string()),
            (dir | N3 | N4 | A2 | A4, "".to_string()),
            //
            (dir | N1 | N4 | A2 | A4, n14_24(invert)),
            (dir | N1 | N4 | A3 | A4, n14_34(invert)),
            (dir | N1 | N4 | A4 | A5, "".to_string()), // Equivalent to 4-5/1-4
            (dir | N1 | N4 | A4 | A6, "".to_string()), // Equivalent to 4-6/1-4
            (dir | N1 | N4 | A4, n14_4(invert)),
            //
            (dir | N1 | N5 | A2 | A5, n15_25(invert)),
            (dir | N1 | N5 | A3 | A5, n15_35(invert)),
            (dir | N1 | N5 | A4 | A5, "".to_string()), //
            (dir | N1 | N5 | A5 | A6, "".to_string()), //
            (dir | N1 | N5 | A5, n15_5(invert)),
            //
            (dir | N2 | N4 | A1 | A4, n24_14(invert)),
            (dir | N2 | N4 | A3 | A4, n24_34(invert)),
            (dir | N2 | N4 | A4 | A5, "".to_string()), //
            (dir | N2 | N4 | A4 | A6, "".to_string()), //
            (dir | N2 | N4 | A4, n24_4(invert)),
            //
            (dir | N2 | N6 | A1 | A6, n26_16(invert)),
            (dir | N2 | N6 | A3 | A6, n26_36(invert)),
            (dir | N2 | N6 | A4 | A6, "".to_string()), //
            (dir | N2 | N6 | A5 | A6, "".to_string()), //
            (dir | N2 | N6 | A6, n26_6(invert)),
            //
            (dir | N3 | N5 | A1 | A5, n35_15(invert)),
            (dir | N3 | N5 | A2 | A5, n35_25(invert)),
            (dir | N3 | N5 | A4 | A5, "".to_string()), //
            (dir | N3 | N5 | A5 | A6, "".to_string()), //
            (dir | N3 | N5 | A5, n35_5(invert)),
            //
            (dir | N3 | N6 | A1 | A6, n36_16(invert)),
            (dir | N3 | N6 | A2 | A6, n36_26(invert)),
            (dir | N3 | N6 | A4 | A6, "".to_string()), //
            (dir | N3 | N6 | A5 | A6, "".to_string()), // Equivalent to 5-6/3-6
            (dir | N3 | N6 | A6, n36_6(invert)),
            // 4-5
            (dir | N4 | N5, n45(invert)),
            (dir | N4 | N5 | A4, n45(invert) + &*n4(invert)),
            (dir | N4 | N5 | A5, n45(invert) + &*n5(invert)),
            (dir | N4 | N5 | A1 | A4, n45_14(invert)),
            (dir | N4 | N5 | A1 | A5, n45_15(invert)),
            (dir | N4 | N5 | A2 | A4 | B3 | B5, n45_24_35(invert)),
            (dir | N4 | N5 | A1 | A4 | B3 | B5, n45_14_35(invert)),
            //
            (dir | N4 | N5 | A2 | A4 | B1 | B5, n45_24_15(invert)),
            (dir | N4 | N5 | A1 | A5 | B2 | B4, n45_24_15(invert)), // Reverse of above
            //
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
            // TODO: Make routine to sort nibbles, except the lowest nibble
            // Insert all combinations of the key with the adjacent nets permeated
            let upper_a = (key >> 6) & NL;
            let upper_b = (key) >> 12 & NL;

            if upper_b != 0 && upper_a.trailing_zeros() > upper_b.trailing_zeros() {
                // Keep the adj bits sorted
                let swapped = (upper_a << 12) | (upper_b << 6) | (key & NL) | (key & !NX);
                paths.insert(swapped.to_usize().unwrap_or_default(), path.clone());
            } else {
                paths.insert(key.to_usize().unwrap_or_default(), path.clone());
            }
        }
    }

    // Change this to a look up function rather than a class with a HashMap.
    let mut file = File::create("../lib/src/renders/fragment.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(b"use ::phf::{Map, phf_map};\n\n");
    let _ = file.write(b"pub static MAP_PATHS: Map<u32, &'static str> = phf_map! {\n");

    let mut keys: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut empty_fragments: HashSet<u32> = HashSet::new();

    for (key, value) in paths {
        // Store to render library
        let key_as_u32 = key.to_u32().unwrap();
        let _ = write!(
            file,
            " /* {} */ {}u32 =>  \"{}\",\n",
            to_6bit_binary(&key_as_u32, false),
            key,
            trim_whitespace(value.as_str())
        );

        if value.is_empty() {
            empty_fragments.insert(key_as_u32);
            continue;
        }

        // Store to documentation library
        let index = key_as_u32 & (IN | N1 | N2 | N3 | N4 | N5 | N6);

        if !keys.contains_key(&index) {
            keys.entry(index).or_insert_with(Vec::new).push(key_as_u32);
        } else {
            if !keys[&index].contains(&key_as_u32) {
                keys.entry(index).or_insert_with(Vec::new).push(key_as_u32);
            }
        }
    }
    let _ = file.write(b"};\n\n");
    let _ = file.flush();

    println!("Generating variants.rs");
    let mut file = File::create("../document/src/variants.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");
    let _ = file.write(
        b"pub fn list_of_variants (mask:u32) -> Vec<u32> {
  match mask {\n",
    );

    let variant_keys = keys.keys().sorted().collect::<Vec<_>>();
    for key in variant_keys {
        let value = keys.get(&key).unwrap();
        // Store to render library

        let value = value
            .iter()
            .map(|x| to_6bit_binary(x, false))
            .sorted()
            .unique()
            .join(", ");

        let _ = write!(
            file,
            "    {} =>  vec![{}],\n",
            to_6bit_binary(&key, false),
            value,
        );
    }
    let _ = file.write(b"    _ => vec![mask]\n   }\n");
    let _ = file.write(b"}\n\n");
    // Add in empty fragments, by key

    let mut file = File::create("../document/src/empty.rs").expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");

    let _ = file.write(
        b"pub fn empty_fragments (mask:u32) -> bool {
  match mask {\n",
    );

    for key in empty_fragments.iter().sorted() {
        let _ = write!(file, "    {} =>  true,\n", to_6bit_binary(&key, false));
    }
    let _ = file.write(b"    _ => false\n   }\n");
    let _ = file.write(b"}\n\n");
    let _ = file.flush();
}
