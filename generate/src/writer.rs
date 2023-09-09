use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn list_to_rust_vector<T: ToString>(list: &[T]) -> String {
    let elements: Vec<String> = list.iter().map(|item| item.to_string()).collect();
    return format!("vec![{}]", elements.join(", "));
}

fn list_of_lists_to_rust_vector<T: ToString>(list: &[Vec<T>]) -> String {
    let elements: Vec<String> = list.iter().map(|item| list_to_rust_vector(item)).collect();
    return list_to_rust_vector(&elements);
}

fn list_of_lists_of_list_to_rust_vector<T: ToString>(list: &[Vec<Vec<T>>]) -> String {
    let elements: Vec<String> = list
        .iter()
        .map(|item| list_of_lists_to_rust_vector(item))
        .collect();
    return list_to_rust_vector(&elements);
}

pub fn write_nets_to_file(nets_by_mask: HashMap<u32, Vec<Vec<u32>>>, file_name: &str) {
    let mut keys = nets_by_mask.keys().collect::<Vec<&u32>>();
    keys.sort();
    let mut file = File::create(file_name).expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");

    let _ = file.write_all(
        b"pub fn list_of_possible_nets_for_a_mask (base4_mask:u32) -> Vec<Vec<Vec<u32>>> {\n",
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

pub fn write_mass_to_file(mass_by_mask: HashMap<u32, Vec<u32>>, file_name: &str) {
    let mut keys = mass_by_mask.keys().collect::<Vec<&u32>>();
    keys.sort();
    let mut file = File::create(file_name).expect("File Creation failed");
    let _ = file.write(b"/* This file is generated. */\n");

    let _ =
        file.write_all(b"pub fn list_of_possible_mass_for_a_mask (base4_mask:u32) -> Vec<u32> {\n");
    let _ = file.write_all(b"  match base4_mask {\n");
    for key in keys {
        let content = format!("    {:6?} => vec!{:?},\n", key, &mass_by_mask[key]);
        let _ = file.write(content.as_bytes());
    }
    let _ = file.write_all(b"    _ => vec![],\n");
    let _ = file.write_all(b"  }\n");
    let _ = file.write_all(b"}\n");
    let _ = file.flush();
}

#[cfg(test)]
mod tests {
    use crate::writer::{
        list_of_lists_of_list_to_rust_vector, list_of_lists_to_rust_vector, list_to_rust_vector,
    };
    use std::{assert_eq, vec};

    #[test]
    fn test_list_to_rust_vector() {
        assert_eq!(
            list_to_rust_vector(&vec![1u32, 2u32, 3u32]),
            "vec![1, 2, 3]"
        );
        assert_eq!(list_to_rust_vector(&vec!["a", "b", "c"]), "vec![a, b, c]");
    }

    #[test]
    fn test_list_of_lists_to_rust_vector() {
        assert_eq!(
            list_of_lists_to_rust_vector(&vec![vec![1, 2], vec![4, 5, 6]]),
            "vec![vec![1, 2], vec![4, 5, 6]]"
        );
    }

    #[test]
    fn test_list_of_lists_of_list_to_rust_vector() {
        assert_eq!(
            list_of_lists_of_list_to_rust_vector(&vec![vec![vec![1, 2], vec![4, 5, 6]]]),
            "vec![vec![vec![1, 2], vec![4, 5, 6]]]"
        );
    }
}
