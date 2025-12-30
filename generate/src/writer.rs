use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn list_to_rust_vector_usize<T: ToString>(list: &[T]) -> String {
    let elements: Vec<String> = list
        .iter()
        .map(|n| format!("{}", n.to_string()))
        .collect();
    return format!("vec![{}]", elements.join(", "));
}

fn list_to_rust_vector_vec<T: ToString>(list: &[T]) -> String {
    let elements: Vec<String> = list.iter().map(|item| item.to_string()).collect();
    format!("vec![{}]", elements.join(", "))
}

fn list_of_lists_to_rust_vector<T: ToString>(list: &[Vec<T>]) -> String {
    let elements: Vec<String> = list
        .iter()
        .map(|item| list_to_rust_vector_usize(item))
        .collect();
    list_to_rust_vector_vec(&elements)
}

pub fn write_nets_to_file(nets_by_mask: HashMap<usize, Vec<Vec<usize>>>, file_name: &str) {
    let mut keys = nets_by_mask.keys().collect::<Vec<&usize>>();
    keys.sort();
    let mut file = File::create(file_name).expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");

    let _ = file.write_all(
        b"pub fn list_of_possible_nets_for_a_mask (base4_mask:usize) -> Vec<Vec<usize>> {\n",
    );
    let _ = file.write_all(b"  match base4_mask {\n");
    for key in keys {
        let content = format!(
            "    {:6?} => {},\n",
            key,
            list_of_lists_to_rust_vector(&nets_by_mask[key])
        );
        let _ = file.write(content.as_bytes());
    }
    let _ = file.write_all(b"    _ => vec![vec![]],\n");
    let _ = file.write_all(b"  }\n");
    let _ = file.write_all(b"}\n");
    let _ = file.flush();
}

pub fn write_mass_to_file(mass_by_mask: HashMap<usize, Vec<usize>>, file_name: &str) {
    let mut keys = mass_by_mask.keys().collect::<Vec<&usize>>();
    keys.sort();
    let mut file = File::create(file_name).expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");

    let _ =
        file.write_all(b"pub fn list_of_possible_mass_for_a_mask (base4_mask:usize) -> Vec<u32> {\n");
    let _ = file.write_all(b"  match base4_mask {\n");
    for key in keys {
        let mut mass = mass_by_mask[key].clone();
        mass.sort();
        let content = format!("    {:6?} => vec!{:?},\n", key, mass);
        let _ = file.write(content.as_bytes());
    }
    let _ = file.write_all(b"    _ => vec![],\n");
    let _ = file.write_all(b"  }\n");
    let _ = file.write_all(b"}\n");
    let _ = file.flush();
}

#[cfg(test)]
mod tests {
  use std::{assert_eq, vec};

  use crate::writer::{list_of_lists_to_rust_vector, list_to_rust_vector_usize};

  #[test]
    fn test_list_to_rust_vector_usize() {
        assert_eq!(
            list_to_rust_vector_usize(&vec![1u32, 2u32, 3u32]),
            "vec![1, 2, 3]"
        );
    }

    #[test]
    fn test_list_of_lists_to_rust_vector() {
        assert_eq!(
            list_of_lists_to_rust_vector(&vec![vec![3u32], vec![4u32, 5u32, 6u32]]),
            "vec![vec![3], vec![4, 5, 6]]"
        );
    }
}
