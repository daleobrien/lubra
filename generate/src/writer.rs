use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

// The lookup functions return `&'static` slices of constant data. Building nested
// `Vec`s inside a match this large needs a stack frame bigger than a 2 MiB thread
// stack in debug builds.

/// `[1, 2]` -> `&[1, 2]`
fn to_rust_slice<T: ToString>(list: &[T]) -> String {
    let elements: Vec<String> = list.iter().map(|n| n.to_string()).collect();
    format!("&[{}]", elements.join(", "))
}

/// `[[3], [4, 5]]` -> `&[&[3], &[4, 5]]`
fn to_rust_slice_of_slices<T: ToString>(list: &[Vec<T>]) -> String {
    let elements: Vec<String> = list.iter().map(|item| to_rust_slice(item)).collect();
    format!("&[{}]", elements.join(", "))
}

pub fn write_nets_to_file(nets_by_mask: HashMap<usize, Vec<Vec<usize>>>, file_name: &str) {
    let mut keys = nets_by_mask.keys().collect::<Vec<&usize>>();
    keys.sort();
    let mut file = File::create(file_name).expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");

    let _ = file.write_all(
        b"pub fn list_of_possible_nets_for_a_mask (base4_mask:usize) -> &'static [&'static [usize]] {\n",
    );
    let _ = file.write_all(b"  match base4_mask {\n");
    for key in keys {
        let content = format!(
            "    {:6?} => {},\n",
            key,
            to_rust_slice_of_slices(&nets_by_mask[key])
        );
        let _ = file.write(content.as_bytes());
    }
    let _ = file.write_all(b"    _ => &[&[]],\n");
    let _ = file.write_all(b"  }\n");
    let _ = file.write_all(b"}\n");
    let _ = file.flush();
}

pub fn write_mass_to_file(mass_by_mask: HashMap<usize, Vec<usize>>, file_name: &str) {
    let mut keys = mass_by_mask.keys().collect::<Vec<&usize>>();
    keys.sort();
    let mut file = File::create(file_name).expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");

    let _ = file.write_all(
        b"pub fn list_of_possible_mass_for_a_mask (base4_mask:usize) -> &'static [u32] {\n",
    );
    let _ = file.write_all(b"  match base4_mask {\n");
    for key in keys {
        // Keep the order of `nets_by_mask[key]`: entry n is the weight of net set n.
        let content = format!("    {:6?} => &{:?},\n", key, mass_by_mask[key]);
        let _ = file.write(content.as_bytes());
    }
    let _ = file.write_all(b"    _ => &[],\n");
    let _ = file.write_all(b"  }\n");
    let _ = file.write_all(b"}\n");
    let _ = file.flush();
}

#[cfg(test)]
mod tests {
    use crate::writer::{to_rust_slice, to_rust_slice_of_slices};

    #[test]
    fn test_to_rust_slice() {
        assert_eq!(to_rust_slice(&[1u32, 2u32, 3u32]), "&[1, 2, 3]");
        assert_eq!(to_rust_slice::<u32>(&[]), "&[]");
    }

    #[test]
    fn test_to_rust_slice_of_slices() {
        assert_eq!(
            to_rust_slice_of_slices(&[vec![], vec![3u32], vec![4u32, 5u32, 6u32]]),
            "&[&[], &[3], &[4, 5, 6]]"
        );
    }
}
