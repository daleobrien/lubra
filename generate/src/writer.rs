use std::collections::HashMap;

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

pub fn write_to_file(nets_by_mask: HashMap<u32, Vec<Vec<Vec<u32>>>>) {
    let mut keys = nets_by_mask.keys().collect::<Vec<&u32>>();
    keys.sort();
    println!("match mask {{");
    for key in keys {
        print!(
            "{:06?} => {:?}\n",
            key,
            list_of_lists_of_list_to_rust_vector(&nets_by_mask[key])
        )
    }
    println!("}}");
}

#[cfg(test)]
mod tests {
    use crate::writer::{
        list_of_lists_of_list_to_rust_vector, list_of_lists_to_rust_vector, list_to_rust_vector,
    };

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
